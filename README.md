# Rust Yew + Bootstrap Modal example

[`wasm-bindgen`](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)

[`Yew`](https://yew.rs)

[`Understanding Yew Part 1`](https://dev.to/rusty_sys_dev/understanding-yew-part-1-3cfn)

### To build and run this project you need this tools

-   [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
-   [`cargo-make`](https://github.com/sagiegurari/cargo-make)
-   [`node`](https://nodejs.org/en/)

### Order of commands

-   `cargo make build`
-   `cargo make serve`

Also you have to install JS libraries with `npm install`

### build

`wasm-pack build --target web` or `cargo make build`

### build for debugging

`wasm-pack build --debug --target web`

### local run

`npm run start` or `cargo make serve`

### test

`wasm-pack test --chrome --headless` or `cargo make test`

### clean

`cargo make clean`

### bundle for deploy

`npm run build` or `cargo make bundle`
