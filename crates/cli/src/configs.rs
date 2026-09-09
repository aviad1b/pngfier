use anyhow::{Result, bail};
use pngfier_core::{chunks::storage::ChunkInfoWidths, elems::Elem};

use crate::{callbacks, commands::ImgSrc};

mod compile;
mod extract;

/// Runs compile operation (selecting configuration based on params).
/// 
/// * `widths` - Field widths used in key storage.
/// * `out_img` - Path to store output image to.
/// * `in_file` - Input file to compile into a PNG.
/// * `img_src` - Image source path (of image to override).
/// * `key_file` - Optional path to store key to (instead of using PNG riding).
/// 
/// Returns error if occurred.
/// 
pub fn apply_compile<E: Elem>(widths: &ChunkInfoWidths,
                              out_img: &String, in_file: &String,
                              img_src: &ImgSrc, key_file: &Option<String>) -> Result<()> {
    match (img_src, key_file) {
        (ImgSrc::Query(_), None) => bail!("Query-based compiling is not supported yet."),
        (ImgSrc::Query(_), Some(_)) => bail!("Query-based compiling is not supported yet."),
        (ImgSrc::Path(_), None) => bail!("Key file is mandatory for now."),
        (ImgSrc::Path(in_img_path), Some(out_key_path)) => compile::path_with_key(
            |streams| callbacks::compile::<E, _, _>(widths, streams),
            in_img_path, in_file, out_img, out_key_path
        ),
    }?;
    Ok(())
}

/// Runs extract operation (selecting configuration based on params).
/// 
/// * `in_img` - Path to compiled image to extract data from.
/// * `out_file` - File path to store extracted data to.
/// * `key_file` - Optional path to read key from (instead of assuming PNG riding).
/// 
/// Returns error if occurred.
/// 
pub fn apply_extract<E: Elem>(in_img: &String, out_file: &String, key_file: &Option<String>) -> Result<()> {
    match key_file {
        Some(in_key_path) => extract::with_key(
            |streams| callbacks::extract::<E, _, _>(streams),
            in_img, in_key_path, out_file
        ),
        None => bail!("Key file is mandatory for now."),
    }
}
