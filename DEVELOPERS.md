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
cargo install uniffi-bindgen-cs --git https://github.com/NordSecurity/uniffi-bindgen-cs --tag v0.10.0+v0.29.4
uniffi-bindgen-cs --library target/release/libplabble_codec.so --out-dir src/
```