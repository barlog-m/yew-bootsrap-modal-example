# Rust Yew + Bootstrap Modal example

[`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html).

[`Yew`](https://yew.rs).

[`Understanding Yew Part 1`](https://dev.to/rusty_sys_dev/understanding-yew-part-1-3cfn)

### To build and run this project you need this tools

- [`wasm-pack`](https://github.com/rustwasm/wasm-pack).
- [`rollup.js`](https://www.rollupjs.org/guide/en/).
- [`cargo-make`](https://github.com/sagiegurari/cargo-make).
- [`simple-http-server`](https://github.com/TheWaWaR/simple-http-server)

### Order of commands

- `cargo make build`
- `cargo make bundle`
- `cargo make serve`

Also you have to install JS libraries with `npm install`

### build

`wasm-pack build --target web` or `cargo make build`

### build for debugging

`wasm-pack build --debug --target web`

### bundle

`rollup ./main.js --format iife --file ./pkg/main.js` or `rolup -c` or `cargo make bundle`

### local run

`simple-http-server --index ./ --ip 127.0.0.1 --port 8080 --nocache --try-file ./index.html` or `cargo make serve`

### test

`wasm-pack test --chrome --headless` or `cargo make test`

### deploy

Access your generated build artifacts, `bundle.js` and `app_bg.wasm`, in ./pkg from your project's root directory.
