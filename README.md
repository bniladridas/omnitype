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
> # (Scaffolded) Runtime tracing for tests
> cargo run -- trace <path> --test <name>
> ```

> [!NOTE]
> Current capabilities:
> - `check`: parses Python and reports per-file function/class counts plus diagnostics (e.g., missing param/return annotations). Exits with code 1 if diagnostics are found.
> - `fix`: adds missing `: Any` on untyped parameters and `-> Any` on functions lacking a return type; inserts `from typing import Any` when needed.
> - TUI: Files/Types/Errors/Logs/Editor tabs. Press `a` on a `.py` file in Files to analyze. Errors tab lists diagnostics; Enter opens the file and the Editor jumps to the diagnostic line. Editor supports Up/Down scrolling.

> [!WARNING]
> Scope and limitations (truth in advertising):
> - Target language: Python. Point it at a `.py` file or a directory of Python source.
> - Analysis: basic parsing + simple diagnostics (primarily missing parameter/return annotations). No full type inference or constraint solving yet.
> - Fixes: heuristic text edits to add `: Any`/`-> Any` and the corresponding import; complex signatures and code styles may not be perfectly handled.
> - Runtime tracing: not implemented yet.

> [!TIP]
> Release notes automation:
> - Pushing a tag like `v0.1.0` triggers a workflow that runs `.github/scripts/generatecodediffs.sh` and uploads `release_<tag>_changes.md` as an artifact.

> [!IMPORTANT]
> Core features (type checking, fixing, and runtime tracing) are currently in development.
> The basic project structure and test framework are in place.

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
> cargo install cargo-udeps                     # For unused dependency checking
> cargo install cargo-audit                     # For security auditing
> rustup install nightly                        # Required for cargo-udeps
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
