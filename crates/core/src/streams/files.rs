use bitstream_io::{BitRead, BitReader, BitWriter, Endianness};
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
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

    /// Rewinds cursor back to the beggining of the file.
    /// 
    /// Returns error if occurred.
    /// 
    fn rewind(&mut self) -> io::Result<()> {
        self.file.rewind()?;
        Ok(())
    }

    /// Gets current position of cursor in file.
    /// 
    /// Returns current position of cursor in file, or error if occurred.
    /// 
    fn get_pos(&mut self) -> io::Result<StreamPos> {
        Ok(self.file.stream_position()? as StreamPos)
    }

    /// Sets current position of cursor in file.
    /// 
    /// * `pos` - New position for cursor in file.
    /// 
    /// Returns error if occurred.
    /// 
    fn set_pos(&mut self, pos: StreamPos) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(pos as u64))?;
        Ok(())
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        let pos = self.file.stream_position()?;
        let size = self.file.seek(SeekFrom::End(0))?;
        self.file.seek(SeekFrom::Start(pos as u64))?;
        Ok(size as StreamPos)
    }
}

/// Abstraction over a file of binary input.
pub struct InputBinaryFileStream {
    base: BinaryFileStreamBase,
}

impl InputBinaryFileStream {
    /// Constructs a new input binary file stream instance.
    /// 
    /// * `path` - Path of existing file to read from.
    /// 
    /// Returns stream instance, or error if occurred.
    /// 
    pub fn new(path: &str) -> io::Result<Self> {
        let mut opts = OpenOptions::new();
        let opts = opts.read(true);
        let base = BinaryFileStreamBase::new(path, opts)?;
        Ok(Self{ base })
    }
}

impl Stream for InputBinaryFileStream {
    fn rewind(&mut self) -> io::Result<()> {
        self.base.rewind()
    }

    fn get_pos(&mut self) -> io::Result<StreamPos> {
        self.base.get_pos()
    }

    fn set_pos(&mut self, pos: super::StreamPos) -> io::Result<()> {
        self.base.set_pos(pos)
    }

    fn get_size(&mut self) -> io::Result<StreamPos> {
        self.base.get_size()
    }
}

impl InputBinaryStream for InputBinaryFileStream {
    fn read_bytes(&mut self, buff: &mut [u8]) -> io::Result<()> {
        self.base.file.read_exact(buff)
    }

    fn obtain_bits_reader(&mut self, endianness: impl Endianness) -> io::Result<impl BitRead> {
        Ok(BitReader::endian(&mut self.base.file, endianness))
    }
}

/// Abstraction over a file of binary output.
pub struct OutputBinaryFileStream {
    base: BinaryFileStreamBase,
}

impl OutputBinaryFileStream {
    /// Constructs a new output binary file stream instance.
    /// 
    /// * `path` - Path of existing file to write to.
    /// 
    /// Returns stream instance, or error if occurred.
    /// 
    pub fn new(path: &str) -> io::Result<Self> {
        let mut opts = OpenOptions::new();
        let opts = opts.create(true).write(true);
        Ok(Self{ base: BinaryFileStreamBase::new(path, opts)? })
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
        self.base.file.write_all(buff)?;
        Ok(())
    }

    fn obtain_bits_writer(&mut self, endianness: impl Endianness) -> io::Result<impl bitstream_io::BitWrite> {
        let _ = endianness;
        Err::<BitWriter<File, bitstream_io::LittleEndian>, io::Error>(
            io::Error::new(io::ErrorKind::NotFound, "To be implemented")
        ) // TODO: Implemen
    }

    fn truncate(&mut self, len: StreamPos) -> io::Result<()> {
        self.base.file.set_len(len as u64)?;
        Ok(())
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
