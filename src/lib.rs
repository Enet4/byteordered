//! This crate provides abstractions for reading and writing data in an
//! endianness only known at run-time. It may be regarded as an extension to
//! [`byteorder`] for the particular case of reading data in a byte order which
//! can only be identified during program execution, which may happen in some
//! formats and protocols.
//!
//! The main contribution in this crate is the [`ByteOrdered`] wrapper type,
//! which implicitly assigns byte order information to a data source or
//! destination (it works for both readers and writers).
//! Moreover, the [`Endian`] trait contains multiple primitive data reading and
//! writing methods, and the [`Endianness`] type provides a basic enumerate
//! for run-time information only known at run-time.
//!
//! # Examples
//! 
//! ```no_run
//! extern crate byteorder_runtime;
//!
//! use byteorder_runtime::{ByteOrdered, BE, LE, Endian};
//! # use std::error::Error;
//! # use std::io::Read;
//!
//! # fn get_data_source() -> Result<Box<Read>, Box<Error>> { unimplemented!() }
//!
//! # fn run() -> Result<(), Box<Error>> {
//! let mut rd = ByteOrdered::le(get_data_source()?); // little endian
//! // read a u16
//! let w = rd.read_u16()?;
//! // choose to read the following data in Little Endian if it's
//! // smaller than 256, otherwise read in Big Endian
//! let mut rd = rd.into_endianness(if w < 256 { LE } else { BE });
//! let value: u32 = rd.read_u32()?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! Both crates work well side by side. You can use `byteorder` in one part of
//! the routine, and wrap the reader or writer when deemed useful.
//!
//! ```
//! extern crate byteorder;
//! extern crate byteorder_runtime;
//!
//! use byteorder::ReadBytesExt;
//! use byteorder_runtime::{ByteOrdered, BE, LE, Endian};
//! # use std::error::Error;
//! # use std::io::Read;
//!
//! # fn get_data_source() -> Result<Box<Read>, Box<Error>> { unimplemented!() }
//!
//! # fn run() -> Result<(), Box<Error>> {
//! let b = 5;
//! // choose to read the following data in Little Endian if it's 0,
//! // otherwise read in Big Endian
//! let mut wt = ByteOrdered::runtime(Vec::new(), if b < 256 { LE } else { BE });
//! // write in this byte order, 
//! wt.write_u16(0x00C0)?;
//! wt.write_u32(0)?;
//! // then invert the byte order
//! let mut wt = wt.into_opposite();
//! wt.write_u16(0xFFEE)?;
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
//! `i128` enables reading and writing 128-bit integers, as in the `byteorder`
//! crate. This library currently requires the standard library (`no_std` is
//! not supported). 
//!
//! [`byteorder`]: ../byteorder/index.html
//! [`Endianness`]: enum.Endianness.html
//! [`ByteOrdered`]: struct.ByteOrdered.html
#![warn(missing_docs)]

pub extern crate byteorder;

mod base;
mod wrap;

pub use base::{Endian, Endianness, StaticEndianness};
pub use wrap::ByteOrdered;

/// Short-hand for Big Endian run-time endianness.
/// Not to be mistaken with [`byteorder::BE`].
///
/// [`byteorder::BE`]: ../byteorder/BE.t.html
pub const BE: Endianness = Endianness::Big;

/// Short-hand for Little Endian run-time endianness.
/// Not to be mistaken with [`byteorder::LE`].
///
/// [`byteorder::LE`]: ../byteorder/LE.t.html
pub const LE: Endianness = Endianness::Little;

#[cfg(test)]
mod tests {}
