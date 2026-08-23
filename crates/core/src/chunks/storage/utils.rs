use std::io;
use bitstream_io::{BigEndian, BitRead, BitWrite};

use crate::{
    chunks::{
        ChunkIndex,
        ChunkInfo,
        ChunkSize,
        storage::ChunkInfoWidths
    },
    elems::Elem,
    obtain_bits_reader,
    obtain_bits_writer,
    return_bits_reader,
    return_bits_writer,
    streams::{
        spans::BinaryElemSpan,
        traits::{
            InputElemStream,
            OutputElemStream,
            InputBinaryStream,
            OutputBinaryStream,
        },
    },
};

/// Reads a chunk (info) from key.
/// 
/// * `E` - Element type.
/// * `S` - Input stream type.
/// 
/// * `input` - Input stream to read info from.
/// * `widths` - Expected bit width of each header field.
/// 
/// Returns read chunk info, or `None` if reached end of key (nothing to read).
/// Returns error if occurred.
/// 
pub fn read_chunk_info<E, S>(input: &mut S, widths: &ChunkInfoWidths) -> io::Result<Option<ChunkInfo<E>>>
where
    E: Elem,
    S: InputBinaryStream,
{
    let mut bits = obtain_bits_reader!(input, BigEndian)?;
    let (is_literal, size) = match read_header(&mut bits, &widths)? {
        None => return Ok(None), // if end-of-stream before header, nothing to read
        Some(header) => header,  // actual read header (success)
    };

    Ok(Some(
        if is_literal {
            return_bits_reader!(bits, input)?;
            
            // read literal chunk info
            read_literal_chunk_info(input, size)?

        } else {
            // read reference chunk info
            let res = read_reference_chunk_info(&mut bits, size, widths)?;

            return_bits_reader!(bits, input)?;

            res
        }
    ))
}

/// Writes a chunk (info) to key.
/// 
/// * `E` - Element type.
/// * `S` - Output stream type.
/// 
/// * `output` - Output stream to write info to.
/// * `widths` - Expected bit width of each header field.
/// * `chunk` - Chunk info to write.
/// 
/// Returns error if occurred.
/// 
pub fn write_chunk_info<E, S>(output: &mut S, widths: &ChunkInfoWidths, chunk: &ChunkInfo<E>) -> io::Result<()>
where
    E: Elem,
    S: OutputBinaryStream,
{
    let header = match chunk {
        ChunkInfo::Reference { size, .. } => (false, *size),
        ChunkInfo::Literal(values) => (true, values.len() as ChunkSize)
    };
    let mut bits = obtain_bits_writer!(output, BigEndian)?;
    write_header(&mut bits, widths, header)?;
    match chunk {
        ChunkInfo::Literal(elems) => {
            return_bits_writer!(bits, output)?;

            // write literal chunk info
            write_literal_chunk_info(output, elems)?;
        },
        ChunkInfo::Reference { index, .. } => {
            // write reference chunk info
            write_reference_chunk_info(&mut bits, *index, widths)?;

            return_bits_writer!(bits, output)?;
        },
    }
    Ok(())
}

/// Reads a chunk info header from key.
/// 
/// * `bits` - A bit reader obtained from the key's input stream.
/// * `widths` - Expected bit width of each field.
/// 
/// Returns read chunk info header (is_literal, size), or `None` if reached end of key (nothing to read).
/// Returns error if occurred.
/// 
/// NOTE: "size" serves as literal size for literal chunks, and reference size for reference chunks.
/// 
pub fn read_header(bits: &mut impl BitRead,
                   widths: &ChunkInfoWidths) -> io::Result<Option<(bool, ChunkSize)>> {
    // try reading is_literal, 
    // if reached EOF it means file ended before header so we return None
    let read_is_literal = bits.read::<u8>(widths.is_literal as u32);
    let is_literal = match read_is_literal {
        Err(err) => match err.kind() {
            io::ErrorKind::UnexpectedEof => return Ok(None),
            _ => return Err(err),
        },
        Ok(is_literal) => 0 != is_literal,
    };

    // try reading size, 
    // if reached end-of-stream it mean file ended mid header so we propogate the error
    let size = bits.read::<ChunkSize>(widths.size as u32)?;

    Ok(Some((is_literal, size)))
}

/// Writes a chunk info header to key.
/// 
/// * `bits` - A bit writer obtained from the key's output stream.
/// * `widths` - Expected bit width of each field.
/// * `header` - Header to write (is_literal, size).
/// 
/// Returns error if occurred.
/// 
/// NOTE: "size" serves as literal size for literal chunks, and reference size for reference chunks.
/// 
pub fn write_header(bits: &mut impl BitWrite,
                    widths: &ChunkInfoWidths,
                    header: (bool, ChunkSize)) -> io::Result<()> {
    let (is_literal, size) = header;
    bits.write(widths.is_literal as u32, if is_literal { 1 } else { 0 })?;
    bits.write(widths.size as u32, size)?;
    Ok(())
}

/// Reads a literal chunk's (remaining) chunk info from key.
/// This function is to be invoked after header fields have already been read.
/// 
/// * `E` - Element type.
/// * `S` - Input stream type.
/// 
/// * `input` - Input stream to read info from.
/// * `size` - Literal chunk size.
/// 
/// Assumes there is data to read.
/// Returns read chunk info, or error if occurred.
/// 
pub fn read_literal_chunk_info<E, S>(input: &mut S, size: ChunkSize) -> io::Result<ChunkInfo<E>>
where
    E: Elem,
    S: InputBinaryStream,
{
    let mut elems = vec![];
    let offset = input.get_pos()?;
    let mut span: BinaryElemSpan<'_, E, S> = BinaryElemSpan::new(input, Some(offset), None);
    for _ in 0..size {
        let elem = span.read_next_elem()?;
        match elem {
            None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected EOF while reading literal chunk")),
            Some(elem) => elems.push(elem),
        }
    }
    Ok(ChunkInfo::Literal(elems))
}

/// Writes a literal chunk's (remaining) chunk info to key.
/// This function is to be invoked after header fields have already been written.
/// 
/// * `E` - Element type.
/// * `S` - Output stream type.
/// 
/// * `output` - Output stream to write info to.
/// * `elems` - Literal chunk data elements.
/// 
/// Returns error if occurred.
/// 
pub fn write_literal_chunk_info<E, S>(output: &mut S, elems: &Vec<E>) -> io::Result<()>
where
    E: Elem,
    S: OutputBinaryStream,
{
    let offset = output.get_pos()?;
    let mut span: BinaryElemSpan<'_, E, S> = BinaryElemSpan::new(output, Some(offset), None);
    for elem in elems {
        span.write_next_elem(*elem)?;
    }
    Ok(())
}

/// Reads a reference chunk's (remaining) chunk info from key.
/// This function is to be invoked after header fields have already been read.
/// 
/// * `E` - Element type.
/// 
/// * `bits` - A bit reader obtained from the key's input stream.
/// * `size` - Reference chunk size.
/// 
/// Assumes there is data to read.
/// Returns read chunk info, or error if occurred.
/// 
pub fn read_reference_chunk_info<E: Elem>(bits: &mut impl BitRead, size: ChunkSize,
                                          widths: &ChunkInfoWidths) -> io::Result<ChunkInfo<E>> {
    let index = bits.read::<ChunkIndex>(widths.index as u32)?;
    Ok(ChunkInfo::Reference { index, size })
}

/// Writes a reference chunk's (remaining) chunk info to key.
/// This function is to be invoked after header fields have already been written.
/// 
/// * `bits` - A bit writer obtained from the key's output stream.
/// * `index` - Start index of referenced data elements.
/// * `widths` - Expected bit width of each field.
/// 
/// Returns error if occurred.
/// 
pub fn write_reference_chunk_info(bits: &mut impl BitWrite,
                                  index: ChunkIndex,
                                  widths: &ChunkInfoWidths) -> io::Result<()> {
    bits.write(widths.index as u32, index)?;
    Ok(())
}

/// Extracts a literal chunk's data elements into a given output stream.
/// 
/// * `E` - Element type.
/// * `Out` - Output stream type.
/// 
/// * `output` - An output stream to write data elements to.
/// * `elems` - Raw literal elements to write.
/// 
/// Returns error if occurred.
/// 
pub fn extract_literal<E, Out>(output: &mut Out, elems: &Vec<E>) -> io::Result<()>
where
    E: Elem,
    Out: OutputElemStream<E>,
{
    for elem in elems {
        output.write_next_elem(*elem)?;
    }
    Ok(())
}

/// Extracts a reference chunk's data elements into a given output stream.
/// 
/// * `E` - Element type.
/// * `In` - Input stream type (to read raw elements from).
/// * `Out` - Output stream type.
/// 
/// * `input` - An input stream to read raw elements from.
/// * `output` - An output stream to write data elements to.
/// * `index` - Start index of referenced data elements.
/// * `size` - Count of referenced data elements.
/// 
/// Returns error if occurred.
/// 
pub fn extract_reference<E, Input, Output>(input: &mut Input,
                                           output: &mut Output,
                                           index: ChunkIndex,
                                           size: ChunkSize) -> io::Result<()>
where
    E: Elem,
    Input: InputElemStream<E>,
    Output: OutputElemStream<E>,
{
    input.set_pos(index)?;
    for _ in 0..size {
        let elem = match input.read_next_elem()? {
            Some(elem) => elem,
            None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Hit EOF while reading input")),
        };
        output.write_next_elem(elem)?;
    }
    Ok(())
}
