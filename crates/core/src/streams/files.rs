use bitstream_io::{BitRead, BitReader, BitWriter, Endianness};
use std::{
    fs::{File, OpenOptions},
    io,
};

use super::{
    StreamPos,
    traits::{
        InputBinaryStream,
        OutputBinaryStream,
        Stream
    }
};

/// Utility used by binary file abstractions.
struct BinaryFileStreamBase {
    file: File,
}

impl BinaryFileStreamBase {
    /// Constructs a new `BinaryFileStreamBase` instance.
    /// 
    /// * `path` - Path of file to open.
    /// * `opts` - Open options to use for opening the file.
    /// 
    /// Returns constructed instance, or error if occurred.
    /// 
    fn new(path: &str, opts: &OpenOptions) -> io::Result<Self> {
        Ok(Self{
            file: opts.open(path)?,
        })
    }
}

/// Abstraction over a file of binary input.
pub struct InputBinaryFileStream {

}

impl InputBinaryFileStream {
    /// Constructs a new input binary file stream instance.
    /// 
    /// * `path` - Path of existing file to read from.
    /// 
    /// Returns stream instance, or error if occurred.
    /// 
    pub fn new(path: &str) -> io::Result<Self> {
        let _ = path;
        todo!() // TODO: Implement
    }
}

impl Stream for InputBinaryFileStream {
    fn rewind(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }

    fn set_pos(&mut self, pos: super::StreamPos) -> io::Result<()> {
        let _ = pos;
        todo!() // TODO: Implement
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }
}

impl InputBinaryStream for InputBinaryFileStream {
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        let _ = endianness;
        Err::<BitReader<File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }
}

/// Abstraction over a file of binary output.
pub struct OutputBinaryFileStream {

}

impl OutputBinaryFileStream {
    /// Constructs a new output binary file stream instance.
    /// 
    /// * `path` - Path of existing file to write to.
    /// 
    /// Returns stream instance, or error if occurred.
    /// 
    pub fn new(path: &str) -> io::Result<Self> {
        let _ = path;
        todo!() // TODO: Implement
    }
}

impl Stream for OutputBinaryFileStream {
    fn rewind(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }

    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        let _ = pos;
        todo!() // TODO: Implement
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }
}

impl OutputBinaryStream for OutputBinaryFileStream {
    fn write_bytes(&mut self, buff: &[u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl bitstream_io::BitWrite> {
        let _ = endianness;
        Err::<BitWriter<File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implemen
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}

/// Abstraction over a file of binary data (input & output).
pub struct TwoWayBinaryFileStream {

}

impl TwoWayBinaryFileStream {
    /// Constructs a new two-way binary file stream instance.
    /// 
    /// * `path` - Path of existing file to read from and write to.
    /// 
    /// Returns stream instance, or error if occurred.
    /// 
    pub fn new(path: &str) -> io::Result<Self> {
        let _ = path;
        todo!() // TODO: Implement
    }
}

impl Stream for TwoWayBinaryFileStream {
    fn rewind(&mut self) -> io::Result<()> {
        todo!() // TODO: Implement
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }

    fn set_pos(&mut self, pos: super::StreamPos) -> io::Result<()> {
        let _ = pos;
        todo!() // TODO: Implement
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        todo!() // TODO: Implement
    }
}

impl InputBinaryStream for TwoWayBinaryFileStream {
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        let _ = endianness;
        Err::<BitReader<File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implement
    }
}

impl OutputBinaryStream for TwoWayBinaryFileStream {
    fn write_bytes(&mut self, buff: &[u8]) -> io::Result<()> {
        let _ = buff;
        todo!() // TODO: Implement
    }

    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl bitstream_io::BitWrite> {
        let _ = endianness;
        Err::<BitWriter<File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implemen
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        let _ = len;
        todo!() // TODO: Implement
    }
}
