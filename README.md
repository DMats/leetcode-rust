# LeetCode with Rust

A repository of my LeetCode answers in Rust.

## Rust Nightly

This repo uses the "nightly" rust toolchain for better tooling, in particular
rustfmt. Configure local clones like so:

```bash
rustup toolchain install nightly
rustup override set nightly
```

## rustfmt

The `rustfmt.toml` file contains the rustfmt config. Run rustfmt like so:

```bash
rustfmt file
```
