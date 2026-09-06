use std::{io, marker::PhantomData};

use generic_array::typenum::U2;

use crate::{
    chunks::ChunkSize, elems::Elem, streams::{
        grouping::UngroupedBinaryStream, spans::BinaryElemSpan, traits::{InputBinaryStreams, OutputBinaryStreams, OutputElemStream, Stream},
    },
};

use super::ChunkInfo;

mod utils;

/// Holds bit widths of chunk info header fields (in key).
#[derive(Clone, Copy)]
pub struct ChunkInfoWidths {
    pub is_literal: u8,
    pub size: u8,
    pub index: u8,
}

impl ChunkInfoWidths {
    /// Gets total size of all fields in bytes.
    pub fn total_size_bytes(&self) -> ChunkSize {
        ((self.is_literal as u64) + (self.size as u64) + (self.index as u64))
            .div_ceil(16) as ChunkSize
    }

    /// Gets maximum possible chunk size.
    pub fn max_size(&self) -> ChunkSize {
        (1 as ChunkSize) << self.size
    }
}

/// Reads chunks from pngfied data.
/// 
/// * `IMG_IDX` - Index of image stream in input set.
/// * `KEY_IDX` - Index of key stream in input set.
/// * `E` - Element type.
/// * `In` - A set type of two binary input streams (image and key).
/// * `Out` - A type of stream to output read chunks into.
/// 
pub struct ChunksReader<'a, 'b, const IMG_IDX: usize, const KEY_IDX: usize, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    input: &'a mut In,
    output: &'b mut Out,
    phantom: PhantomData<E>,
}

impl<'a, 'b, const IMG_IDX: usize, const KEY_IDX: usize, E, In, Out>
ChunksReader<'a, 'b, IMG_IDX, KEY_IDX, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    /// Constructs a new instance.
    /// 
    /// * `input` - A set of two binary input streams (image and key).
    /// * `output` - A stream to output read chunks into.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(input: &'a mut In, output: &'b mut Out) -> Self {
        Self { input, output, phantom: PhantomData }
    }

    /// Reads all chunks from input and writes it to output (using streams provided at construction).
    /// 
    /// Returns error if occurred.
    /// 
    pub fn extract_all(&mut self) -> io::Result<()> {
        let widths = self.read_widths()?;
        while let Some(_) = self.extract_next(&widths)? { }
        Ok(())
    }

    /// Reads widths from key.
    /// 
    /// Returns read widths, or error if occurred.
    /// 
    fn read_widths(&mut self) -> io::Result<ChunkInfoWidths> {
        let mut input = UngroupedBinaryStream::<'_, KEY_IDX, _, _>::new(self.input);
        utils::read_widths(&mut input)
    }

    /// Reads next chunk from input and writes it to output (using streams provided at construction).
    /// 
    /// * `widths` - Expected width of header fields (for each chunk in key).
    /// 
    /// Returns `None` if reached end of chunks (no chunks are left).
    /// Returns error if occurred.
    /// 
    fn extract_next(&mut self, widths: &ChunkInfoWidths) -> io::Result<Option<()>> {
        match self.read_next_chunk_info(widths)? {
            None => return Ok(None), // nothing more to read
            Some(ChunkInfo::Literal(elems)) =>
                utils::extract_literal(self.output, &elems)?,
            Some(ChunkInfo::Reference { index, size }) => {
                // only passing elements span of image stream to extract_reference
                let mut input = UngroupedBinaryStream::<'_, IMG_IDX, _, _>::new(self.input);
                let pos = input.get_pos()?;
                let mut input = BinaryElemSpan::new(&mut input, Some(pos), None);
                utils::extract_reference(&mut input, self.output, index, size)?
            },
        }
        Ok(Some(()))
    }

    /// Reads information (header) of next chunk from key.
    /// 
    /// * `widths` - Expected width of header fields (for each chunk in key).
    /// 
    /// Returns read chunk info, or `None` if no chunks are left.
    /// Returns error if occurred.
    /// 
    fn read_next_chunk_info(&mut self, widths: &ChunkInfoWidths) -> io::Result<Option<ChunkInfo<E>>> {
        // only passing key stream to read_chunk_info
        let mut input = UngroupedBinaryStream::<'_, KEY_IDX, _, _>::new(self.input);
        utils::read_chunk_info(&mut input, widths)
    }
}

/// Writes chunks into pngfied data.
/// 
/// * `IMG_IDX` - Index of image stream in output set.
/// * `KEY_IDX` - Index of key stream in output set.
/// * `E` - Element type.
/// * `In` - An iterator type of chunks to write.
/// * `Out` - A set type of two binary output streams (image and key).
/// 
pub struct ChunksWriter<'a, 'b, 'c, const IMG_IDX: usize, const KEY_IDX: usize, E, In, Out>
where
    E: Elem + 'a,
    In: Iterator<Item = &'a ChunkInfo<E>>,
    Out: OutputBinaryStreams<U2>,
{
    widths: ChunkInfoWidths,
    input: &'b mut In,
    output: &'c mut Out,
    cached_literals: Vec<E>,
    phantom: PhantomData<E>,
}

impl<'a, 'b, 'c, const IMG_IDX: usize, const KEY_IDX: usize, E, In, Out>
ChunksWriter<'a, 'b, 'c, IMG_IDX, KEY_IDX, E, In, Out>
where
    E: Elem + 'a,
    In: Iterator<Item = &'a ChunkInfo<E>>,
    Out: OutputBinaryStreams<U2>,
{
    /// Constructs a new instance.
    /// 
    /// * `widths` - Expected width of header fields (for each chunk in key).
    /// * `input` - An iterator of chunks to write.
    /// * `output` - A set of two binary output streams (image and key).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(widths: ChunkInfoWidths, input: &'b mut In, output: &'c mut Out) -> Self {
        Self { widths, input, output, cached_literals: vec![], phantom: PhantomData }
    }

    /// Writes chunks into pngfied data (using streams provided at construction).
    /// 
    /// Returns error if occurred.
    /// 
    pub fn write(&mut self) -> io::Result<()> {
        self.write_widths()?;

        let input = &mut *self.input;
        for chunk in input {
            // if current chunk is literal, cache to write later
            if let ChunkInfo::Literal(values) = chunk {
                self.cached_literals.extend(values);
            } else { // if current chunk is not literal
                // write all cached literals as one chunk
                Self::flush_cached_literals(self.output, &self.widths, &mut self.cached_literals)?;
                
                // write current chunk itself (only passing key stream to write_chunk_info)
                let mut output = UngroupedBinaryStream::<'_, KEY_IDX, _, _>::new(self.output);
                utils::write_chunk_info(&mut output, &self.widths, &chunk)?;
            }
        }
        Self::flush_cached_literals(self.output, &self.widths, &mut self.cached_literals)?;
        Ok(())
    }

    /// Writes widths field to key.
    /// 
    /// Returns error if occurred.
    /// 
    fn write_widths(&mut self) -> io::Result<()> {
        let mut output = UngroupedBinaryStream::<'_, KEY_IDX, _, _>::new(self.output);
        utils::write_widths(&mut output, &self.widths)
    }

    /// Flushes vector of cached literals as one literal chunk.
    fn flush_cached_literals(output: &mut Out,
                             widths: &ChunkInfoWidths,
                             cached_literals: &mut Vec<E>) -> io::Result<()> {
        if !cached_literals.is_empty() {
            let cached_literals = std::mem::take(cached_literals);

            // only passing key stream to write_chunk_info
            let mut output = UngroupedBinaryStream::<'_, KEY_IDX, _, _>::new(output);
            utils::write_chunk_info(&mut output, widths, &ChunkInfo::Literal(cached_literals))?;
        }
        Ok(())
    }
}
