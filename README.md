# Omnitype

[![CI](https://github.com/bniladridas/omnitype/workflows/CI/badge.svg)](https://github.com/bniladridas/omnitype/actions)
[![Release](https://github.com/bniladridas/omnitype/actions/workflows/release.yml/badge.svg)](https://github.com/bniladridas/omnitype/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org)

Experimental type-checker for Python and dynamic languages.

## Features

- **Check**: Parse Python files and report diagnostics (e.g., missing annotations).
- **Fix**: Add missing `: Any` and `-> Any` annotations automatically.
- **Trace**: Runtime type tracing for function calls.
- **TUI**: Terminal UI for file analysis and error navigation.

## Usage

```bash
# Check files
cargo run -- check <path>

# Fix annotations
cargo run -- fix <path> --in-place

# Launch TUI
cargo run

# Run tests
cargo test
```

## Development

```bash
cargo check
cargo clippy -- -D warnings
cargo fmt
```

— @omnitype by [harper](https://github.com/harpertoken)
