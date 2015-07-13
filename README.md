# capsize

[![Build Status](https://travis-ci.org/softprops/capsize.svg?branch=master)](https://travis-ci.org/softprops/capsize)

> Conversions between units of capacity

## docs

Find them [here](http://softprops.github.io/capsize)

## install

Add the following to your `Cargo.toml`

```toml
[dependencies]
capsize = "0.1.0"
```

## usage

```rust
extern crate capsize;

use capize::Capacity;

pub fn main() {
  // get the equivilent number of bytes in 4 megabytes
  let bytes = 4.megabytes();
  println!("4 megabytes is {} bytes", bytes);
  // resolve byte size to the closest human form
  println!(bytes.capacity()); // 4.0K
}
```

Doug Tangren (softprops) 2015
