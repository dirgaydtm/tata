# CLAUDE.md — Ratype Project Instructions

Ratype is a local-first code typing test in Rust (2024 Edition) for the web (Ratzilla) and the native terminal (Crossterm) from one codebase. Structure, layers, and data flow are in `ARCHITECTURE.md`.

## Commands

- **Check**: `cargo check` and `cargo check --target wasm32-unknown-unknown`
- **Test**: `cargo test`
- **Lint**: `cargo clippy -- -D warnings` and `cargo clippy --target wasm32-unknown-unknown -- -D warnings`
- **Run native**: `cargo run`
- **Run web**: `trunk serve` (http://localhost:8080); release build: `trunk build --release`

Every change must pass both targets with zero warnings.

## Rules

- Keep it simple (KISS, YAGNI, DRY, SOLID). Do not add features or abstractions that were not asked for.
- `engine/` must never import UI modules (`ratatui::Frame`, `Widget`, ...).
- `data/` must never touch UI types. `utils/theme.rs` maps `ThemeChoice` to `ratcn::Theme`.
- `data/` returns `Result<T, AppError>`, with no unwrap or panic in production paths.
- Platform runners in `platform/` hold no app logic. Panics are allowed only during init, via `expect("descriptive message")`.
- No `mod.rs`. Use `foo.rs` plus a `foo/` directory.
- Do not hand-edit files in `src/ratcn/` (managed by `cargo ratcn`, see `ratcn.toml`). After adding a component, register it with `pub mod` in `src/ratcn.rs`, and keep `#![allow(dead_code)]` there.
- App-specific views live next to the screen that uses them, not in `ratcn/`.
- UI-triggered state changes go through the `AppMsg` enum in `src/app.rs`.
- No emojis in code, comments, or docs, except the search icon in `screens/settings/language_picker.rs`.

## Platform

- Use `#[cfg(target_arch = "wasm32")]` / `#[cfg(not(target_arch = "wasm32"))]`, never runtime checks.
- Keep `cfg` code in `src/platform/`, `src/audio.rs`, `src/data/storage.rs`, and the `KeyCode` re-export in `src/app.rs`.
- Put target-specific dependencies in the matching `[target.'cfg(...)'.dependencies]` block of `Cargo.toml`.
- `web.rs` must call `console_error_panic_hook::set_once()` first.
- No `println!` on WASM. Use `web_sys::console::log_1(...)`.

## Adding a Programming Language

1. Add a variant to `Language` in `src/data/languages.rs` (`#[strum(serialize = "...")]` for display names like `C++`).
2. Create `assets/snippets/<name>.json` following an existing file.
3. Add a match arm in `Language::snippet_data` with `include_str!`.
4. Run both `cargo check` commands and `cargo test`.
