use anyhow::{Context, Result, bail};

use generic_array::typenum::U2;

use crate::streams::{
    StreamPos,
    spans::{BinaryElemSpan, BinarySpans, opt_arr},
    traits::{InputBinaryStream, InputElemStream},
};

const IEND: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

pub struct PngRider<'a, const IDX_IMG: usize, const IDX_TLR: usize, S: InputBinaryStream> {
    pub streams: BinarySpans<'a, S, U2>,
}

impl<'a, const IDX_IMG: usize, const IDX_TLR: usize, S: InputBinaryStream>
PngRider<'a, IDX_IMG, IDX_TLR, S> {
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
        byte_offsets[IDX_TLR] = Some(png_end);
        byte_ends[IDX_IMG] = Some(png_end);

        Ok(PngRider { streams: BinarySpans::new(
            stream,
            opt_arr(&byte_offsets),
            opt_arr(&byte_ends))
        })
    }
}
