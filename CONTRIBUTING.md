# Contributing to tokencut

Thanks for helping improve **tokencut**, a command-line tool for cleaning noisy terminal logs before sharing them with coding assistants.

## Before you start

- Search [existing issues](https://github.com/Tanvir-Azad-Mahir/tokencut/issues) and pull requests for similar work.
- For large changes, open an issue first to discuss the intended behavior.
- Keep changes focused, and explain which log patterns they affect.

## Local development

Install the [Rust toolchain](https://www.rust-lang.org/tools/install), then:

```bash
git clone https://github.com/Tanvir-Azad-Mahir/tokencut.git
cd tokencut
cargo build
cargo test
```

For a release build:

```bash
cargo build --release
```

The Rust CLI reads from standard input. For example:

```bash
printf 'retry\nretry\nfailed\n' | cargo run --
```

Use `--no-prune` to keep third-party stack frames while testing:

```bash
printf 'example error\n' | cargo run -- --no-prune
```

For usage options:

```bash
cargo run -- --help
```

> The repository also contains a Python implementation (`tokencut.py`). If you modify it, describe how its behavior compares with the Rust CLI and list the Python dependencies needed to reproduce your test.

## What makes a useful contribution?

- Improve ANSI escape-sequence handling and newline normalization.
- Add safe, well-scoped stack-trace patterns for common ecosystems.
- Improve collapsing of consecutive duplicate lines without losing useful error details.
- Add reproducible tests for tricky logs, including examples that should **not** be pruned.
- Improve platform compatibility, documentation, or diagnostic messages.

## Testing and quality

Before opening a pull request:

```bash
cargo fmt --check
cargo test
cargo build --release
```

If `cargo fmt --check` reports differences, run `cargo fmt` and review the changes. Include tests where practical, especially for cleaning rules that could discard diagnostic context.

When proposing benchmark or token-saving claims, supply the input sample or generation method, tokenization method, command used, and resulting output. Do not present illustrative examples as measured performance.

## Pull request checklist

- [ ] The change has a clear purpose and scope.
- [ ] Existing CLI flags and standard-input behavior still work.
- [ ] The change preserves meaningful application errors and exception context.
- [ ] Relevant tests or reproducible examples are included.
- [ ] Documentation is updated if user-facing behavior changes.
- [ ] `cargo test` and `cargo build --release` pass locally.

## Security and privacy

Never include real API keys, private tokens, passwords, or personal data in issue reports, fixtures, or pull requests. tokencut is a **log-noise filter**, not a secret-redaction or sanitization tool; review logs before sharing them publicly.

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).
