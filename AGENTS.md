# About
This is Nemesis, a web application built using Rust and dioxus. Nemesis is a space situational awareness platform, used to ingest conjunction data messages, provide reports to operators, and calculate probabilities of collisions of simulated collision avoidance manoeuvres.

# Repository Guidelines

## Project Structure & Module Organization
- Source: `src/` (entry: `src/main.rs`).
- Assets: `assets/` (CSS, SVG, favicon) referenced via Dioxus `asset!` macros.
- Config: `Cargo.toml` (Rust 2021), `Dioxus.toml` (web app settings), `clippy.toml` (lint rules).

## Build and Development Commands
- `cargo build`: Compiles the project for the current target.
- `dx build --platform web --release`: Produces an optimized web build.
- `cargo clippy --all-targets -- -D warnings`: Lints; CI should treat warnings as errors.
- `cargo fmt --all`: Formats the codebase.

## Coding Style & Naming Conventions
- Use Rust 2021 and idiomatic Rust style. Format with `rustfmt` (4-space indent, no tabs).
- Keep modules/files in `snake_case` (e.g., `status.rs`), types in `UpperCamelCase`, functions/variables in `snake_case`.
- Dioxus components: annotate with `#[component]` and use clear, noun-based names.
- Place feature-specific UI in a module directory (e.g., `src/module_name/`) and re-export via `mod.rs` when helpful.

## Design Style
- Use a clean, minimalistic design style. Ensure designs evoke a sense of utilitarianism, maximizing readability and real-world usability.
- Designs should be inspired by industrial or military user interfaces. The UI should feel right at home being viewed inside the dark cabin of a supersonic stealth fighter.

## Testing Guidelines
- Unit tests live alongside code using `#[cfg(test)] mod tests { ... }`. Run with `cargo test`.
- Integration tests (if added) go in `tests/` with descriptive filenames.
- Focus tests on logic. UI snapshot tests are optional.

## Commit & Pull Request Guidelines
- Commits: Use concise, imperative messages ("add", "fix", "refactor"). Group related changes; avoid noisy churn. Examples seen: short, task-focused commits.
- PRs: Include a clear summary, rationale, and testing steps (commands like `dx serve` to verify). Attach screenshots/GIFs for UI changes.
- Checks: Ensure `cargo clippy -D warnings` and `cargo fmt --all` pass. Note any platform tested (`--platform web|desktop`).
- Link issues when applicable and call out breaking changes explicitly.

## Architecture Notes
- The app is a Dioxus UI using dioxus version 0.6.3. `main.rs` wires global assets and composes feature components.
