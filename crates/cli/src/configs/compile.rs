use anyhow::{Context, Result};
use pngfier_core::{
    elems::Elem,
    streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        spans::BinaryElemSpan,
    },
};

use crate::configs::streams::CompileStreams;

/// Generates configuration for compile operation with source image path and output key path, 
/// then performs compile operation via a given callback.
/// 
/// * `callback` - Callback function which performs compile operation on streams.
/// * `in_img_path` - Path to input image file.
/// * `in_data_path` - Path to input data file.
/// * `out_img_path` - Path to output image file.
/// * `out_key_path` - Path to output key file.
/// 
/// Returns error if occurred.
/// 
pub fn path_with_key<E, Callback>(mut callback: Callback,
                                  in_img_path: &str, in_data_path: &str,
                                  out_img_path: &str, out_key_path: &str) -> Result<()>
where
    E: Elem,
    Callback: for <'a> FnMut(&mut [CompileStreams<E,
                                                  BinaryElemSpan::<'a, E, InputBinaryFileStream>,
                                                  OutputBinaryFileStream>]) -> Result<()>,
{
    // output image is identical to input one in this config
    std::fs::copy(in_img_path, out_img_path)
        .context("Failed to copy source to output")?;
    let out_img = OutputBinaryFileStream::new(&out_img_path)
        .context("Failed to write to output file")?;

    // key is stored separately in this config
    let out_key = OutputBinaryFileStream::new(&out_key_path)
        .context("Failed to write to key file")?;

    let mut in_img = InputBinaryFileStream::new(&in_img_path)
        .context("Failed to write to output")?;
    let in_img = BinaryElemSpan::<'_, E, _>::new(&mut in_img, None, None);

    let mut in_data = InputBinaryFileStream::new(&in_data_path)
        .context("Failed to read from input data")?;
    let in_data = BinaryElemSpan::<'_, E, _>::new(&mut in_data, None, None);

    callback(&mut [CompileStreams::new(in_img, in_data, out_img, out_key)])
}
