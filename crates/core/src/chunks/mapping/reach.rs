use anyhow::{Context, Result};

use generic_array::typenum::U2;
use std::marker::PhantomData;

use crate::{
    elems::{Elem, ElemIndexesMatrix}, streams::{StreamPos, grouping::UngroupedElemStream, traits::InputElemStreams},
};

use super::{reach_utils::{self, Path}, super::{ChunkIndex, ChunkSize}};

/// For a match starting at some position `data_start` in the data stream:
/// `image[src_start + t] == data[data_start + t]` for all `t` in `0..(reach - data_start)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Returns indexes count, or error if occurred.
    /// 
    /// NOTE: Returns length as `ChunkIndex` for convenience of comparison.
    /// 
    fn len(&self) -> Result<ChunkIndex>;

    /// Gets best match found starting exactly at data-position `index`.
    /// 
    /// * `index` - Data index to get mapping of.
    /// 
    /// Returns best match for `index`, as stated above.
    /// If nothing matches there, returns MatchInfo { reach: index, src_start: index } (no progress).
    /// Returns error if occurred.
    /// 
    fn get(&self, index: ChunkIndex) -> Result<MatchInfo>;

    /// Gets literal elements from input data.
    /// 
    /// * `start` - Data index to start at.
    /// * `count` - Amount of elements to get.
    /// 
    /// Returns vector of elements, or error if occurred.
    /// 
    /// NOTE: This method it `mut` as it may need to read from an internal file (and move its cursor).
    /// 
    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> Result<Vec<E>>;
}


/// `ReachMapper` implementation that is based on a matrix of index sets, mapped by elements.
/// A slot i,j in the matrix contains all indexes in the input image where element i comes after element j.
/// 
/// * `IDX_IMG` - Input image stream index.
/// * `IDX_DAT` - Input data stream index.
/// * `E` - Element type.
/// * `Streams` - Type of streams used to read input image and data. Must implement `InputElemStreams<E, U2>`.
/// * `M` - An implementation of `ElemIndexesMatrix`, used for storing indexes in a matrix as stated above.
/// 
pub struct MatrixBasedReachMapper<'a, 'b, const IDX_IMG: usize, const IDX_DAT: usize, E, Streams, M>
where
    E: Elem,
    Streams: InputElemStreams<E, U2>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    reach: Vec<MatchInfo>,

    streams: &'a mut Streams,

    // `img_matrix[prev,curr]` is all indexes in image wheren `curr` comes after `prev`
    img_matrix: &'b mut M,
    
    phantom: PhantomData<E>,
}

impl<'a, 'b, const IDX_IMG: usize, const IDX_DAT: usize, E, Streams, M>
MatrixBasedReachMapper<'a, 'b, IDX_IMG, IDX_DAT, E, Streams, M>
where
    E: Elem,
    Streams: InputElemStreams<E, U2>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    /// Constructs a new instance.
    /// 
    /// * `streams` - Streams used to read input image and data.
    /// * `img_matrix` - Matrix instance to use for mapping.
    /// 
    /// Returns constructed instance, or error if occurred.
    /// 
    pub fn new(streams: &'a mut Streams,
               img_matrix: &'b mut M) -> Result<Self> {
        let mut res = Self{
            reach: Vec::new(),
            streams,
            img_matrix,
            phantom: PhantomData,
        };

        res.init_img_matrix()?;
        res.init_reach()?;

        Ok(res)
    }

    /// Initializes internal matrix such that every slot i,j contains all indexes 
    /// in input image where element i comes before element j.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_img_matrix(&mut self) -> Result<()> {
        let mut image = UngroupedElemStream::<'_, IDX_IMG, _, _, _>::new(self.streams);
        reach_utils::init_img_matrix(&mut image, self.img_matrix)
            .context("Failed to initialize image matrix")
    }

    /// Initializes reach vector based on internal matrix.
    /// See documentation of `MatchInfo` for more information.
    /// Assumes internal matrix has already been initialized.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_reach(&mut self) -> Result<()> {
        self.streams.rewind::<IDX_DAT>()?;
        self.streams.rewind::<IDX_IMG>()?;

        let data_size = self.streams.get_size::<IDX_DAT>()?;
        self.reach.resize(data_size as usize, MatchInfo { reach: 0, src_start: -1 });
        for data_start in 0..data_size {
            self.init_reach_for(data_start)?;
        }

        Ok(())
    }

    /// Initializes reach for a specific `data_start`` index.
    /// See documentation of `init_reach` for more information.
    /// 
    /// * `data_start` - Image index to initialize reach for.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_reach_for(&mut self, data_start: ChunkIndex) -> Result<()> {
        let paths = self.get_path_starts_vec(data_start)?;

        // for each path start, walk through entire path for as long as exists in both data and image
        let longest_path = self.walk_paths(data_start, paths)?;

        self.reach[data_start as usize] = match longest_path {
            // reach is based on the longest path found
            Some(longest_path) => MatchInfo {
                reach: data_start + longest_path.len,
                src_start: longest_path.src_start,
            },

            // no path was found
            None => MatchInfo {
                reach: data_start,
                src_start: -1, // unread value (negative - always lower than other `src_start`s)
            }
        };

        Ok(())
    }

    /// For a given `data_start` index, gets vector of all possible starts to paths mutual for data and image.
    /// A "possible start" is an index where the same two elements appear in a row in both data and image.
    /// 
    /// * `data_start` - Index in data to find possible path starts for.
    /// 
    /// Returns vector of `Path`s, each of length two.
    /// Returns error if occurred.
    /// 
    fn get_path_starts_vec(&mut self, data_start: ChunkIndex) -> Result<Vec<Path>> {
        let mut data = UngroupedElemStream::<'_, IDX_DAT, _, _, _>::new(self.streams);
        reach_utils::get_path_starts_vec(&mut data, self.img_matrix, data_start)
            .context("Failed to get vector of path starts")
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
    /// NOTE: Takes ownership over `paths`.
    /// 
    fn walk_paths(&mut self, data_start: ChunkIndex, paths: Vec<Path>) -> Result<Option<Path>> {
        let mut data = UngroupedElemStream::<'_, IDX_DAT, _, _, _>::new(self.streams);
        reach_utils::walk_paths(&mut data, self.img_matrix, data_start, paths)
            .context("Failed to walk paths")
    }
}

impl<'a, 'b, const IDX_IMG: usize, const IDX_DAT: usize, E, Streams, M>
ReachMapper<E> for MatrixBasedReachMapper<'a, 'b, IDX_IMG, IDX_DAT, E, Streams, M>
where
    E: Elem,
    Streams: InputElemStreams<E, U2>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    fn len(&self) -> Result<ChunkIndex> {
        Ok(self.reach.len() as ChunkIndex)
    }

    fn get(&self, index: ChunkIndex) -> Result<MatchInfo> {
        Ok(self.reach[index as usize])
    }

    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> Result<Vec<E>> {
        self.streams.set_pos::<IDX_DAT>(start as StreamPos)?;
        (0..count).map(|_| {
            self.streams.read_next_elem::<IDX_DAT>()?
                .context("ReachMapper::get_elems: ran out of data")
        }).collect()
    }
}
