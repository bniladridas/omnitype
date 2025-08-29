# Contributing to harper's omnitype

Thank you for your interest in contributing. This document provides practical guidelines for contributors.

## Code of Conduct

This project follows a [Code of Conduct](CODE_OF_CONDUCT.md). Participation requires adherence to it.

## Getting Started

### Prerequisites

* Rust 1.89.0 or later
* Git

### Setup

```bash
git clone https://github.com/harpertoken/omnitype.git
cd omnitype
rustup install nightly
cargo install cargo-udeps cargo-audit
cargo test
```

## Workflow

### Before Changes

Create a branch and ensure checks pass:

```bash
git checkout -b feature/your-feature
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
cargo +nightly udeps --all-targets
```

### During Changes

* Write clear commit messages
* Add tests for new functionality
* Update documentation as needed
* Follow existing code style

### Submitting Changes

Push your branch and open a Pull Request with:

* Clear description
* Linked issues
* Tests and documentation (if relevant)

## Pull Requests

* CI must pass
* Tests and docs updated
* Commit messages clear
* No merge conflicts

Reviews: automated checks → maintainer review → feedback → approval → merge

## Issues

### Bug Reports

Provide: description, steps to reproduce, expected vs actual, environment, logs.

### Feature Requests

Provide: description, use case, motivation, possible implementation or alternatives.

## Development Tips

Run targeted tests:

```bash
cargo test analyzer
cargo test test_analyzer_initialization
```

Debug:

```bash
RUST_LOG=debug cargo run -- check src/
RUST_BACKTRACE=1 cargo test
```

Performance:

```bash
cargo build --release
perf record cargo run --release -- check large_project/
perf report
```

## Architecture

* `src/analyzer/` — type analysis
* `src/parser/` — source parsing
* `src/fixer/` — annotation fixes
* `src/types/` — type definitions
* `src/ui/` — terminal UI
* `src/error.rs` — error handling
* `src/main.rs` — CLI entry

## Standards

* Use `rustfmt` and `clippy`
* Prefer explicit types for clarity
* Write comprehensive tests
* Document public APIs

### Commit Messages

Conventional commits:

* `feat:` new feature
* `fix:` bug fix
* `docs:` documentation
* `style:` formatting
* `refactor:` restructuring
* `test:` tests
* `chore:` maintenance

### Testing

* Unit tests near code
* Integration tests in `tests/`
* Descriptive test names
* Test both success and error cases

## Help

* Check existing issues and discussions
* Ask in issue comments
* Read documentation

## Recognition

Contributors are acknowledged in commits, release notes, and documentation.
