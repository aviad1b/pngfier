use std::{io, marker::PhantomData};

use crate::{elems::{Elem, ElemIndexesMatrix}, streams::traits::InputElemStream};

use super::{reach_utils::{self, Path}, super::{ChunkIndex, ChunkSize}};

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

/// `ReachMapper` implementation that is based on a matrix of index sets, mapped by elements.
/// A slot i,j in the matrix contains all indexes in the input image where element i comes after element j.
/// 
/// * `E` - Element type.
/// * `ImageStream` - Type of stream used to read input image. Must implement `InputElemStream<E>`.
/// * `DataStream` - Type of stream used to read input data. Must implement `InputElemStream<E>`.
/// * `M` - An implementation of `ElemIndexesMatrix`, used for storing indexes in a matrix as stated above.
/// 
pub struct MatrixBasedReachMapper<'a, 'b, 'c, E, ImageStream, DataStream, M>
where
    E: Elem,
    ImageStream: InputElemStream<E>,
    DataStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    reach: Vec<MatchInfo>,

    image: &'a mut ImageStream,
    data: &'b mut DataStream,

    // `img_matrix[prev,curr]` is all indexes in image wheren `curr` comes after `prev`
    img_matrix: &'c mut M,
    
    phantom: PhantomData<E>,
}

impl<'a, 'b, 'c, E, ImageStream, DataStream, M>
MatrixBasedReachMapper<'a, 'b, 'c, E, ImageStream, DataStream, M>
where
    E: Elem,
    ImageStream: InputElemStream<E>,
    DataStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    /// Constructs a new instance.
    /// 
    /// * `image` - Stream used to read input image.
    /// * `data` - Stream used to read input data.
    /// * `img_matrix` - Matrix instance to use for mapping.
    /// 
    /// Returns constructed instance, or error if occurred.
    /// 
    pub fn new(image: &'a mut ImageStream,
               data: &'b mut DataStream,
               img_matrix: &'c mut M) -> io::Result<Self> {
        let _ = (image, data, img_matrix);
        todo!() // TODO: Implement
    }

    /// Initializes internal matrix such that every slot i,j contains all indexes 
    /// in input image where element i comes before element j.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_img_matrix(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    /// Initializes reach vector based on internal matrix.
    /// See documentation of `MatchInfo` for more information.
    /// Assumes internal matrix has already been initialized.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_reach(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    /// Initializes reach for a specific `data_start`` index.
    /// See documentation of `init_reach` for more information.
    /// 
    /// * `data_start` - Image index to initialize reach for.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_reach_for(&mut self, data_start: ChunkIndex) -> io::Result<()> {
        let _ = data_start;
        todo!() // TODO: Implement
    }

    /// For a given `data_start` index, gets vector of all possible starts to paths mutual for data and image.
    /// A "possible start" is an index where the same two elements appear in a row in both data and image.
    /// 
    /// * `data_start` - Index in data to find possible path starts for.
    /// 
    /// Returns vector of `Path`s, each of length two.
    /// Returns error if occurred.
    /// 
    fn get_path_starts_vec(&mut self, data_start: ChunkIndex) -> io::Result<Vec<Path>> {
        let _ = data_start;
        todo!() // TODO: Implement
    }

    /// Given a `data_start` index and a mutual paths vector reference,
    /// walks through full paths while finding them and finds the longest one.
    /// 
    /// * `data_start` - A starting index for paths in data.
    /// * `paths` - A vector of paths starts, for paths to walk through.
    /// 
    /// Returns the longest path (or `None` if no paths were provided/found).
    /// Returns error if occurred.
    /// 
    /// Note: Takes ownership over `paths`.
    /// 
    fn walk_paths(&mut self, data_start: ChunkIndex, paths: Vec<Path>) -> io::Result<Option<Path>> {
        let _ = (data_start, paths);
        todo!() // TODO: Implement
    }
}

impl<'a, 'b, 'c, E, ImageStream, DataStream, M>
ReachMapper<E> for MatrixBasedReachMapper<'a, 'b, 'c, E, ImageStream, DataStream, M>
where
    E: Elem,
    ImageStream: InputElemStream<E>,
    DataStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    fn len(&self) -> ChunkIndex {
        todo!() // TODO: Implement
    }

    fn get(&self, index: ChunkIndex) -> MatchInfo {
        let _ = index;
        todo!() // TODO: Implement
    }

    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> io::Result<Vec<E>> {
        let _ = (start, count);
        todo!() // TODO: Implement
    }
}
