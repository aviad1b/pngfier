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
