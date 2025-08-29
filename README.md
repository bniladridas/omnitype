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
