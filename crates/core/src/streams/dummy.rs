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
    fn rewind(&mut self) -> std::io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn get_pos(&mut self) -> std::io::Result<StreamPos> {
        Ok(self.pos)
    }

    fn set_pos(&mut self, pos: StreamPos) -> std::io::Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn get_size(&mut self) -> std::io::Result<StreamPos> {
        Ok(self.elems.len() as StreamPos)
    }
}

impl<E: Copy> InputElemStream<E> for DummyInputElemStream<E> {
    fn read_next_elem(&mut self) -> std::io::Result<Option<E>> {
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
    fn rewind(&mut self) -> std::io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn get_pos(&mut self) -> std::io::Result<StreamPos> {
        Ok(self.pos)
    }

    fn set_pos(&mut self, pos: StreamPos) -> std::io::Result<()> {
        self.pos = pos;
        Ok(())
    }

    fn get_size(&mut self) -> std::io::Result<StreamPos> {
        Ok(self.elems.len() as StreamPos)
    }
}

impl<E: Copy> OutputElemStream<E> for DummyOutputElemStream<E> {
    fn write_next_elem(&mut self, elem: E) -> std::io::Result<()> {
        if self.pos as usize >= self.elems.len() {
            self.elems.push(elem);
        } else {
            self.elems[self.pos as usize] = elem;
        }
        self.pos += 1;
        Ok(())
    }

    fn truncate(&mut self, len: StreamPos) -> std::io::Result<()> {
        if 0 == len {
            self.elems.clear();
        } else {
            self.elems.resize(len as usize, self.elems[0]);
        }
        Ok(())
    }
}
