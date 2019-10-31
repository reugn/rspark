# rspark ▁▂▆▇▁▄█▁
[![Build Status](https://travis-ci.org/reugn/rspark.svg?branch=master)](https://travis-ci.org/reugn/rspark)
[![Crate](https://img.shields.io/crates/v/rspark.svg)](https://crates.io/crates/rspark)

Sparklines for Rust apps.  
Rust port of https://github.com/holman/spark

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
rspark = "0.1.0"
```

Example:
```rust
let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
let res = rspark::render(&v).unwrap();
assert_eq!("▁▂▆▇▁▄▁█▁▁", res);
```

## License
MIT