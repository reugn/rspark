# rspark ▁▂▆▇▁▄█▁
[![Build](https://github.com/reugn/rspark/actions/workflows/build.yml/badge.svg)](https://github.com/reugn/rspark/actions/workflows/build.yml)
[![Crate](https://img.shields.io/crates/v/rspark.svg)](https://crates.io/crates/rspark)

Sparklines library for Rust applications.  
A Rust port of [spark](https://github.com/holman/spark).

## Usage

Add `rspark` to your `Cargo.toml`:
```toml
[dependencies]
rspark = "0.2.0"
```

Example:
```rust
let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
let res = rspark::render(&v).unwrap();
assert_eq!("▁▂▆▇▁▄▁█▁▁", res);
```

## License
MIT