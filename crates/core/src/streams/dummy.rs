use std::io::{self, Cursor, Read, Write};

use bitstream_io::{BitRead, BitReader, BitWrite, BitWriter, Endianness};

use crate::streams::traits::{InputBinaryStream, OutputBinaryStream};

use super::{
    StreamPos,
    traits::{
        InputElemStream,
        OutputElemStream,
        Stream,
    },
};

/// Dummy implementation of `InputElemStream` trait, uses runtime vector.
pub struct DummyInputElemStream<E: Copy> {
    elems: Vec<E>,
    pos: StreamPos
}

impl<E: Copy> DummyInputElemStream<E> {
    /// Constructs a new DummyInputElemStream.
    /// 
    /// * `elems` - Elems vector (takes ownership).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(elems: Vec<E>) -> Self {
        Self {
            elems: elems,
            pos: 0,
        }
    }
}

impl<E: Copy> Stream for DummyInputElemStream<E> {
    fn rewind(&mut self) -> io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        Ok(self.pos)
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        Ok(self.elems.len() as StreamPos)
    }
}

impl<E: Copy> InputElemStream<E> for DummyInputElemStream<E> {
    fn read_next_elem(&mut self) -> io::Result<Option<E>> {
        if self.pos >= self.elems.len() as StreamPos {
            return Ok(None); // end-of-stream
        }
        let res = self.elems[self.pos as usize];
        self.pos += 1;
        Ok(Some(res))
    }
}

/// Dummy implementation of `OutputElemStream` trait, uses runtime vector.
pub struct DummyOutputElemStream<E: Copy> {
    elems: Vec<E>,
    pos: StreamPos
}

impl<E: Copy> DummyOutputElemStream<E> {
    /// Constructs a new DummyOutputElemStream.
    /// 
    /// * `elems` - Elems vector (takes ownership).
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(elems: Vec<E>) -> Self {
        Self {
            elems: elems,
            pos: 0,
        }
    }

    /// Gets slice to all stream elements.
    pub fn get_all(&self) -> &[E] {
        &self.elems
    }
}

impl<E: Copy> Stream for DummyOutputElemStream<E> {
    fn rewind(&mut self) -> io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        Ok(self.pos)
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        Ok(self.elems.len() as StreamPos)
    }
}

impl<E: Copy> OutputElemStream<E> for DummyOutputElemStream<E> {
    fn write_next_elem(&mut self, elem: E) -> io::Result<()> {
        if self.pos as usize >= self.elems.len() {
            self.elems.push(elem);
        } else {
            self.elems[self.pos as usize] = elem;
        }
        self.pos += 1;
        Ok(())
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        if 0 == len {
            self.elems.clear();
        } else {
            self.elems.resize(len as usize, self.elems[0]);
        }
        Ok(())
    }
}

/// Dummy implementation of `InputBinaryStream` and `OutputBinaryStream` traits, 
/// uses runtime vector.
pub struct DummyBinaryStream {
    cursor: Cursor<Vec<u8>>,
}

impl DummyBinaryStream {
    /// Constructs a new instance.
    /// 
    /// * `data` - Initial data (bytes vector) for stream.
    /// 
    /// Returns constructed instance.
    /// 
    pub fn new(data: Vec<u8>) -> Self {
        Self { cursor: Cursor::new(data) }
    }
}

impl Stream for DummyBinaryStream {
    fn rewind(&mut self) -> io::Result<()> {
        self.cursor.set_position(0);
        Ok(())
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        Ok(self.cursor.position() as StreamPos)
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.cursor.set_position(pos as u64);
        Ok(())
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        Ok(self.cursor.get_ref().len() as StreamPos)
    }
}

impl InputBinaryStream for DummyBinaryStream {
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()> {
        self.cursor.read_exact(buff)
    }

    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        Ok(BitReader::endian(&mut self.cursor, endianness))
    }
}

impl OutputBinaryStream for DummyBinaryStream {
    fn write_bytes(&mut self, buff: &[u8]) -> io::Result<()> {
        self.cursor.write_all(buff)
    }

    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl BitWrite> {
        Ok(BitWriter::endian(&mut self.cursor, endianness))
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let pos = self.cursor.position();
        self.cursor.get_mut().resize(len as usize, 0);
        if pos > len as u64 {
            self.cursor.set_position(len as u64);
        }
        Ok(())
    }
}
