use anyhow::{Context, Result};

use generic_array::GenericArray;
use pngfier_core::{
    chunks::{
        mapping::{ChunkMapper, reach::MatrixBasedReachMapper},
        storage::{ChunkInfoWidths, ChunksReader, ChunksWriter},
    }, elems::{Elem, RuntimeElemIndexesMatrix}, streams::{
        grouping::GroupedBinaryStreams,
        traits::{
            InputBinaryStream,
            InputElemStream,
            OutputBinaryStream,
            OutputElemStream,
        },
    },
};

use crate::configs::{compile::CompileStreams, extract::ExtractStreams};

/// Callback function which performs compile operation.
/// 
/// * `widths` - Field widths used in key storage.
/// * `streams` - Input & output streams to perform compile operation on.
/// 
/// Returns error if occurred.
/// 
pub fn compile<E, In, Out>(widths: &ChunkInfoWidths, streams: &mut [CompileStreams<E, In, Out>]) -> Result<()>
where
    E: Elem,
    In: InputElemStream<E>,
    Out: OutputBinaryStream,
{
    // cap minimum reference chunk size by size of fields sum (reference chunk size)
    // cap maximum reference chunk size by maximum representable chunk size
    let min_cap = widths.total_size_bytes();
    let max_cap = widths.max_size();

    for streams in streams {
        const IMG_IDX: usize = 0;
        const KEY_IDX: usize = 1;
        let mut output = GroupedBinaryStreams::new(
            GenericArray::from_array([&mut streams.out_img, &mut streams.out_key])
        );

        let mut img_matrix = RuntimeElemIndexesMatrix::new();
        let mut reach = MatrixBasedReachMapper::new(&mut streams.in_img, &mut streams.in_data, &mut img_matrix)
            .context("Failed to construct reach mapper")?;

        let chunks = ChunkMapper::new(&mut reach)
            .map_chunks(Some(min_cap), Some(max_cap))
            .context("Failed to map chunks")?;
        let chunks = &mut chunks.iter();
        let mut writer = ChunksWriter::<'_, '_, '_, IMG_IDX, KEY_IDX, _, _, _>::new(
            *widths, chunks, &mut output
        );

        writer.write().context("Failed to write chunks into output")?;
    }

    Ok(())
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
    In: InputBinaryStream,
    Out: OutputElemStream<E>,
{
    const IMG_IDX: usize = 0;
    const KEY_IDX: usize = 1;
    let mut input = GroupedBinaryStreams::new(
        GenericArray::from_array([&mut streams.in_img, &mut streams.in_key])
    );

    let mut reader = ChunksReader::<'_, '_, IMG_IDX, KEY_IDX, _, _, _>::new(
        &mut input, &mut streams.out_data
    );

    reader.extract_all().context("Failed to extract chunks")?;

    Ok(())
}
