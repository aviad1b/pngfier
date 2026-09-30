use anyhow::{Context, Result, bail};
use generic_array::typenum::U2;

use crate::streams::{
    StreamPos,
    spans::{BinaryElemSpan, BinaryElemSpans, opt_arr},
    traits::{ConstBinParsible, InputBinaryStream, InputElemStream},
};

const IEND: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

pub const PNGR_IMG_IDX: usize = 0; // image index
pub const PNGR_TLR_IDX: usize = 1; // tailer index

pub struct PngRider<'a, E: ConstBinParsible, S: InputBinaryStream> {
    pub streams: BinaryElemSpans<'a, E, S, U2>,
}

impl<'a, E: ConstBinParsible, S: InputBinaryStream> PngRider<'a, E, S> {
    pub fn new(stream: &'a mut S) -> Result<Self> {
        // tailer comes after IEND chunk (lookup + len)
        let png_end = {
            let mut temp_stream = BinaryElemSpan::new(stream, Some(0), None);
            match temp_stream.lookup(&IEND)? {
                None => bail!("IEND chunk not found"),
                Some(x) => x,
            }
        } + IEND.len() as StreamPos;
        stream.rewind()
            .context("Failed to rewind during construction of PNG rider")?;

        // tailer starts at `png_end`, image ends at `png_end`
        let mut byte_offsets = [None, None];
        let mut byte_ends = [None, None];
        byte_offsets[PNGR_TLR_IDX] = Some(png_end);
        byte_ends[PNGR_IMG_IDX] = Some(png_end);

        Ok(PngRider { streams: BinaryElemSpans::new(
            stream,
            opt_arr(&byte_offsets),
            opt_arr(&byte_ends))
        })
    }
}
