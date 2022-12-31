# Compiling from Rust to WebAssembly Tutorial - Mozilla

[https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

Build WASM package

```rust
wasm-pack build --target web
```

Build WASM package for NPM

```rust
wasm-pack build --target bundler
```

Start webpack web server

```bash
npm install
npm run serve
```
