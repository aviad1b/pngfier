use std::io;

use bitstream_io::{BitRead, BitWrite, Endianness};
use generic_array::{ArrayLength, GenericArray, typenum::U1};

use super::{StreamPos, utils};

/// Used for types that can be parsed from&to bytes, using a fixed-sized buffer.
pub trait ConstBinParsible {
    /// Size of buffer needed for parsing.
    type BuffSize: ArrayLength;

    /// Parses `Self` from a corresponding buffer of bytes.
    /// 
    /// * `buff` - Buffer to parse `Self` from.
    /// 
    /// Returns parsed `Self` instance.
    /// 
    fn const_bin_parse(buff: &GenericArray<u8, Self::BuffSize>) -> Self;

    /// Parses `self` into a corresponding buffer of bytes.
    /// 
    /// * `buff` - Buffer to parse into.
    /// 
    fn const_bin_unparse(&self, buff: &mut GenericArray<u8, Self::BuffSize>);
}

impl ConstBinParsible for u8 {
    type BuffSize = U1;

    fn const_bin_parse(buff: &GenericArray<u8, Self::BuffSize>) -> Self {
        buff[0]
    }

    fn const_bin_unparse(&self, buff: &mut GenericArray<u8, Self::BuffSize>) {
        buff[0] = *self;
    }
}

/// Abstraction over a cursor-based stream.
pub trait Stream {
    /// Rewinds cursor back to the beggining of the stream.
    /// 
    /// Returns error if occurred.
    /// 
    fn rewind(&mut self) -> io::Result<()>;

    /// Gets current position of cursor in stream.
    /// 
    /// Returns current position of cursor in stream, or error if occurred.
    /// 
    fn get_pos(&mut self) -> io::Result<StreamPos>;

    /// Sets current position of cursor in stream.
    /// 
    /// * `pos` - New position for cursor in stream.
    /// 
    /// Returns error if occurred.
    /// 
    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()>;

    /// Gets current size of entire stream.
    /// 
    /// Returns stream size, or error if occurred.
    /// 
    fn get_size(&mut self) -> io::Result<StreamPos>;

    /// Updates internal state of stream (for internal module use).
    /// 
    /// Returns error if occurred.
    /// 
    fn update(&mut self) -> io::Result<()> { Ok(()) }
}

/// Abstraction over an indexed set of cursor-based streams.
/// 
/// * `N` - Amount of streams in set.
/// 
pub trait Streams<N: ArrayLength> {
    /// Rewinds cursor back to the beggining of the stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn rewind<const I: usize>(&mut self) -> io::Result<()>;

    /// Gets current position of cursor in stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns current position of cursor in stream, or error if occurred.
    /// 
    fn get_pos<const I: usize>(&mut self) -> io::Result<StreamPos>;

    /// Sets current position of cursor in stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `pos` - New position for cursor in stream.
    /// 
    /// Returns error if occurred.
    /// 
    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()>;

    /// Gets current size of entire stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns stream size, or error if occurred.
    /// 
    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos>;

    /// Updates internal state of stream (for internal module use).
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn update<const I: usize>(&mut self) -> io::Result<()> { Ok(()) }
}

/// Abstraction over a cursor-based stream of binary input.
pub trait InputBinaryStream : Stream {
    /// Reads bytes from stream.
    /// 
    /// * `buff` - Buffer (bytes slice) to read data into (exact size).
    /// 
    /// Returns error if occurred.
    /// 
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()>;

    /// Gets object used for reading bits from stream.
    /// The returned object mutably borrows the stream.
    /// To revoke the borrow, invoke macro `return_bits_reader!`.
    /// 
    /// * `endianness` - Endianness to use when reading binary data (for both bytes and bits).
    /// 
    /// Returns bit reader object, or error if occurred.
    /// 
    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead>;
}

/// Abstraction over an indexed set of cursor-based stream of binary input.
/// 
/// * `N` - Amount of streams in set.
/// 
pub trait InputBinaryStreams<N: ArrayLength> : Stream {
    /// Reads bytes from stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `buff` - Buffer (bytes slice) to read data into (exact size).
    /// 
    /// Returns error if occurred.
    /// 
    fn read_bytes<const I: usize>(&mut self, buff: &mut [u8]) -> io::Result<()>;

    /// Gets object used for reading bits from stream.
    /// The returned object mutably borrows the stream.
    /// To revoke the borrow, invoke macro `return_bits_reader!`.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `endianness` - Endianness to use when reading binary data (for both bytes and bits).
    /// 
    /// Returns bit reader object, or error if occurred.
    /// 
    fn obtain_bits_reader<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead>;
}

/// Gets object used for reading bits from a binary input stream.
/// The returned object mutably borrows the stream.
/// To revoke the borrow, invoke macro `return_bits_reader!`.
/// 
/// * `stream` - Binary input stream to obtain bits reader from.
/// * `endianness` - Endianness to use when reading binary data (for both bytes and bits).
/// * `i` - Stream index in set (if relevant).
/// 
/// Returns bit reader object, or error if occurred.
/// 
#[macro_export]
macro_rules! obtain_bits_reader {
    ($stream:expr, $endianness:expr) => {
        $stream.obtain_bits_reader($endianness)
    };

    ($stream:expr, $endianness:expr, $i:expr) => {
        $stream.obtain_bits_reader::<i>($endianness)
    };
}

/// Revokes stream borrow done by `obtain_bits_reader!` or 
/// `{InputBinaryStream,InputBinaryStreams}::obtain_bits_reader`.
/// 
/// * `reader` - Bits reader to revoke borrow of its stream.
/// * `parent` - Stream to revoke its borrow.
/// * `i` - Stream index in set (if relevant).
/// 
/// Assumes `reader` was indeed obtained from `parent`.
/// Otherwise, behaviour is considered undefined.
/// 
/// Returns error if occurred.
/// 
#[macro_export]
macro_rules! return_bits_reader {
    ($reader:expr, $parent:expr) => {{
        {
            let mut reader = $reader; // take local ownership, discarding borrow
            reader.byte_align(); // ensure no bits are left in queue
        }
        $parent.update()?; // force an internal stream update
        Ok::<(), io::Error>(())
    }};

    ($reader:expr, $parent:expr, $i:expr) => {{
        {
            let mut reader = $reader; // take local ownership, discarding borrow
            reader.byte_align(); // ensure no bits are left in queue
        }
        $parent.update::<i>()?; // force an internal stream update
        Ok::<(), io::Error>(())
    }};
}

/// Abstraction over a cursor-based stream of binary output.
pub trait OutputBinaryStream : Stream {
    /// Writes bytes to stream.
    /// 
    /// * `buff` - Buffer (bytes slice) of data to write.
    /// 
    /// Returns error if occurred.
    /// 
    fn write_bytes(&mut self, buff: &[u8]) -> io::Result<()>;

    /// Gets object used for writing bits from stream.
    /// The returned object mutably borrows the stream.
    /// To revoke the borrow, invoke macro `return_bits_writer!`.
    /// 
    /// * `endianness` - Endianness to use when writing binary data (for both bytes and bits).
    /// 
    /// Returns bit writer object, or error if occurred.
    /// 
    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite>;

    /// Truncates stream data.
    /// 
    /// * `len` - New total length for stream data to have.
    /// 
    /// Note: If cursor is beyond `len` at the time of truncating, 
    /// it should be moved to `len`.
    /// 
    /// Returns error if occurred.
    /// 
    fn truncate(&mut self, len: StreamPos) -> io::Result<()>;
}

/// Abstraction over an indexed set of cursor-based stream of binary output.
/// 
/// * `N` - Amount of streams in set.
/// 
pub trait OutputBinaryStreams<N: ArrayLength> : Streams<N> {
    /// Writes bytes to stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `buff` - Buffer (bytes slice) of data to write.
    /// 
    /// Returns error if occurred.
    /// 
    fn write_bytes<const I: usize>(&mut self, buff: &[u8]) -> io::Result<()>;

    /// Gets object used for writing bits from stream.
    /// The returned object mutably borrows the stream.
    /// To revoke the borrow, invoke macro `return_bits_writer!`.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `endianness` - Endianness to use when writing binary data (for both bytes and bits).
    /// 
    /// Returns bit writer object, or error if occurred.
    /// 
    fn obtain_bits_writer<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite>;

    /// Truncates stream data.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `len` - New total length for stream data to have.
    /// 
    /// Note: If cursor is beyond `len` at the time of truncating, 
    /// it should be moved to `len`.
    /// 
    /// Returns error if occurred.
    /// 
    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()>;
}

/// Gets object used for writing bits from stream.
/// The returned object mutably borrows the stream.
/// To revoke the borrow, invoke macro `return_bits_writer!`.
/// 
/// * `stream` - Binary output stream to obtain bits writer from.
/// * `endianness` - Endianness to use when writing binary data (for both bytes and bits).
/// * `i` - Stream index in set (if relevant).
/// 
/// Returns bit writer object, or error if occurred.
/// 
#[macro_export]
macro_rules! obtain_bits_writer {
    ($stream:expr, $endianness:expr) => {
        $stream.obtain_bits_writer($endianness)
    };

    ($stream:expr, $endianness:expr, $i:expr) => {
        $stream.obtain_bits_writer::<i>($endianness)
    };
}

/// Revokes stream borrow done by `obtain_bits_writer!` or 
/// `{OutputBinaryStream,OutputBinaryStreams}::obtain_bits_writer`.
/// 
/// * `writer` - Bits writer to revoke borrow of its stream.
/// * `parent` - Stream to revoke its borrow.
/// * `i` - Stream index in set (if relevant).
/// 
/// Assumes `writer` was indeed obtained from `parent`.
/// Otherwise, behaviour is considered undefined.
/// 
/// Returns error if occurred.
/// 
#[macro_export]
macro_rules! return_bits_writer {
    ($writer:expr, $parent:expr) => {{
        {
            let mut writer = $writer; // take local ownership, discarding borrow
            writer.byte_align()?; // ensure no bits are left in queue
        }
        $parent.update()?; // force an internal stream update
        Ok::<(), io::Error>(())
    }};

    ($writer:expr, $parent:expr, $i:expr) => {{
        {
            let mut writer = $writer; // take local ownership, discarding borrow
            writer.byte_align()?; // ensure no bits are left in queue
        }
        $parent.update::<i>()?; // force an internal stream update
        Ok::<(), io::Error>(())
    }};
}

/// Abstraction over a cursor-based stream of input made of elements.
pub trait InputElemStream<E> : Stream {
    /// Reads next element from stream.
    /// 
    /// Returns read element, or `None` if reached end-of-stream.
    /// Returns error if occurred.
    /// 
    fn read_next_elem(&mut self) -> io::Result<Option<E>>;

    /// Locates the first instance of a chunk of data in the file, 
    /// starting at the cursor's current position.
    /// 
    /// Requires `E` implementing `Eq` for comparation.
    /// 
    /// * `data` - Chunk of data to look for in the stream.
    /// 
    /// Returns starting position of chunk found in stream,
    /// or `None` if wasn't found.
    /// Returns error if occurred.
    /// 
    fn lookup(&mut self, data: &[E]) -> io::Result<Option<StreamPos>>
    where E: Eq {
        utils::lookup(
            self.get_pos()?,
            || self.read_next_elem(),
            data
        )
    }
}

/// Abstraction over an indexed set of cursor-based stream of input made of elements.
/// 
/// * `N` - Amount of streams in set.
/// 
pub trait InputElemStreams<E, N: ArrayLength> : Streams<N> {
    /// Reads next element from stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns read element, or `None` if reached end-of-stream.
    /// Returns error if occurred.
    /// 
    fn read_next_elem<const I: usize>(&mut self) -> io::Result<Option<E>>;

    /// Locates the first instance of a chunk of data in the file, 
    /// starting at the cursor's current position.
    /// 
    /// Requires `E` implementing `Eq` for comparation.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `data` - Chunk of data to look for in the stream.
    /// 
    /// Returns starting position of chunk found in stream,
    /// or `None` if wasn't found.
    /// Returns error if occurred.
    /// 
    fn lookup<const I: usize>(&mut self, data: &[E]) -> io::Result<Option<StreamPos>>
    where E: Eq {
        utils::lookup(
            self.get_pos::<I>()?,
            || self.read_next_elem::<I>(),
            data
        )
    }
}

/// Abstraction over a cursor-based stream of output made of elements.
pub trait OutputElemStream<E> : Stream {
    /// Writes element stream.
    /// 
    /// * `elem` - Element to write.
    /// 
    /// Returns error if occurred.
    /// 
    fn write_next_elem(&mut self, elem: E) -> io::Result<()>;

    /// Truncates stream data.
    /// 
    /// * `len` - New total length for stream data to have.
    /// 
    /// Note: If cursor is beyond `len` at the time of truncating, 
    /// it should be moved to `len`.
    /// 
    /// Returns error if occurred.
    /// 
    fn truncate(&mut self, len: StreamPos) -> io::Result<()>;
}

/// Abstraction over an indexed set of cursor-based stream of output made of elements.
/// 
/// * `N` - Amount of streams in set.
/// 
pub trait OutputElemStreams<E, N: ArrayLength> : Streams<N> {
    /// Writes element stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `elem` - Element to write.
    /// 
    /// Returns error if occurred.
    /// 
    fn write_next_elem<const I: usize>(&mut self, elem: E) -> io::Result<()>;

    /// Truncates stream data.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// * `len` - New total length for stream data to have.
    /// 
    /// Note: If cursor is beyond `len` at the time of truncating, 
    /// it should be moved to `len`.
    /// 
    /// Returns error if occurred.
    /// 
    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()>;
}
