use anyhow::{Context, Result};
use pngfier_core::{
    elems::Elem,
    streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
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
    Callback: for <'a> FnMut(&mut ExtractStreams<E,
                                                 InputBinaryFileStream,
                                                 BinaryElemSpan::<'a, E, OutputBinaryFileStream>>) -> Result<()>,
{
    let in_img = InputBinaryFileStream::new(&in_img_path)
        .context("Failed to read from output file")?;

    let in_key = InputBinaryFileStream::new(&in_key_path)
        .context("Failed to read from key file")?;

    let mut out_data = OutputBinaryFileStream::new(&out_data_path)
        .context("Failed to write to output file")?;
    let out_data = BinaryElemSpan::<'_, E, _>::new(&mut out_data, None, None);

    callback(&mut ExtractStreams::new(in_img, in_key, out_data))
}
