//! Wrapper types providing byte order awareness.

use byteorder::{
    BigEndian, LittleEndian, NativeEndian, NetworkEndian, ReadBytesExt, WriteBytesExt,
};
use std::fmt::Arguments;
use std::io::{BufRead, Read, Result as IoResult, Seek, SeekFrom, Write};
use {Endian, Endianness, StaticEndianness};

/// Wrapper type for a reader or writer with an assumed byte order.
///
/// More details can be found at the [crate level documentation][1].
///
/// [1]: index.html
#[derive(Debug, Clone)]
pub struct ByteOrdered<T, E> {
    inner: T,
    endianness: E,
}

impl<T, E> ByteOrdered<T, E>
where
    E: Default,
{
    #[inline]
    fn new_default(inner: T) -> Self {
        ByteOrdered {
            inner,
            endianness: Default::default(),
        }
    }
}

impl<T> ByteOrdered<T, StaticEndianness<LittleEndian>> {
    /// Obtains a new reader or writer that assumes data in _little endian_.
    #[inline]
    pub fn le(inner: T) -> Self {
        ByteOrdered::new_default(inner)
    }
}

impl<T> ByteOrdered<T, StaticEndianness<BigEndian>> {
    /// Obtains a new reader or writer that assumes data in _big endian_.
    #[inline]
    pub fn be(inner: T) -> Self {
        ByteOrdered::new_default(inner)
    }
}

impl<T> ByteOrdered<T, StaticEndianness<NativeEndian>> {
    /// Obtains a new reader or writer that assumes data in the system's
    /// _native endianness_. While this method might sounds a bit pointless,
    /// it enables easier byte order changes through method chaining).
    #[inline]
    pub fn native(inner: T) -> Self {
        ByteOrdered::new_default(inner)
    }
}

impl<T> ByteOrdered<T, StaticEndianness<NetworkEndian>> {
    /// Obtains a new reader or writer that assumes _network order_.
    #[inline]
    pub fn network(inner: T) -> Self {
        ByteOrdered::new_default(inner)
    }
}

impl<T> ByteOrdered<T, Endianness> {
    /// Creates a new reader or writer that assumes data in the given byte
    /// order known at _run-time_.
    /// That is, the type representing the byte order
    /// is the enum type [`Endianness`].
    ///
    /// Although it is equivalent to [`ByteOrdered::new`][`new`], this function
    /// leaves a code signal that subsequent calls depend on conditions
    /// resolved at run-time. If you know the data's endianness in compile
    /// time, the other constructors are preferred (e.g. [`new`], [`le`] or
    /// [`be`]), so as to avoid the overhead of dynamic dispatching.
    ///
    /// [`Endianness`]: ../base/struct.Endianness.html
    /// [`new`]: struct.ByteOrdered.html#method.new
    /// [`le`]: struct.ByteOrdered.html#method.le
    /// [`be`]: struct.ByteOrdered.html#method.be
    #[inline]
    pub fn runtime(inner: T, endianness: Endianness) -> Self {
        ByteOrdered::new(inner, endianness)
    }
}

impl<T, E> From<(T, E)> for ByteOrdered<T, E> {
    #[inline]
    fn from((inner, endianness): (T, E)) -> Self {
        ByteOrdered { inner, endianness }
    }
}

impl<T, E> ByteOrdered<T, E>
where
    E: Endian,
{
    /// Creates a new reader or writer that assumes data in the given byte
    /// order. This flexible constructor admits any kind of byte order (static
    /// and dynamic).
    ///
    /// **Note:** The other constructors ([`le`], [`be`], [`native`], and
    /// [`runtime`]) are more recommended because they are easier to use and
    /// leave a better signal of whether the endianness is known at compile
    /// time or at run time. For example, if you pass a value literal of type
    /// `Endianness` (such as `Endianness::Little`), the program will perform
    /// dynamic dispatching in spite of the fixed byte order. The use of this
    /// method is more appropriate when constructing functions which are
    /// generic over the endianness type.
    ///
    /// [`le`]: struct.ByteOrdered.html#method.le
    /// [`be`]: struct.ByteOrdered.html#method.be
    /// [`native`]: struct.ByteOrdered.html#method.native
    /// [`runtime`]: struct.ByteOrdered.html#method.runtime
    #[inline]
    pub fn new(inner: T, endianness: E) -> Self {
        ByteOrdered { inner, endianness }
    }

    /// Recovers the inner reader or writer from this wrapper. Information
    /// about the assumed byte order is discarded.
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Obtains an exclusive mutable reference to the inner reader or writer in
    /// this wrapper. Information about the assumed byte order is ignored until
    /// the reference is dropped.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Converts from `ByteOrdered<T, E>` to `ByteOrdered<&mut T, E>`,
    /// copying the endianness information.
    #[inline]
    pub fn as_mut(&mut self) -> ByteOrdered<&mut T, E>
    where
        E: Copy,
    {
        let e = self.endianness;
        ByteOrdered::new(self.inner_mut(), e)
    }

    /// Disbands a `ByteOrder` into its parts.
    #[inline]
    pub fn into_parts(self) -> (T, E) {
        (self.inner, self.endianness)
    }

    /// Maps a `ByteOrdered<T, E>` into a `ByteOrdered<O, E>` by applying the
    /// given function to the inner reader or writer.
    #[inline]
    pub fn map<F, U>(self, f: F) -> ByteOrdered<U, E>
    where
        F: FnOnce(T) -> U,
    {
        let (src, e) = self.into_parts();
        ByteOrdered::new(f(src), e)
    }

    /// Changes the assumed byte order of the reader or writer.
    #[inline]
    pub fn into_endianness<E2: Endian>(self, endianness: E2) -> ByteOrdered<T, E2> {
        ByteOrdered::new(self.inner, endianness)
    }

    /// Modifies the assumed byte order of the reader or writer
    /// inline with the value.
    /// Since the endianness type needs to be the same,
    /// this function is only relevant when
    /// `E` is a run-time defined byte order
    /// (see [`Endianness`]).
    ///
    /// [`Endianness`]: ../base/struct.Endianness.html
    #[inline]
    pub fn set_endianness(&mut self, endianness: E) {
        self.endianness = endianness;
    }

    /// Changes the assumed byte order of the reader or writer to
    /// little endian.
    #[inline]
    pub fn into_le(self) -> ByteOrdered<T, StaticEndianness<LittleEndian>> {
        ByteOrdered::le(self.inner)
    }

    /// Changes the assumed byte order of the reader or writer to
    /// little endian.
    #[inline]
    pub fn into_be(self) -> ByteOrdered<T, StaticEndianness<BigEndian>> {
        ByteOrdered::be(self.inner)
    }

    /// Changes the assumed byte order of the reader or writer to
    /// the system's native endianness.
    #[inline]
    pub fn into_native(self) -> ByteOrdered<T, StaticEndianness<NativeEndian>> {
        ByteOrdered::native(self.inner)
    }

    /// Converts the assumed endianness to the opposite of the current order.
    #[inline]
    pub fn into_opposite(self) -> ByteOrdered<T, E::Opposite>
    where
        E: Endian,
    {
        let e = self.endianness.into_opposite();
        ByteOrdered {
            inner: self.inner,
            endianness: e,
        }
    }

    /// Retrieves the byte order assumed by this wrapper.
    #[inline]
    pub fn endianness(&self) -> E
    where
        E: Copy,
    {
        self.endianness
    }

    /// Checks whether the assumed endianness is the system's native byte
    /// order.
    #[inline]
    pub fn is_native(&self) -> bool
    where
        E: Endian,
    {
        self.endianness.is_native()
    }
}

impl<R, E> Read for ByteOrdered<R, E>
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

impl<W, E> Write for ByteOrdered<W, E>
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

impl<R, E> ByteOrdered<R, E>
where
    R: ReadBytesExt,
    E: Endian,
{
    /// Reads a signed 8 bit integer from the underlying reader.
    ///
    /// This method does exactly the same thing as `read_i8` in
    /// `byteorder::ReadBytesExt`. It is included so that users do not have to
    /// import the former trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use byteordered::ByteOrdered;
    ///
    /// # fn run() -> std::io::Result<()> {
    /// let mut rdr = ByteOrdered::native(&[2, 5][..]);
    /// assert_eq!(2, rdr.read_i8()?);
    /// assert_eq!(5, rdr.read_i8()?);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn read_i8(&mut self) -> IoResult<i8> {
        ReadBytesExt::read_i8(self)
    }

    /// Reads an unsigned 8 bit integer from the underlying reader.
    ///
    /// This method does exactly the same thing as `read_u8` in
    /// `byteorder::ReadBytesExt`. It is included so that users do not have to
    /// import the former trait.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use byteordered::ByteOrdered;
    ///
    /// # fn run() -> std::io::Result<()> {
    /// let mut rdr = ByteOrdered::native(&[2, 5][..]);
    /// assert_eq!(2, rdr.read_u8()?);
    /// assert_eq!(5, rdr.read_u8()?);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn read_u8(&mut self) -> IoResult<u8> {
        ReadBytesExt::read_u8(self)
    }

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
    /// use byteordered::ByteOrdered;
    ///
    /// # fn run() -> std::io::Result<()> {
    /// let mut rdr = ByteOrdered::be(&[0x00, 0xc1, 0xff, 0x7c][..]);
    /// assert_eq!(193, rdr.read_i16()?);
    /// assert_eq!(-132, rdr.read_i16()?);
    /// # Ok(())
    /// # }
    /// # run().unwrap();
    /// ```
    #[inline]
    pub fn read_i16(&mut self) -> IoResult<i16> {
        self.endianness.read_i16(self.inner.by_ref())
    }

    /// Reads a sequence of signed 16 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read two signed 16 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// # use byteordered::ByteOrdered;
    /// # fn run() -> std::io::Result<()> {
    /// let mut out = [0; 2];
    /// let mut rdr = ByteOrdered::be(&[0x00, 0xc1, 0xff, 0x7c][..]);
    /// rdr.read_i16_into(&mut out)?;
    /// assert_eq!(out, [193, -132]);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn read_i16_into(&mut self, dst: &mut [i16]) -> IoResult<()> {
        self.endianness.read_i16_into(self.inner.by_ref(), dst)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u16(&mut self) -> IoResult<u16> {
        self.endianness.read_u16(self.inner.by_ref())
    }

    /// Reads a sequence of unsigned 16 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u16_into(&mut self, dst: &mut [u16]) -> IoResult<()> {
        self.endianness.read_u16_into(self.inner.by_ref(), dst)
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i32(&mut self) -> IoResult<i32> {
        self.endianness.read_i32(self.inner.by_ref())
    }

    /// Reads a sequence of signed 32 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i32_into(&mut self, dst: &mut [i32]) -> IoResult<()> {
        self.endianness.read_i32_into(self.inner.by_ref(), dst)
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u32(&mut self) -> IoResult<u32> {
        self.endianness.read_u32(self.inner.by_ref())
    }

    /// Reads a sequence of unsigned 32 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u32_into(&mut self, dst: &mut [u32]) -> IoResult<()> {
        self.endianness.read_u32_into(self.inner.by_ref(), dst)
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i64(&mut self) -> IoResult<i64> {
        self.endianness.read_i64(self.inner.by_ref())
    }

    /// Reads a sequence of signed 64 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i64_into(&mut self, dst: &mut [i64]) -> IoResult<()> {
        self.endianness.read_i64_into(self.inner.by_ref(), dst)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u64(&mut self) -> IoResult<u64> {
        self.endianness.read_u64(self.inner.by_ref())
    }

    /// Reads a sequence of unsigned 64 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u64_into(&mut self, dst: &mut [u64]) -> IoResult<()> {
        self.endianness.read_u64_into(self.inner.by_ref(), dst)
    }

    /// Reads a signed 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i128(&mut self) -> IoResult<i128> {
        self.endianness.read_i128(self.inner.by_ref())
    }

    /// Reads a sequence of signed 128 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_i128_into(&mut self, dst: &mut [i128]) -> IoResult<()> {
        self.endianness.read_i128_into(self.inner.by_ref(), dst)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u128(&mut self) -> IoResult<u128> {
        self.endianness.read_u128(self.inner.by_ref())
    }

    /// Reads a sequence of unsigned 128 bit integers from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_u128_into(&mut self, dst: &mut [u128]) -> IoResult<()> {
        self.endianness.read_u128_into(self.inner.by_ref(), dst)
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number from
    /// the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_f32(&mut self) -> IoResult<f32> {
        self.endianness.read_f32(self.inner.by_ref())
    }

    /// Reads a sequence of IEEE754 single-precision (4 bytes) floating point numbers
    /// from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_f32_into(&mut self, dst: &mut [f32]) -> IoResult<()> {
        self.endianness.read_f32_into(self.inner.by_ref(), dst)
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number from
    /// the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_f64(&mut self) -> IoResult<f64> {
        self.endianness.read_f64(self.inner.by_ref())
    }

    /// Reads a sequence of IEEE754 double-precision (8 bytes) floating point numbers
    /// from the underlying reader.
    ///
    /// The given buffer is either filled completely or an error is returned.
    /// If an error is returned,
    /// the contents of `dst` are unspecified.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    #[inline]
    pub fn read_f64_into(&mut self, dst: &mut [f64]) -> IoResult<()> {
        self.endianness.read_f64_into(self.inner.by_ref(), dst)
    }
}

impl<W, E> ByteOrdered<W, E>
where
    W: WriteBytesExt,
    E: Endian,
{
    /// Writes a signed 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_i8(&mut self, x: i8) -> IoResult<()> {
        self.inner.write_i8(x)
    }

    /// Writes an unsigned 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_u8(&mut self, x: u8) -> IoResult<()> {
        self.inner.write_u8(x)
    }

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
    /// use byteordered::ByteOrdered;
    ///
    /// let mut wtr = ByteOrdered::be(Vec::new());
    /// wtr.write_i16(193).unwrap();
    /// wtr.write_i16(-132).unwrap();
    /// assert_eq!(wtr.into_inner(), b"\x00\xc1\xff\x7c");
    /// ```
    #[inline]
    pub fn write_i16(&mut self, x: i16) -> IoResult<()> {
        self.endianness.write_i16(self.inner.by_ref(), x)
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_u16(&mut self, x: u16) -> IoResult<()> {
        self.endianness.write_u16(self.inner.by_ref(), x)
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_i32(&mut self, x: i32) -> IoResult<()> {
        self.endianness.write_i32(self.inner.by_ref(), x)
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_u32(&mut self, x: u32) -> IoResult<()> {
        self.endianness.write_u32(self.inner.by_ref(), x)
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_i64(&mut self, x: i64) -> IoResult<()> {
        self.endianness.write_i64(self.inner.by_ref(), x)
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_u64(&mut self, x: u64) -> IoResult<()> {
        self.endianness.write_u64(self.inner.by_ref(), x)
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_i128(&mut self, x: i128) -> IoResult<()> {
        self.endianness.write_i128(self.inner.by_ref(), x)
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_u128(&mut self, x: u128) -> IoResult<()> {
        self.endianness.write_u128(self.inner.by_ref(), x)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number to
    /// the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_f32(&mut self, x: f32) -> IoResult<()> {
        self.endianness.write_f32(self.inner.by_ref(), x)
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number to
    /// the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    pub fn write_f64(&mut self, x: f64) -> IoResult<()> {
        self.endianness.write_f64(self.inner.by_ref(), x)
    }
}

impl<T, E> BufRead for ByteOrdered<T, E>
where
    T: BufRead,
{
    #[inline]
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }

    #[inline]
    fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> IoResult<usize> {
        self.inner.read_until(byte, buf)
    }

    #[inline]
    fn read_line(&mut self, buf: &mut String) -> IoResult<usize> {
        self.inner.read_line(buf)
    }
}

impl<T, E> Seek for ByteOrdered<T, E>
where
    T: Seek,
{
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
        self.inner.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    // TODO test moar
    use super::ByteOrdered;
    use base::Endianness;
    static TEST_BYTES: &'static [u8] = &[0x12, 0x34, 0x56, 0x78, 0x21, 0x43, 0x65, 0x87];

    static TEST_U64DATA_LE: &'static [u64] = &[0x87654321_78563412];
    static TEST_U64DATA_BE: &'static [u64] = &[0x12345678_21436587];

    #[test]
    fn test_read_u64() {
        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::le(&mut data);
        let words = [reader.read_u64().unwrap()];
        assert_eq!(words, TEST_U64DATA_LE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::be(&mut data);
        let words = [reader.read_u64().unwrap()];
        assert_eq!(words, TEST_U64DATA_BE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::runtime(&mut data, Endianness::Little);
        let words = [reader.read_u64().unwrap()];
        assert_eq!(words, TEST_U64DATA_LE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::runtime(&mut data, Endianness::Big);
        let words = [reader.read_u64().unwrap()];
        assert_eq!(words, TEST_U64DATA_BE);
    }

    #[test]
    fn test_write_u64() {
        let mut writer = ByteOrdered::le(Vec::new());
        for v in TEST_U64DATA_LE {
            writer.write_u64(*v).unwrap();
        }
        assert_eq!(&*writer.into_inner(), TEST_BYTES);

        let mut writer = ByteOrdered::be(Vec::new());
        for v in TEST_U64DATA_BE {
            writer.write_u64(*v).unwrap();
        }
        assert_eq!(&*writer.into_inner(), TEST_BYTES);

        let mut writer = ByteOrdered::runtime(Vec::new(), Endianness::Little);
        for v in TEST_U64DATA_LE {
            writer.write_u64(*v).unwrap();
        }
        assert_eq!(&*writer.into_inner(), TEST_BYTES);

        let mut writer = ByteOrdered::runtime(Vec::new(), Endianness::Big);
        for v in TEST_U64DATA_BE {
            writer.write_u64(*v).unwrap();
        }
        assert_eq!(&*writer.into_inner(), TEST_BYTES);
    }

    /// the test bytes as two u32s in little endian
    static TEST_U32DATA_LE: &'static [u32] = &[0x7856_3412, 0x8765_4321];
    /// the test bytes as two u32s in big endian
    static TEST_U32DATA_BE: &'static [u32] = &[0x1234_5678, 0x2143_6587];

    #[test]
    fn test_read_u32_into() {
        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::le(&mut data);
        let mut words = [0; 2];
        reader.read_u32_into(&mut words).unwrap();
        assert_eq!(words, TEST_U32DATA_LE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::be(&mut data);
        let mut words = [0; 2];
        reader.read_u32_into(&mut words).unwrap();
        assert_eq!(words, TEST_U32DATA_BE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::runtime(&mut data, Endianness::Little);
        let mut words = [0; 2];
        reader.read_u32_into(&mut words).unwrap();
        assert_eq!(words, TEST_U32DATA_LE);

        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::runtime(&mut data, Endianness::Big);
        let mut words = [0; 2];
        reader.read_u32_into(&mut words).unwrap();
        assert_eq!(words, TEST_U32DATA_BE);
    }

    #[test]
    fn test_read_u32_and_set_endianness() {
        let mut data = TEST_BYTES;
        let mut reader = ByteOrdered::runtime(&mut data, Endianness::Little);
        let v1 = reader.read_u32().unwrap();
        assert_eq!(v1, TEST_U32DATA_LE[0]);

        // change to big endian
        reader.set_endianness(Endianness::Big);
        let v2 = reader.read_u32().unwrap();
        assert_eq!(v2, TEST_U32DATA_BE[1]);
    }
}
