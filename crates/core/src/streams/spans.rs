use std::{cmp, io, marker::PhantomData};

use generic_array::{ArrayLength, GenericArray, functional::FunctionalSequence, typenum::Unsigned};

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

/// Makes a generic array of stream pos options from a slice of such.
pub fn opt_array<N: ArrayLength>(vals: &[Option<StreamPos>]) -> GenericArray<Option<StreamPos>, N> {
	GenericArray::from_iter(vals.iter().copied())
}

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
        let byte_offsets = byte_offsets.map(|byte_offset| byte_offset.map_or(0, |x| x));
        let poses = GenericArray::default(); // all start at local index 0
        Self {
            stream,
            byte_offsets,
            byte_ends,
            poses,
            phantom: PhantomData,
        }
    }

    /// Utility size which returns size of element based on its `ConstBinParsible` implementation.
    fn elem_size() -> StreamPos {
        E::BuffSize::to_usize() as StreamPos
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
        (byte_pos - self.byte_offsets[I]) / Self::elem_size()
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
        (elem_pos * Self::elem_size()) + self.byte_offsets[I]
    }

    /// Updates cached pos to match file pos
    /// 
    /// * `I` - Span index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn update_pos<const I: usize>(&mut self) -> io::Result<()> {
        let byte_pos = self.stream.get_pos()?;
        self.poses[I] = self.pos_global_to_local::<I>(byte_pos);
        Ok(())
    }

    /// Ensures file pos matches cached pos
    /// 
    /// * `I` - Span index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn ensure_pos<const I: usize>(&mut self) -> io::Result<()> {
        self.set_pos::<I>(self.poses[I])?;
        Ok(())
    }
}

impl<'a, E: ConstBinParsible, S: Stream, N: ArrayLength> Streams<N> for BinaryElemSpans<'a, E, S, N> {
    fn rewind<const I: usize>(&mut self) -> io::Result<()> {
        self.set_pos::<I>(0)
    }

    fn get_pos<const I: usize>(&mut self) -> io::Result<StreamPos> {
        Ok(self.poses[I])
    }

    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()> {
        let byte_pos = self.pos_local_to_global::<I>(pos);
        self.stream.set_pos(byte_pos)?;
        self.poses[I] = pos;
        Ok(())
    }

    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos> {
        let max_byte_size = self.byte_ends[I].map(|byte_end| byte_end - self.byte_offsets[I] + 1);
        let read_byte_size = self.stream.get_size()? - self.byte_offsets[I];
        let actual_byte_size = max_byte_size.map_or(read_byte_size, |max_byte_size| {
            cmp::min(read_byte_size, max_byte_size)
        });

        Ok(actual_byte_size / Self::elem_size())
    }
}

impl<'a, E: ConstBinParsible, S: InputBinaryStream, N: ArrayLength>
InputElemStreams<E, N> for BinaryElemSpans<'a, E, S, N> {
    fn read_next_elem<const I: usize>(&mut self) -> io::Result<Option<E>> {
        self.ensure_pos::<I>()?;

        let bytes_left = self.stream.get_size()? - self.stream.get_pos()?;
        assert!(bytes_left >= 0, "Internal error: Negative byte count in read_next_elem");

        if 0 == bytes_left {
            return Ok(None); // nothing to read
        }

        if bytes_left < Self::elem_size() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF while reading element"));
        }

        let mut buff: GenericArray<u8, E::BuffSize> = GenericArray::default();
        self.stream.read_bytes(buff.as_mut())?;
        self.update_pos::<I>()?;
        Ok(Some(E::const_bin_parse(&buff)))
    }
}

impl<'a, E: ConstBinParsible, S: OutputBinaryStream, N: ArrayLength>
OutputElemStreams<E, N> for BinaryElemSpans<'a, E, S, N> {
    fn write_next_elem<const I: usize>(&mut self, elem: E) -> io::Result<()> {
        let mut buff: GenericArray<u8, E::BuffSize> = GenericArray::default();
        elem.const_bin_unparse(&mut buff);
        self.stream.write_bytes(buff.as_ref())?;
        self.update_pos::<I>()?;
        Ok(())
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        self.byte_ends[I] = Some(self.byte_offsets[I] + len);
        if self.get_pos::<I>()? > len {
            self.set_pos::<I>(len)?;
        }
        Ok(())
    }
}
