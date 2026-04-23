curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup update
cargo install wasm-pack
wasm-pack build --release --target web
cargo clean
