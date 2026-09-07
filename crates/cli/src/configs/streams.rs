use std::marker::PhantomData;

use pngfier_core::{
    elems::Elem,
    streams::traits::{
        InputBinaryStream,
        InputElemStream,
        OutputBinaryStream,
        OutputElemStream
    }
};

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
    pub fn new(in_img: In, in_data: In, out_img: Out, out_key: Out) -> Self {
        Self { in_img, in_data, out_img, out_key, phantom: PhantomData }
    }
}

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
    pub fn new(in_img: In, in_key: In, out_data: Out) -> Self {
        Self { out_data, in_img, in_key, phantom: PhantomData }
    }
}
