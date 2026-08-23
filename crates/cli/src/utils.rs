use std::sync::atomic::{AtomicU64, Ordering};

/// Output file with automatically generated unique path.
pub struct OutputFile {
    path: std::path::PathBuf,
}

impl OutputFile {
    /// Constructs a new `OutputFile` instance.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "pngfier_output_{}_{}",
            std::process::id(),
            id
        ));
        Self { path }
    }

    /// Gets file's path as a string slice.
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}
