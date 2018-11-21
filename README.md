# byteordered

[![Build Status](https://travis-ci.org/Enet4/byteordered.svg?branch=master)](https://travis-ci.org/Enet4/byteordered) ![Minimum Rust Version 1.14](https://img.shields.io/badge/Minimum%20Rust%20Version-1.13-brightgreen.svg)

A library for reading and writing data in some byte order.

## Why yet another data parsing crate

While `byteorder` is well established in the Rust ecosystem, it relies on immaterial zero-constructor types for declaring the intended byte order. As such, it lacks a construct for reading and writing data in an endianness that is not originally known at compile time. For example, there are file formats in which the encoding may be either in little endian or in big endian order.

In addition, some users feel that adding the type parameter on each read/write method call is unnecessarily verbose and ugly.

Rather than building yet another new library, this crates aims to provide an alternative public API to `byteorder`, so that it becomes suitable for this particular case while preserving its familiarity and core capabilities.

## Using

An example follows. Please see [the documentation](https://docs.rs/byteordered) for more information.

```rust
use byteordered::{ByteOrdered, BE, LE};

let mut rd = ByteOrdered::le(get_data_source()?);
// read 1st byte
let b1 = rd.read_u8()?;
// choose to read the following data in Little Endian if it's 0,
// otherwise read in Big Endian
let endianness = if b1 != 0 { LE } else { BE };
let mut rd = rd.into_endianness(endianness);
let value: u32 = rd.read_u32()?;
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
