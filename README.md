update toolchain and libraries:
* `rustup update`
* `cargo install wasm-pack`
* `cargo update`

build used for the web:
* `wasm-pack build --release --target web && cargo clean`
