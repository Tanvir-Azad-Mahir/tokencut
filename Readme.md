<div align="center">

# ✂️ tokencut

### Less log noise. More room to reason.

**A lightweight Rust CLI that turns noisy terminal output into concise, LLM-ready context.**

Strip ANSI escape codes, collapse repetitive lines, and prune distracting stack frames before sharing build logs and test failures with a coding assistant.

[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MIT License](https://img.shields.io/badge/License-MIT-2EA44F?style=flat-square)](LICENSE)
[![Contributions Welcome](https://img.shields.io/badge/PRs-welcome-8257E5?style=flat-square)](CONTRIBUTING.md)
[![CI](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/ci.yml/badge.svg)](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/ci.yml)
[![Benchmarks](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/benchmarks.yml/badge.svg)](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/benchmarks.yml)
[![Release](https://img.shields.io/github/v/release/Tanvir-Azad-Mahir/tokencut?include_prereleases&style=flat-square)](https://github.com/Tanvir-Azad-Mahir/tokencut/releases)

[**Quickstart**](#-quickstart) · [**Before & after**](#-before--after) · [**Benchmarks**](#-benchmarks) · [**Contributing**](#-contributing)

</div>

---

## The problem

Coding assistants work best when they receive the **relevant failure**, not thousands of lines of terminal clutter. Raw logs can contain:

- ANSI color and control sequences
- Repeated retries, polling messages, and progress updates
- Long third-party stack traces that bury application frames
- Extra text that consumes context without improving diagnosis

**tokencut** is a terminal-friendly filter designed to preserve useful signals while reducing that noise.

```text
Build / test / runtime logs
           │
           ▼
      ┌─────────────┐
      │  ✂ tokencut │  Strip ANSI · Collapse repeats · Prune frames
      └─────────────┘
           │
           ▼
   Cleaner LLM context ──► Paste into your coding assistant
```

> [!IMPORTANT]
> Log reduction is a trade-off. Filtering can occasionally remove context needed for debugging. Keep the original log available when investigating difficult failures.

## ✨ Features

| Capability | What it does |
| --- | --- |
| **ANSI cleanup** | Removes ANSI terminal escape sequences that add visual noise. |
| **Duplicate collapse** | Condenses consecutive repeated lines into readable counts, such as `[... repeated 42 times ...]`. |
| **Stack-trace pruning** | Hides selected dependency/runtime frames so application-relevant lines are easier to find. |
| **Token accounting** | Reports raw and cleaned token counts using a local `cl100k_base` tokenizer. |
| **Clipboard support** | Copies the cleaned result with `-c` / `--copy`. |
| **Optional pruning** | Keeps stack frames with `--no-prune` when more detail is needed. |
| **CLI-first workflow** | Accepts piped or redirected terminal output. |

## 🚀 Quickstart

### Option 1: Install a prebuilt release

Prebuilt `v0.1.0` executables are available for these platforms:

| Platform | Archive |
| --- | --- |
| Linux x64 | `tokencut-v0.1.0-linux-x64.tar.gz` |
| Windows x64 | `tokencut-v0.1.0-windows-x64.zip` |
| macOS Apple Silicon (ARM64) | `tokencut-v0.1.0-macos-arm64.tar.gz` |
| macOS Intel (x64) | `tokencut-v0.1.0-macos-x64.tar.gz` |

**[Download tokencut v0.1.0](https://github.com/Tanvir-Azad-Mahir/tokencut/releases/tag/v0.1.0)** · The initial release is marked as a **pre-release**.

You can extract the binary manually or use the repository's installer scripts. Review the scripts before executing them.

**Linux / macOS**

```bash
curl -fsSLo install.sh https://raw.githubusercontent.com/Tanvir-Azad-Mahir/tokencut/main/scripts/install.sh
# Inspect install.sh before executing it.
bash install.sh v0.1.0
```

**Windows PowerShell (x64)**

```powershell
Invoke-WebRequest https://raw.githubusercontent.com/Tanvir-Azad-Mahir/tokencut/main/scripts/install.ps1 -OutFile install.ps1
# Inspect install.ps1 before executing it.
.\install.ps1 -Version v0.1.0
```

The installers download a release archive, compare its SHA-256 checksum against the published `SHA256SUMS.txt`, and install the executable without administrator privileges. They **do not automatically update `PATH`**. By default, they install into `~/.local/bin` on Linux/macOS and `$HOME\.local\bin` on Windows.

See **[INSTALLATION.md](INSTALLATION.md)** for custom installation paths, `PATH` setup, and platform limitations. The checksum verifies consistency with the release's checksum file; it is not an independent digital signature.

### Option 2: Build from source

Install [Rust](https://www.rust-lang.org/tools/install), then:

```bash
git clone https://github.com/Tanvir-Azad-Mahir/tokencut.git
cd tokencut
cargo build --release --locked
```

The compiled executable is located at:

| Platform | Binary |
| --- | --- |
| Linux / macOS | `./target/release/tokencut` |
| Windows | `.\target\release\tokencut.exe` |

### Clean a log

If the binary is installed and on your `PATH`:

**Linux / macOS**

```bash
cat build.log | tokencut
```

**Windows PowerShell**

```powershell
Get-Content build.log | tokencut.exe
```

If you built from source instead, use `./target/release/tokencut` (Linux/macOS) or `.\target\release\tokencut.exe` (Windows) in place of `tokencut`.

### Copy the cleaned output

```bash
cat build.log | tokencut --copy
```

On Windows PowerShell:

```powershell
Get-Content build.log | tokencut.exe --copy
```

## 🧪 Usage examples

Pipe test and build output directly into tokencut:

```bash
pytest 2>&1 | tokencut -c
npm test 2>&1 | tokencut -c
cargo build 2>&1 | tokencut
```

`2>&1` combines standard error with standard output so diagnostic messages are not skipped.

Want to preserve stack-trace details while still cleaning ANSI sequences and repeats?

```bash
pytest 2>&1 | tokencut --no-prune
```

The CLI prints a Markdown-formatted log on standard output and token-saving metrics on standard error. The token counts depend on the tokenizer and the input data.

## 🔍 Before & after

<details open>
<summary><strong>Example: noisy logs → concise context</strong></summary>

**Raw terminal output**

```text
[INFO] Starting build process...
[INFO] Compiling module A...
[INFO] Compiling module A...
[INFO] Compiling module A...
[WARN] Connection retrying...
[WARN] Connection retrying...
[WARN] Connection retrying...
Traceback (most recent call last):
  File "app.py", line 42, in main
    process_data()
  File "site-packages/requests/api.py", line 73, in get
    return request("get", url, **kwargs)
  File "site-packages/requests/sessions.py", line 589, in request
    response = self.send(prep, **send_kwargs)
ConnectionRefusedError: [Errno 111] Connection refused
[ERROR] Build failed!
```

**Illustrative cleaned content**

```text
[INFO] Starting build process...
[INFO] Compiling module A...
  [... repeated 2 times ...]
[WARN] Connection retrying...
  [... repeated 2 times ...]
Traceback (most recent call last):
  File "app.py", line 42, in main
    process_data()
  [... external stack frames hidden ...]
ConnectionRefusedError: [Errno 111] Connection refused
[ERROR] Build failed!
```

</details>

Actual CLI output includes a Markdown code fence. The example above focuses on the cleaned content; exact pruning depends on the input paths and filtering rules.

## 📊 Benchmarks

On **three deterministic synthetic log workloads**, tokencut reduced the combined token count from **69,200 to 34,999**—a **49.42% aggregate reduction**. These are measured results supplied from a successful benchmark run, **not** an estimate of savings for arbitrary real-world logs.

| Workload | Input tokens | Cleaned tokens | Tokens saved | Reduction | Median runtime |
| --- | ---: | ---: | ---: | ---: | ---: |
| Pytest-style logs | 24,800 | 11,999 | 12,801 | 51.62% | 207.60 ms |
| Node.js logs | 23,800 | 10,000 | 13,800 | 57.98% | 193.97 ms |
| Cargo logs | 20,600 | 13,000 | 7,600 | 36.89% | 197.98 ms |
| **Total** | **69,200** | **34,999** | **34,201** | **49.42%** | **—** |

**Methodology:** Each workload contains 200 repeated synthetic blocks. The runner performs 2 warm-up runs and 7 measured runs, reporting median end-to-end subprocess time (including process startup, cleaning, and tokenization). The tokenizer is `cl100k_base`; token figures exclude the Markdown code fences and CLI metrics. The benchmark checks that designated diagnostic markers survive the cleanup, but does not establish that every relevant detail is preserved.

The aggregate reduction is calculated as `(69,200 - 34,999) / 69,200 × 100`, rather than as an unweighted average of percentages. Runtime figures are platform-dependent and should not be generalized to other machines.

### Reproduce the results

```bash
cargo build --release --locked
python3 benchmarks/run.py --blocks 200 --runs 7 --warmup 2
```

On Windows, use `py -3 benchmarks/run.py --blocks 200 --runs 7 --warmup 2` if that is your Python launcher.

The runner writes `results.csv`, `results.json`, input fixtures, and cleaned outputs to `benchmarks/results/`. The JSON includes execution-environment metadata. For reproduction details and limitations, see [benchmarks/README.md](benchmarks/README.md) and [benchmarks/run.py](benchmarks/run.py).

You can also trigger the [Benchmarks GitHub Actions workflow](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/benchmarks.yml) and download its result artifacts.

> [!NOTE]
> These figures come from deliberately repetitive **synthetic** logs. They are not independently validated real-world performance claims, and actual savings can be lower or higher. The `cl100k_base` tokenizer may differ from the tokenizer used by your coding assistant.

## 🔐 Privacy & design

Log processing runs locally in the CLI; the tool does not require a cloud service to perform cleanup. However, cleaned logs can still contain sensitive information. Review output before sharing it with any external system: **tokencut is not a secret-redaction or anonymization tool**.

## 🤝 Contributing

Bug reports, new filtering rules, compatibility improvements, reproducible benchmarks, and documentation pull requests are welcome.

1. Fork the repository.
2. Create a branch: `git checkout -b feature/my-improvement`.
3. Commit your changes: `git commit -m "feat: improve log filtering"`.
4. Push the branch and open a pull request.

Read [CONTRIBUTING.md](CONTRIBUTING.md) for project guidelines.

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

<div align="center">

**Give your coding agent the error—not the entire terminal history.**

[⭐ Star tokencut](https://github.com/Tanvir-Azad-Mahir/tokencut) · [Report an issue](https://github.com/Tanvir-Azad-Mahir/tokencut/issues) · [Download a release](https://github.com/Tanvir-Azad-Mahir/tokencut/releases)

</div>
