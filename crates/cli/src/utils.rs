use std::{io, sync::atomic::{AtomicU64, Ordering}};

/// Generates a unique path for the temporary directory.
/// Directory gets deleted automatically at end of scope.
/// 
pub struct TempDir {
    path: std::path::PathBuf,
}

impl TempDir {
    /// Constructs a new `TempDir` instance.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new() -> io::Result<Self> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = std::env::temp_dir();
        path.push(format!("pngfier_{}_{}", std::process::id(), id));
        std::fs::create_dir(&path)?;
        Ok(Self { path })
    }

    /// Gets directory's path as a string slice.
    /// 
    pub fn path_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // delete temp directory on scope exit
        let _ = std::fs::remove_dir(&self.path);
    }
}
