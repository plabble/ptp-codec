# Developers

Run all tests:
```sh
cargo test --all-features
```

## WASM installation
```sh
sudo dnf install openssl-devel
curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install cargo-generate
```

## WASM build
```sh
wasm-pack build --features wasm
# of:
wasm-pack build --target web --features wasm
```