#!/bin/bash

# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu &&

# Windows x86_64
cargo build --release --target x86_64-pc-windows-gnu &&

# macOS x86_64
cargo build --release --target x86_64-apple-darwin &&

# macOS arch64
cargo build --release --target aarch64-apple-darwin &&

# Linux arch64
cargo build --release --target aarch64-unknown-linux-gnu &&







