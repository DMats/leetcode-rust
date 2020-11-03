# LeetCode with Rust

A repository of my LeetCode answers in Rust.

## VSCode

This repo is developed in VSCode using the `rust-analyzer` extension. The
workspace file `leetcode-rust.code-workspace` configures the `rust-analyzer`
extension to automatically download "nightly" toolchain and run `rustfmt` on
save, using the config options stored in `rustfmt.toml`.

## Developing outside VSCode

When developing outside of VSCode, you'll need to manage your toolchain and run
rustfmt from the command line.

### Nightly Rust

This repo uses the "nightly" rust toolchain for better tooling, in particular
rustfmt. Configure local clones like so:

```bash
rustup toolchain install nightly
rustup override set nightly
```

### rustfmt

The `rustfmt.toml` file contains the rustfmt config. Run rustfmt from the
command line like so:

```bash
rustfmt file
```
