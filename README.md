# LifeStats

Stats about your life

## Building with Yew.rs

To build the WASM locally
- Install rust, rustup and cargo
```bash
rustup target add wasm32-unknown-unknown
# note that this might take a while to install
cargo install --locked trunk
trunk build --release
trunk serve
```

Your website will now be in `dist/` directory.

## Github-pages

The compiled static files live in the `gh-pages` branch, and all the code lives in the `master` branch.

## Contribution

All contributions are welcome!

Quick Start

  - Fork the repo
  - Make your changes
  - Open a PR

:)
