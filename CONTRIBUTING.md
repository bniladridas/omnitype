# Contributing to Harper's Omnitype

Thank you for your interest in contributing. This document provides practical guidelines for contributors.

## Code of Conduct

This project follows a [Code of Conduct](./CODE_OF_CONDUCT.md). Participation requires adherence to it.

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
RUST_LOG=debug cargo run -- check ./src/
RUST_BACKTRACE=1 cargo test
```

Performance profiling:

```bash
cargo build --release
perf record cargo run --release -- check ./large_project/
perf report
```

## Architecture

* `src/analyzer/` — Type analysis engine
* `src/parser/` — Source code parsing
* `src/fixer/` — Annotation fixes and suggestions
* `src/types/` — Type definitions and structures
* `src/solver/` — Constraint solver
* `src/tracer/` — Runtime type tracing/instrumentation
* `src/ui/` — Terminal user interface
* `src/error.rs` — Error handling and reporting
* `src/main.rs` — Command-line interface entry point
## Standards

* Use `rustfmt` and `clippy`
* Prefer explicit types for clarity
* Write comprehensive tests
* Document public APIs

### Commit Messages

Follow conventional commit format:

* `feat:` A new feature
* `fix:` A bug fix
* `docs:` Documentation only changes
* `style:` Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
* `refactor:` A code change that neither fixes a bug nor adds a feature
* `test:` Adding missing tests or correcting existing tests
* `chore:` Changes to the build process or auxiliary tools and libraries

### Testing

* Unit tests should be placed near the code they test
* Integration tests should be placed in the `tests/` directory
* Use descriptive test names that explain what is being tested
* Test both success and error cases to ensure comprehensive coverage

## Help

* Check existing issues and discussions
* Ask in issue comments
* Read documentation

## Recognition

Contributors are acknowledged in commits, release notes, and documentation.
