# Leptos QRScanner CSR example

## Requirements
If you don't have Rust nightly, you can install it with
```sh
rustup toolchain install nightly --allow-downgrade
```
You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```

## Developing
To develop this CSR example, runningTo develop this CSR example, running
```sh
trunk serve --port 3000 --open
```
will open your app in your default browser at `http://localhost:3000`.

## Deploying
To build this CSR example for release, use the command
```sh
trunk build --release
```
This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

## More infos
- [Leptos](https://github.com/leptos-rs/leptos)
- [Trunk](https://github.com/trunk-rs/trunk)
- [Trunk instructions](https://trunkrs.dev/assets/)
- [Deploy CSR](https://book.leptos.dev/deployment/csr.html)
