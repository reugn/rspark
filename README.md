# rspark ▁▂▆▇▁▄▁█▁▁
Sparklines for Rust apps.  
Rust port of https://github.com/holman/spark

## Usage
```rust
let v = vec![2, 250, 670, 890, 2, 430, 11, 908, 123, 57];
let res = rspark::render(&v).unwrap();
assert_eq!("▁▂▆▇▁▄▁█▁▁", res);
```

## License
MIT