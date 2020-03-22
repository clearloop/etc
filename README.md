# etc

[![Rust](https://github.com/clearloop/etc/workflows/Rust/badge.svg)](https://github.com/clearloop/etc)
[![crate](https://img.shields.io/crates/v/etc.svg)](https://crates.io/crates/etc)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/etc/)
[![dependency status](https://deps.rs/repo/github/clearloop/etc/status.svg)](https://deps.rs/repo/github/clearloop/etc)
[![downloads](https://img.shields.io/crates/d/etc.svg)](https://crates.io/crates/etc)
[![LICENSE](https://img.shields.io/crates/l/etc.svg)](https://choosealicense.com/licenses/mit/)

It's time to bundle etc for your awesome project!

```rust
use etc::Etc;

fn main() {
    // config root path
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".etc");

    // generate ~/.etc dir
    Etc::new(&dir).unwrap();

    // check if root exits and remove it
    assert!(dir.exists());
    assert!(::std::fs::remove_dir(dir).is_ok());
}
```

## LICENSE

MIT
