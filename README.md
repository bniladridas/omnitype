• Check stock releases [->](https://github.com/bniladridas/omnitype/releases)

<img width="1440" height="900" alt="Image" src="https://github.com/user-attachments/assets/ba1a1c16-6d58-49a8-8a94-2aa7a2236eaa" />

[![CI](https://github.com/bniladridas/omnitype/workflows/CI/badge.svg)](https://github.com/bniladridas/omnitype/actions)
[![Release](https://github.com/bniladridas/omnitype/actions/workflows/release.yml/badge.svg)](https://github.com/bniladridas/omnitype/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org)

<p align="middle">
  <img src="https://github.com/user-attachments/assets/ce273fbf-3e59-41b3-b0cf-a699327963e0" width="700" alt="Alert 7" />
</p>

> [!TIP]
> Available commands for omnitype:
> ```bash
> # Run all tests
> cargo test
> 
> # Build in release mode
> cargo build --release
> 
> # Run with debug logging
> RUST_LOG=debug cargo run -- check src/
> 
> # Launch the Terminal UI (default if no subcommand)
> cargo run
> # or explicitly
> cargo run -- --tui
> 
> # Check Python files in a path (text or JSON output)
> cargo run -- check <path> --format text
> cargo run -- check <path> --format json
> 
> # Set log level for any command
> cargo run -- --log-level debug check <path>
> 
> # Fix annotations in place (adds : Any / -> Any and import)
> cargo run -- fix <path> --in-place
> 
> # Runtime tracing for tests (captures actual types during execution)
> cargo run -- trace <path> --test <name>
> ```

> [!NOTE]
> Current capabilities:
> - `check`: parses Python and reports per-file function/class counts plus diagnostics (e.g., missing param/return annotations). Exits with code 1 if diagnostics are found.
> - `fix`: adds missing `: Any` on untyped parameters and `-> Any` on functions lacking a return type; inserts `from typing import Any` when needed.
> - `trace`: executes Python files with runtime type tracing to collect actual type information from function calls and returns. Supports tracing specific test functions.
> - TUI: Files/Types/Errors/Logs/Editor tabs. Press `a` on a `.py` file in Files to analyze. Errors tab lists diagnostics; Enter opens the file and the Editor jumps to the diagnostic line. Editor supports Up/Down scrolling.

> [!WARNING]
> Scope and limitations (truth in advertising):
> - Target language: Python. Point it at a `.py` file or a directory of Python source.
> - Analysis: basic parsing + simple diagnostics (primarily missing parameter/return annotations). No full type inference or constraint solving yet.
> - Fixes: heuristic text edits to add `: Any`/`-> Any` and the corresponding import; complex signatures and code styles may not be perfectly handled.
> - Runtime tracing: collects type information from actual function executions. Requires Python 3.x and executable test code.

> [!TIP]
> Release notes automation:
> - Pushing a tag like `v0.1.0` triggers a workflow that runs `.github/scripts/generate.sh` and uploads `release_<tag>_changes.md` as an artifact.
> - Use `./scripts/test.sh` to preview what a release would look like
> - Use `./scripts/bump.sh patch` to create a new patch release

> [!IMPORTANT]
> Core features status:
> - **Type checking**: Basic parsing and diagnostics implemented
> - **Fixing**: Automatic type annotation insertion implemented  
> - **Runtime tracing**: Function call type collection implemented
> - **Advanced type inference**: In development
> - **Multi-language support**: Planned

## Development & CI Commands

> [!TIP]
> **Complete command suite for development and CI troubleshooting:**
> 
> ```bash
> # Basic development workflow
> cargo check                                    # Quick compile check
> cargo test                                     # Run all tests
> cargo clippy --all-targets --all-features -- -D warnings  # Lint with warnings as errors
> cargo fmt -- --check                          # Check formatting
> 
> # CI-specific checks (matches GitHub Actions)
> cargo clippy -- -D warnings                   # Clippy check (CI format)
> cargo +nightly udeps --all-targets            # Check for unused dependencies
> cargo audit                                    # Security audit
> 
> # Fix common CI issues
> cargo fmt                                      # Auto-format code
> cargo clippy --fix --all-targets --all-features  # Auto-fix clippy issues
> 
> # Install required tools for full CI compatibility
> rustup install nightly                        # Required for cargo-udeps
> cargo install cargo-udeps                     # For unused dependency checking
> cargo install cargo-audit                     # For security auditing
> 
> # Git workflow after fixes
> git add .
> git commit -m "fix: resolve clippy lints and remove unused dependencies"
> git push origin main
> ```
> 
> **Common CI failure fixes:**
> - **Clippy errors**: Run `cargo clippy --fix --all-targets --all-features` then commit changes
> - **Unused dependencies**: Run `cargo +nightly udeps --all-targets`, remove unused deps from `Cargo.toml`
> - **Format issues**: Run `cargo fmt` then commit changes
> - **Security vulnerabilities**: Run `cargo audit` and update vulnerable dependencies
> - **clippy.toml issues**: Ensure no duplicate keys and use threshold-based config (see `clippy.toml`)
> 
> **Clippy configuration notes:**
> - Use `*-threshold` settings instead of allow/deny lists in `clippy.toml`
> - Common thresholds: `cognitive-complexity-threshold = 15`, `too-many-arguments-threshold = 7`
> - For recursive functions, add `#[allow(clippy::only_used_in_recursion)]` attribute

A hybrid type checker for Python and other dynamic languages.

— @omnitype by [harper](https://github.com/harpertoken) 
