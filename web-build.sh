curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"
rustup update
cargo install wasm-pack
wasm-pack build --release --target web
cargo clean
