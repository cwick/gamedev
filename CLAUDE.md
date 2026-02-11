# AI Agent Guide

Rust WebAssembly game engine with multiple games. Rust handles all game logic; JS handles rendering and input.

## Build Rule

Local tooling now runs `wasm-pack` automatically (e.g., via `just dev`/`cargo watch`), so manual rebuilds are no longer required. The browser still loads WASM from `web/dist/`, but you can rely on the dev workflow to keep it up to date.

## Architecture

```
src/lib.rs                  → re-exports wasm_api
src/wasm_api.rs             → #[wasm_bindgen] FFI layer (GameState struct exposed to JS)
src/engine/                 → ECS engine (components, systems, world, schedule, resources)
src/games/                  → game definitions (registry, per-game modules)
src/games/pong/             → Pong game logic and resources
src/games/arkanoid/         → Arkanoid game logic
web/index.html              → Pong: canvas rendering, input handling, game loop
web/arkanoid/index.html     → Arkanoid: canvas rendering, input handling, game loop
web/app.js                  → game launcher / shared app shell
web/input.js                → shared input handling
```

**Data flow is strictly one-way: JS calls Rust, never the reverse.** Rust does NOT import or call any JS functions. Game state is communicated via a `[f32; 12]` snapshot array that JS reads as a `Float32Array`.

## Rules

- Do NOT add dependencies to `Cargo.toml` without discussion
- Do NOT edit anything in `web/dist/` or `target/` (auto-generated)
- Do NOT use `web-sys` or `extern "C"` blocks — Rust only exports, never imports
- Do NOT commit build artifacts (`target/`, `web/dist/`, `Cargo.lock` are gitignored)
- Run `cargo clippy` and `cargo fmt` before committing
- Prefer self-documenting code over comments. Only add comments when the code cannot be made clearer

## Code Quality

All code must pass `cargo clippy` without warnings and be formatted with `cargo fmt`. Run `cargo test` after making changes to verify correctness.

## Commit Messages

Keep commit messages to one or two lines. The first line should be a concise summary of the change. If additional context is needed, add a blank line and then provide details on subsequent lines.
