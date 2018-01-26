# rust-jieba

[![Build Status](https://travis-ci.org/messense/rust-jieba.svg?branch=master)](https://travis-ci.org/messense/rust-jieba)
[![codecov](https://codecov.io/gh/messense/rust-jieba/branch/master/graph/badge.svg)](https://codecov.io/gh/messense/rust-jieba)
[![Crates.io](https://img.shields.io/crates/v/rust-jieba.svg)](https://crates.io/crates/rust-jieba)
[![docs.rs](https://docs.rs/rust-jieba/badge.svg)](https://docs.rs/rust-jieba/)

[cppjieba](https://github.com/yanyiwu/cppjieba) Rust binding

## Installation

Add it to your `Cargo.toml`:

```toml
[dependencies]
rust-jieba = "0.1"
```

## Example

```rust
extern crate rust_jieba;

use rust_jieba::Jieba;

fn main() {
    let jieba = Jieba::from_dir("cjieba-sys/cppjieba-cabi/cppjieba/dict");
    let words = jieba.cut("南京市长江大桥", true);
    assert_eq!(vec!["南京市", "长江大桥"], words);
}
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
