# cfg-fake-main

## Requirements

- Nightly

  See <https://github.com/rust-lang/rust/issues/54726>.

## Installation

```shell
cargo add fake-main
```

## Usage

Assuming you have defined a `skip` feature at `Cargo.toml`:

```rust
#![feature(prelude_import)] // needed, idk why
#![feature(custom_inner_attributes)] // needed to use this macro cause it is a nightly feature
#![::fake_main::cfg_(not(feature = "skip"))]

/// The real main fn.
fn main() {
    println!("Real fn main");
}
```

```shell
$ cargo run

...

Real fn main
```

```shell
$ cargo run --features skip

...

thread 'main' panicked at .../src/main.rs:3:1:
not implemented: not(feature = "skip")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
