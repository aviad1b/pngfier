use anyhow::{Context, Result};
use generic_array::{GenericArray, typenum::U2};
use image_search::{Arguments, download};
use pngfier_core::{
    elems::Elem,
    png::riding::PngRider,
    streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream, TwoWayBinaryFileStream},
        grouping::{GroupedBinaryStreams, GroupedElemStreams},
        spans::{BinaryElemSpan, BinarySpans},
    },
};

use crate::{streams::{COMPL_OUT_IDX_IMG, COMPL_OUT_IDX_KEY, CompileStreams}, utils::TempDir};

const IMG_LIMIT: usize = 3;

pub async fn query_no_key<E, Callback>(mut callback: Callback, in_img_query: &str,
                                       in_data_path: &str, out_img_path: &str) -> Result<f64>
where
    E: Elem,
    Callback: for <'a, 'b> FnMut(
        &mut CompileStreams<E,
                            GroupedElemStreams<'a, E, U2, BinaryElemSpan<'b, E, InputBinaryFileStream>>,
                            BinarySpans<'a, TwoWayBinaryFileStream, U2>>) -> Result<f64>,
{
    let dir = TempDir::new().context("Failed to create temporary directory")?;
    let args = Arguments::new(in_img_query, IMG_LIMIT)
        .directory(dir.path_str());
    let paths = download(args).await.context("Failed to download image(s)")?;
    let mut overheads = vec![];

    // try compiling based on each downloaded image
    for path in &paths {
        let path = &path.display().to_string();
        let mut out_file = TwoWayBinaryFileStream::new(&path)
            .context("Failed to write to output file")?;

        // key is stored using PNG riding
        let mut output = PngRider::<'_, COMPL_OUT_IDX_IMG, COMPL_OUT_IDX_KEY, _>::new(&mut out_file)
            .context("Failed to initialize PNG rider")?;

        let mut in_img = InputBinaryFileStream::new(&path)
            .context("Failed to write to output")?;
        let mut in_img = BinaryElemSpan::<'_, E, _>::new(&mut in_img, None, None);

        let mut in_data = InputBinaryFileStream::new(&in_data_path)
            .context("Failed to read from input data")?;
        let mut in_data = BinaryElemSpan::<'_, E, _>::new(&mut in_data, None, None);

        let mut input = GroupedElemStreams::new(GenericArray::from_array([
            &mut in_img, &mut in_data
        ]));

        overheads.push(callback(&mut CompileStreams::new(&mut input, &mut output.streams))?);
    }

    // select image with minimal overhead and copy onto output
    let (min_idx, min_overhead) = overheads.iter().enumerate()
        .min_by(|(_, a), (_, b)| f64::total_cmp(a, b))
        .context("Failed to find fitting image")?;
    
    std::fs::copy(&paths[min_idx], out_img_path)
        .context("Failed to copy result to output")?;

    Ok(*min_overhead)
}

/// Generates configuration for compile operation with source image path and no output key file 
/// (key is to be stored using PNG riding), then performs compile operation via a given callback.
/// 
/// * `callback` - Callback function which performs compile operation on streams (returns space overhead).
/// * `in_img_path` - Path to input image file.
/// * `in_data_path` - Path to input data file.
/// * `out_img_path` - Path to output image file.
/// 
/// Returns space overhead (fraction), or error if occurred.
/// 
pub fn path_no_key<E, Callback>(mut callback: Callback, in_img_path: &str,
                                in_data_path: &str, out_img_path: &str) -> Result<f64>
where
    E: Elem,
    Callback: for <'a, 'b> FnMut(
        &mut CompileStreams<E,
                            GroupedElemStreams<'a, E, U2, BinaryElemSpan<'b, E, InputBinaryFileStream>>,
                            BinarySpans<'a, TwoWayBinaryFileStream, U2>>) -> Result<f64>,
{
    // output image is identical to input one in this config
    std::fs::copy(in_img_path, out_img_path)
        .context("Failed to copy source to output")?;
    let mut out_file = TwoWayBinaryFileStream::new(&out_img_path)
        .context("Failed to write to output file")?;

    // key is stored using PNG riding
    let mut output = PngRider::<'_, COMPL_OUT_IDX_IMG, COMPL_OUT_IDX_KEY, _>::new(&mut out_file)
        .context("Failed to initialize PNG rider")?;

    let mut in_img = InputBinaryFileStream::new(&in_img_path)
        .context("Failed to write to output")?;
    let mut in_img = BinaryElemSpan::<'_, E, _>::new(&mut in_img, None, None);

    let mut in_data = InputBinaryFileStream::new(&in_data_path)
        .context("Failed to read from input data")?;
    let mut in_data = BinaryElemSpan::<'_, E, _>::new(&mut in_data, None, None);

    let mut input = GroupedElemStreams::new(GenericArray::from_array([
        &mut in_img, &mut in_data
    ]));

    callback(&mut CompileStreams::new(&mut input, &mut output.streams))
}

/// Generates configuration for compile operation with source image path and output key path, 
/// then performs compile operation via a given callback.
/// 
/// * `callback` - Callback function which performs compile operation on streams (returns space overhead).
/// * `in_img_path` - Path to input image file.
/// * `in_data_path` - Path to input data file.
/// * `out_img_path` - Path to output image file.
/// * `out_key_path` - Path to output key file.
/// 
/// Returns space overhead (fraction), or error if occurred.
/// 
pub fn path_with_key<E, Callback>(mut callback: Callback,
                                  in_img_path: &str, in_data_path: &str,
                                  out_img_path: &str, out_key_path: &str) -> Result<f64>
where
    E: Elem,
    Callback: for <'a, 'b> FnMut(
        &mut CompileStreams<E,
                            GroupedElemStreams<'a, E, U2, BinaryElemSpan<'b, E, InputBinaryFileStream>>,
                            GroupedBinaryStreams<'a, U2, OutputBinaryFileStream>>) -> Result<f64>,
{
    // output image is identical to input one in this config
    std::fs::copy(in_img_path, out_img_path)
        .context("Failed to copy source to output")?;
    let mut out_img = OutputBinaryFileStream::new(&out_img_path)
        .context("Failed to write to output file")?;

    // key is stored separately in this config
    let mut out_key = OutputBinaryFileStream::new(&out_key_path)
        .context("Failed to write to key file")?;

    let mut output = GroupedBinaryStreams::new(GenericArray::from_array([
        &mut out_img, &mut out_key
    ]));

    let mut in_img = InputBinaryFileStream::new(&in_img_path)
        .context("Failed to write to output")?;
    let mut in_img = BinaryElemSpan::<'_, E, _>::new(&mut in_img, None, None);

    let mut in_data = InputBinaryFileStream::new(&in_data_path)
        .context("Failed to read from input data")?;
    let mut in_data = BinaryElemSpan::<'_, E, _>::new(&mut in_data, None, None);

    let mut input = GroupedElemStreams::new(GenericArray::from_array([
        &mut in_img, &mut in_data
    ]));

    callback(&mut CompileStreams::new(&mut input, &mut output))
}
