use anyhow::{Result, bail};
use pngfier_core::{chunks::storage::ChunkInfoWidths, elems::Elem};

use crate::{callbacks, commands::ImgSrc};

pub mod streams;
pub mod compile;
pub mod extract;

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
    }
}

pub fn apply_extract<E: Elem>(in_img: &String, out_file: &String, key_file: &Option<String>) -> Result<()> {
    match key_file {
        Some(in_key_path) => extract::with_key(
            |streams| callbacks::extract::<E, _, _>(streams),
            in_img, in_key_path, out_file
        ),
        None => bail!("Key file is mandatory for now."),
    }
}
