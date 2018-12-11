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
//! let mut rd = rd.into_endianness(Endianness::le_iff(w < 256));
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
//! [`byteorder`]: https://docs.rs/byteorder
//! [`Endian`]: trait.Endian.html
//! [`Endianness`]: enum.Endianness.html
//! [`ByteOrdered`]: struct.ByteOrdered.html
#![warn(missing_docs)]

pub extern crate byteorder;

mod base;
mod wrap;

pub use base::{Endian, Endianness, StaticEndianness};
pub use wrap::ByteOrdered;


/// Creates a scope for reading or writing with run-time byte order awareness.
/// 
/// The condition of whether to read or write data in big endian or little
/// endian is evaluated only once, at the beginning of the scope. The given
/// expression `$e` is then monomorphized for both cases.
/// 
/// # Examples
/// 
/// Pass something that implements `Read` or `Write`, and something which
/// evaluates to a byte order descriptor (typically [`Endianness`]).
/// What follows is a pseudo-closure declaration exposing the same value
/// with the expected byte order awareness.
///  
/// ```
/// ##[macro_use] extern crate byteordered;
/// use byteordered::Endianness;
/// # fn get_endianness() -> Endianness { Endianness::Little }
/// # fn run() -> Result<(), ::std::io::Error> {
///
/// let e: Endianness = get_endianness();
/// let mut sink = Vec::new(); 
/// with_order!(&mut sink, e, |dest| {
///     // dset is a `ByteOrdered<_, StaticEndianness<_>>`
///     dest.write_u32(8)?;
///     dest.write_u32(1024)?;
///     dest.write_u32(0xF0FF_F0FF)?;
/// });
/// assert_eq!(sink.len(), 12);
/// # Ok(())
/// # }
/// # fn main() {
/// #   run().unwrap();
/// # }
/// ```
/// 
/// Moreover, you can pass multiple readers or writers to be augmented with
/// the same implicit byte order. Note that the macro requires a literal tuple
/// expression.
/// 
/// ```
/// ##[macro_use] extern crate byteordered;
/// use byteordered::Endianness;
///
/// # fn get_endianness() -> Endianness { Endianness::Little }
/// # fn run() -> Result<(), ::std::io::Error> {
/// let e: Endianness = get_endianness();
/// let (mut sink1, mut sink2) = (Vec::new(), Vec::new());
/// with_order!((&mut sink1, &mut sink2), e, |dest1, dest2| {
///     dest1.write_u32(0x0000_EEFF)?;
///     dest2.write_u32(0xFFEE_0000)?;
/// });
/// assert_eq!(&sink1, &[0xFF, 0xEE, 0x00, 0x00]);
/// assert_eq!(&sink2, &[0x00, 0x00, 0xEE, 0xFF]);
/// # Ok(())
/// # }
/// # fn main() {
/// #   run().unwrap();
/// # }
/// ```
/// 
/// One might think that this improves performance, since a runtime-bound
/// `ByteOrdered` with a sequence of reads/writes would expand into one check
/// for each method call:
/// 
/// ```
/// use byteordered::{ByteOrdered, Endianness};
/// # fn get_endianness() -> Endianness { Endianness::Little }
/// # fn run() -> Result<(), ::std::io::Error> {
/// let mut dst = ByteOrdered::runtime(Vec::new(), get_endianness());
/// // dynamic dispatch each time (or is it?)
/// dst.write_u32(8)?;
/// dst.write_u32(1024)?;
/// dst.write_u32(0xF0FF_F0FF)?;
/// # Ok(())
/// # }
/// # run().unwrap();
/// ```
/// 
/// However, because the compiler is known to optimize these checks away in
/// the same context, making a scope for that purpose is not always necessary.
/// On the other hand, this can be seen as yet another way to create and manage
/// data sources/destinations with byte order awareness.
///
/// [`Endianness`]: enum.Endianness.html
#[macro_export]
macro_rules! with_order {
    ( ($($src: expr ),*), $endianness: expr, |$($bo: ident ),*| $e: expr ) => {
        match $endianness {
            Endianness::Big => {
                $(
//                let mut src_or_dst = $src;
                let mut $bo = ::byteordered::ByteOrdered::new(
                    $src,
                    ::byteordered::StaticEndianness::<byteorder::BigEndian>::default());
                )*
                $e
            }
            Endianness::Little => {
                $(
//                let mut src_or_dst = $src;
                let mut $bo = ::byteordered::ByteOrdered::new(
                    $src,
                    ::byteordered::StaticEndianness::<byteorder::LittleEndian>::default());
                )*
                $e
            }
        }
    };
    ($src: expr, $endianness: expr, |$bo: ident| $e: expr ) => {
        match $endianness {
            Endianness::Big => {
                let mut $bo = ::byteordered::ByteOrdered::new(
                    $src,
                    ::byteordered::StaticEndianness::<byteorder::BigEndian>::default());
                $e
            }
            Endianness::Little => {
                let mut $bo = ::byteordered::ByteOrdered::new(
                    $src,
                    ::byteordered::StaticEndianness::<byteorder::LittleEndian>::default());
                $e
            }
        }
    };
}
