use std::marker::PhantomData;

use anyhow::{Context, Result, bail};
use pngfier_core::{
    chunks::storage::ChunkInfoWidths, elems::Elem, streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        spans::BinaryElemSpan,
        traits::{InputElemStream, OutputBinaryStream},
    },
};

use crate::{callbacks, commands::ImgSrc};

/// Holds input & output streams for compile operation.
/// 
/// * `E` - Element type (for input/output).
/// * `In` - Input stream type (elements stream).
/// * `Out` - Output stream type (binary stream).
/// 
pub struct CompileStreams<E, In, Out>
where
    E: Elem,
    In: InputElemStream<E>,
    Out: OutputBinaryStream,
{
    /// Input image stream (of image to ride).
    pub in_img: In,

    /// Input data stream (of data to compile).
    pub in_data: In,

    /// Output image stream (for result image).
    pub out_img: Out,

    /// Output key stream (for result key).
    pub out_key: Out,

    phantom: PhantomData<E>,
}

impl<E, In, Out> CompileStreams<E, In, Out>
where
    E: Elem,
    In: InputElemStream<E>,
    Out: OutputBinaryStream,
{
    /// Constructs a new instance.
    /// 
    /// * `in_img` - Input image stream (of image to ride).
    /// * `in_data` - Input data stream (of data to compile).
    /// * `out_img` - Output image stream (for result image).
    /// * `out_key` - Output key stream (for result key).
    /// 
    /// Returns constructed instance.
    /// 
    fn new(in_img: In, in_data: In, out_img: Out, out_key: Out) -> Self {
        Self { in_img, in_data, out_img, out_key, phantom: PhantomData }
    }
}

pub fn apply<E: Elem>(widths: &ChunkInfoWidths,
                      out_img: &String, in_file: &String,
                      img_src: &ImgSrc, key_file: &Option<String>) -> Result<()> {
    match (img_src, key_file) {
        (ImgSrc::Query(_), None) => bail!("Query-based compiling is not supported yet."),
        (ImgSrc::Query(_), Some(_)) => bail!("Query-based compiling is not supported yet."),
        (ImgSrc::Path(_), None) => bail!("Key file is mandatory for now."),
        (ImgSrc::Path(in_img_path), Some(out_key_path)) => path_with_key(
            |streams| callbacks::compile::<E, _, _>(widths, streams),
            &in_img_path, &in_file, &out_img, &out_key_path
        ),
    }
}

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
