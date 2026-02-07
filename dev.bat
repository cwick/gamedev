@echo off
REM Build WASM and start development server
echo Building WASM...
wasm-pack build --target web --out-dir web/dist
if %errorlevel% neq 0 (
    echo Build failed!
    exit /b %errorlevel%
)
echo Build successful! Starting server...
miniserve web --index index.html -p 8080 -i 127.0.0.1
