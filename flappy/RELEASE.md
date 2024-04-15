# Release

## Linux

EZ: `cargo build --release`

## Windows

From Linux, we can use [cargo-xwin](https://github.com/rust-cross/cargo-xwin)

```
cargo install cargo-xwin
rustup target add x86_64-pc-windows-msvc
cargo xwin build --target x86_64-pc-windows-msvc --release
```
