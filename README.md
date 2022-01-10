```bash
cargo build && readelf -s -W target/debug/libmonomorphize.rlib | rustfilt -h
```
