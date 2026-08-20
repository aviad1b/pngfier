use super::super::{ChunkIndex, ChunkSize};

// To temporarily store paths of chunks that exist in both image and data
// `src_start` is starting index at image.
#[derive(Clone, Copy)]
struct Path {
    len: ChunkSize,
    src_start: ChunkIndex,
}

/// For a match starting at some position `data_start` in the data stream:
/// `image[src_start + t] == data[data_start + t]` for all `t` in `0..(reach - data_start)`.
#[derive(Clone, Copy)]
pub struct MatchInfo {
    /// Furthest position in `data` this match covers
    pub reach: ChunkIndex,

    /// Where in `image` this match starts
    pub src_start: ChunkIndex,
}
