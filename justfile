watch-wasm:
    cargo watch -w src -w Cargo.toml -s "wasm-pack build  --target web --out-dir web/dist"

serve:
    miniserve web --index index.html -p 8080 -i 127.0.0.1

[parallel]
dev: watch-wasm serve