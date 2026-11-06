# Reproducible tokencut benchmarks

This suite measures **observed** `tokencut` token reduction and CLI wall-clock runtime on three deterministic **synthetic** workloads: Pytest-style logs, Node.js stack traces, and Rust/Cargo compiler diagnostics.

## Run locally

From the `tokencut` repository root:

```bash
cargo build --release --locked
python benchmarks/run.py
```

Windows PowerShell:

```powershell
cargo build --release --locked
py -3 benchmarks/run.py
```

For a larger input and more repetitions:

```bash
python benchmarks/run.py --blocks 1000 --runs 15 --warmup 3
```

The script uses only Python's standard library. No benchmark numbers are hardcoded. The default test creates 200 repeated blocks per workload, performs two warm-ups, then measures seven subprocess executions. Use `--binary PATH` to specify a different binary.

## Outputs

Generated locally under `benchmarks/results/`:

- `results.csv` — per-workload input/cleaned token totals, percentage saved, and timing summary
- `results.json` — the same results plus environment, tool version, settings, timestamp
- `sample-*.log` — exact input fixture for reproduction
- `cleaned-*.md` — resulting LLM-ready Markdown output

**Token metric:** The Rust CLI's `cl100k_base` counter applied to raw input and cleaned content. This does **not** include the surrounding Markdown code fence or telemetry printed on stderr. Other LLM tokenizers may produce different savings.

**Timing metric:** Median elapsed subprocess time across the measured runs. This includes process startup, cleaning, and tokenization. It is **not** isolated algorithm latency or a statistically rigorous cross-machine benchmark.

The script checks that critical error markers survive cleanup and that outputs/counts are deterministic. It fails if they do not.

## How to report benchmarks responsibly

When publishing numbers, include the exact binary version / commit, OS and CPU, command, input fixture(s), tokenizer, repetitions, and generated CSV/JSON. Do not claim that synthetic results establish general savings on real-world logs. For credible real-world benchmarks, collect consented/redacted logs from real toolchains, disclose sampling, and compare relevant error preservation as well as reduction.

Do not commit sensitive build logs or credentials. This tool is **not** a secret-redaction system.
