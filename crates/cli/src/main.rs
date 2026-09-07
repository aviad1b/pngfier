use anyhow::{Context, Result, bail};
use clap::Parser;

use generic_array::GenericArray;
use pngfier_core::{
    chunks::{
        mapping::{ChunkMapper, reach::MatrixBasedReachMapper},
        storage::{ChunkInfoWidths, ChunksReader, ChunksWriter},
    },
    elems::RuntimeElemIndexesMatrix,
    streams::{
        files::{InputBinaryFileStream, OutputBinaryFileStream},
        grouping::GroupedBinaryStreams,
        spans::BinaryElemSpan,
        traits::{InputElemStream, OutputBinaryStream},
    },
};

use crate::{commands::{Command, ImgSrc}, configs::CompileStreams};

mod commands;
mod configs;

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
    let in_img_path = match img_src {
        ImgSrc::Query(_) => bail!("Query-based compiling is not supported yet."),
        ImgSrc::Path(path) => path,
    };

    let out_key_path = match key_file {
        Some(x) => x,
        None => bail!("Key file is mandatory for now."),
    };

    configs::compile_path_with_key(
        |streams| compile_callback(streams),
        &in_img_path, &in_file, &out_img, &out_key_path
    )?;

    println!("Output saved at {}", &out_img);

    Ok(())
}

/// Callback function which performs compile operation.
/// 
/// * `streams` - Input & output streams to perform compile operation on.
/// 
/// Returns error if occurred.
/// 
fn compile_callback<In, Out>(streams: &mut [CompileStreams<u8, In, Out>]) -> Result<()>
where
    In: InputElemStream<u8>,
    Out: OutputBinaryStream,
{
    // cap minimum reference chunk size by size of fields sum (reference chunk size)
    // cap maximum reference chunk size by maximum representable chunk size
    let min_cap = WIDTHS.total_size_bytes();
    let max_cap = WIDTHS.max_size();

    for streams in streams {
        const IMG_IDX: usize = 0;
        const KEY_IDX: usize = 1;
        let mut output = GroupedBinaryStreams::new(
            GenericArray::from_array([&mut streams.out_img, &mut streams.out_key])
        );

        let mut img_matrix = RuntimeElemIndexesMatrix::new();
        let mut reach = MatrixBasedReachMapper::new(&mut streams.in_img, &mut streams.in_data, &mut img_matrix)
            .context("Failed to construct reach mapper")?;

        let chunks = ChunkMapper::new(&mut reach)
            .map_chunks(Some(min_cap), Some(max_cap))
            .context("Failed to map chunks")?;
        let chunks = &mut chunks.iter();
        let mut writer = ChunksWriter::<'_, '_, '_, IMG_IDX, KEY_IDX, _, _, _>::new(
            WIDTHS, chunks, &mut output
        );

        writer.write().context("Failed to write chunks into output")?;
    }

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
        &mut input, &mut out_chunks
    );

    reader.extract_all().context("Failed to extract chunks")?;

    Ok(())
}
