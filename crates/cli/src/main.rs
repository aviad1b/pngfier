use anyhow::{Context, Result, bail};
use clap::Parser;

use generic_array::GenericArray;
use pngfier_core::{
    chunks::{
        mapping::{ChunkMapper, reach::MatrixBasedReachMapper},
        storage::{ChunkInfoWidths, ChunksReader, ChunksWriter},
    },
    elems::RuntimeElemIndexesMatrix, streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        grouping::GroupedBinaryStreams, spans::BinaryElemSpan,
    },
};

use crate::commands::{Command, ImgSrc};

mod commands;

/// Main CLI parser.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Widths for chunks I/O.
/// Reference chunk is saved as: false(1), size(15), index(16)
/// Literal chunk is saved as: true(1), size(15), elems...
const WIDTHS: ChunkInfoWidths = ChunkInfoWidths {
    is_literal: 1,
    size: 15,
    index: 32,
};

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
    let input_image_path = match img_src {
        ImgSrc::Query(_) => bail!("Query-based compiling is not supported yet."),
        ImgSrc::Path(path) => path,
    };

    let mut out_img_stream = OutputBinaryFileStream::new(&out_img)
        .context("Failed to write to output file")?;
    let mut out_key_stream = match key_file {
        None => bail!("Key file is mandatory for now."),
        Some(key_file_path) => OutputBinaryFileStream::new(&key_file_path)
            .context("Failed to write to key file")?
    };

    const IMG_IDX: usize = 0;
    const KEY_IDX: usize = 1;
    let mut output = GroupedBinaryStreams::new(
        GenericArray::from_array([&mut out_img_stream, &mut out_key_stream])
    );

    let mut image = InputBinaryFileStream::new(&input_image_path)
        .context("Failed to write to output")?;
    let mut image = BinaryElemSpan::<'_, u8, _>::new(&mut image, None, None);

    let mut data = InputBinaryFileStream::new(&in_file)
        .context("Failed to read from input data")?;
    let mut data = BinaryElemSpan::new(&mut data, None, None);

    let mut img_matrix = RuntimeElemIndexesMatrix::new();
    let mut reach = MatrixBasedReachMapper::new(&mut image, &mut data, &mut img_matrix)
        .context("Failed to construct reach mapper")?;

    // cap minimum reference chunk size by size of fields sum (reference chunk size)
    // cap maximum reference chunk size by maximum representable chunk size
    let chunks = ChunkMapper::new(&mut reach)
        .map_chunks(Some(WIDTHS.total_size_bytes()), Some(WIDTHS.max_size()))
        .context("Failed to map chunks")?;
    let chunks = &mut chunks.iter();
    let mut writer = ChunksWriter::<'_, '_, '_, IMG_IDX, KEY_IDX, _, _, _>::new(
        WIDTHS, chunks, &mut output
    );

    writer.write().context("Failed to write chunks into output")?;

    println!("Output saved at {}", &out_img);

    Ok(())
}

/// Handles 'extract' command.
/// * `in_img` - Path to compiled image to extract data from.
/// * `out_file` - File path to store extracted data to.
/// * `key_file` - Optional path to read key from (instead of assuming PNG riding).
/// Returns error if occured.
fn handle_extract(in_img: String, out_file: String, key_file: Option<String>) -> Result<()> {
    let mut in_img_stream = InputBinaryFileStream::new(&in_img)
        .context("Failed to read from output file")?;
    let mut in_key_stream = match key_file {
        None => bail!("Key file is mandatory for now."),
        Some(key_file_path) => InputBinaryFileStream::new(&key_file_path)
            .context("Failed to read from key file")?
    };

    const IMG_IDX: usize = 0;
    const KEY_IDX: usize = 1;
    let mut input = GroupedBinaryStreams::new(
        GenericArray::from_array([&mut in_img_stream, &mut in_key_stream])
    );

    let mut out_chunks = OutputBinaryFileStream::new(&out_file)
        .context("Failed to write to output file")?;
    let mut out_chunks = BinaryElemSpan::<'_, u8, _>::new(&mut out_chunks, None, None);

    let mut reader = ChunksReader::<'_, '_, IMG_IDX, KEY_IDX, _, _, _>::new(
        WIDTHS, &mut input, &mut out_chunks
    );

    reader.extract_all().context("Failed to extract chunks")?;

    Ok(())
}
