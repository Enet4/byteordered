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
//! Both crates work well side by side.
//!
//! ```no_run
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
//! let mut data = get_data_source()?;
//! // read 1st byte
//! let b1 = data.read_u8()?;
//! // choose to read the following data in Little Endian if it's 0,
//! // otherwise read in Big Endian
//! let endianness = if b1 != 0 { LE } else { BE };
//! let mut rd = ByteOrdered::runtime(data, endianness);
//! let value: u32 = rd.read_u32()?;
//! # Ok(())
//! # }
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
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
