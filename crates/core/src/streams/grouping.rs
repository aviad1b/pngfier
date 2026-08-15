use std::{io, marker::PhantomData};

use bitstream_io::{BitRead, BitWrite, Endianness};
use generic_array::{ArrayLength, GenericArray};

use super::{
    StreamPos,
    traits::{
        Stream,
        Streams,
        InputBinaryStream,
        InputBinaryStreams,
        OutputBinaryStream,
        OutputBinaryStreams,
        InputElemStream,
        InputElemStreams,
        OutputElemStream,
        OutputElemStreams,
    },
};

/// Implementation of element-stream-set abstractions that simply groups together
/// a bunch of streams of the same type.
/// 
/// * `E` - Element type.
/// * `N` - Amount of streams in set (amount of grouped streams).
/// * `S` - Base stream type.
/// 
pub struct GroupedElemStreams<'a, E, N: ArrayLength, S: Stream> {
    streams: GenericArray<&'a mut S, N>,
    phantom: PhantomData<E>,
}

impl<'a, E, N: ArrayLength, S: Stream> GroupedElemStreams<'a, E, N, S> {
    /// Constructs a new instance.
    /// 
    /// * `streams` - Array of streams to group together as a set.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(streams: GenericArray<&'a mut S, N>) -> Self {
        Self { streams, phantom: PhantomData }
    }
}

impl<'a, E, N: ArrayLength, S: Stream> Streams<N> for GroupedElemStreams<'a, E, N, S> {
    fn rewind<const I: usize>(&mut self) -> io::Result<()> {
        self.streams[I].rewind()
    }

    fn get_pos<const I: usize>(&mut self) -> io::Result<StreamPos> {
        self.streams[I].get_pos()
    }

    fn set_pos<const I: usize>(&mut self, pos: StreamPos) -> io::Result<()> {
        self.streams[I].set_pos(pos)
    }

    fn get_size<const I: usize>(&mut self) -> io::Result<StreamPos> {
        self.streams[I].get_size()
    }
}

impl<'a, E, N: ArrayLength, S: InputElemStream<E>> InputElemStreams<E, N>
for GroupedElemStreams<'a, E, N, S> {
    fn read_next_elem<const I: usize>(&mut self) -> io::Result<Option<E>> {
        self.streams[I].read_next_elem()
    }
}

impl<'a, E, N: ArrayLength, S: OutputElemStream<E>> OutputElemStreams<E, N>
for GroupedElemStreams<'a, E, N, S> {
    fn write_next_elem<const I: usize>(&mut self, elem: E) -> io::Result<()> {
        self.streams[I].write_next_elem(elem)
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        self.streams[I].truncate(len)
    }
}

/// Implementation of element-based stream abstractions that simply reference 
/// one stream out of a set.
/// 
/// * `I` - Index of referenced stream in set.
/// * `E` - Element type.
/// * `N` - Amount of streams in original set.
/// * `S` - Base streams set type.
/// 
pub struct UngroupedElemStream<'a, const I: usize, E, N: ArrayLength, S: Streams<N>> {
    streams: &'a mut S,
    phantom: PhantomData<(E, N)>,
}

impl<'a, const I: usize, E, N: ArrayLength, S: Streams<N>> UngroupedElemStream<'a, I, E, N, S> {
    /// Constructs a new instance.
    /// 
    /// * `streams` - Streams set to reference one stream from (index `I`).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(streams: &'a mut S) -> Self {
        Self { streams, phantom: PhantomData }
    }
}

impl<'a, const I: usize, E, N: ArrayLength, S: Streams<N>>
Stream for UngroupedElemStream<'a, I, E, N, S> {
    fn rewind(&mut self) -> io::Result<()> {
        self.streams.rewind::<I>()
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        self.streams.get_pos::<I>()
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.streams.set_pos::<I>(pos)
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        self.streams.get_size::<I>()
    }
}

impl<'a, const I: usize, E, N: ArrayLength, S: InputElemStreams<E, N>>
InputElemStream<E> for UngroupedElemStream<'a, I, E, N, S> {
    fn read_next_elem(&mut self) -> io::Result<Option<E>> {
        self.streams.read_next_elem::<I>()
    }
}

impl<'a, const I: usize, E, N: ArrayLength, S: OutputElemStreams<E, N>>
OutputElemStream<E> for UngroupedElemStream<'a, I, E, N, S> {
    fn write_next_elem(&mut self, elem: E) -> io::Result<()> {
        self.streams.write_next_elem::<I>(elem)
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}

/// Implementation of binary-stream-set abstractions that simply groups together
/// a bunch of streams of the same type.
/// 
/// * `N` - Amount of streams in set (amount of grouped streams).
/// * `S` - Base stream type.
/// 
pub struct GroupedBinaryStreams<'a, N: ArrayLength, S: Stream> {
    streams: GenericArray<&'a mut S, N>,
    phantom: PhantomData<N>,
}

impl<'a, N: ArrayLength, S: Stream> GroupedBinaryStreams<'a, N, S> {
    /// Constructs a new instance.
    /// 
    /// * `streams` - Array of streams to group together as a set.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(streams: GenericArray<&'a mut S, N>) -> Self {
        let _ = streams;
        todo!() // TODO: Implement
    }
}

impl<'a, N: ArrayLength, S: Stream> Streams<N> for GroupedBinaryStreams<'a, N, S> {
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

impl<'a, N: ArrayLength, S: InputBinaryStream> InputBinaryStreams<N>
for GroupedBinaryStreams<'a, N, S> {
    fn read_bytes<const I: usize>(&mut self, buff: &mut [u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_reader<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        let _ = endianness;
        Err::<bitstream_io::BitReader<std::fs::File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }
}

impl<'a, N: ArrayLength, S: OutputBinaryStream> OutputBinaryStreams<N>
for GroupedBinaryStreams<'a, N, S> {
    fn write_bytes<const I: usize>(&mut self, buff: &[u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_writer<const I: usize>(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite> {
        let _ = endianness;
        Err::<bitstream_io::BitWriter<std::fs::File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}

/// Implementation of binary stream abstractions that simply reference 
/// one stream out of a set.
/// 
/// * `I` - Index of referenced stream in set.
/// * `N` - Amount of streams in original set.
/// * `S` - Base streams set type.
/// 
pub struct UngroupedBinaryStream<'a, const I: usize, N: ArrayLength, S: Streams<N>> {
    streams: &'a mut S,
    phantom: PhantomData<N>,
}

impl<'a, const I: usize, N: ArrayLength, S: Streams<N>> UngroupedBinaryStream<'a, I, N, S> {
    /// Constructs a new instance.
    /// 
    /// * `streams` - Streams set to reference one stream from (index `I`).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(streams: &'a mut S) -> Self {
        let _ = streams;
        todo!() // TODO: Implement
    }
}

impl<'a, const I: usize, N: ArrayLength, S: Streams<N>>
Stream for UngroupedBinaryStream<'a, I, N, S> {
    fn rewind(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        let _ = pos;
        todo!() // TODO: Implement
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }
}

impl<'a, const I: usize, N: ArrayLength, S: InputBinaryStreams<N>>
InputBinaryStream for UngroupedBinaryStream<'a, I, N, S> {
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        let _ = endianness;
        Err::<bitstream_io::BitReader<std::fs::File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }
}

impl<'a, const I: usize, N: ArrayLength, S: OutputBinaryStreams<N>>
OutputBinaryStream for UngroupedBinaryStream<'a, I, N, S> {
    fn write_bytes(&mut self, buff: &[u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite> {
        let _ = endianness;
        Err::<bitstream_io::BitWriter<std::fs::File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}
