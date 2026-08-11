use anyhow::{Result, bail};
use clap::Subcommand;

// pngfier compile <input> <img-query>|<img-path> [<key-file>]
// pngfier extract <image> <output> [<key-file>]

#[derive(Subcommand)]
pub enum Command {
    Compile {
        /// Input file to compile into a PNG.
        input: String,

        /// Query of image to generate.
        #[arg(short = 'q', long)]
        img_query: Option<String>,

        /// Path to image file to base off of.
        #[arg(short = 'p', long)]
        img_path: Option<String>,

        /// Optional path to store key to (instead of using PNG riding).
        #[arg(long)]
        key_file: Option<String>
    },
    Extract {
        /// Path to compiled image to extract data from.
        img_path: String,

        /// File path to store extracted data to.
        output: String,

        /// Optional path to read key from (instead of assuming PNG riding).
        #[arg(long)]
        key_file: Option<String>,
    },
}

/// Represents image source (search query / direct path).
#[derive(Debug)]
pub enum ImgSrc {
    Query(String),
    Path(String),
}

impl ImgSrc {
    /// Gets image source based on input arguments.
    /// * `query` - 'img-query' argument if was provided, or `None` if was not.
    /// * `path` - 'img-path' argument if was provided, or `None` if was not.
    /// Returns an `ImgSrc` instance, or an error if neither or both arguments were provided.
    pub fn from_args(query: Option<String>, path: Option<String>) -> Result<Self> {
        match (query, path) {
            (None, None) => bail!(
                "Image source not provided: One of the arguments --img-query or --img-path must be provided."
            ),
            (None, Some(path)) => Ok(ImgSrc::Path(path)),
            (Some(query), None) => Ok(ImgSrc::Query(query)),
            (Some(_), Some(_)) => bail!(
                "Image source is ambiguous: Only one of the arguments --img-query or --img-path must be provided."
            ),
        }
    }
}
