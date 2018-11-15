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
//! extern crate byteorder;
//! extern crate byteorder_runtime;
//! 
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
//! # fn main() {
//! # run().unwrap();
//! # }
//! ```
//!
//! [`byteorder`]: ../byteorder/index.html
//! [`Endianness`]: enum.Endianness.html
#![warn(missing_docs)]

pub extern crate byteorder;

use std::io::{Read, Write, Result as IoResult};

mod base;
mod wrap;

pub use base::Endianness;
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

/// Extension to the `Read` trait for reading bytes in an implicitly assumed
/// byte order. It resembles [`byteorder::ReadBytesExt`], differing in the lack
/// of a type parameter for defining the expected byte order. Instead, the
/// endianness is expected to be implicitly known by the underlying reader.
/// 
/// When adding this trait, it is recommended not to include
/// [`byteorder::ReadBytesExt`], otherwise you will have to disambiguate every
/// call.
///
/// Blanket implementations will decode data in the system's native byte order.
/// 
/// [`byteorder::ReadBytesExt`]: ../byteorder/trait.ReadBytesExt.html
pub trait ReadBytesExt: byteorder::ReadBytesExt {
    /// Reads a signed 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 16 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder_runtime::{ByteOrdered, ReadBytesExt};
    ///
    /// let mut rdr = ByteOrdered::be(Cursor::new(vec![0x00, 0xc1, 0xff, 0x7c]));
    /// assert_eq!(193, rdr.read_i16().unwrap());
    /// assert_eq!(-132, rdr.read_i16().unwrap());
    /// ```
    fn read_i16(&mut self) -> IoResult<i16> {
        byteorder::ReadBytesExt::read_i16::<byteorder::NativeEndian>(self)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u16(&mut self) -> IoResult<u16> {
        byteorder::ReadBytesExt::read_u16::<byteorder::NativeEndian>(self)
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_i32(&mut self) -> IoResult<i32> {
        byteorder::ReadBytesExt::read_i32::<byteorder::NativeEndian>(self)
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u32(&mut self) -> IoResult<u32> {
        byteorder::ReadBytesExt::read_u32::<byteorder::NativeEndian>(self)
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_i64(&mut self) -> IoResult<i64> {
        byteorder::ReadBytesExt::read_i64::<byteorder::NativeEndian>(self)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_u64(&mut self) -> IoResult<u64> {
        byteorder::ReadBytesExt::read_u64::<byteorder::NativeEndian>(self)
    }

    /// Reads a signed 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[cfg(feature = "i128")]
    fn read_i128(&mut self) -> IoResult<i128> {
        byteorder::ReadBytesExt::read_i128::<byteorder::NativeEndian>(self)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[cfg(feature = "i128")]
    fn read_u128(&mut self) -> IoResult<u128> {
        byteorder::ReadBytesExt::read_u128::<byteorder::NativeEndian>(self)
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number from
    /// the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_f32(&mut self) -> IoResult<f32> {
        byteorder::ReadBytesExt::read_f32::<byteorder::NativeEndian>(self)
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number from
    /// the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    fn read_f64(&mut self) -> IoResult<f64> {
        byteorder::ReadBytesExt::read_f64::<byteorder::NativeEndian>(self)
    }
}

/// Extension to the `Write` trait for writing bytes in an implicitly assumed
/// byte order. It resembles [`byteorder::WriteBytesExt`], differing in the
/// lack of a type parameter for defining the expected byte order. Instead, the
/// endianness is expected to be implicitly known by the underlying writer.
/// 
/// When adding this trait, it is recommended not to include
/// [`byteorder::WriteBytesExt`], otherwise you will have to disambiguate every
/// call.
///
/// Blanket implementations will encode data in the system's native byte order.
/// 
/// [`byteorder::WriteBytesExt`]: ../byteorder/trait.WriteBytesExt.html
pub trait WriteBytesExt: byteorder::WriteBytesExt {
    /// Writes a signed 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    ///
    /// # Examples
    ///
    /// Write signed 16 bit big-endian integers to a `Write`:
    ///
    /// ```rust
    /// use byteorder_runtime::{ByteOrdered, WriteBytesExt};
    ///
    /// let mut wtr = ByteOrdered::be(Vec::new());
    /// wtr.write_i16(193).unwrap();
    /// wtr.write_i16(-132).unwrap();
    /// assert_eq!(wtr.into_inner(), b"\x00\xc1\xff\x7c");
    /// ```
    fn write_i16(&mut self, x: i16) -> IoResult<()> {
        byteorder::WriteBytesExt::write_i16::<byteorder::NativeEndian>(self, x)
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u16(&mut self, x: u16) -> IoResult<()> {
        byteorder::WriteBytesExt::write_u16::<byteorder::NativeEndian>(self, x)
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_i32(&mut self, x: i32) -> IoResult<()> {
        byteorder::WriteBytesExt::write_i32::<byteorder::NativeEndian>(self, x)
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u32(&mut self, x: u32) -> IoResult<()> {
        byteorder::WriteBytesExt::write_u32::<byteorder::NativeEndian>(self, x)
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_i64(&mut self, x: i64) -> IoResult<()> {
        byteorder::WriteBytesExt::write_i64::<byteorder::NativeEndian>(self, x)
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_u64(&mut self, x: u64) -> IoResult<()> {
        byteorder::WriteBytesExt::write_u64::<byteorder::NativeEndian>(self, x)
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[cfg(feature = "i128")]
    fn write_i128(&mut self, x: i128) -> IoResult<()> {
        byteorder::WriteBytesExt::write_i128::<byteorder::NativeEndian>(self, x)
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[cfg(feature = "i128")]
    fn write_u128(&mut self, x: u128) -> IoResult<()> {
        byteorder::WriteBytesExt::write_u128::<byteorder::NativeEndian>(self, x)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number to
    /// the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_f32(&mut self, x: f32) -> IoResult<()> {
        byteorder::WriteBytesExt::write_f32::<byteorder::NativeEndian>(self, x)
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number to
    /// the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    fn write_f64(&mut self, x: f64) -> IoResult<()> {
        byteorder::WriteBytesExt::write_f64::<byteorder::NativeEndian>(self, x)
    }
}


#[cfg(test)]
mod tests {

}