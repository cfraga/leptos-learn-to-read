
# Learn to Read

Simple web app that rotates through a list of words written only with a specific set of letters, to help my kid practice reading as he learns new letters. Built in rust/leptos because I wanna gain experience with the lang/framework.

## Features/Wishlist
- [X] Select allowed chars
- [X] dont let the same word repeat on a play session
- [X] difficulty levels (increasing length of words)
- [X] counter of read words
- [X] counter of read words resets when current session is over
- [X] link to online dictionary
- [X] localstorage to store game settings
- [X] Preprocess file instead of loading it on every request
- [X] on screen keyboard for letters
- [X] support characters with accent marks ('e.g. é') when filtering by their corresponding non accented character
- [ ] sentence mode instead of single words
- [ ] leaderboards/streak/high score calculation
- [ ] time attack mode
- [ ] fetch words from API instead of static medium


## Running your project

`cargo leptos watch`  
By default, you can access your local project at `http://localhost:3000`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
learn-to-read
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="learn-to-read"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Notes about CSR and Trunk:
Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:

`trunk serve --open --features csr`

This may be useful for integrating external tools which require a static site, e.g. `tauri`.
