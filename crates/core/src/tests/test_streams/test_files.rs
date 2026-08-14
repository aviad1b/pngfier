use bitstream_io::BigEndian;
use bitstream_io::BitRead;
use bitstream_io::BitWrite;
use bitstream_io::LittleEndian;

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
