use std::{cmp, io, marker::PhantomData};

use bitstream_io::{BitRead, BitWrite, Endianness};
use generic_array::{
    ArrayLength,
    GenericArray,
    functional::FunctionalSequence,
    typenum::{U1, Unsigned},
};

use super::{
    StreamPos,
    traits::{
        ConstBinParsible,
        InputBinaryStream,
        InputBinaryStreams,
        InputElemStream,
        InputElemStreams,
        OutputBinaryStream,
        OutputBinaryStreams,
        OutputElemStream,
        OutputElemStreams,
        Stream,
        Streams
    },
};

/// Makes a generic array of stream pos options from a slice of such.
pub fn opt_array<N: ArrayLength>(vals: &[Option<StreamPos>]) -> GenericArray<Option<StreamPos>, N> {
	GenericArray::from_iter(vals.iter().copied())
}

/// Reads spans of elements over a binary stream.
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

        let stream_pos = self.stream.get_pos()?;
        if let Some(byte_end) = self.byte_ends[I] && stream_pos >= byte_end {
            return Ok(None); // nothing to read
        }

        let bytes_left = self.stream.get_size()? - stream_pos;
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
        self.ensure_pos::<I>()?;
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

/// Reads a span of elements over a binary stream.
pub struct BinaryElemSpan<'a, E: ConstBinParsible, S: Stream> {
    base: BinaryElemSpans<'a, E, S, U1>,
}

impl<'a, E: ConstBinParsible, S: Stream> BinaryElemSpan<'a, E, S> {
    /// Constructs a new instance.
    /// 
    /// * `stream` - Base binary stream.
    /// * `byte_offset` - Optional starting byte offset index.
    /// * `byte_end` - Optional ending byte index.
    /// Resulting span will read `stream` in range [byte_offset,byte_end).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(stream: &'a mut S,
               byte_offset: Option<StreamPos>,
               byte_end: Option<StreamPos>) -> Self {
        Self {
			base: BinaryElemSpans::new(
				stream,
                GenericArray::from_array([byte_offset]),
				GenericArray::from_array([byte_end]),
			)
		}
    }
}

impl<'a, E: ConstBinParsible, S: Stream> Stream for BinaryElemSpan<'a, E, S> {
    fn rewind(&mut self) -> io::Result<()> {
        self.base.rewind::<0>()
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        self.base.get_pos::<0>()
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.base.set_pos::<0>(pos)
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        self.base.get_size::<0>()
    }
}

impl<'a, E: ConstBinParsible, S: InputBinaryStream> InputElemStream<E> for BinaryElemSpan<'a, E, S> {
    fn read_next_elem(&mut self) -> io::Result<Option<E>> {
        self.base.read_next_elem::<0>()
    }
}

impl<'a, E: ConstBinParsible, S: OutputBinaryStream> OutputElemStream<E> for BinaryElemSpan<'a, E, S> {
    fn write_next_elem(&mut self, elem: E) -> io::Result<()> {
        self.base.write_next_elem::<0>(elem)
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        self.base.truncate::<0>(len)
    }
}

/// Reads spans of bytes over a binary stream.
pub struct BinarySpans<'a, S: Stream, N: ArrayLength> {
    stream: &'a mut S,
    offsets: GenericArray<StreamPos, N>,
    ends: GenericArray<Option<StreamPos>, N>, // non-inclusive
    poses: GenericArray<StreamPos, N>,
}

impl<'a, S: Stream, N: ArrayLength> BinarySpans<'a, S, N> {
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
               offsets: GenericArray<Option<StreamPos>, N>,
               ends: GenericArray<Option<StreamPos>, N>) -> Self {
        let offsets = offsets.map(|byte_offset| byte_offset.map_or(0, |x| x));
        let poses = GenericArray::default(); // all start at local index 0
        Self {
            stream,
            offsets,
            ends,
            poses,
        }
    }

    /// Converts a global (stream) position to a local (span) one.
    /// 
    /// * `I` - Span index in set.
    /// 
    /// * `global_pos` - Global position to convert to a local one.
    /// 
    /// Returns local position (for stream indexed `I`).
    /// 
    fn pos_global_to_local<const I: usize>(&self, global_pos: StreamPos) -> StreamPos {
        global_pos - self.offsets[I]
    }

    /// Converts a local (span) position to a global (stream) one.
    /// 
    /// * `I` - Span index in set.
    /// 
    /// * `local_pos` - Local position to convert to a global one.
    /// 
    /// Returns global position (converted from local of stream indexed `I`).
    /// 
    fn pos_local_to_global<const I: usize>(&self, local_pos: StreamPos) -> StreamPos {
        local_pos + self.offsets[I]
    }

    /// Updates cached pos to match file pos
    /// 
    /// * `I` - Span index in set.
    /// 
    /// Returns error if occurred.
    /// 
    fn update_pos<const I: usize>(&mut self) -> io::Result<()> {
        let global_pos = self.stream.get_pos()?;
        self.poses[I] = self.pos_global_to_local::<I>(global_pos);
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

impl<'a, S: Stream, N: ArrayLength> Streams<N> for BinarySpans<'a, S, N> {
    fn rewind<const I: usize>(&mut self) -> io::Result<()> {
        self.set_pos::<I>(0)
    }

    fn get_pos<const I: usize>(&mut self) -> io::Result<StreamPos> {
        Ok(self.poses[I])
    }

    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()> {
        let global_pos = self.pos_local_to_global::<I>(pos);
        self.stream.set_pos(global_pos)?;
        self.poses[I] = pos;
        Ok(())
    }

    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos> {
        let max_size = self.ends[I].map(|byte_end| byte_end - self.offsets[I] + 1);
        let read_size = self.stream.get_size()? - self.offsets[I];
        let actual_size = max_size.map_or(read_size, |max_size| {
            cmp::min(read_size, max_size)
        });

        Ok(actual_size)
    }
}

impl<'a, S: InputBinaryStream, N: ArrayLength> InputBinaryStreams<N> for BinarySpans<'a, S, N> {
    fn read_bytes<const I: usize>(&mut self, buff: &mut [u8]) -> io::Result<()> {
        self.ensure_pos::<I>()?;

        let bytes_left = self.get_size::<I>()? - self.get_pos::<I>()?;
        if bytes_left < buff.len() as StreamPos {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached end of span early"));
        }

        self.stream.read_bytes(buff)?;
        self.update_pos::<I>()?;
        Ok(())
    }

    fn obtain_bits_reader<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        self.ensure_pos::<I>()?;
        self.stream.obtain_bits_reader(endianness)
    }
}

impl<'a, S: OutputBinaryStream, N: ArrayLength> OutputBinaryStreams<N> for BinarySpans<'a, S, N> {
    fn write_bytes<const I: usize>(&mut self, buff: &[u8]) -> io::Result<()> {
        self.ensure_pos::<I>()?;
        self.stream.write_bytes(buff)?;
        self.update_pos::<I>()?;
        Ok(())
    }

    fn obtain_bits_writer<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite> {
        self.ensure_pos::<I>()?;
        self.stream.obtain_bits_writer(endianness)
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        self.ensure_pos::<I>()?;
        self.stream.truncate(len)?;
        self.update_pos::<I>()?;
        Ok(())
    }
}
