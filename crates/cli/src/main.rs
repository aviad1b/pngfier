use anyhow::Result;
use clap::Parser;

use crate::commands::{Command, ImgSrc};

mod commands;

/// Main CLI parser.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { input, img_query, img_path, key_file } =>
            handle_compile(input, ImgSrc::from_args(img_query, img_path)?, key_file),
        Command::Extract { img_path, output, key_file } =>
            handle_extract(img_path, output, key_file),
    }
}

/// Handles 'compile' command.
/// * `input` - Input file to compile into a PNG.
/// * `img_src` - Image source path (of image to override).
/// * `key_file` - Optional path to store key to (instead of using PNG riding).
/// Returns error if occured.
fn handle_compile(input: String, img_src: ImgSrc, key_file: Option<String>) -> Result<()> {
    todo!()
}

/// Handles 'extract' command.
/// * `img_path` - Path to compiled image to extract data from.
/// * `output` - File path to store extracted data to.
/// * `key_file` - Optional path to read key from (instead of assuming PNG riding).
/// Returns error if occured.
fn handle_extract(img_path: String, output: String, key_file: Option<String>) -> Result<()> {
    todo!()
}
