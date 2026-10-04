use anyhow::{Context, Result};

use generic_array::typenum::U2;
use pngfier_core::{
    chunks::{
        mapping::{ChunkMapper, reach::MatrixBasedReachMapper},
        storage::{ChunkInfoWidths, ChunksReader, ChunksWriter},
    },
    elems::{Elem, RuntimeElemIndexesMatrix},
    streams::traits::{
        InputBinaryStreams,
        InputElemStreams,
        OutputBinaryStreams,
        OutputElemStream,
    },
};

use crate::streams::{
    COMPL_IN_IDX_DAT,
    COMPL_IN_IDX_IMG,
    COMPL_OUT_IDX_IMG,
    COMPL_OUT_IDX_KEY,
    XTRCT_IN_IDX_IMG,
    XTRCT_IN_IDX_KEY,
    CompileStreams,
    ExtractStreams,
};

/// Callback function which performs compile operation.
/// 
/// * `widths` - Field widths used in key storage.
/// * `streams` - Input & output streams to perform compile operation on.
/// 
/// Returns space overhead (fraction), or error if occurred.
/// 
pub fn compile<E, In, Out>(widths: &ChunkInfoWidths, streams: &mut CompileStreams<E, In, Out>) -> Result<f64>
where
    E: Elem,
    In: InputElemStreams<E, U2>,
    Out: OutputBinaryStreams<U2>,
{
    // cap minimum reference chunk size by size of fields sum (reference chunk size)
    // cap maximum reference chunk size by maximum representable chunk size
    let min_cap = widths.total_size_bytes();
    let max_cap = widths.max_size();

    let mut img_matrix = RuntimeElemIndexesMatrix::new();
    let mut reach = MatrixBasedReachMapper::<'_, '_, COMPL_IN_IDX_IMG, COMPL_IN_IDX_DAT, _, _, _>::new(
        streams.input, &mut img_matrix
    ).context("Failed to construct reach mapper")?;

    let chunks = ChunkMapper::new(&mut reach)
        .map_chunks(Some(min_cap), Some(max_cap))
        .context("Failed to map chunks")?;
    let chunks = &mut chunks.iter();
    let mut writer = ChunksWriter::<'_, '_, '_, COMPL_OUT_IDX_IMG, COMPL_OUT_IDX_KEY, _, _, _>::new(
        *widths, chunks, streams.output
    );

    writer.write().context("Failed to write chunks into output")?;

    // overhead is defined as (dst_size-src_size)/src_size
    let src_size = streams.input.get_size::<COMPL_IN_IDX_DAT>()? + 
        streams.input.get_size::<COMPL_IN_IDX_IMG>()?;
    let dst_size = streams.output.get_size::<COMPL_OUT_IDX_IMG>()? + 
        streams.output.get_size::<COMPL_OUT_IDX_KEY>()?;
    Ok((dst_size - src_size) as f64 / dst_size as f64)
}

/// Callback function which performs extract operation.
/// 
/// * `streams` - Input & output streams to perform extract operation on.
/// 
/// Returns error if occurred.
/// 
pub fn extract<E, In, Out>(streams: &mut ExtractStreams<E, In, Out>) -> Result<()>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    let mut reader = ChunksReader::<'_, '_, XTRCT_IN_IDX_IMG, XTRCT_IN_IDX_KEY, _, _, _>::new(
        streams.input, streams.out_data
    );

    reader.extract_all().context("Failed to extract chunks")?;

    Ok(())
}
