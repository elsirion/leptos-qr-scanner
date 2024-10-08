# Leptos QRScanner SSR example

## Requirements
If you don't have `cargo-leptos` installed you can install it with
```sh
cargo install cargo-leptos --locked
```
You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```
By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.
1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Developing
To develop this CSR example, running
```sh
cargo leptos watch
```
will run your app at `http://localhost:3000`.

## Deploying
```sh
cargo leptos build --release
```
Will generate your server binary in target/server/release and your site package in target/site

## Testing
Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.
```sh
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

## More infos
- [Leptos](https://github.com/leptos-rs/leptos)
- [Cargo Leptos](https://github.com/leptos-rs/cargo-leptos)
- [SSR chapter](https://book.leptos.dev/ssr/index.html)
- [Deploy SSR](https://book.leptos.dev/deployment/ssr.html)
