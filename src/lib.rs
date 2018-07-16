//! This crate provides abstractions for reading and writing data in an
//! endianness only known at run-time. It may be regarded as an extension to
//! [`byteorder`] for the particular case of reading data in a byte order which
//! can only be identified during program execution, which may happen in some
//! formats and protocols.
//!
//! The main contribution in this crate is the [`Endianness`] type, which
//! contains multiple primitive data reading and writing methods.
//!
//! # Examples
//!
//! ```no_run
//! # extern crate byteorder;
//! # extern crate byteorder_runtime;
//! use byteorder::ReadBytesExt;
//! use byteorder_runtime::{BE, LE};
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
//! let value: u32 = endianness.read_u32(&mut data)?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```
//!
//! [`byteorder`]: ../byteorder/index.html
//! [`Endianness`]: enum.Endianness.html

pub extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result as IoResult, Write};

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

/// Enumerate for materializing the two kinds of machine byte order supported
/// by Rust.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Endianness {
    /// Little Endian
    Little,
    /// Big Endian
    Big,
}

impl From<LittleEndian> for Endianness {
    fn from(_: LittleEndian) -> Self {
        Endianness::Little
    }
}

impl From<BigEndian> for Endianness {
    fn from(_: BigEndian) -> Self {
        Endianness::Big
    }
}

macro_rules! impl_endianness_read {
    ($method:ident, $out:ty) => {
        impl Endianness {
            /// Read a primitive value with this endianness from the given source.
            pub fn $method<S>(&self, mut src: S) -> IoResult<$out>
            where
                S: Read,
            {
                match *self {
                    Endianness::Little => src.$method::<LittleEndian>(),
                    Endianness::Big => src.$method::<BigEndian>(),
                }
            }
        }
    };
}

impl_endianness_read!(read_i16, i16);
impl_endianness_read!(read_u16, u16);
impl_endianness_read!(read_i32, i32);
impl_endianness_read!(read_u32, u32);
impl_endianness_read!(read_i64, i64);
impl_endianness_read!(read_u64, u64);
impl_endianness_read!(read_f32, f32);
impl_endianness_read!(read_f64, f64);
#[cfg(feature = "i128")]
impl_endianness_read!(read_i128, i128);
#[cfg(feature = "i128")]
impl_endianness_read!(read_u128, u128);

macro_rules! impl_endianness_write {
    ($method:ident, $i:ty) => {
        impl Endianness {
            /// Write a primitive value with this endianness to the given write source.
            pub fn $method<S>(&self, mut src: S, v: $i) -> IoResult<()>
            where
                S: Write,
            {
                match *self {
                    Endianness::Little => src.$method::<LittleEndian>(v),
                    Endianness::Big => src.$method::<BigEndian>(v),
                }
            }
        }
    };
}

impl_endianness_write!(write_i16, i16);
impl_endianness_write!(write_u16, u16);
impl_endianness_write!(write_i32, i32);
impl_endianness_write!(write_u32, u32);
impl_endianness_write!(write_i64, i64);
impl_endianness_write!(write_u64, u64);
impl_endianness_write!(write_f32, f32);
impl_endianness_write!(write_f64, f64);
#[cfg(feature = "i128")]
impl_endianness_write!(write_i128, i128);
#[cfg(feature = "i128")]
impl_endianness_write!(write_u128, u128);

impl Endianness {
    /// Obtain this system's native endianness.
    ///
    /// On this platform, the function returns `Endianness::Little`.
    #[cfg(target_endian = "little")]
    pub fn native() -> Endianness {
        Endianness::Little
    }

    /// Obtain this system's native endianness.
    ///
    /// On this platform, the function returns `Endianness::Big`.
    #[cfg(target_endian = "big")]
    pub fn native() -> Endianness {
        Endianness::Big
    }

    /// Obtain the opposite endianness: Little Endian returns Big Endian and vice versa.
    pub fn to_opposite(self) -> Endianness {
        if self == Endianness::Little {
            Endianness::Big
        } else {
            Endianness::Little
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO write tests
}
