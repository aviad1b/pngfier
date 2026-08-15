use bitstream_io::{BigEndian, BitRead, BitWrite, LittleEndian};

use crate::{
    obtain_bits_reader,
    obtain_bits_writer,
    return_bits_reader,
    return_bits_writer,
};

use super::super::super::streams::traits::*;
use super::super::super::streams::files::*;
use super::utils::TempFile;

/////////////////////////////////////////////////
// ---------- InputBinaryFileStream ---------- //
/////////////////////////////////////////////////

#[test]
fn input_new_fails_for_nonexistent_file() {
    let path = {
        let tmp = TempFile::new("input_missing.bin");
        String::from(tmp.path_str())
    }; // temp file is deleted here
	
    // file doesn't exist, so opening for reading should fail
	let result = InputBinaryFileStream::new(&path);
	assert!(result.is_err());
}

#[test]
fn input_new_opens_existing_file() {
	let tmp = TempFile::new("input_open.bin");
	tmp.write_initial(&[1, 2, 3]);
	let stream = InputBinaryFileStream::new(tmp.path_str());
	assert!(stream.is_ok());
}

#[test]
fn input_read_bytes_reads_correct_data() {
	let tmp = TempFile::new("input_read.bin");
	tmp.write_initial(&[10, 20, 30, 40]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

	let mut buf = [0_u8; 4];
	stream.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [10, 20, 30, 40]);
}

#[test]
fn input_get_size_reports_file_len_without_moving_pos() {
	let tmp = TempFile::new("input_size.bin");
	tmp.write_initial(&[0; 10]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.set_pos(3).unwrap();
	let size = stream.get_size().unwrap();
	assert_eq!(size, 10);
	
    // position should be restored after `get_size`
	assert_eq!(stream.get_pos().unwrap(), 3);
}

#[test]
fn input_set_pos_and_get_pos_roundtrip() {
	let tmp = TempFile::new("input_pos.bin");
	tmp.write_initial(&[1, 2, 3, 4, 5]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.set_pos(2).unwrap();
	assert_eq!(stream.get_pos().unwrap(), 2);

	let mut buf = [0_u8; 2];
	stream.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [3, 4]);
}

#[test]
fn input_rewind_resets_to_start() {
	let tmp = TempFile::new("input_rewind.bin");
	tmp.write_initial(&[7, 8, 9]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

	let mut buf = [0_u8; 2];
	stream.read_bytes(&mut buf).unwrap();
	stream.rewind().unwrap();
	assert_eq!(stream.get_pos().unwrap(), 0);

	let mut buf2 = [0_u8; 1];
	stream.read_bytes(&mut buf2).unwrap();
	assert_eq!(buf2, [7]);
}

#[test]
fn input_bits_reader_reads_expected_big_endian() {
	let tmp = TempFile::new("input_bits_be.bin");
	tmp.write_initial(&[0b10110000]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

    let mut reader = obtain_bits_reader!(stream, BigEndian).unwrap();
	let bit0: u8 = reader.read(1).unwrap();
	let bit1: u8 = reader.read(1).unwrap();
	let bit2: u8 = reader.read(1).unwrap();
	let bit3: u8 = reader.read(1).unwrap();
    return_bits_reader!(reader, stream).unwrap();
	assert_eq!((bit0, bit1, bit2, bit3), (1, 0, 1, 1));
}

#[test]
fn input_bits_reader_reads_expected_little_endian() {
	let tmp = TempFile::new("input_bits_le.bin");
	tmp.write_initial(&[0b00000001]);
	let mut stream = InputBinaryFileStream::new(tmp.path_str()).unwrap();

    let mut reader = obtain_bits_reader!(stream, LittleEndian).unwrap();
	let bit0: u8 = reader.read(1).unwrap();
    return_bits_reader!(reader, stream).unwrap();
	assert_eq!(bit0, 1);
}

//////////////////////////////////////////////////
// ---------- OutputBinaryFileStream ---------- //
//////////////////////////////////////////////////

#[test]
fn output_new_works_for_existent_file() {
	let tmp = TempFile::new("output_missing.bin");
	let result = OutputBinaryFileStream::new(tmp.path_str());
	assert!(!result.is_err());
}

#[test]
fn output_new_works_for_nonexistent_file() {
	let path = {
        let tmp = TempFile::new("output_missing.bin");
        String::from(tmp.path_str())
    };
	let result = OutputBinaryFileStream::new(&path);
	assert!(!result.is_err());
}

#[test]
fn output_write_bytes_writes_correct_data() {
	let tmp = TempFile::new("output_write.bin");
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.write_bytes(&[9, 8, 7, 6]).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![9, 8, 7, 6]);
}

#[test]
fn output_write_bytes_overwrites_correct_data() {
	let tmp = TempFile::new("output_write.bin");
	tmp.write_initial(&[0; 4]); // pre-create with placeholder content to be overwritten
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.write_bytes(&[9, 8, 7, 6]).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![9, 8, 7, 6]);
}

#[test]
fn output_truncate_to_zero_empties_file() {
	let tmp = TempFile::new("output_truncate_zero.bin");
	tmp.write_initial(&[1, 2, 3, 4]);
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.truncate(0).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert!(contents.is_empty());
}

#[test]
fn output_truncate_shrinks_file() {
	let tmp = TempFile::new("output_truncate_shrink.bin");
	tmp.write_initial(&[1, 2, 3, 4, 5]);
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.truncate(2).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![1, 2]);
}

#[test]
fn output_truncate_grows_file() {
	let tmp = TempFile::new("output_truncate_grow.bin");
	tmp.write_initial(&[1, 2]);
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.truncate(5).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents[0], 1);
	assert_eq!(contents[1], 2);
	assert_eq!(contents.len(), 5);
}

#[test]
fn output_truncate_moves_cursor_back() {
    let tmp = TempFile::new("output_truncate_back.bin");
    let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
    stream.write_bytes(&[1, 2, 3, 4]).unwrap();
    stream.truncate(2).unwrap();
    assert_eq!(stream.get_pos().unwrap(), 2);
}

#[test]
fn output_set_pos_overwrites_at_offset() {
	let tmp = TempFile::new("output_set_pos.bin");
	tmp.write_initial(&[1, 2, 3, 4]);
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
		stream.set_pos(2).unwrap();
		stream.write_bytes(&[99, 99]).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![1, 2, 99, 99]);
}

#[test]
fn output_get_size_reports_file_len_without_moving_pos() {
	let tmp = TempFile::new("input_size.bin");
	tmp.write_initial(&[0; 10]);
	let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.set_pos(3).unwrap();
	let size = stream.get_size().unwrap();
	assert_eq!(size, 10);
	
    // position should be restored after `get_size`
	assert_eq!(stream.get_pos().unwrap(), 3);
}

#[test]
fn output_set_pos_and_get_pos_roundtrip() {
	let tmp = TempFile::new("input_pos.bin");
	tmp.write_initial(&[1, 2, 3, 4, 5]);
	let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.set_pos(2).unwrap();
	assert_eq!(stream.get_pos().unwrap(), 2);

	let buf = [6, 7];
	stream.write_bytes(&buf).unwrap();
	
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![1, 2, 6, 7, 5]);
}

#[test]
fn output_rewind_resets_to_start() {
	let tmp = TempFile::new("input_rewind.bin");
	tmp.write_initial(&[7, 8, 5]);
	let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();

	let buf1 = [1, 2];
	stream.write_bytes(&buf1).unwrap();
	stream.rewind().unwrap();
	assert_eq!(stream.get_pos().unwrap(), 0);
	let buf2 = [4, 3];
	stream.write_bytes(&buf2).unwrap();

	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents, vec![4, 3, 5]);
}

#[test]
fn output_bits_writer_writes_expected() {
	let tmp = TempFile::new("output_bits.bin");
	tmp.write_initial(&[0; 1]);
	{
		let mut stream = OutputBinaryFileStream::new(tmp.path_str()).unwrap();
        let mut writer = obtain_bits_writer!(stream, BigEndian).unwrap();
		writer.write(1, 1u8).unwrap();
		writer.write(1, 0u8).unwrap();
		writer.write(1, 1u8).unwrap();
		writer.write(1, 1u8).unwrap();
		writer.write(4, 0u8).unwrap();
        return_bits_writer!(writer, stream).unwrap();
	}
	let contents = std::fs::read(&tmp.path_str()).unwrap();
	assert_eq!(contents[0], 0b10110000);
}

//////////////////////////////////////////////////////////////////
// ---------- TwoWayBinaryFileStream (read + write) ---------- ///
//////////////////////////////////////////////////////////////////

#[test]
fn two_way_new_fails_for_nonexistent_file() {
	let tmp = TempFile::new("two_way_missing.bin");
	let result = TwoWayBinaryFileStream::new(tmp.path_str());
	assert!(result.is_err());
}

#[test]
fn two_way_can_write_then_read_back() {
	let tmp = TempFile::new("two_way_rw.bin");
	tmp.write_initial(&[0; 4]);
	let mut stream = TwoWayBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.write_bytes(&[1, 2, 3, 4]).unwrap();
	stream.rewind().unwrap();

	let mut buf = [0u8; 4];
	stream.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [1, 2, 3, 4]);
}

#[test]
fn two_way_get_pos_tracks_both_read_and_write() {
	let tmp = TempFile::new("two_way_pos.bin");
	tmp.write_initial(&[0; 6]);
	let mut stream = TwoWayBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.write_bytes(&[1, 2, 3]).unwrap();
	assert_eq!(stream.get_pos().unwrap(), 3);

	stream.rewind().unwrap();
	let mut buf = [0u8; 2];
	stream.read_bytes(&mut buf).unwrap();
	assert_eq!(stream.get_pos().unwrap(), 2);
	assert_eq!(buf, [1, 2]);
}

#[test]
fn two_way_truncate_affects_size() {
	let tmp = TempFile::new("two_way_truncate.bin");
	tmp.write_initial(&[1, 2, 3, 4, 5]);
	let mut stream = TwoWayBinaryFileStream::new(tmp.path_str()).unwrap();

	stream.truncate(3).unwrap();
	assert_eq!(stream.get_size().unwrap(), 3);
}

#[test]
fn two_way_truncate_moves_cursor_back() {
    let tmp = TempFile::new("two_way_truncate_back.bin");
    tmp.write_initial(&[]); // empty file

    let mut stream = TwoWayBinaryFileStream::new(tmp.path_str()).unwrap();
    stream.write_bytes(&[1, 2, 3, 4]).unwrap();
    stream.truncate(2).unwrap();
    assert_eq!(stream.get_pos().unwrap(), 2);
}

#[test]
fn two_way_bits_reader_and_writer_roundtrip() {
	let tmp = TempFile::new("two_way_bits.bin");
	tmp.write_initial(&[0; 1]);
	let mut stream = TwoWayBinaryFileStream::new(tmp.path_str()).unwrap();

	{
        let mut writer = obtain_bits_writer!(stream, BigEndian).unwrap();
		writer.write(4, 0b1010u8).unwrap();
		writer.write(4, 0b0101u8).unwrap();
		return_bits_writer!(writer, stream).unwrap();
	}
	stream.rewind().unwrap();
	{
        let mut reader = obtain_bits_reader!(stream, BigEndian).unwrap();
		let high: u8 = reader.read(4).unwrap();
		let low: u8 = reader.read(4).unwrap();
		assert_eq!(high, 0b1010);
		assert_eq!(low, 0b0101);
        return_bits_reader!(reader, stream).unwrap();
	}
}
