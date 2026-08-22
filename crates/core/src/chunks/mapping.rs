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
        Self { reach, phantom: PhantomData }
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
        // single forward pass, same shape as Jump Game II / minimum-interval-cover:
        // track the best (furthest-reaching) match seen so far without resetting,
        // and commit a chunk only when the scan catches up to the current frontier

        let mut chunks = Vec::new();
        let mut cur_end: ChunkIndex = 0;
        let mut farthest = MatchInfo { reach: 0, src_start: 0 };
        let mut farthest_data_start: ChunkIndex = 0;
        let mut i: ChunkIndex = 0;

        while i < self.reach.len()? {
            let mi = self.reach.get(i)?;
            if mi.reach > farthest.reach {
                farthest = mi;
                farthest_data_start = i;
            }

            if i == cur_end {
                if cur_end < farthest.reach {
                    // if can used reach to go further in reference chunk
                    self.map_reference_chunk(min_cap, max_cap, &mut chunks, &mut cur_end, &mut farthest, farthest_data_start)?;
                }
                else {
                    // if this position isn't representable as a reference chunk at all
                    self.map_literal_chunk(&mut chunks, &mut cur_end, &mut farthest)?;
                    farthest_data_start = cur_end; // keep in sync with farthest's update in `map_literal_chunk`

                    // continuing after literal chunk:
                    i = cur_end;
                    continue;
                }
            }

            i += 1;
        }

        Ok(chunks)
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
        // trim the winning match down to only the unclaimed suffix,
        // since it may have started before cur_end
        let offset = *cur_end - farthest_data_start;
        let index = farthest.src_start + offset;
        let size = farthest.reach - *cur_end;

        // if greater than min cap, use reference chunk(s)
        if min_cap.is_none_or(|min_cap| min_cap < size) {
            // split into pieces no larger than max_cap, if was given
            let mut remaining = size;
            let mut piece_index = index;
            while remaining > 0 {
                let piece_size = max_cap.map_or(remaining, |cap| remaining.min(cap));
                chunks.push(ChunkInfo::Reference { index: piece_index, size: piece_size });
                piece_index += piece_size;
                remaining -= piece_size;
            }
            *cur_end = farthest.reach;
        
        // if not greater than min cap, use literal chunk instead
        } else {
            let elem = self.reach.get_elems(*cur_end, 1)?;
            chunks.push(ChunkInfo::Literal(elem));
            *cur_end += 1;
            *farthest = MatchInfo { reach: *cur_end, src_start: *cur_end };
        }

        Ok(())
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
        // extend literal chunk as long as consecutive positions also have no match
        let literal_start = *cur_end;
        while *cur_end < self.reach.len()? && self.reach.get(*cur_end)?.reach <= *cur_end {
            *cur_end += 1;
        }
        let elems = self.reach.get_elems(literal_start, *cur_end - literal_start)?;
        chunks.push(ChunkInfo::Literal(elems));

        // farthest/i bookkeeping needs re-syncing to the new cur_end here —
        // e.g. reset farthest to {reach: cur_end, src_start: cur_end} and
        // continue the outer scan from i = cur_end
        *farthest = MatchInfo { reach: *cur_end, src_start: *cur_end };

        Ok(())
    }
}
