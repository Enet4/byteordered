//! Base Endianness type module.

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result as IoResult, Write};

/// General trait for types representing a byte order.
pub trait Endian {}

/// A data type representing the Little Endian byte order.
/// Unlike `byteorder::LittleEndian`, this type has a default constructor.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LittleEndianness;

impl Endian for LittleEndianness {}
impl Endian for BigEndianness {}

/// A data type representing the Big Endian byte order.
/// Unlike `byteorder::BigEndian`, this type has a default constructor.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BigEndianness;

/// Enumerate for materializing the two kinds of machine byte order supported
/// by Rust.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Endianness {
    /// Little Endian
    Little,
    /// Big Endian
    Big,
}

impl From<LittleEndianness> for Endianness {
    fn from(_: LittleEndianness) -> Self {
        Endianness::Little
    }
}

impl From<BigEndianness> for Endianness {
    fn from(_: BigEndianness) -> Self {
        Endianness::Big
    }
}

impl Endian for Endianness {}

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
    use super::*;
    static TEST_BYTES: &'static [u8] = &[0x12, 0x34, 0x56, 0x78, 0x21, 0x43, 0x65, 0x87];

    static TEST_U64DATA_LE: &'static [u64] = &[0x87654321_78563412];
    static TEST_U64DATA_BE: &'static [u64] = &[0x12345678_21436587];

    #[test]
    fn test_read_u64() {
        let mut data = TEST_BYTES;
        let e = Endianness::Little;
        let words = [
            e.read_u64(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U64DATA_LE);

        let mut data = TEST_BYTES;
        let e = Endianness::Big;
        let words = [
            e.read_u64(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U64DATA_BE);
    }

    static TEST_U32DATA_LE: &'static [u32] = &[0x7856_3412, 0x8765_4321];
    static TEST_U32DATA_BE: &'static [u32] = &[0x1234_5678, 0x2143_6587];

    #[test]
    fn test_read_u32() {
        let mut data = TEST_BYTES;
        let e = Endianness::Little;
        let words = [
            e.read_u32(&mut data).unwrap(),
            e.read_u32(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U32DATA_LE);

        let mut data = TEST_BYTES;
        let e = Endianness::Big;
        let words = [
            e.read_u32(&mut data).unwrap(),
            e.read_u32(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U32DATA_BE);
    }

    static TEST_U16DATA_LE: &'static [u16] = &[0x3412, 0x7856, 0x4321, 0x8765];
    static TEST_U16DATA_BE: &'static [u16] = &[0x1234, 0x5678, 0x2143, 0x6587];

    #[test]
    fn test_read_u16() {
        let mut data = TEST_BYTES;
        let e = Endianness::Little;
        let words = [
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U16DATA_LE);

        let mut data = TEST_BYTES;
        let e = Endianness::Big;
        let words = [
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
            e.read_u16(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U16DATA_BE);
    }

    #[test]
    fn test_native_is_le() {
        if cfg!(target_endian = "little") {
            assert_eq!(Endianness::native(), Endianness::Little);
        } else if cfg!(target_endian = "big") {
            assert_eq!(Endianness::native(), Endianness::Big);
        } else {
            unreachable!();
        }
    }

    // TODO test writing
}
