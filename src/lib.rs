//! This crate provides an alternative API for reading and writing data in an
//! endianness that might only be known at run-time. It encapsulates the
//! existing capabilities of the [`byteorder`] crate with an interface that
//! assumes an implicitly acknowledged byte order.
//!
//! The benefits of this API is two-fold. This crate supports use cases where
//! the data's endianness is only known during program execution, which may
//! happen in some formats and protocols. The same API can be used to reduce
//! redundancy by indicating the intended byte order once for the entire
//! routine, instead of one for each method call.
//!
//! The main contribution in this crate is the [`ByteOrdered`] wrapper type,
//! which infuses byte order information to a data source or destination (it
//! works for both readers and writers). Moreover, the [`Endian`] trait
//! contains multiple primitive data reading and writing methods, and the
//! [`Endianness`] type provides a basic enumerate for run-time information
//! only known at run-time.
//!
//! # Examples
//!
//! Use one of [`ByteOrdered`]'s constructors to create a wrapper with byte
//! order awareness.
//!
//! ```no_run
//! use byteordered::{ByteOrdered, Endianness};
//! # use std::error::Error;
//! # use std::io::Read;
//!
//! # fn get_data_source() -> Result<Box<Read>, Box<Error>> {
//! #     unimplemented!()
//! # }
//! # fn run() -> Result<(), Box<Error>> {
//! let mut rd = ByteOrdered::le(get_data_source()?); // little endian
//! // read a u16
//! let w = rd.read_u16()?;
//! // choose to read the following data in Little Endian if it's
//! // smaller than 256, otherwise read in Big Endian
//! let mut rd = rd.into_endianness(
//!     if w < 256 { Endianness::Little } else { Endianness::Big });
//! let value: u32 = rd.read_u32()?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! Both `byteordered` and [`byteorder`] work well side by side. You can use
//! [`byteorder`] in one part of
//! the routine, and wrap the reader or writer when deemed useful.
//!
//! ```
//! extern crate byteorder;
//! extern crate byteordered;
//!
//! use byteorder::ReadBytesExt;
//! use byteordered::{ByteOrdered, Endianness};
//! # use std::error::Error;
//! # use std::io::Read;
//!
//! # fn get_data_source() -> Result<Box<Read>, Box<Error>> { unimplemented!() }
//! # fn run() -> Result<(), Box<Error>> {
//! let b = 5;
//! // choose to read the following data in Little Endian if it's 0,
//! // otherwise read in Big Endian (what happens in this case)
//! let mut wt = ByteOrdered::runtime(
//!     Vec::new(),
//!     if b == 0 { Endianness::Little } else { Endianness::Big }
//! );
//! // write in this byte order
//! wt.write_u16(0xC000)?;
//! wt.write_u32(0)?;
//! // then invert the byte order
//! let mut wt = wt.into_opposite();
//! wt.write_u16(0xEEFF)?;
//! assert_eq!(&*wt.into_inner(), &[0xC0, 0, 0, 0, 0, 0, 0xFF, 0xEE]);
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! # Features
//!
//! `i128` enables reading and writing 128-bit integers, as in [`byteorder`].
//! This library requires the standard library (`no_std` is currently not
//! supported).
//!
//! [`byteorder`]: ../byteorder/index.html
//! [`Endian`]: trait.Endian.html
//! [`Endianness`]: enum.Endianness.html
//! [`ByteOrdered`]: struct.ByteOrdered.html
#![warn(missing_docs)]

pub extern crate byteorder;

mod base;
mod wrap;

pub use base::{Endian, Endianness, StaticEndianness};
pub use wrap::ByteOrdered;

#[cfg(test)]
mod tests {}
