use std::marker::PhantomData;

use anyhow::{Context, Result};
use pngfier_core::{
    elems::Elem, streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        spans::BinaryElemSpan,
        traits::{InputBinaryStream, OutputElemStream},
    },
};

/// Holds input & output streams for extract operation.
/// 
/// * `E` - Element type (for input/output).
/// * `In` - Input stream type (binary stream).
/// * `Out` - Output stream type (elements stream).
/// 
pub struct ExtractStreams<E, In, Out>
where
    E: Elem,
    In: InputBinaryStream,
    Out: OutputElemStream<E>,
{
    /// Input image stream (of compiled image).
    pub in_img: In,

    /// Input key stream (of compiled image key).
    pub in_key: In,

    /// Output data stream (for extracted data).
    pub out_data: Out,

    phantom: PhantomData<E>,
}

impl<E, In, Out> ExtractStreams<E, In, Out>
where
    E: Elem,
    In: InputBinaryStream,
    Out: OutputElemStream<E>,
{
    /// Constructs a new instance.
    /// 
    /// * `in_img` - Input image stream (of compiled image).
    /// * `in_key` - Input key stream (of compiled image key).
    /// * `out_data` - Output data stream (for extracted data).
    /// 
    /// Returns constructed instance.
    /// 
    fn new(in_img: In, in_key: In, out_data: Out) -> Self {
        Self { out_data, in_img, in_key, phantom: PhantomData }
    }
}

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
