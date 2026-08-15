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
