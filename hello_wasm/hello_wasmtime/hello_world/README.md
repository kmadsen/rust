
# About

https://docs.wasmtime.dev/tutorial-run-hello-world.html

Getting started with wasmtime

Installing
``` sh
$ curl https://wasmtime.dev/install.sh -sSf | bash
```

Building
```
$ cargo build --target wasm32-wasi
```

Running
```
$ wasmtime target/wasm32-wasi/debug/hello-world.wasm
```

Later we'll include wasmtime as a dependency in Rust
https://docs.wasmtime.dev/lang-rust.html
