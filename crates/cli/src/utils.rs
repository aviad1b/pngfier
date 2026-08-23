use std::{io, sync::atomic::{AtomicU64, Ordering}};

/// Output file with automatically generated unique path.
pub struct OutputFile {
    path: std::path::PathBuf,
}

impl OutputFile {
    /// Constructs a new `OutputFile` instance.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new() -> io::Result<Self> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::current_dir()?;
        path.push(format!(
            "pngfier_output_{}_{}",
            std::process::id(),
            id
        ));
        Ok(Self { path })
    }

    /// Gets file's path as a string slice.
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}
