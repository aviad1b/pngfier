use std::{io, marker::PhantomData};

use generic_array::{ArrayLength, GenericArray};

use super::{
    StreamPos,
    traits::{
        Stream,
        Streams,
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
    pub fn new(streams: GenericArray<&'a mut S, N>) -> Self {
        let _ = streams;
        todo!() // TODO: Implement
    }
}

impl<'a, E, N: ArrayLength, S: Stream> Streams<N> for GroupedElemStreams<'a, E, N, S> {
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

impl<'a, E, N: ArrayLength, S: InputElemStream<E>> InputElemStreams<E, N>
for GroupedElemStreams<'a, E, N, S> {
    fn read_next_elem<const I: usize>(&mut self) -> io::Result<Option<E>> {
        todo!() // TODO: Implement
    }
}

impl<'a, E, N: ArrayLength, S: OutputElemStream<E>> OutputElemStreams<E, N>
for GroupedElemStreams<'a, E, N, S> {
    fn write_next_elem<const I: usize>(&mut self, elem: E) -> io::Result<()> {
        let _ = elem;
        todo!() // TODO: Implement
    }

    fn truncate<const I: usize>(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
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
    pub fn new(streams: &'a mut S) -> Self {
        let _ = streams;
        todo!() // TODO: Implement
    }
}

impl<'a, const I: usize, E, N: ArrayLength, S: Streams<N>>
Stream for UngroupedElemStream<'a, I, E, N, S> {
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

impl<'a, const I: usize, E, N: ArrayLength, S: InputElemStreams<E, N>>
InputElemStream<E> for UngroupedElemStream<'a, I, E, N, S> {
    fn read_next_elem(&mut self) -> io::Result<Option<E>> {
        todo!() // TODO: Implement
    }
}

impl<'a, const I: usize, E, N: ArrayLength, S: OutputElemStreams<E, N>>
OutputElemStream<E> for UngroupedElemStream<'a, I, E, N, S> {
    fn write_next_elem(&mut self, elem: E) -> io::Result<()> {
        let _ = elem;
        todo!() // TODO: Implement
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}
