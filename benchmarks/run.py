#!/usr/bin/env python3
"""Deterministic end-to-end benchmarks for tokencut (stdlib only)."""
from __future__ import annotations

import argparse
import csv
import json
import os
import platform
import re
import statistics
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
METRICS_RE = re.compile(r"tokencut:\s*([\d,]+)\s+tokens\s*→\s*([\d,]+)\s+tokens\s*\(([\d.]+)% saved\)")


def make_log(kind: str, blocks: int) -> tuple[str, tuple[str, ...]]:
    """Use synthetic logs so users can reproduce measurements without private data."""
    if kind == "pytest":
        part = (
            "\x1b[36m[INFO]\x1b[0m running test_login\n"
            + "[WARN] retrying database connection\n" * 5
            + '  File "project/auth.py", line 42, in authenticate\n'
            + '    raise ConnectionError("database unavailable")\n'
            + '  File "/opt/venv/site-packages/requests/api.py", line 73, in get\n'
            + '  File "/opt/venv/site-packages/requests/sessions.py", line 589, in request\n'
            + "ConnectionError: database unavailable\n"
        )
        markers = ("project/auth.py", "ConnectionError: database unavailable")
    elif kind == "node":
        part = (
            "\x1b[33m[webpack]\x1b[0m compiling routes\n"
            + "[INFO] waiting for hot reload\n" * 7
            + "    at node_modules/lodash/index.js:20:14\n"
            + "    at Module._compile (node:internal/modules/cjs/loader:1256:14)\n"
            + "    at app/src/server.js:88:9\n"
            + "TypeError: cannot read property 'id'\n"
        )
        markers = ("app/src/server.js:88:9", "TypeError: cannot read property 'id'")
    elif kind == "cargo":
        part = (
            "\x1b[32mCompiling\x1b[0m tokencut v0.1.0\n"
            + "warning: unused variable: `value`\n" * 6
            + "  --> src/main.rs:22:9\n"
            + "error[E0308]: mismatched types\n"
            + "  --> src/main.rs:42:17\n"
            + "expected `usize`, found `String`\n"
        )
        markers = ("error[E0308]: mismatched types", "src/main.rs:42:17")
    else:
        raise ValueError(kind)
    return part * blocks, markers


def run_once(exe: Path, text: str, timeout: int) -> tuple[int, int, float, str]:
    start = time.perf_counter()
    done = subprocess.run([str(exe)], input=text, text=True, capture_output=True,
                          timeout=timeout, encoding="utf-8", errors="replace")
    elapsed = (time.perf_counter() - start) * 1000
    if done.returncode:
        raise RuntimeError(f"tokencut exited {done.returncode}: {done.stderr[-1200:]}")
    match = METRICS_RE.search(done.stderr)
    if not match:
        raise RuntimeError("Unable to parse token counts from tokencut stderr; CLI output may have changed: " + done.stderr[-500:])
    before, after = int(match.group(1).replace(",", "")), int(match.group(2).replace(",", ""))
    if not done.stdout.startswith("```log\n"):
        raise RuntimeError("Expected Markdown log output; CLI format changed")
    return before, after, elapsed, done.stdout


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    default_exe = ROOT / "target" / "release" / ("tokencut.exe" if os.name == "nt" else "tokencut")
    parser.add_argument("--binary", type=Path, default=default_exe, help="Path to a built tokencut binary")
    parser.add_argument("--blocks", type=int, default=200, help="Number of repeated synthetic blocks per workload")
    parser.add_argument("--runs", type=int, default=7, help="Measured runs per workload (median reported)")
    parser.add_argument("--warmup", type=int, default=2, help="Untimed warm-up runs per workload")
    parser.add_argument("--timeout", type=int, default=45, help="Subprocess timeout in seconds")
    parser.add_argument("--output", type=Path, default=ROOT / "benchmarks" / "results")
    args = parser.parse_args()
    if args.blocks < 1 or args.runs < 1 or args.warmup < 0:
        parser.error("--blocks and --runs must be positive; --warmup must be nonnegative")
    binary = args.binary.resolve()
    if not binary.is_file():
        parser.error(f"Executable missing: {binary}. Build first with `cargo build --release --locked`.")
    ver = subprocess.run([str(binary), "--version"], capture_output=True, text=True, check=True).stdout.strip()
    args.output.mkdir(parents=True, exist_ok=True)
    rows = []
    for kind in ("pytest", "node", "cargo"):
        log, markers = make_log(kind, args.blocks)
        sample = args.output / f"sample-{kind}.log"
        sample.write_text(log, encoding="utf-8")
        for _ in range(args.warmup):
            run_once(binary, log, args.timeout)
        measurements = [run_once(binary, log, args.timeout) for _ in range(args.runs)]
        before, after, _, stdout = measurements[0]
        if any((m[0], m[1], m[3]) != (before, after, stdout) for m in measurements):
            raise RuntimeError(f"Nondeterministic cleaned output for {kind}")
        if not all(marker in stdout for marker in markers):
            raise RuntimeError(f"Critical diagnostic marker missing from cleaned {kind} output")
        cleaned_log = args.output / f"cleaned-{kind}.md"
        cleaned_log.write_text(stdout, encoding="utf-8")
        times = [m[2] for m in measurements]
        row = {
            "workload": kind,
            "input_bytes_utf8": len(log.encode("utf-8")),
            "input_tokens": before,
            "cleaned_tokens": after,
            "saved_tokens": before - after,
            "reduction_pct": round((before - after) / before * 100, 2) if before else 0,
            "median_elapsed_ms": round(statistics.median(times), 2),
            "min_elapsed_ms": round(min(times), 2),
            "max_elapsed_ms": round(max(times), 2),
            "sample_log": sample.name,
            "cleaned_log": cleaned_log.name,
            "diagnostics_preserved": True,
        }
        rows.append(row)
        print(f"{kind:8s} {before:>8,d} -> {after:>8,d} tokens  {row['reduction_pct']:6.2f}% saved  median {row['median_elapsed_ms']:.2f} ms")
    csv_path = args.output / "results.csv"
    with csv_path.open("w", newline="", encoding="utf-8") as file:
        writer = csv.DictWriter(file, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    metadata = {
        "timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "binary": str(binary), "binary_version": ver,
        "os": platform.platform(), "python": platform.python_version(),
        "blocks": args.blocks, "runs": args.runs, "warmup": args.warmup,
        "token_measurement": "CLI cl100k_base input/cleaned token counts (excludes Markdown fences and metrics text)",
        "timing": "wall clock subprocess execution including startup and token counting; median across runs",
        "fixtures": "deterministic synthetic logs; NOT independent real-world performance estimates",
        "results": rows,
    }
    (args.output / "results.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
    print(f"Saved {csv_path} and {args.output / 'results.json'}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (RuntimeError, subprocess.TimeoutExpired) as exc:
        print(f"Benchmark failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
