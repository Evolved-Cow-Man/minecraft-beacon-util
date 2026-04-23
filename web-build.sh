rustup update
cargo install wasm-pack
wasm-pack build --release --target web
cargo clean
