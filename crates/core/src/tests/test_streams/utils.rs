use std::sync::atomic::{AtomicU64, Ordering};

/// Used for I/O tests.
/// Generates a unique path for the temporary file.
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
            "pngfier_test_{}_{}_{}",
            std::process::id(),
            id,
            name
        ));
        Self { path }
    }

    /// Gets file's path as a string slice.
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }

    /// Writes initial data into temp file for test to read.
    pub fn write_initial(&self, data: &[u8]) {
        std::fs::write(&self.path, data).unwrap();
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // delete temp file on scope exit
        let _ = std::fs::remove_file(&self.path);
    }
}
