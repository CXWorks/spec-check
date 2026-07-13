#!/usr/bin/env python3
"""
extract_rules_from_html.py

Extract one JSONL record per rule from the Rust Coding Guidelines print book.

The script is intentionally dependency-free: it uses only the Python standard
library so it can run in a clean workspace without extra installation steps.

Typical usage:

    python3 extract_rules_from_html.py \
        --input https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/print.html \
        --output rules.jsonl

The emitted JSONL schema is one object per rule with fields such as:

    rule_id, title, section, category, level, has_lint, lint_names,
    source_url, version, content
"""

from __future__ import annotations

import argparse
import html as html_lib
import json
import os
import re
import sys
import ssl
from dataclasses import dataclass
from typing import Iterable
from urllib.error import URLError
from urllib.request import Request, urlopen


RULE_ID_RE = re.compile(r"^(?P<rule_id>[PG]\.[A-Z]{3}(?:\.[A-Z]{3})?\.\d{2})\s+(?P<title>.+)$")
SECTION_RE = re.compile(r"^\d+(?:\.\d+)+\s+.+$")
HEADING_RE = re.compile(r"<h(?P<level>[1-6])(?P<attrs>[^>]*)>(?P<body>.*?)</h(?P=level)>", re.I | re.S)
SCRIPT_STYLE_RE = re.compile(r"(?is)<(script|style)[^>]*>.*?</\1>")
PRE_RE = re.compile(r"(?is)<pre[^>]*>(.*?)</pre>")
TAG_RE = re.compile(r"<[^>]+>")
BR_RE = re.compile(r"(?i)<br\s*/?>")
BLOCK_END_RE = re.compile(r"(?i)</(p|div|li|tr|h[1-6]|ul|ol|table|blockquote)>")

DEFAULT_VERSION = "V1.0 beta"
DEFAULT_USER_AGENT = "Mozilla/5.0 (compatible; spec-check/1.0)"
DEFAULT_SOURCE_URL = "https://rust-coding-guidelines.github.io/rust-coding-guidelines-zh/print.html"
DEFAULT_OUTPUT_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "rules.jsonl")


@dataclass(frozen=True)
class Heading:
    level: int
    text: str
    start: int
    end: int


def normalize_whitespace(text: str) -> str:
    text = text.replace("\r\n", "\n").replace("\r", "\n")
    text = re.sub(r"[ \t]+\n", "\n", text)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()


def strip_tags(fragment: str) -> str:
    fragment = SCRIPT_STYLE_RE.sub("", fragment)
    fragment = BR_RE.sub("\n", fragment)
    fragment = BLOCK_END_RE.sub("\n", fragment)
    fragment = TAG_RE.sub("", fragment)
    fragment = html_lib.unescape(fragment)
    return normalize_whitespace(fragment)


def html_chunk_to_text(fragment: str) -> str:
    """Convert a chunk of HTML to a readable plain-text representation."""

    def replace_pre(match: re.Match[str]) -> str:
        inner = strip_tags(match.group(1))
        if not inner:
            return ""
        return f"\n```text\n{inner}\n```\n"

    fragment = SCRIPT_STYLE_RE.sub("", fragment)
    fragment = PRE_RE.sub(replace_pre, fragment)
    fragment = BR_RE.sub("\n", fragment)
    fragment = BLOCK_END_RE.sub("\n", fragment)
    fragment = TAG_RE.sub("", fragment)
    fragment = html_lib.unescape(fragment)
    fragment = fragment.replace("\u00a0", " ")
    fragment = re.sub(r"[ \t]+\n", "\n", fragment)
    fragment = re.sub(r"\n{3,}", "\n\n", fragment)
    return fragment.strip()


def fetch_source(source: str) -> tuple[str, str]:
    """Load HTML from a URL."""

    if not re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*://", source):
        raise ValueError("--input must be a URL (e.g. https://.../print.html)")

    request = Request(source, headers={"User-Agent": DEFAULT_USER_AGENT})
    try:
        with urlopen(request) as response:
            raw = response.read()
    except Exception as exc:
        reason = exc
        if isinstance(exc, URLError) and getattr(exc, "reason", None) is not None:
            reason = exc.reason

        message = str(reason)
        if "CERTIFICATE_VERIFY_FAILED" not in message and "certificate verify failed" not in message:
            raise

        # Some macOS Python installations do not ship with a usable CA bundle.
        # Falling back to an unverified context keeps the extraction script
        # self-contained for local data preparation workflows.
        with urlopen(request, context=ssl._create_unverified_context()) as response:
            raw = response.read()
    return raw.decode("utf-8", errors="replace"), source


def parse_headings(raw_html: str) -> list[Heading]:
    headings: list[Heading] = []
    for match in HEADING_RE.finditer(raw_html):
        level = int(match.group("level"))
        body = match.group("body")
        text = strip_tags(body)
        text = re.sub(r"^[#\s]+", "", text).strip()
        text = re.sub(r"\s+", " ", text)
        if not text:
            continue
        headings.append(Heading(level=level, text=text, start=match.start(), end=match.end()))
    return headings


def find_section(headings: list[Heading], rule_heading_index: int) -> str:
    """Find the nearest numbered section heading before a rule heading."""

    for idx in range(rule_heading_index - 1, -1, -1):
        candidate = headings[idx].text
        if SECTION_RE.match(candidate):
            return candidate
    return ""


def extract_lint_names(rule_text: str) -> list[str]:
    start = rule_text.find("【Lint 检测】")
    lint_names: set[str] = set()

    lint_text = rule_text[start:] if start != -1 else rule_text

    # First pass: explicit lint markers such as clippy::missing_errors_doc or
    # rustc-lint: non_snake_case.
    for match in re.finditer(r"(?:clippy::|rustc-lint:|rustc:|cargo:)([a-z][a-z0-9_]+)", lint_text):
        lint_names.add(match.group(1))

    # Second pass: underscore-style identifiers often used in the lint table
    # or quoted in examples.
    for match in re.finditer(r"\b[a-z][a-z0-9_]*_[a-z0-9_]+\b", lint_text):
        token = match.group(0)
        if token not in {"allow_unused", "unused_must_use"}:
            lint_names.add(token)

    for line in lint_text.splitlines():
        if "|" not in line:
            continue

        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if not cells:
            continue

        first = cells[0]
        if not first or first == "_":
            continue

        match = re.search(r"(?:Rustc|rustc-lint|Clippy|clippy|cargo)?[:：]?\s*([a-z][a-z0-9_]+)", first)
        if match:
            lint_names.add(match.group(1))
            continue

        if re.fullmatch(r"[a-z][a-z0-9_]+", first):
            lint_names.add(first)

    return sorted(lint_names)


def extract_level(rule_text: str) -> str | None:
    match = re.search(r"【级别】\s*(要求|建议)", rule_text)
    if match:
        return match.group(1)
    return None


def build_source_url(source_ref: str, rule_id: str) -> str:
    # The source itself is the authoritative document, so the most stable
    # traceability URL is the source reference plus a rule-id fragment.
    return f"{source_ref}#{rule_id}"


def extract_rules(raw_html: str, source_ref: str, version: str) -> list[dict]:
    headings = parse_headings(raw_html)
    if not headings:
        return []

    rule_meta: list[tuple[int, str, str]] = []

    for idx, heading in enumerate(headings):
        match = RULE_ID_RE.match(heading.text)
        if not match:
            continue
        rule_meta.append((idx, match.group("rule_id"), match.group("title")))

    rules: list[dict] = []
    for order, (heading_idx, rule_id, title) in enumerate(rule_meta):
        heading = headings[heading_idx]
        next_start = len(raw_html)
        if order + 1 < len(rule_meta):
            next_start = headings[rule_meta[order + 1][0]].start

        chunk_html = raw_html[heading.end:next_start]
        chunk_text = html_chunk_to_text(chunk_html)
        content = normalize_whitespace(f"{rule_id} {title}\n\n{chunk_text}")
        section = find_section(headings, heading_idx)
        lint_names = extract_lint_names(content)

        rules.append(
            {
                "rule_id": rule_id,
                "title": title,
                "section": section,
                "category": rule_id.split(".")[1],
                "level": extract_level(content),
                "has_lint": bool(lint_names),
                "lint_names": lint_names,
                "source_url": build_source_url(source_ref, rule_id),
                "version": version,
                "content": content,
            }
        )

    return rules


def write_jsonl(records: Iterable[dict], output_path: str) -> None:
    os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as fh:
        for record in records:
            fh.write(json.dumps(record, ensure_ascii=False) + "\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Extract Rust guideline rules from print.html into JSONL.")
    parser.add_argument(
        "--input",
        default=DEFAULT_SOURCE_URL,
        help="URL of print.html (default: official print.html URL)",
    )
    parser.add_argument(
        "--output",
        default=DEFAULT_OUTPUT_PATH,
        help="Path to write JSONL output (default: rules.jsonl next to this script)",
    )
    parser.add_argument(
        "--version",
        default=DEFAULT_VERSION,
        help='Version string to store in metadata (default: "V1.0 beta")',
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    try:
        raw_html, source_ref = fetch_source(args.input)
    except Exception as exc:  # pragma: no cover - surfaced to CLI user
        print(f"[ERROR] Failed to load input source: {exc}", file=sys.stderr)
        return 1

    rules = extract_rules(raw_html, source_ref=source_ref, version=args.version)
    if not rules:
        print("[WARN] No rules found. Check that the input is the Rust guidelines print book.", file=sys.stderr)
        return 2

    write_jsonl(rules, args.output)

    print(f"Extracted {len(rules)} rules -> {args.output}")
    print(f"Source: {source_ref}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())