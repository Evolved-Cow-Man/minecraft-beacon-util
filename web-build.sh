curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
cargo install wasm-pack
wasm-pack build --release --target web
cargo clean
