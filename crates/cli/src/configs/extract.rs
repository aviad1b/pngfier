use anyhow::{Context, Result};

use generic_array::{GenericArray, typenum::U2};
use pngfier_core::{
    elems::Elem, streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        grouping::GroupedBinaryStreams,
        spans::BinaryElemSpan,
    },
};

use crate::streams::ExtractStreams;

/// Generates configuration for extract operation with input key path, 
/// then performs compile operation via a given callback.
/// 
/// * `callback` - Callback function which performs extract operation on streams.
/// * `in_img_path` - Path to input compiled image file.
/// * `in_key_path` - Path to input compiled key file.
/// * `out_data_path` - Path to output data file.
/// 
/// Returns error if occurred.
/// 
pub fn with_key<E, Callback>(mut callback: Callback,
                             in_img_path: &str, in_key_path: &str,
                             out_data_path: &str) -> Result<()>
where
    E: Elem,
    Callback: for <'a, 'b> FnMut(
        &mut ExtractStreams<E,
                            GroupedBinaryStreams<'a, U2, InputBinaryFileStream>,
                            BinaryElemSpan::<'b, E, OutputBinaryFileStream>>) -> Result<()>,
{
    let mut in_img = InputBinaryFileStream::new(&in_img_path)
        .context("Failed to read from output file")?;

    let mut in_key = InputBinaryFileStream::new(&in_key_path)
        .context("Failed to read from key file")?;

    let mut input = GroupedBinaryStreams::new(GenericArray::from_array([
        &mut in_img, &mut in_key
    ]));

    let mut out_data = OutputBinaryFileStream::new(&out_data_path)
        .context("Failed to write to output file")?;
    let mut out_data = BinaryElemSpan::<'_, E, _>::new(&mut out_data, None, None);

    callback(&mut ExtractStreams::new(&mut input, &mut out_data))
}
