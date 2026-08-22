#!/usr/bin/env python3
"""
claude_cli_model.py

Drop-in replacement for prompt_engineering.ClaudeModel that reaches the model
through the `claude -p` CLI (Claude Code, subscription auth) instead of the
`anthropic` Python SDK.  Same duck-typed interface the harness already relies
on -- `generate(messages, temperature=None) -> str` -- so it substitutes
anywhere ClaudeModel / QwenLocalModel is used.

Why a CLI backend at all: the subscription plan's rolling 5-hour usage limit
is the binding constraint on a ~1000-call evaluation.  This wrapper treats
hitting that limit as *normal operation*: it detects the limit, sleeps until
the reported reset (or a backoff), probes with a cheap call, and then retries
the very same request.  Combined with the on-disk response cache it means the
whole evaluation can be interrupted at any point (quota, crash, reboot) and
resumed at exact call granularity.

Usage:
    from claude_cli_model import ClaudeCLIModel
    model = ClaudeCLIModel(model="claude-opus-5", effort="high",
                           cache_dir=Path("results/ab_test/v3_opus5/.cli_cache"),
                           ledger_path=Path("results/ab_test/v3_opus5/cli_calls.jsonl"))
    text = model.generate([{"role": "system", ...}, {"role": "user", ...}])
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, List, Optional


DEFAULT_MODEL = "claude-opus-5"
DEFAULT_EFFORT = "high"

# Tools are disabled outright: this is a single-shot generation call, not an
# agent loop.  --safe-mode additionally drops CLAUDE.md / skills / plugins /
# hooks / MCP so the prompt is exactly what we pass (auth still works normally,
# unlike --bare which would force ANTHROPIC_API_KEY and bypass the plan).
_DISALLOWED_TOOLS = "Bash Edit Write Read Glob Grep WebFetch WebSearch Task NotebookEdit TodoWrite"

_QUOTA_PATTERNS = (
    r"usage limit reached",
    r"rate.?limit",
    r"limit will reset",
    r"exceeded your .* limit",
    r"quota",
    r"too many requests",
    # Observed live: "You've hit your session limit - resets 7am (America/New_York)".
    # This matched none of the patterns above and was only caught because the
    # CLI happened to also set api_error_status=429 -- do not rely on that.
    r"hit your .{0,20}limit",
    r"session limit",
    r"reset[a-z]*\s+(?:at\s+)?\d{1,2}(?::\d{2})?\s*(?:am|pm)",
)

# "Claude AI usage limit reached|1755561600"
_EPOCH_RE = re.compile(r"(?:reached|reset[^0-9]{0,20})\|?\s*(1[0-9]{9})")
# Observed formats:
#   "You've hit your session limit · resets 7am (America/New_York)"
#   "You've hit your session limit · resets 1:40am (America/New_York)"
#   "...your limit will reset at 3:00pm"
# The "at" is optional -- the live CLI message omits it.
_CLOCK_RE = re.compile(
    r"reset[a-z]*\s+(?:at\s+)?(\d{1,2})(?::(\d{2}))?\s*(am|pm)?\b"
    r"(?:\s*\(([A-Za-z_]+/[A-Za-z_]+)\))?",
    re.I,
)

# Backoff ladder (seconds) when the reset time is not parseable.
_BLIND_BACKOFF = [10 * 60, 15 * 60, 20 * 60, 30 * 60]
# Longest single sleep before re-probing (s).
_MAX_SLEEP_CHUNK = 2 * 60 * 60
# Server-overload (529) retries back off to this cap and then persist.
_OVERLOAD_MAX_BACKOFF = 10 * 60
# Transient (network / parse) retry ladder, mirroring ClaudeModel.generate.
_TRANSIENT_BACKOFF = [15, 30, 60, 120, 300, 300, 600, 600, 900, 900]


class QuotaExhausted(Exception):
    """Raised internally when a call fails because of the plan's usage limit."""

    def __init__(self, message: str, reset_epoch: Optional[float] = None):
        super().__init__(message)
        self.reset_epoch = reset_epoch


def _now() -> float:
    return time.time()


def _iso(ts: float) -> str:
    return datetime.fromtimestamp(ts, tz=timezone.utc).isoformat()


def _looks_like_quota(text: str) -> bool:
    low = (text or "").lower()
    return any(re.search(p, low) for p in _QUOTA_PATTERNS)


def parse_reset_epoch(text: str, now: Optional[float] = None) -> Optional[float]:
    """Best-effort extraction of when the usage limit resets."""
    if not text:
        return None
    now = now if now is not None else _now()

    m = _EPOCH_RE.search(text)
    if m:
        epoch = float(m.group(1))
        # Sanity: within the next 24h (and not in the past by more than an hour).
        if now - 3600 <= epoch <= now + 24 * 3600:
            return epoch

    # Bare epoch after a pipe, e.g. "...reached|1755561600"
    for cand in re.findall(r"\|(\d{10})\b", text):
        epoch = float(cand)
        if now - 3600 <= epoch <= now + 24 * 3600:
            return epoch

    m = _CLOCK_RE.search(text)
    if m:
        hour = int(m.group(1))
        minute = int(m.group(2) or 0)
        ampm = (m.group(3) or "").lower()
        tzname = m.group(4)
        if ampm == "pm" and hour < 12:
            hour += 12
        if ampm == "am" and hour == 12:
            hour = 0

        tz = None
        if tzname:
            try:
                from zoneinfo import ZoneInfo

                tz = ZoneInfo(tzname)
            except Exception:
                tz = None

        base = datetime.fromtimestamp(now, tz=tz)
        target = base.replace(hour=hour % 24, minute=minute, second=0, microsecond=0)
        epoch = target.timestamp()
        if epoch <= now:
            # Already past: the window should be open, so let the caller use a
            # short blind backoff + probe rather than rolling forward a day.
            return None
        if epoch - now <= 6 * 3600:
            return epoch

    return None


def classify_cli_result(returncode: int, stdout: str, stderr: str) -> Dict[str, Any]:
    """Classify one `claude -p --output-format json` invocation.

    Returns {"kind": "success"|"quota"|"transient", "text": str, "payload": dict|None,
             "reset_epoch": float|None, "detail": str}
    """
    payload: Optional[Dict[str, Any]] = None
    out = (stdout or "").strip()
    if out:
        try:
            payload = json.loads(out)
        except Exception:
            # stream of JSON lines or partial output: try the last complete object
            for line in reversed(out.splitlines()):
                line = line.strip()
                if line.startswith("{") and line.endswith("}"):
                    try:
                        payload = json.loads(line)
                        break
                    except Exception:
                        continue

    combined = f"{out}\n{stderr or ''}"

    if payload is not None:
        result_text = payload.get("result") or ""
        is_error = bool(payload.get("is_error")) or payload.get("subtype") not in (None, "success")
        api_status = payload.get("api_error_status")
        blob = f"{result_text}\n{stderr or ''}"
        if api_status == 529 or "overloaded" in blob.lower():
            return {
                "kind": "transient",
                "text": "",
                "payload": payload,
                "reset_epoch": None,
                "detail": (result_text or "529 overloaded")[:400],
            }
        if api_status == 429 or (is_error and _looks_like_quota(blob)):
            return {
                "kind": "quota",
                "text": "",
                "payload": payload,
                "reset_epoch": parse_reset_epoch(blob),
                "detail": (result_text or f"api_error_status={api_status}")[:400],
            }
        if returncode == 0 and not is_error and isinstance(result_text, str) and result_text.strip():
            return {
                "kind": "success",
                "text": result_text.strip(),
                "payload": payload,
                "reset_epoch": None,
                "detail": "",
            }
        return {
            "kind": "transient",
            "text": "",
            "payload": payload,
            "reset_epoch": None,
            "detail": (result_text or payload.get("subtype") or "empty result")[:400],
        }

    # No parseable JSON at all.
    if _looks_like_quota(combined):
        return {
            "kind": "quota",
            "text": "",
            "payload": None,
            "reset_epoch": parse_reset_epoch(combined),
            "detail": combined.strip()[:400],
        }
    return {
        "kind": "transient",
        "text": "",
        "payload": None,
        "reset_epoch": None,
        "detail": (combined.strip() or f"exit={returncode}, no output")[:400],
    }


class ClaudeCLIModel:
    """`claude -p` backed generator with quota suspend/probe/resume + disk cache."""

    def __init__(
        self,
        model: str = DEFAULT_MODEL,
        effort: str = DEFAULT_EFFORT,
        cache_dir: Optional[Path] = None,
        ledger_path: Optional[Path] = None,
        stage: str = "generate",
        timeout: int = 1800,
        cli_bin: str = "claude",
        throttle: float = 1.0,
    ):
        self.name = model or DEFAULT_MODEL
        self.effort = effort or DEFAULT_EFFORT
        self.cli_bin = cli_bin
        self.timeout = timeout
        self.throttle = throttle
        self.stage = stage
        self.call_count = 0
        self.cached_count = 0
        self.quota_waits = 0
        self.quota_wait_seconds = 0.0
        self.total_cost_usd = 0.0
        # Set by callers so the ledger says which command a call belongs to.
        self.current_command = ""
        self.current_attempt = 0

        self.cache_dir = Path(cache_dir) if cache_dir else None
        if self.cache_dir:
            self.cache_dir.mkdir(parents=True, exist_ok=True)
        self.ledger_path = Path(ledger_path) if ledger_path else None
        if self.ledger_path:
            self.ledger_path.parent.mkdir(parents=True, exist_ok=True)

        # How many times this process has already consumed each prompt hash.
        # n_samples=5 sends an identical prompt 5x, so the k-th repeat must get
        # the k-th cached response, not the 1st.
        self._consumed: Dict[str, int] = {}

    # ---------------------------------------------------------------- cache

    def _key(self, system: str, user: str) -> str:
        h = hashlib.sha256()
        for part in (self.name, self.effort, system, user):
            h.update(part.encode("utf-8", errors="replace"))
            h.update(b"\x00")
        return h.hexdigest()

    def _cache_path(self, key: str) -> Optional[Path]:
        return self.cache_dir / f"{key}.json" if self.cache_dir else None

    def _cache_load(self, key: str) -> List[str]:
        p = self._cache_path(key)
        if not p or not p.exists():
            return []
        try:
            d = json.loads(p.read_text(encoding="utf-8"))
            return list(d.get("responses") or [])
        except Exception:
            return []

    def _cache_append(self, key: str, system: str, user: str, response: str) -> None:
        p = self._cache_path(key)
        if not p:
            return
        responses = self._cache_load(key)
        responses.append(response)
        payload = {
            "model": self.name,
            "effort": self.effort,
            "system_sha": hashlib.sha256(system.encode("utf-8", "replace")).hexdigest(),
            "user_sha": hashlib.sha256(user.encode("utf-8", "replace")).hexdigest(),
            "responses": responses,
        }
        tmp = p.with_suffix(".json.tmp")
        tmp.write_text(json.dumps(payload), encoding="utf-8")
        os.replace(tmp, p)

    # --------------------------------------------------------------- ledger

    def _log(self, **fields: Any) -> None:
        if not self.ledger_path:
            return
        row = {
            "ts": _iso(_now()),
            "stage": self.stage,
            "command": self.current_command,
            "attempt": self.current_attempt,
            "model": self.name,
            "effort": self.effort,
        }
        row.update(fields)
        try:
            with self.ledger_path.open("a", encoding="utf-8") as fh:
                fh.write(json.dumps(row) + "\n")
        except Exception:
            pass

    # ------------------------------------------------------------------ cli

    def _run_cli(self, system: str, user: str, effort: Optional[str], timeout: int) -> Dict[str, Any]:
        cmd = [
            self.cli_bin,
            "-p",
            "--model", self.name,
            "--output-format", "json",
            "--no-session-persistence",
            "--safe-mode",
            "--strict-mcp-config",
            "--permission-mode", "dontAsk",
            "--disallowed-tools", _DISALLOWED_TOOLS,
            "--system-prompt", system,
        ]
        if effort:
            cmd += ["--effort", effort]

        started = _now()
        try:
            proc = subprocess.run(
                cmd,
                input=user,
                capture_output=True,
                text=True,
                timeout=timeout,
            )
            rc, out, err = proc.returncode, proc.stdout, proc.stderr
        except subprocess.TimeoutExpired:
            rc, out, err = -1, "", f"timeout after {timeout}s"
        except FileNotFoundError as e:
            raise SystemExit(f"`{self.cli_bin}` not found on PATH: {e}")

        res = classify_cli_result(rc, out, err)
        res["duration_s"] = round(_now() - started, 2)
        res["returncode"] = rc
        return res

    def _probe(self) -> bool:
        """Cheap call used to detect that the usage window has reopened."""
        res = self._run_cli(
            system="Reply with the single word OK.",
            user="OK",
            effort=None,
            timeout=180,
        )
        self._log(event="probe", kind=res["kind"], duration_s=res["duration_s"],
                  detail=res.get("detail", ""))
        return res["kind"] == "success"

    def _wait_for_quota(self, reset_epoch: Optional[float], blind_idx: int) -> float:
        """Sleep until the plan's window reopens, then probe until it really has."""
        waited = 0.0
        if reset_epoch:
            delay = max(60.0, reset_epoch - _now() + 60.0)
            reason = f"reported reset at {_iso(reset_epoch)}"
        else:
            delay = float(_BLIND_BACKOFF[min(blind_idx, len(_BLIND_BACKOFF) - 1)])
            reason = "no reset time reported; blind backoff"

        while True:
            # Never sleep more than one chunk without probing: a mis-parsed or
            # over-long reset time should cost one extra probe, not hours.
            delay = min(delay, _MAX_SLEEP_CHUNK)
            self._log(event="quota_wait", sleep_s=round(delay), reason=reason)
            print(
                f"    [quota] usage limit hit ({reason}); sleeping {delay / 60:.1f} min "
                f"-> retry at {_iso(_now() + delay)}",
                flush=True,
            )
            time.sleep(delay)
            waited += delay
            if self._probe():
                print(f"    [quota] window reopened after {waited / 60:.1f} min", flush=True)
                self.quota_waits += 1
                self.quota_wait_seconds += waited
                return waited

            # Still limited.  If the reported reset is still ahead of us, keep
            # aiming at it rather than falling back to the blind ladder -- the
            # chunk cap must not discard a reset time we already know.
            if reset_epoch and _now() < reset_epoch:
                delay = max(60.0, reset_epoch - _now() + 60.0)
                reason = f"still before reported reset at {_iso(reset_epoch)}"
            else:
                blind_idx += 1
                delay = float(_BLIND_BACKOFF[min(blind_idx, len(_BLIND_BACKOFF) - 1)])
                reason = "probe still limited; blind backoff"

    # ------------------------------------------------------------- generate

    def generate(self, messages: List[Dict[str, str]], temperature: float = None) -> str:
        """Generate one completion.

        `temperature` is accepted for interface compatibility and ignored --
        the CLI exposes no sampling controls.  (Note this was already inert on
        Opus via the SDK path: ClaudeModel.supports_sampling_params is False
        for every non-Haiku model.)  Callers that relied on temperature to
        break stuck retry cycles should vary the prompt text instead.
        """
        system = "\n\n".join(m["content"] for m in messages if m.get("role") == "system").strip()
        user = "\n\n".join(m["content"] for m in messages if m.get("role") != "system").strip()

        key = self._key(system, user)
        idx = self._consumed.get(key, 0)
        cached = self._cache_load(key)
        if idx < len(cached):
            self._consumed[key] = idx + 1
            self.cached_count += 1
            self._log(event="cache_hit", key=key, sample_index=idx, chars=len(cached[idx]))
            return cached[idx]

        transient_idx = 0
        blind_idx = 0
        overload_idx = 0
        while True:
            res = self._run_cli(system, user, self.effort, self.timeout)

            if res["kind"] == "success":
                self.call_count += 1
                self._consumed[key] = idx + 1
                self._cache_append(key, system, user, res["text"])
                payload = res.get("payload") or {}
                usage = payload.get("usage") or {}
                cost = float(payload.get("total_cost_usd") or 0.0)
                self.total_cost_usd += cost
                self._log(
                    event="call",
                    key=key,
                    sample_index=idx,
                    duration_s=res["duration_s"],
                    cost_usd=cost,
                    input_tokens=usage.get("input_tokens"),
                    output_tokens=usage.get("output_tokens"),
                    thinking_tokens=(usage.get("output_tokens_details") or {}).get("thinking_tokens"),
                    cache_read_input_tokens=usage.get("cache_read_input_tokens"),
                    cache_creation_input_tokens=usage.get("cache_creation_input_tokens"),
                    model_usage=payload.get("modelUsage"),
                    chars=len(res["text"]),
                )
                if self.throttle:
                    time.sleep(self.throttle)
                return res["text"]

            if res["kind"] == "quota":
                self._log(event="quota_hit", detail=res.get("detail", ""),
                          reset_epoch=res.get("reset_epoch"), duration_s=res["duration_s"])
                self._wait_for_quota(res.get("reset_epoch"), blind_idx)
                blind_idx = 0
                # Retry the same request; do NOT count this as a transient failure.
                continue

            # transient
            self._log(event="transient_error", detail=res.get("detail", ""),
                      returncode=res.get("returncode"), duration_s=res["duration_s"])

            if "overloaded" in (res.get("detail", "") or "").lower():
                overload_idx += 1
                delay = float(min(60 * (2 ** min(overload_idx, 3)), _OVERLOAD_MAX_BACKOFF))
                print(f"    [overload] server 529 (attempt {overload_idx}); "
                      f"retry in {delay / 60:.0f} min", flush=True)
                time.sleep(delay)
                continue

            if transient_idx >= len(_TRANSIENT_BACKOFF):
                print(f"    [warn] giving up after transient errors: {res.get('detail', '')}", flush=True)
                self._log(event="give_up", detail=res.get("detail", ""))
                return "ERROR"
            delay = _TRANSIENT_BACKOFF[transient_idx]
            transient_idx += 1
            print(f"    [warn] transient CLI failure ({res.get('detail', '')[:120]}); "
                  f"retry in {delay}s", flush=True)
            time.sleep(delay)

    def summary(self) -> Dict[str, Any]:
        return {
            "model": self.name,
            "effort": self.effort,
            "calls": self.call_count,
            "cache_hits": self.cached_count,
            "quota_waits": self.quota_waits,
            "quota_wait_seconds": round(self.quota_wait_seconds),
            "total_cost_usd": round(self.total_cost_usd, 4),
        }


def build_cli_model_for_results(
    results_root: Path,
    model: str = DEFAULT_MODEL,
    effort: str = DEFAULT_EFFORT,
    stage: str = "generate",
) -> ClaudeCLIModel:
    """Convention: cache + ledger live next to the run's results directory."""
    results_root = Path(results_root)
    return ClaudeCLIModel(
        model=model,
        effort=effort,
        cache_dir=results_root / ".cli_cache",
        ledger_path=results_root / "cli_calls.jsonl",
        stage=stage,
    )


if __name__ == "__main__":
    # Tiny manual smoke test: python3 claude_cli_model.py "say hi"
    prompt = sys.argv[1] if len(sys.argv) > 1 else "Output the word: PROBE"
    m = ClaudeCLIModel(cache_dir=Path("/tmp/cli_cache_smoke"), ledger_path=Path("/tmp/cli_smoke.jsonl"))
    print(m.generate([
        {"role": "system", "content": "You output only what is requested."},
        {"role": "user", "content": prompt},
    ]))
    print(m.summary())
