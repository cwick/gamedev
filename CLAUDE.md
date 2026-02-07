# AI Agent Development Guide

This document provides guidance for AI agents (like Claude) working on this Rust WebAssembly project.

## Project Overview

**Type:** Rust → WebAssembly → Browser pipeline
**Purpose:** Export Rust functions to JavaScript for use in web browsers
**Approach:** Export-only (no web-sys), using wasm-pack with vanilla HTML/JS
**Server:** miniserve on port 8080

## Architecture

```
Rust (src/lib.rs)
    ↓ [wasm-pack build]
WebAssembly (web/dist/*.wasm + *.js)
    ↓ [ES modules]
Browser (web/index.html)
```

### Key Principle
**Strictly one-way dependency: JS → Rust** - JavaScript calls Rust functions, never the reverse. Rust code does NOT call into JavaScript (no `extern "C"` blocks, no console.log imports, nothing). This keeps the architecture simple and the dependency direction clear.

## Project Structure

```
c:\Users\carme\dev\gamedev\
├── .git/                          # Git repository
├── .gitignore                     # Ignores: target/, web/dist/, Cargo.lock
├── Cargo.toml                     # Only dependency: wasm-bindgen = "0.2"
├── src/
│   └── lib.rs                     # Rust functions exported to JS
├── target/                        # Rust build artifacts (gitignored)
└── web/
    ├── index.html                 # Web interface (loads WASM via ES modules)
    └── dist/                      # wasm-pack output (gitignored)
        ├── *.wasm                 # Compiled WebAssembly binary
        ├── *.js                   # JavaScript glue code
        └── *.d.ts                 # TypeScript definitions
```

## Critical Files

### [Cargo.toml](Cargo.toml)
- **Must have:** `crate-type = ["cdylib", "rlib"]` for WASM compilation
- **Only dependency:** `wasm-bindgen = "0.2"`
- **Profile:** `opt-level = "s"` for smaller binaries

### [src/lib.rs](src/lib.rs)
- **Pattern:** Export public functions with `#[wasm_bindgen]` attribute
- **No imports:** Do NOT import JS functions via `extern "C"` blocks
- **No web-sys:** Do not add web-sys or call browser APIs directly from Rust
- **Strictly exports only:** Rust only exports functions, never calls into JavaScript

### [web/index.html](web/index.html)
- **Module script:** Uses `<script type="module">` for ES module imports
- **WASM loading:** `import init, { functions } from './dist/gamedev_wasm_hello.js'`
- **Initialization:** Must call `await init()` before using Rust functions

## Development Workflow

### Standard Rebuild Process

When modifying Rust code:

```bash
# 1. Edit src/lib.rs

# 2. Rebuild WASM (from project root)
wasm-pack build --target web --out-dir web/dist

# 3. Hard refresh browser (Ctrl+Shift+R or Ctrl+F5)
```

**Note:** The server doesn't need to restart - it serves files as-is. Just rebuild and refresh.

### Build Options

**Development build (faster, larger, with debug symbols):**
```bash
wasm-pack build --dev --target web --out-dir web/dist
```

**Production build (slower, smaller, optimized):**
```bash
wasm-pack build --release --target web --out-dir web/dist
```

Default (no flag) uses release mode.

## Common Tasks

### Adding a New Rust Function

1. **Add to [src/lib.rs](src/lib.rs):**
   ```rust
   #[wasm_bindgen]
   pub fn my_function(input: &str) -> String {
       format!("Processed: {}", input)
   }
   ```

2. **Rebuild:**
   ```bash
   wasm-pack build --target web --out-dir web/dist
   ```

3. **Update [web/index.html](web/index.html):**
   ```javascript
   import init, { hello_console, hello_string, greet, my_function } from './dist/gamedev_wasm_hello.js';

   // Use it:
   const result = my_function("test");
   ```

### Supported Types for WASM Export

**Primitive types:** `i32`, `u32`, `i64`, `u64`, `f32`, `f64`, `bool`
**String types:** `&str` (input), `String` (output)
**Complex types:** Use `wasm-bindgen` guide for structs, arrays, etc.

**Example:**
```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn concatenate(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}
```

## Testing & Verification

### Quick Verification Checklist

After making changes:

- [ ] Rust compiles without errors: `wasm-pack build --target web --out-dir web/dist`
- [ ] Files exist in `web/dist/`: `*.wasm`, `*.js`, `*.d.ts`
- [ ] Browser loads page: Navigate to http://localhost:8080
- [ ] Check browser console (F12) for JavaScript errors
- [ ] Test new/modified functions via UI or browser console

### Common Error Messages

**"Cannot find module './dist/gamedev_wasm_hello.js'"**
- Cause: WASM not built yet
- Fix: Run `wasm-pack build --target web --out-dir web/dist`

**"Module parse failed"**
- Cause: Browser doesn't support ES modules or wrong import path
- Fix: Ensure `<script type="module">` and correct relative path

**"Uncaught TypeError: ... is not a function"**
- Cause: Function not exported from Rust or typo in JS
- Fix: Check `#[wasm_bindgen]` attribute and function name

**"WebAssembly module is being compiled"**
- Cause: WASM still loading (async)
- Fix: Ensure `await init()` is called before using functions

## Important Conventions

### Naming

- **Rust functions:** Use `snake_case` (e.g., `hello_console`, `my_function`)
- **JavaScript imports:** Automatically matches Rust names
- **Project name:** Must match in Cargo.toml and generated file names

### File Editing Rules

**Never edit manually:**
- `web/dist/*` - Auto-generated by wasm-pack
- `target/*` - Rust build artifacts
- `Cargo.lock` - Auto-generated (and gitignored)

**Safe to edit:**
- `src/lib.rs` - Your Rust code
- `web/index.html` - Web interface
- `Cargo.toml` - Only if adding metadata (NOT dependencies unless discussed)

### Git Workflow

**Before committing:**
1. Ensure `.gitignore` excludes: `target/`, `web/dist/`, `Cargo.lock`
2. Only commit source files: `src/`, `web/index.html`, `Cargo.toml`, `.gitignore`
3. Do NOT commit build artifacts

## Quick Reference

### Essential Commands

```bash
# Build WASM
wasm-pack build --target web --out-dir web/dist

# View generated files
ls web/dist/
```

### File Locations

| What | Where |
|------|-------|
| Rust source | `src/lib.rs` |
| Web interface | `web/index.html` |
| WASM output | `web/dist/*.wasm` |
| JS glue code | `web/dist/gamedev_wasm_hello.js` |
| Project config | `Cargo.toml` |
| Git config | `.gitignore` |

### URL

| Purpose | URL |
|---------|-----|
| Local server | http://localhost:8080 |

## Additional Resources

- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)

---
