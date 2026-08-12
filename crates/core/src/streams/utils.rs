use std::io;

use super::StreamPos;

/// Utility function for `InputElemStream::lookup` and `InputElemStreams::lookup`.
pub fn lookup<E: Eq>(stream_start: StreamPos,
                     mut read_next: impl FnMut() -> io::Result<Option<E>>,
                     data: &[E]) -> io::Result<Option<StreamPos>> {
    let mut stream_index = stream_start;
    let mut data_index: usize = 0;

    // while still has element to read and hasn't gone through entire data yet
    while let Some(elem) = read_next()? && data_index < data.len() {
        if elem == data[data_index] {
            data_index += 1;
        } else {
            data_index = 0;
        }
        stream_index += 1;
    }

    // if data_index managed to go through entire data, return stream_index
    // otherwise, data wasn't found (return `None`)
    return if data_index < data.len() { Ok(None) } else { Ok(Some(stream_index)) };
}
