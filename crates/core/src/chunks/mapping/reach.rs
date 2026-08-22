use std::{io, marker::PhantomData};

use crate::{
    elems::{Elem, ElemIndexesMatrix},
    streams::{StreamPos, traits::InputElemStream},
};

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
    /// Returns indexes count, or error if occurred.
    /// 
    /// NOTE: Returns length as `ChunkIndex` for convenience of comparison.
    /// 
    fn len(&self) -> io::Result<ChunkIndex>;

    /// Gets best match found starting exactly at data-position `index`.
    /// 
    /// * `index` - Data index to get mapping of.
    /// 
    /// Returns best match for `index`, as stated above.
    /// If nothing matches there, returns MatchInfo { reach: index, src_start: index } (no progress).
    /// Returns error if occurred.
    /// 
    fn get(&self, index: ChunkIndex) -> io::Result<MatchInfo>;

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
        let mut res = Self{
            reach: Vec::new(),
            image,
            data,
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
    fn init_img_matrix(&mut self) -> io::Result<()> {
        reach_utils::init_img_matrix(self.image, self.img_matrix)
    }

    /// Initializes reach vector based on internal matrix.
    /// See documentation of `MatchInfo` for more information.
    /// Assumes internal matrix has already been initialized.
    /// 
    /// Returns error if occurred.
    /// 
    fn init_reach(&mut self) -> io::Result<()> {
        self.data.rewind()?;
        self.image.rewind()?;

        let data_size = self.data.get_size()?;
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
    fn init_reach_for(&mut self, data_start: ChunkIndex) -> io::Result<()> {
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
    fn get_path_starts_vec(&mut self, data_start: ChunkIndex) -> io::Result<Vec<Path>> {
        reach_utils::get_path_starts_vec(self.data, self.img_matrix, data_start)
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
    fn walk_paths(&mut self, data_start: ChunkIndex, paths: Vec<Path>) -> io::Result<Option<Path>> {
        reach_utils::walk_paths(self.data, self.img_matrix, data_start, paths)
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
    fn len(&self) -> io::Result<ChunkIndex> {
        Ok(self.reach.len() as ChunkIndex)
    }

    fn get(&self, index: ChunkIndex) -> io::Result<MatchInfo> {
        Ok(self.reach[index as usize])
    }

    fn get_elems(&mut self, start: ChunkIndex, count: ChunkSize) -> io::Result<Vec<E>> {
        self.data.set_pos(start as StreamPos)?;
        (0..count).map(|_| {
            self.data.read_next_elem()?.ok_or_else(|| {
                io::Error::new(io::ErrorKind::UnexpectedEof, "ReachMapper::get_elems: ran out of data")
            })
        }).collect()
    }
}
