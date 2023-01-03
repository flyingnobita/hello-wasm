import("./node_modules/hello-wasm/hello_wasm.js").then((js) => {
  js.greet("WebAssembly with npm");
  // js.sum([1, 2, 3]);
});
