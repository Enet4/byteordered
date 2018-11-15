//! Wrapper types providing byte order awareness.

use std::fmt::Arguments;
use std::io::{Read, Result as IoResult, Write};
use {Endianness, ReadBytesExt, WriteBytesExt};

/// Wrapper type for a reader or writer with an assumed byte order.
#[derive(Debug)]
pub struct ByteOrdered<T> {
    inner: T,
    endianness: Endianness,
}

impl<T> ByteOrdered<T> {
    /// Creates a new reader or writer that assumes the given byte order.  
    pub fn new(inner: T, endianness: Endianness) -> Self {
        ByteOrdered { inner, endianness }
    }

    /// Creates a new reader or writer that assumes that assumes little endian
    /// for everything.
    pub fn le(inner: T) -> Self {
        ByteOrdered {
            inner,
            endianness: Endianness::Little,
        }
    }

    /// Creates a new reader or writer that assumes that assumes big endian
    /// for everything.
    pub fn be(inner: T) -> Self {
        ByteOrdered {
            inner,
            endianness: Endianness::Big,
        }
    }

    /// Recovers the inner reader or writer from this wrapper. Information
    /// about the assumed byte order is discarded.
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Change the assumed byte order of the reader or writer.
    pub fn into_endianness(self, endianness: Endianness) -> Self {
        ByteOrdered {
            inner: self.inner,
            endianness,
        }
    }

    /// Change the assumed byte order of the reader or writer to
    /// little endian.
    pub fn into_le(self) -> Self {
        ByteOrdered {
            inner: self.inner,
            endianness: Endianness::Little,
        }
    }

    /// Change the assumed byte order of the reader or writer to
    /// little endian.
    pub fn into_be(self) -> Self {
        ByteOrdered {
            inner: self.inner,
            endianness: Endianness::Big,
        }
    }

    /// Retrieves the byte order assumed by this wrapper.
    pub fn endianness(&self) -> Endianness {
        self.endianness
    }
}

impl<R> Read for ByteOrdered<R>
where
    R: Read,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.inner.read(buf)
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> IoResult<usize> {
        self.inner.read_to_end(buf)
    }

    #[inline]
    fn read_to_string(&mut self, buf: &mut String) -> IoResult<usize> {
        self.inner.read_to_string(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        self.inner.read_exact(buf)
    }
}

impl<W> Write for ByteOrdered<W>
where
    W: Write,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> IoResult<()> {
        self.inner.flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        self.inner.write_all(buf)
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments) -> IoResult<()> {
        self.inner.write_fmt(fmt)
    }
}

impl<R> ReadBytesExt for ByteOrdered<R>
where
    R: byteorder::ReadBytesExt,
{
    fn read_i16(&mut self) -> IoResult<i16> {
        self.endianness.read_i16(&mut self.inner)
    }

    fn read_u16(&mut self) -> IoResult<u16> {
        self.endianness.read_u16(&mut self.inner)
    }

    fn read_i32(&mut self) -> IoResult<i32> {
        self.endianness.read_i32(&mut self.inner)
    }

    fn read_u32(&mut self) -> IoResult<u32> {
        self.endianness.read_u32(&mut self.inner)
    }

    fn read_i64(&mut self) -> IoResult<i64> {
        self.endianness.read_i64(&mut self.inner)
    }

    fn read_u64(&mut self) -> IoResult<u64> {
        self.endianness.read_u64(&mut self.inner)
    }

    #[cfg(feature = "i128")]
    fn read_i128(&mut self) -> IoResult<i128> {
        self.endianness.read_i128(&mut self.inner)
    }

    #[cfg(feature = "i128")]
    fn read_u128(&mut self) -> IoResult<u128> {
        self.endianness.read_u128(&mut self.inner)
    }

    fn read_f32(&mut self) -> IoResult<f32> {
        self.endianness.read_f32(&mut self.inner)
    }

    fn read_f64(&mut self) -> IoResult<f64> {
        self.endianness.read_f64(&mut self.inner)
    }
}

impl<W> WriteBytesExt for ByteOrdered<W>
where
    W: byteorder::WriteBytesExt,
{
    fn write_i16(&mut self, x: i16) -> IoResult<()> {
        self.endianness.write_i16(&mut self.inner, x)
    }

    fn write_u16(&mut self, x: u16) -> IoResult<()> {
        self.endianness.write_u16(&mut self.inner, x)
    }

    fn write_i32(&mut self, x: i32) -> IoResult<()> {
        self.endianness.write_i32(&mut self.inner, x)
    }

    fn write_u32(&mut self, x: u32) -> IoResult<()> {
        self.endianness.write_u32(&mut self.inner, x)
    }

    fn write_i64(&mut self, x: i64) -> IoResult<()> {
        self.endianness.write_i64(&mut self.inner, x)
    }

    fn write_u64(&mut self, x: u64) -> IoResult<()> {
        self.endianness.write_u64(&mut self.inner, x)
    }

    #[cfg(feature = "i128")]
    fn write_i128(&mut self, x: i128) -> IoResult<()> {
        self.endianness.write_i128(&mut self.inner, x)
    }

    #[cfg(feature = "i128")]
    fn write_u128(&mut self, x: u128) -> IoResult<()> {
        self.endianness.write_u128(&mut self.inner, x)
    }

    fn write_f32(&mut self, x: f32) -> IoResult<()> {
        self.endianness.write_f32(&mut self.inner, x)
    }

    fn write_f64(&mut self, x: f64) -> IoResult<()> {
        self.endianness.write_f64(&mut self.inner, x)
    }
}

#[cfg(test)]
mod tests {
    // TODO test moar

    use super::*;
    static TEST_BYTES: &'static [u8] = &[0x12, 0x34, 0x56, 0x78, 0x21, 0x43, 0x65, 0x87];

    static TEST_U64DATA_LE: &'static [u64] = &[0x87654321_78563412];
    static TEST_U64DATA_BE: &'static [u64] = &[0x12345678_21436587];

    #[test]
    fn test_read_u64() {
        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::le(&mut data);
        let words = [
            reader.read_u64().unwrap(),
        ];
        assert_eq!(words, TEST_U64DATA_LE);

        let mut data = TEST_BYTES;
        let e = Endianness::Big;
        let words = [
            e.read_u64(&mut data).unwrap(),
        ];
        assert_eq!(words, TEST_U64DATA_BE);
    }
}
