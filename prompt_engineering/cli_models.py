#!/usr/bin/env python3
"""Subscription-CLI model backends for the prompt-engineering harness.

`ClaudeHaikuModel` (prompt_engineering.py) talks to the Anthropic API with an API
key. These two backends instead shell out to the locally authenticated `codex`
and `claude` CLIs, so the Baseline 1 general-model runs bill against the user's
subscription rather than an API key.

Both expose the same duck type as `ClaudeHaikuModel`:

    model.generate([{"role": "system", ...}, {"role": "user", ...}]) -> str

Asymmetry worth knowing when reading the results: `claude -p --tools ""` is a
clean single-turn completion with our system prompt installed as *the* system
prompt, while `codex exec` wraps the request in its own agent harness (its
system prompt is not replaceable), so our system prompt is prepended to the user
message instead. The user-visible instructions are identical; the scaffolding
around them is not.
"""

import json
import os
import re
import subprocess
import tempfile
import time
from pathlib import Path
from typing import Dict, List, Optional

DEFAULT_TIMEOUT = 600

# Substrings that mean "you are out of quota", not "this request failed".
QUOTA_PATTERNS = (
    "usage limit",
    "usage_limit",
    "rate limit",
    "rate_limit",
    "quota",
    "429",
    "insufficient_quota",
    "resets at",
    "upgrade to",
)

SPEC_FN_RE = re.compile(r"pub\s+open\s+spec\s+fn\s+\w+\s*\(")


class QuotaExhausted(RuntimeError):
    """Raised when the CLI reports a subscription/rate limit rather than an error."""


def _looks_like_quota_error(text: str) -> bool:
    low = (text or "").lower()
    return any(p in low for p in QUOTA_PATTERNS)


def _strip_fences(text: str) -> str:
    """Drop markdown code fences, keeping the fenced content."""
    if "```" not in text:
        return text.strip()
    blocks = re.findall(r"```(?:[a-zA-Z_+-]*)\n(.*?)```", text, re.DOTALL)
    if blocks:
        # Prefer a block that actually contains a spec fn.
        for b in blocks:
            if SPEC_FN_RE.search(b):
                return b.strip()
        return blocks[0].strip()
    return text.replace("```", "").strip()


def extract_spec_fn(text: str) -> str:
    """Return the first complete `pub open spec fn ...` item found in `text`.

    Models sometimes wrap the answer in prose or emit a trailing explanation.
    Falls back to the fence-stripped text when no function item is found, so
    downstream CodeBLEU still sees whatever the model produced.
    """
    cleaned = _strip_fences(text)
    m = SPEC_FN_RE.search(cleaned)
    if not m:
        return cleaned

    start = m.start()
    depth = 0
    seen_open = False
    for i in range(start, len(cleaned)):
        c = cleaned[i]
        if c == "{":
            depth += 1
            seen_open = True
        elif c == "}":
            depth -= 1
            if seen_open and depth == 0:
                return cleaned[start:i + 1].strip()
    return cleaned[start:].strip()


class CliModel:
    """Base class: run a CLI subprocess, retry on transient failure, log calls."""

    name = "cli"

    def __init__(
        self,
        timeout: int = DEFAULT_TIMEOUT,
        retries: int = 2,
        log_path: Optional[Path] = None,
        cwd: Optional[Path] = None,
    ):
        self.timeout = timeout
        self.retries = retries
        self.log_path = Path(log_path) if log_path else None
        # Run somewhere neutral so no repo CLAUDE.md / AGENTS.md leaks into the prompt.
        self.cwd = Path(cwd) if cwd else Path(tempfile.gettempdir())
        self.call_count = 0

    # -- subclass hooks ---------------------------------------------------
    def _build(self, system: str, user: str, workdir: Path):
        """Return (argv, stdin_text, read_output_fn)."""
        raise NotImplementedError

    # -- shared machinery -------------------------------------------------
    def _log(self, record: Dict):
        if not self.log_path:
            return
        self.log_path.parent.mkdir(parents=True, exist_ok=True)
        with self.log_path.open("a", encoding="utf-8") as f:
            f.write(json.dumps(record) + "\n")

    @staticmethod
    def _split_messages(messages: List[Dict[str, str]]):
        system = ""
        user_parts = []
        for m in messages:
            if m.get("role") == "system":
                system = m.get("content", "")
            else:
                user_parts.append(m.get("content", ""))
        return system, "\n\n".join(user_parts)

    def generate(self, messages: List[Dict[str, str]]) -> str:
        system, user = self._split_messages(messages)
        self.call_count += 1
        last_err = ""

        for attempt in range(1, self.retries + 2):
            with tempfile.TemporaryDirectory(dir=self.cwd) as tmp:
                workdir = Path(tmp)
                argv, stdin_text, read_output = self._build(system, user, workdir)
                started = time.time()
                try:
                    proc = subprocess.run(
                        argv,
                        input=stdin_text,
                        capture_output=True,
                        text=True,
                        timeout=self.timeout,
                        cwd=workdir,
                        env=self._env(),
                    )
                    out = read_output(proc)
                    rc = proc.returncode
                    stderr = proc.stderr or ""
                except subprocess.TimeoutExpired:
                    out, rc, stderr = "", -1, f"timeout after {self.timeout}s"

                elapsed = time.time() - started
                stdout_tail = (proc.stdout or "")[-500:] if rc != -1 else ""
                self._log({
                    "model": self.name,
                    "call": self.call_count,
                    "attempt": attempt,
                    "returncode": rc,
                    "elapsed_s": round(elapsed, 1),
                    "output_chars": len(out or ""),
                    "stderr_tail": stderr[-500:],
                    "stdout_tail": stdout_tail if rc != 0 else "",
                })

                if rc == 0 and out and out.strip():
                    return extract_spec_fn(out)

                last_err = f"rc={rc} stderr={stderr[-500:]} stdout={stdout_tail}"
                # The claude CLI prints its usage-limit message on stdout and
                # exits 1 with an empty stderr, so both streams must be checked.
                if _looks_like_quota_error(stderr) or _looks_like_quota_error(stdout_tail):
                    raise QuotaExhausted(
                        f"{self.name}: CLI reported a usage/rate limit -- "
                        f"{(stderr or stdout_tail)[-300:]}"
                    )
                print(f"    [warn] {self.name} attempt {attempt} failed: {last_err}")
                time.sleep(min(5 * attempt, 20))

        raise RuntimeError(f"{self.name}: all {self.retries + 1} attempts failed ({last_err})")

    def _env(self):
        return os.environ.copy()


class CodexCliModel(CliModel):
    """OpenAI `codex exec`, defaulting to gpt-5.6-sol at high reasoning effort."""

    def __init__(self, model: str = "gpt-5.6-sol", effort: str = "high", **kw):
        super().__init__(**kw)
        self.model = model
        self.effort = effort
        self.name = f"codex:{model}:{effort}"

    def _build(self, system: str, user: str, workdir: Path):
        out_file = workdir / "last_message.txt"
        argv = [
            "codex", "exec",
            "-m", self.model,
            "-c", f'model_reasoning_effort="{self.effort}"',
            "-s", "read-only",
            "--skip-git-repo-check",
            "--ephemeral",
            "--color", "never",
            "-o", str(out_file),
            "-",
        ]
        # No system-prompt flag exists; fold it into the single prompt.
        stdin_text = f"{system}\n\n---\n\n{user}\n" if system else user

        def read_output(proc):
            if out_file.exists():
                text = out_file.read_text(encoding="utf-8", errors="ignore")
                if text.strip():
                    return text
            return proc.stdout or ""

        return argv, stdin_text, read_output

    def _env(self):
        env = super()._env()
        # Codex is chatty about the sandbox on stderr; keep it from paging.
        env.setdefault("NO_COLOR", "1")
        return env


class ClaudeCliModel(CliModel):
    """Anthropic `claude -p`, defaulting to Opus 5 at high effort.

    Deliberately NOT using `--bare`: that flag forces ANTHROPIC_API_KEY auth and
    would bill the API instead of the subscription. `--tools ""` disables every
    built-in tool so this is a plain single-turn completion.
    """

    def __init__(self, model: str = "claude-opus-5", effort: str = "high", **kw):
        super().__init__(**kw)
        self.model = model
        self.effort = effort
        self.name = f"claude:{model}:{effort}"

    def _build(self, system: str, user: str, workdir: Path):
        argv = [
            "claude", "-p",
            "--model", self.model,
            "--effort", self.effort,
            "--tools", "",
            "--output-format", "text",
        ]
        if system:
            argv += ["--system-prompt", system]
        return argv, user, (lambda proc: proc.stdout or "")

    def _env(self):
        env = super()._env()
        # Force subscription/OAuth auth: an inherited API key would silently
        # bill the API and change which account the run lands on.
        env.pop("ANTHROPIC_API_KEY", None)
        return env


MODELS = {
    "codex": CodexCliModel,
    "claude": ClaudeCliModel,
}


if __name__ == "__main__":
    import sys

    key = sys.argv[1] if len(sys.argv) > 1 else "codex"
    m = MODELS[key]()
    print(f"Testing {m.name} ...")
    print(m.generate([
        {"role": "system", "content": "You output only Verus code, nothing else."},
        {"role": "user", "content": "Write pub open spec fn ping_spec(x: int) -> bool that says x > 0."},
    ]))
