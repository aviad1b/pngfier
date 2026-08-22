use std::io;

use crate::{
    elems::{Elem, ElemIndexesMatrix, ElemIndexesMatrixSlot, ElemIndexesMatrixSlotMut},
    streams::{StreamPos, traits::InputElemStream},
};

use super::super::{ChunkIndex, ChunkSize};

// To temporarily store paths of chunks that exist in both image and data
// `src_start` is starting index at image.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Path {
    pub len: ChunkSize,
    pub src_start: ChunkIndex,
}

/// Initializes indexes matrix such that every slot i,j contains all indexes 
/// in input image where element i comes before element j.
/// 
/// * `E` - Element type.
/// * `ImageStream` - Type of stream used to read input image. Must implement `InputElemStream<E>`.
/// * `M` - An implementation of `ElemIndexesMatrix`, used for storing indexes in a matrix as stated above.
/// 
/// * `image` - Stream used to read input image.
/// * `img_matrix` - Matrix instance to use for mapping.
/// 
/// Returns error if occurred.
/// 
pub fn init_img_matrix<E, ImageStream, M>(image: &mut ImageStream,
                                          img_matrix: &mut M) -> io::Result<()>
where
    E: Elem,
    ImageStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    let mut i: ChunkIndex = 0;

	image.rewind()?;

	// read `prev` from first image elem if exists
    let mut prev = match image.read_next_elem()? {
        Some(first) => first,
        None => return Ok(()),
    };
	
	// read `curr` from second image elem, loop while has curr
	let mut curr_opt = image.read_next_elem()?;
	while let Some(curr) = curr_opt {
		// `curr` comes after `prev` in index `i`
		img_matrix.at_mut(prev, curr)?.insert(i)?;
		
		prev = curr;
		curr_opt = image.read_next_elem()?;
		i += 1;
	}

	Ok(())
}

/// For a given `data_start` index, gets vector of all possible starts to paths mutual for data and image.
/// A "possible start" is an index where the same two elements appear in a row in both data and image.
/// 
/// * `E` - Element type.
/// * `DataStream` - Type of stream used to read input data. Must implement `InputElemStream<E>`.
/// * `M` - An implementation of `ElemIndexesMatrix`, used for storing indexes in a matrix as stated above.
/// 
/// * `data` - Stream used to read input data.
/// * `img_matrix` - Matrix instance mapping indexes where one element value comes after another.
/// * `data_start` - Index in data to find possible path starts for.
/// 
/// Returns vector of `Path`s, each of length two.
/// Returns error if occurred.
/// 
pub fn get_path_starts_vec<E, DataStream, M>(data: &mut DataStream,
                                             img_matrix: &M,
                                             data_start: ChunkIndex) -> io::Result<Vec<Path>>
where
    E: Elem,
    DataStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    // start at `data_start` in data
	data.set_pos(data_start as StreamPos)?;

	// read `prev` from first data elem if exists
    let prev = match data.read_next_elem()? {
        Some(first) => first,
        None => return Ok(vec![]),
    };
	
	// read `curr` from second data elem if exists
    let curr = match data.read_next_elem()? {
        Some(second) => second,
        None => return Ok(vec![]),
    };

	// each place where `curr` comes after `prev` starts a possible path
	Ok(img_matrix.at(prev, curr)?
		.iter()?
		.map(|&index| Path {
			len: 2, // at least 2 elems in path, `first` and `second`
			src_start: index,
		})
		.collect()
	)
}

/// Given a `data_start` index and a mutual paths vector reference,
/// walks through full paths while finding them and finds the longest one.
/// 
/// * `E` - Element type.
/// * `ImageStream` - Type of stream used to read input image. Must implement `InputElemStream<E>`.
/// * `DataStream` - Type of stream used to read input data. Must implement `InputElemStream<E>`.
/// * `M` - An implementation of `ElemIndexesMatrix`, used for storing indexes in a matrix as stated above.
/// 
/// * `data_start` - A starting index for paths in data.
/// * `paths` - A vector of paths starts, for paths to walk through.
/// 
/// Returns the longest path (or `None` if no paths were provided/found).
/// Returns error if occurred.
/// 
/// Note: Takes ownership over `paths`.
/// 
pub fn walk_paths<E, DataStream, M>(data: &mut DataStream,
                                    img_matrix: &M,
                                    data_start: ChunkIndex,
                                    paths: Vec<Path>) -> io::Result<Option<Path>>
where
    E: Elem,
    DataStream: InputElemStream<E>,
    M: ElemIndexesMatrix<E, ChunkIndex>,
{
    let _ = (data, img_matrix, data_start, paths);
    todo!() // TODO: Implement
}
