use std::io;

use generic_array::{ArrayLength, GenericArray, typenum::U1};

use super::StreamPos;

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
    /// Returns error if occured.
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
    /// Returns error if occured.
    /// 
    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()>;

    /// Gets current size of entire stream.
    /// 
    /// Returns stream size, or error if occured.
    /// 
    fn get_size(&mut self) -> io::Result<StreamPos>;

    /// Updates internal state of stream (for internal module use).
    /// 
    /// Returns error if occured.
    /// 
    fn update(&mut self) -> io::Result<()> { Ok(()) }
}

/// Abstraction over an indexed set of cursor-based streams.
/// * `N` - Amount of streams in set.
/// 
pub trait Streams<N: ArrayLength> {
    /// Rewinds cursor back to the beggining of the stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns error if occured.
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
    /// Returns error if occured.
    /// 
    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()>;

    /// Gets current size of entire stream.
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns stream size, or error if occured.
    /// 
    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos>;

    /// Updates internal state of stream (for internal module use).
    /// 
    /// * `I` - Stream index in set.
    /// 
    /// Returns error if occured.
    /// 
    fn update<const I: usize>(&mut self) -> io::Result<()> { Ok(()) }
}
