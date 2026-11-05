<div align="center">

# ✂️ tokencut

### Less log noise. More room to reason.

**A lightweight Rust CLI that turns noisy terminal output into concise, LLM-ready context.**

Strip ANSI escape codes, collapse repetitive lines, and prune distracting stack frames before pasting build logs and test failures into your coding assistant.

[![Rust](https://img.shields.io/badge/Built_with-Rust-CE422B?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust 1.70+](https://img.shields.io/badge/Rust-1.70%2B-555?style=flat-square)](https://www.rust-lang.org/)
[![MIT License](https://img.shields.io/badge/License-MIT-2EA44F?style=flat-square)](LICENSE)
[![Contributions Welcome](https://img.shields.io/badge/PRs-welcome-8257E5?style=flat-square)](CONTRIBUTING.md)
[![CI](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/ci.yml/badge.svg)](https://github.com/Tanvir-Azad-Mahir/tokencut/actions/workflows/ci.yml)

[**Get started**](#-quickstart) · [**See an example**](#-before--after) · [**Explore features**](#-features) · [**Contribute**](#-contributing)

</div>

---

## The problem

Coding assistants work best when they receive the **relevant failure**, not thousands of lines of terminal clutter. Raw logs often contain:

- ANSI color and control sequences
- Repeated retries, polling messages, and progress updates
- Long third-party stack traces that bury application frames
- Extra text that consumes context without improving diagnosis

**tokencut** is a terminal-friendly filter designed to keep useful signals while reducing that noise.

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
| **ANSI cleanup** | Removes terminal escape sequences such as `\x1B[...` that add visual noise. |
| **Duplicate collapse** | Condenses repeated lines into readable counts, e.g. `[... repeated 42 times ...]`. |
| **Stack-trace pruning** | Reduces noisy dependency/runtime frames to surface application-relevant lines. |
| **Token accounting** | Reports estimated input and output token counts using a local `cl100k_base` tokenizer. |
| **Clipboard support** | Copies the cleaned result with `-c` / `--copy`. |
| **CLI-first workflow** | Works with redirected files and shell pipelines. |

## 🚀 Quickstart

### 1. Build from source

Install [Rust](https://www.rust-lang.org/tools/install), then:

```bash
git clone https://github.com/Tanvir-Azad-Mahir/tokencut.git
cd tokencut
cargo build --release
```

The compiled executable is at:

| Platform | Binary |
| --- | --- |
| Linux / macOS | `./target/release/tokencut` |
| Windows | `.\target\release\tokencut.exe` |

### 2. Clean a log

**Linux / macOS**

```bash
cat build.log | ./target/release/tokencut
```

**Windows PowerShell**

```powershell
Get-Content build.log | .\target\release\tokencut.exe
```

### 3. Copy the cleaned output

```bash
cat build.log | ./target/release/tokencut --copy
```

Or in PowerShell:

```powershell
Get-Content build.log | .\target\release\tokencut.exe --copy
```

> **Note:** The examples above use the locally built binary. If you have separately installed `tokencut` on your `PATH`, use `tokencut` instead of its full path.

## 🧪 Usage examples

Pipe test and build output directly into tokencut:

```bash
pytest 2>&1 | tokencut -c
npm test 2>&1 | tokencut -c
cargo build 2>&1 | tokencut
```

`2>&1` combines standard error with standard output so diagnostic messages are not skipped.

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

**Illustrative cleaned output**

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

Example CLI metrics from the project description:

```text
✂ tokencut: 224 tokens → 127 tokens (43.3% saved)
📋 Copied formatted log to clipboard!
```

> The example is illustrative; output and savings depend on the actual input and filtering configuration.

## 📊 Benchmark examples

The following figures were supplied as **sample results**, not independently reproduced measurements. Replace them with benchmark scripts, sample logs, environment details, and repeatable results before presenting them as verified performance claims.

| Workload | Raw log size | Input tokens | Output tokens | Reported reduction |
| :--- | ---: | ---: | ---: | ---: |
| Pytest suite output | 1.2 MB | 45,200 | 8,100 | 82.1% |
| Next.js production build | 850 KB | 28,400 | 5,900 | 79.2% |
| Cargo build stack trace | 320 KB | 11,100 | 2,800 | 74.7% |

**Token reduction formula:** `(input tokens − output tokens) / input tokens × 100`.

## 🔐 Privacy & design

Logs are processed locally by the CLI. Review generated output before sharing it with any external service: reducing noise does **not** automatically remove credentials, secrets, or personal data.

## 🤝 Contributing

Bug reports, filtering rules, documentation improvements, and pull requests are welcome.

1. Fork the repository.
2. Create a branch: `git checkout -b feature/my-improvement`.
3. Commit your changes: `git commit -m "feat: improve log filtering"`.
4. Push the branch and open a pull request.

See [CONTRIBUTING.md](CONTRIBUTING.md) for any additional project guidelines.

## 📄 License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for details.

---

<div align="center">

**Give your coding agent the error—not the entire terminal history.**

[⭐ Star tokencut](https://github.com/Tanvir-Azad-Mahir/tokencut) · [Report an issue](https://github.com/Tanvir-Azad-Mahir/tokencut/issues)

</div>
