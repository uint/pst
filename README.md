# pb
Share code or text without leaving the command line.

## Install
Requirements: [Rust and Cargo](https://www.rust-lang.org/tools/install)!

```sh
> cargo install pb
```

If you can't invoke `pb` after that, make sure to read the section in the link above about `PATH`.

## Usage
Get help:
```sh
pb --help
```

Paste a file to the default bin:
```sh
pb Cargo.toml
```

Use a different bin:
```sh
pb -b clbin Cargo.toml
```

Pipe your output into a bin:
```sh
echo "There is no place like /home" | pb
```
