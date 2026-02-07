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
**Rust exports only** - This project focuses on exporting Rust functions to JavaScript. We do NOT import browser APIs into Rust (no web-sys). Any DOM manipulation or browser interaction happens on the JavaScript side.

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
- **Imports:** Can import JS functions via `extern "C"` blocks (e.g., console.log)
- **No web-sys:** Do not add web-sys or call browser APIs directly from Rust

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

### Server Management

**Check if server is running:**
```bash
netstat -ano | findstr :8080
# OR
tasklist | findstr miniserve
```

**Start server (if not running):**
```bash
cd web
miniserve . --index index.html -p 8080
```

**Stop server:**
```bash
taskkill /F /IM miniserve.exe
```

**View server logs:**
```bash
# If running as background task (check task ID from context)
cat C:\Users\carme\AppData\Local\Temp\claude\c--Users-carme-dev-gamedev\tasks\*.output
```

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

### Importing JavaScript Functions into Rust

If you need to call JS functions from Rust:

```rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // For window.alert:
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn my_function() {
    log("Hello from Rust!");
    alert("Alert from Rust!");
}
```

**Do NOT** add web-sys dependency for this - only import what you need via `extern "C"`.

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

### Browser Console Testing

Open DevTools (F12) and test functions directly:

```javascript
// After page loads, WASM module is available globally
const result = hello_string();
console.log(result);

const greeting = greet("Claude");
console.log(greeting);
```

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

## Troubleshooting

### WASM Binary Not Loading

**Symptoms:** Page loads but functions don't work, console shows "init is not a function"

**Checks:**
1. Verify files exist: `ls web/dist/`
2. Check file size: `ls -lh web/dist/*.wasm` (should be 20-50 KB)
3. View Network tab in DevTools: Confirm `.wasm` file loads (status 200)
4. Check CORS: Must use HTTP server, not `file://` protocol

### Changes Not Reflected in Browser

**Symptoms:** Edited Rust code but browser shows old behavior

**Solutions:**
1. Rebuild: `wasm-pack build --target web --out-dir web/dist`
2. Hard refresh: Ctrl+Shift+R (Chrome/Edge) or Ctrl+F5 (Firefox)
3. Clear cache: Open DevTools → Network tab → Disable cache (while DevTools open)
4. Verify timestamps: `ls -lt web/dist/` (check if files are recent)

### Compilation Errors

**If Rust doesn't compile:**
1. Read error message carefully - Rust errors are descriptive
2. Check syntax: Missing semicolons, wrong types, etc.
3. Verify `#[wasm_bindgen]` is on the line immediately before function
4. Ensure all exported functions are `pub`

**Common issues:**
- Forgot `pub` keyword on exported function
- Used unsupported types (e.g., `Vec<String>` without proper serialization)
- Syntax error (missing semicolon, brace, etc.)

### Server Won't Start

**Port already in use:**
```bash
# Find process using port 8080
netstat -ano | findstr :8080

# Kill it
taskkill /F /PID <process_id>
```

**miniserve not found:**
```bash
cargo install miniserve
```

## Project-Specific Notes

### Current Exported Functions

As of initial setup, these functions are available:

1. **`hello_console()`** - Logs message to browser console
2. **`hello_string()`** - Returns greeting string
3. **`greet(name: &str)`** - Returns personalized greeting

### When Adding Dependencies

**Think twice before adding dependencies:**
- This is a minimal project - keep it simple
- Avoid web-sys unless there's a strong reason
- Each dependency increases WASM binary size

**If you must add a dependency:**
1. Add to `Cargo.toml` under `[dependencies]`
2. Rebuild: `wasm-pack build --target web --out-dir web/dist`
3. Check binary size: `ls -lh web/dist/*.wasm`
4. Document the reason in commit message

### Browser Compatibility

**Tested on:** Chrome, Edge, Firefox (all modern versions)

**Requirements:**
- ES module support (all modern browsers)
- WebAssembly support (all modern browsers)
- LocalStorage/CORS: Must use HTTP server

**Not supported:**
- Internet Explorer (no WASM support)
- Very old browsers (no ES modules)

## Quick Reference

### Essential Commands

```bash
# Build WASM
wasm-pack build --target web --out-dir web/dist

# Start server
cd web && miniserve . --index index.html -p 8080

# Stop server
taskkill /F /IM miniserve.exe

# Check what's running on port 8080
netstat -ano | findstr :8080

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

### URLs

| Purpose | URL |
|---------|-----|
| Local server | http://localhost:8080 |
| Network access | http://127.0.0.1:8080 |
| LAN access | http://192.168.50.227:8080 |

## Additional Resources

- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)

---

**Last Updated:** Initial project setup
**Maintainer:** AI-assisted development
**Project Status:** Active development
