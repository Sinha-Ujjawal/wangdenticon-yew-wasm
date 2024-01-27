## Wangdenticon-Yew

Rewrite of [Wangdenticon-WASM](https://github.com/Sinha-Ujjawal/wangdenticon-wasm) in [Yew](https://yew.rs/)

## Getting Started

1. Install [rustup](https://rustup.rs/)
2. Install WebAssembly target
```console
rustup target add wasm32-unknown-unknown
```
3. Install Trunk
```console
# note that this might take a while to install because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install --locked trunk
```
4. View your web application
```console
trunk serve
```

## Coyrights

Licensed under [@MIT](./LICENSE)
