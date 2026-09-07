use anyhow::Result;
use clap::Parser;

use pngfier_core::chunks::storage::ChunkInfoWidths;

use crate::commands::{Command, ImgSrc};

mod callbacks;
mod commands;
mod streams;
mod configs;

/// Element type used in I/O operation.
type E = u8;

/// Widths for chunks I/O.
/// Reference chunk is saved as: false(1), size(15), index(16)
/// Literal chunk is saved as: true(1), size(15), elems...
const WIDTHS: ChunkInfoWidths = ChunkInfoWidths {
    is_literal: 1,
    size: 15,
    index: 32,
};

/// Main CLI parser.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { out_img, in_file, img_query, img_path, key_file } =>
            handle_compile(out_img, in_file, ImgSrc::from_args(img_query, img_path)?, key_file),
        Command::Extract { in_img, out_file, key_file } =>
            handle_extract(in_img, out_file, key_file),
    }
}

/// Handles 'compile' command.
/// * `out_img` - Path to store output image to.
/// * `in_file` - Input file to compile into a PNG.
/// * `img_src` - Image source path (of image to override).
/// * `key_file` - Optional path to store key to (instead of using PNG riding).
/// Returns error if occured.
fn handle_compile(out_img: String, in_file: String, img_src: ImgSrc, key_file: Option<String>) -> Result<()> {
    configs::apply_compile::<E>(&WIDTHS, &out_img, &in_file, &img_src, &key_file)?;

    println!("Output saved at {}", &out_img);

    Ok(())
}

/// Handles 'extract' command.
/// * `in_img` - Path to compiled image to extract data from.
/// * `out_file` - File path to store extracted data to.
/// * `key_file` - Optional path to read key from (instead of assuming PNG riding).
/// Returns error if occured.
fn handle_extract(in_img: String, out_file: String, key_file: Option<String>) -> Result<()> {
    configs::apply_extract::<E>(&in_img, &out_file, &key_file)?;

    println!("Output saved at {}", &out_file);

    Ok(())
}
