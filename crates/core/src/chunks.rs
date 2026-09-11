use crate::elems::Elem;

/// Represents an index used by a chunk (to reference an element).
pub type ChunkIndex = i64;

/// Represents a chunk's (elemental) size.
pub type ChunkSize = i64;

/// Holds information about a chunk within a key.
/// 
/// * `E` - Element type.
/// 
#[derive(Debug, PartialEq, Eq)]
pub enum ChunkInfo<E: Elem> {
    /// A chunk that references data from the image.
    Reference { index: ChunkIndex, size: ChunkSize },

    /// A chunk that is provided as a direct sequence of literals.
    Literal(Vec<E>),
}

pub mod mapping;

pub mod storage;
