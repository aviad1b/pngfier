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
        todo!()
    }

    fn get_pos(&mut self) -> std::io::Result<StreamPos> {
        todo!()
    }

    fn set_pos(&mut self, pos: StreamPos) -> std::io::Result<()> {
        todo!()
    }

    fn get_size(&mut self) -> std::io::Result<StreamPos> {
        todo!()
    }
}

impl<E: Copy> InputElemStream<E> for DummyInputElemStream<E> {
    fn read_next_elem(&mut self) -> std::io::Result<Option<E>> {
        todo!()
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
        todo!()
    }

    fn get_pos(&mut self) -> std::io::Result<StreamPos> {
        todo!()
    }

    fn set_pos(&mut self, pos: StreamPos) -> std::io::Result<()> {
        todo!()
    }

    fn get_size(&mut self) -> std::io::Result<StreamPos> {
        todo!()
    }
}

impl<E: Copy> OutputElemStream<E> for DummyOutputElemStream<E> {
    fn write_next_elem(&mut self, elem: E) -> std::io::Result<()> {
        todo!()
    }

    fn truncate(&mut self, len: StreamPos) -> std::io::Result<()> {
        todo!()
    }
}
