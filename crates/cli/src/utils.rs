use std::sync::atomic::{AtomicU64, Ordering};

/// Temporary file with automatically generated unique path.
/// File gets deleted automatically at end of scope.
pub struct TempFile {
    path: std::path::PathBuf,
}

impl TempFile {
    /// Constructs a new `TempFile` instance.
    /// 
    /// * `name` - Base name to base file path off.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(name: &str) -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "pngfier_{}_{}_{}",
            std::process::id(),
            name,
            id
        ));
        Self { path }
    }

    /// Gets file's path as a string slice.
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // delete temp file on scope exit
        let _ = std::fs::remove_file(&self.path);
    }
}
