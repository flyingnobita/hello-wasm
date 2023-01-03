# Compiling from Rust to WebAssembly Tutorial - Mozilla

[https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

## WASM Package

### Build WASM package in .pkg

```rust
wasm-pack build --target web
```

### Run the package

```bash
python3 -m http.server
```

## Convert Package for NPM

### Build

```rust
wasm-pack build --target bundler
```

### Link WASM package

#### Step 1

```bash
cd pkg
npm link
```

#### Step 2

```bash
cd site
npm link hello-wasm
```

### Start webpack web server

```bash
cd site
npm install
npm run serve
```

## Build with Rayon

```rust
wasm-pack build --target web
python3 -m http.server 8000
```
