use std::{io, marker::PhantomData};

use crate::elems::Elem;

use super::{
    ChunkIndex, ChunkInfo, ChunkSize,
    mapping::reach::{MatchInfo, ReachMapper},
};

pub mod reach;
pub mod reach_utils;

/// Maps chunks from input data to chunks in input image.
/// For more information, see documentation of `ChunkInfo` variants.
/// 
/// * `E` - Element type (each chunk is made of elements).
/// * `Reach` - An implementation of `ReachMapper<E>`, used as utility.
/// 
pub struct ChunkMapper<'a, E: Elem, Reach: ReachMapper<E>>
{
    reach: &'a mut Reach,
    phantom: PhantomData<E>,
}

impl<'a, E: Elem, Reach: ReachMapper<E>> ChunkMapper<'a, E, Reach> {
    /// Constructs a new instance.
    /// 
    /// * `reach` - Borrowed `Reach` instance to use as utility.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(reach: &'a mut Reach) -> Self {
        let _ = reach;
        todo!() // TODO: Implement
    }

    /// Computes mapping from data chunks to image chunks.
    /// 
    /// * `min_cap` - Optional lower bound for length of a reference chunk (non-inclusive).
    /// * `max_cap` - Optional upper bound for length of a reference chunk (non-inclusive).
    /// 
    /// If a reference chunk exceeds a bound, it is discarded from the computation.
    /// 
    /// Reeturns vector of mapped chunks, or error if occurred.
    /// 
    pub fn map_chunks(&mut self,
                      min_cap: Option<ChunkSize>,
                      max_cap: Option<ChunkSize>) -> io::Result<Vec<ChunkInfo<E>>> {
        let _ = (min_cap, max_cap);
        todo!() // TODO: Implement
    }

    /// Utility of `map_chunks`, maps a reference chunk (if does not exceed bounds).
    /// 
    /// * `min_cap` - Optional lower bound for length of a reference chunk (non-inclusive).
    /// * `max_cap` - Optional upper bound for length of a reference chunk (non-inclusive).
    /// * `chunks` - Borrowed vector of mapped chunks.
    /// * `cur_end` - End index of currently computed chunk in input data.
    /// * `farthest` - Farthest reach match found so far.
    /// * `farthest_data_start` - Start index in input data of darthest reach match found so far.
    /// 
    /// Appends reference chunk into `chunks` (if relevant).
    /// 
    /// Returns error if occurred.
    /// 
    fn map_reference_chunk(&mut self, min_cap: Option<ChunkSize>, max_cap: Option<ChunkSize>, 
                           chunks: &mut Vec<ChunkInfo<E>>, cur_end: &mut ChunkIndex, 
                           farthest: &mut MatchInfo, farthest_data_start: ChunkIndex) -> io::Result<()> {
        let _ = (min_cap, max_cap, chunks, farthest, cur_end, farthest_data_start);
        todo!() // TODO: Implement
    }

    /// Utility of `map_chunks`, maps a reference chunk (if does not exceed bounds).
    /// 
    /// * `chunks` - Borrowed vector of mapped chunks.
    /// * `cur_end` - End index of currently computed chunk in input data.
    /// * `farthest` - Farthest reach match found so far.
    /// 
    /// Appends literal chunk into `chunks`.
    /// 
    /// Returns error if occurred.
    /// 
    fn map_literal_chunk(&mut self, chunks: &mut Vec<ChunkInfo<E>>,
                         cur_end: &mut ChunkIndex, farthest: &mut MatchInfo) -> io::Result<()> {
        let _ = (chunks, cur_end, farthest);
        todo!() // TODO: Implement
    }
}
