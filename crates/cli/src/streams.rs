use std::marker::PhantomData;

use generic_array::typenum::U2;
use pngfier_core::{
    elems::Elem,
    streams::traits::{
        InputBinaryStreams,
        InputElemStreams,
        OutputBinaryStreams,
        OutputElemStream,
    },
};

/// Compile streams index of input image in input streams set.
pub const COMPL_IN_IDX_IMG: usize = 0;

/// Compile streams index of input data in input streams set.
pub const COMPL_IN_IDX_DAT: usize = 1;

/// Compile streams index of output image in output streams set.
pub const COMPL_OUT_IDX_IMG: usize = 0;

/// Compile streams index of output key in output streams set.
pub const COMPL_OUT_IDX_KEY: usize = 1;

/// Holds input & output streams for compile operation.
/// 
/// * `E` - Element type (for input/output).
/// * `In` - Input streams type (elements streams set - in_img, in_data).
/// * `Out` - Output streams type (binary streams set - out_img, out_key).
/// 
pub struct CompileStreams<'a, E, In, Out>
where
    E: Elem,
    In: InputElemStreams<E, U2>,
    Out: OutputBinaryStreams<U2>,
{
    /// Input image stream (of image to ride) and data stream (of data to compile).
    pub input: &'a mut In,

    /// Output image stream (for result image) and key stream (for result key).
    pub output: &'a mut Out,

    phantom: PhantomData<E>,
}

impl<'a, E, In, Out> CompileStreams<'a, E, In, Out>
where
    E: Elem,
    In: InputElemStreams<E, U2>,
    Out: OutputBinaryStreams<U2>,
{
    /// Constructs a new instance.
    /// 
    /// * `input` - Input image stream (of image to ride), data stream (of data to compile).
    /// * `output` - Output image stream (for result image), key stream (for result key).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(input: &'a mut In, output: &'a mut Out) -> Self {
        Self { input, output, phantom: PhantomData }
    }
}

/// Extract streams index of input image in output streams set.
pub const XTRCT_IN_IDX_IMG: usize = 0;

/// ComExtractpile streams index of input key in output streams set.
pub const XTRCT_IN_IDX_KEY: usize = 1;

/// Holds input & output streams for extract operation.
/// 
/// * `E` - Element type (for input/output).
/// * `In` - Input streams type (binary streams set - in_img, in_key).
/// * `Out` - Output stream type (elements stream).
/// 
pub struct ExtractStreams<'a, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    /// Input image stream (of compiled image), key stream (of compiled image key).
    pub input: &'a mut In,

    /// Output data stream (for extracted data).
    pub out_data: &'a mut Out,

    phantom: PhantomData<E>,
}

impl<'a, E, In, Out> ExtractStreams<'a, E, In, Out>
where
    E: Elem,
    In: InputBinaryStreams<U2>,
    Out: OutputElemStream<E>,
{
    /// Constructs a new instance.
    /// 
    /// * `input` - Input image stream (of compiled image), key stream (of compiled image key).
    /// * `out_data` - Output data stream (for extracted data).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(input: &'a mut In, out_data: &'a mut Out) -> Self {
        Self { input, out_data, phantom: PhantomData }
    }
}
