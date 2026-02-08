# AI Agent Guide

Rust WebAssembly Pong game. Rust handles all game logic; JS handles rendering and input.

## Build Rule

**After ANY change to Rust code, you MUST run:**
```bash
wasm-pack build --target web --out-dir web/dist
```
The browser only loads compiled WASM from `web/dist/`. Rust changes have no effect until you rebuild. Use `--dev` for faster builds during iteration.

## Architecture

```
src/lib.rs          → re-exports wasm_api
src/wasm_api.rs     → #[wasm_bindgen] FFI layer (GameState struct exposed to JS)
src/engine.rs       → pure game simulation logic (no WASM dependencies)
src/engine/tests.rs → unit tests for engine
web/index.html      → canvas rendering, input handling, game loop
```

**Data flow is strictly one-way: JS calls Rust, never the reverse.** Rust does NOT import or call any JS functions. Game state is communicated via a `[f32; 12]` snapshot array that JS reads as a `Float32Array`.

## Rules

- Do NOT add dependencies to `Cargo.toml` without discussion
- Do NOT edit anything in `web/dist/` or `target/` (auto-generated)
- Do NOT use `web-sys` or `extern "C"` blocks — Rust only exports, never imports
- Do NOT commit build artifacts (`target/`, `web/dist/`, `Cargo.lock` are gitignored)
- Run `cargo clippy` and `cargo fmt` before committing
- Prefer self-documenting code over comments

## Code Quality

All code must pass `cargo clippy` without warnings and be formatted with `cargo fmt`.
