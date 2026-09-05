# Developers

Run all tests:
```sh
cargo test --all-features
```

## WASM/FFI installation
```sh
sudo dnf install openssl-devel
curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install cargo-generate
```

## WASM build
```sh
wasm-pack build --features wasm
# or:
wasm-pack build --target web --features wasm
```

## UniFFI C# bindings
```sh
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.11.0+v0.31.0
ln -sf /usr/local/rust/bin/uniffi-bindgen-cs /usr/local/bin/uniffi-bindgen-cs
cargo build --release
uniffi-bindgen-cs --library target/release/libplabble_codec.so --out-dir bindings/
```

## UniFFI Kotlin bindings
```sh
cargo build --release
cargo run --bin uniffi-bindgen --features uniffi/cli,unicli generate --library target/release/libplabble_codec.so --language kotlin --out-dir bindings/
```