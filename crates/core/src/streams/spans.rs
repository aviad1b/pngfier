use std::{io, marker::PhantomData};

use generic_array::{ArrayLength, GenericArray};

use super::{
    StreamPos,
    traits::{
        ConstBinParsible,
        InputBinaryStream,
        InputElemStreams,
        OutputBinaryStream,
        OutputElemStreams,
        Stream,
        Streams
    },
};

/// For types that can be constructed from a base stream, a start offset and an end index.
pub trait ElemSpanConstructible<'a, S: Stream> {
    /// Constructs a new instance.
    /// 
    /// * `stream` - Base stream.
    /// * `byte_offset` - Optional starting byte offset index.
    /// * `byte_end` - Optional ending byte index.
    /// Resulting instance will read `stream` in range [byte_offset,byte_end).
    /// 
    /// Returns constructed instance.
    /// 
    fn new(stream: &'a mut S,
           byte_offset: Option<StreamPos>,
           byte_end: Option<StreamPos>) -> Self;
}

/// For types that can be constructed from a base stream, start offsets and an end indexes.
/// Used for having multiple spans over the same base stream.
pub trait ElemSpansConstructible<'a, S: Stream, N: ArrayLength> {
    /// Constructs a new instance.
    /// 
    /// * `stream` - Base stream.
    /// * `byte_offsets` - Optional starting byte offset index for each span in the set.
    /// * `byte_ends` - Optional ending byte index for each span in the set.
    /// Resulting spans in set will each read `stream` in range [byte_offset,byte_end).
    /// 
    /// Returns constructed instance.
    /// 
    fn new(stream: &'a mut S,
           byte_offsets: GenericArray<Option<StreamPos>, N>,
           byte_ends: GenericArray<Option<StreamPos>, N>) -> Self;
}

/// Reads a span of elements over a binary stream.
pub struct BinaryElemSpans<'a, E: ConstBinParsible, S: Stream, N: ArrayLength> {
    stream: &'a mut S,
    byte_offsets: GenericArray<StreamPos, N>,
    byte_ends: GenericArray<Option<StreamPos>, N>, // non-inclusive
    poses: GenericArray<StreamPos, N>,
    phantom: PhantomData<E>,
}

impl<'a, E: ConstBinParsible, S: Stream, N: ArrayLength> BinaryElemSpans<'a, E, S, N> {
    /// Constructs a new instance.
    /// 
    /// * `stream` - Base binary stream.
    /// * `byte_offsets` - Optional starting byte offset index for each span in the set.
    /// * `byte_ends` - Optional ending byte index for each span in the set.
    /// Resulting spans in set will each read `stream` in range [byte_offset,byte_end).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(stream: &'a mut S,
               byte_offsets: GenericArray<Option<StreamPos>, N>,
               byte_ends: GenericArray<Option<StreamPos>, N>) -> Self {
        let _ = (stream, byte_offsets, byte_ends);
        todo!() // TODO: Implement
    }

    /// Utility size which returns size of element based on its `ConstBinParsible` implementation.
    fn elem_size() -> StreamPos {
        todo!() // TODO: Implement
    }

    /// Converts a (global, stream) byte position to a (local, span) element one.
    /// 
    /// * `I` - Span index in set.
    /// 
    /// * `byte_pos` - Global position to convert to a local one.
    /// 
    /// Returns local position (for stream indexed `I`).
    /// 
    fn pos_global_to_local<const I: usize>(&self, byte_pos: StreamPos) -> StreamPos {
        let _ = byte_pos;
        todo!() // TODO: Implement
    }

    /// Converts a (local, span) element position to a (global, stream) byte one.
    /// 
    /// * `I` - Span index in set.
    /// 
    /// * `elem_pos` - Local position to convert to a global one.
    /// 
    /// Returns global position (converted from local of stream indexed `I`).
    /// 
    fn pos_local_to_global<const I: usize>(&self, elem_pos: StreamPos) -> StreamPos {
        let _ = elem_pos;
        todo!() // TODO: Implement
    }

    /// Updates cached pos to match file pos
    /// 
    /// * `I` - Span index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn update_pos<const I: usize>(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    /// Ensures file pos matches cached pos
    /// 
    /// * `I` - Span index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn ensure_pos<const I: usize>(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }
}

impl<'a, E: ConstBinParsible, S: Stream, N: ArrayLength> Streams<N> for BinaryElemSpans<'a, E, S, N> {
    fn rewind<const I: usize>(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    fn get_pos<const I: usize>(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }

    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()> {
        let _ = pos;
        todo!() // TODO: Implement
    }

    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }
}

impl<'a, E: ConstBinParsible, S: InputBinaryStream, N: ArrayLength>
InputElemStreams<E, N> for BinaryElemSpans<'a, E, S, N> {
    fn read_next_elem<const I: usize>(&mut self) -> io::Result<Option<E>> {
        todo!() // TODO: Implement
    }
}

impl<'a, E: ConstBinParsible, S: OutputBinaryStream, N: ArrayLength>
OutputElemStreams<E, N> for BinaryElemSpans<'a, E, S, N> {
    fn write_next_elem<const I: usize>(&mut self, elem: E) -> io::Result<()> {
        let _ = elem;
        todo!() // TODO: Implement
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}
