use super::super::{ChunkIndex, ChunkSize};

// To temporarily store paths of chunks that exist in both image and data
// `src_start` is starting index at image.
#[derive(Clone, Copy)]
pub struct Path {
    len: ChunkSize,
    src_start: ChunkIndex,
}
