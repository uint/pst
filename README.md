# pst
Share code or text without leaving the command line.

## Install
Requirements: [Rust and Cargo](https://www.rust-lang.org/tools/install)!

```sh
git clone https://github.com/uint/pst.git
cargo install --path pst
```

If you can't invoke `pst` after that, make sure to read the section in the link above about `PATH`.

When/if this project becomes a little more mature, I'll start uploading releases to crates.io, I promise.

## Usage
Get help:
```sh
pst --help
```

Paste a file to the default bin:
```sh
pst Cargo.toml
```

Use a different bin:
```sh
pst -b clbin Cargo.toml
```

Pipe your output into a bin:
```sh
echo "There is no place like /home" | pst
```
