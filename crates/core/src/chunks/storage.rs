use std::{io, marker::PhantomData};

use generic_array::typenum::U2;

use crate::{
    elems::Elem, streams::{grouping::UngroupedBinaryStream, spans::BinaryElemSpan, traits::{InputBinaryStreams, OutputBinaryStreams, OutputElemStream}},
};

use super::ChunkInfo;

mod utils;

/// Holds bit widths of chunk info header fields (in key).
#[derive(Clone, Copy)]
pub struct ChunkInfoWidths {
    pub is_literal: usize,
    pub size: usize,
    pub index: usize,
}

/// Reads chunks from pngfied data.
/// 
/// * `IMG_IDX` - Index of image stream in input set.
/// * `TAILER_IDX` - Index of tailed (key) stream in input set.
/// * `E` - Element type.
/// * `In` - A set type of two binary input streams (image and tailer/key).
/// * `Out` - A type of stream to output read chunks into.
/// 
pub struct ChunksReader<'a, 'b, const IMG_IDX: usize, const TAILER_IDX: usize, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    widths: ChunkInfoWidths,
    input: &'a mut In,
    output: &'b mut Out,
    phantom: PhantomData<E>,
}

impl<'a, 'b, const IMG_IDX: usize, const TAILER_IDX: usize, E, In, Out>
ChunksReader<'a, 'b, IMG_IDX, TAILER_IDX, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    /// Constructs a new instance.
    /// 
    /// * `widths` - Expected width of header fields (for each chunk in key).
    /// * `input` - A set of two binary input streams (image and tailer/key).
    /// * `output` - A stream to output read chunks into.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(widths: ChunkInfoWidths, input: &'a mut In, output: &'b mut Out) -> Self {
        Self { widths, input, output, phantom: PhantomData }
    }

    /// Reads next chunk from input and writes it to output (using streams provided at construction).
    /// 
    /// Returns `None` if reached end of chunks (no chunks are left).
    /// Returns error if occurred.
    /// 
    pub fn extract_next(&mut self) -> io::Result<Option<()>> {
        match self.read_next_chunk_info()? {
            None => return Ok(None), // nothing more to read
            Some(ChunkInfo::Literal(elems)) =>
                utils::extract_literal(self.output, &elems)?,
            Some(ChunkInfo::Reference { index, size }) => {
                // only passing elements span of image stream to extract_reference
                let mut input = UngroupedBinaryStream::<'_, IMG_IDX, _, _>::new(self.input);
                let mut input = BinaryElemSpan::new(&mut input, None, None);
                utils::extract_reference(&mut input, self.output, index, size)?
            },
        }
        Ok(Some(()))
    }

    /// Reads information (header) of next chunk from key.
    /// 
    /// Returns read chunk info, or `None` if no chunks are left.
    /// Returns error if occurred.
    /// 
    fn read_next_chunk_info(&mut self) -> io::Result<Option<ChunkInfo<E>>> {
        // only passing tailer stream to read_chunk_info
        let mut input = UngroupedBinaryStream::<'_, TAILER_IDX, _, _>::new(self.input);
        utils::read_chunk_info(&mut input, &self.widths)
    }
}

/// Writes chunks into pngfied data.
/// 
/// * `IMG_IDX` - Index of image stream in output set.
/// * `TAILER_IDX` - Index of tailed (key) stream in output set.
/// * `E` - Element type.
/// * `In` - An iterator type of chunks to write.
/// * `Out` - A set type of two binary output streams (image and tailer/key).
/// 
pub struct ChunksWriter<'a, 'b, const IMG_IDX: usize, const TAILER_IDX: usize, E, In, Out>
where
    E: Elem,
    In: Iterator<Item = ChunkInfo<E>>,
    Out: OutputBinaryStreams<U2>,
{
    widths: ChunkInfoWidths,
    input: &'a mut In,
    output: &'b mut Out,
    cached_literals: Vec<E>,
    phantom: PhantomData<E>,
}

impl<'a, 'b, const IMG_IDX: usize, const TAILER_IDX: usize, E, In, Out>
ChunksWriter<'a, 'b, IMG_IDX, TAILER_IDX, E, In, Out>
where
    E: Elem,
    In: Iterator<Item = ChunkInfo<E>>,
    Out: OutputBinaryStreams<U2>,
{
    /// Constructs a new instance.
    /// 
    /// * `widths` - Expected width of header fields (for each chunk in key).
    /// * `input` - An iterator of chunks to write.
    /// * `output` - A set of two binary output streams (image and tailer/key).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(widths: ChunkInfoWidths, input: &'a mut In, output: &'b mut Out) -> Self {
        let _ = (widths, input, output);
        todo!() // TODO: Implement
    }

    /// Writes chunks into pngfied data (using streams provided at construction).
    /// 
    /// Returns error if occurred.
    /// 
    pub fn write(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }
}
