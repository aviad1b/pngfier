use std::io;

use crate::elems::Elem;

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

/// Abstraction of reach mapping.
/// A mapping between an index in the input data and the furthest index we 
/// can reach in the data such that the chunk formed between said two indexes 
/// exists in the input image.
/// 
/// * `E` - Element type.
/// 
pub trait ReachMapper<E: Elem> {
    /// Gets amount of indexes mapped.
    /// 
    /// NOTE: Returns as `ChunkIndex` for convenience of comparison.
    /// 
    fn len(&self) -> ChunkIndex;

    /// Gets best match found starting exactly at data-position `index`.
    /// 
    /// * `index` - Data index to get mapping of.
    /// 
    /// Returns best match for `index`, as stated above.
    /// If nothing matches there, returns MatchInfo { reach: index, src_start: index } (no progress).
    fn get(&self, index: ChunkIndex) -> MatchInfo;

    /// Gets literal elements from input data.
    /// 
    /// * `start` - Data index to start at.
    /// * `count` - Amount of elements to get.
    /// 
    /// Returns vector of elements, or error if occurred.
    /// 
    /// NOTE: This method it `mut` as it may need to read from an internal file (and move its cursor).
    /// 
    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> io::Result<Vec<E>>;
}
