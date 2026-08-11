use anyhow::{Result, bail};

/// Represents image source (search query / direct path).
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
