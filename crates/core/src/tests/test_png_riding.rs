use generic_array::{GenericArray, typenum::U2};

use crate::{
    png::riding::PngRider,
    streams::{
        dummy::DummyBinaryStream,
        traits::{ConstBinParsible, InputElemStreams, OutputElemStreams},
    },
};

const IDX_IMG: usize = 0;
const IDX_TLR: usize = 1;

/// Parsible element used for testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestElem(u16);

impl ConstBinParsible for TestElem {
	type BuffSize = U2;

	fn const_bin_parse(buff: &GenericArray<u8, U2>) -> Self {
		TestElem(u16::from_be_bytes([buff[0], buff[1]]))
	}

	fn const_bin_unparse(&self, buff: &mut GenericArray<u8, U2>) {
		let bytes = self.0.to_be_bytes();
		buff[0] = bytes[0];
		buff[1] = bytes[1];
	}
}

#[test]
fn png_riding_data_read_correctly() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
        0xAB, 0xCD, 0xEF, 0x12,
    ];

    let mut input = DummyBinaryStream::new(data);

    let mut rider = PngRider::<'_, IDX_IMG, IDX_TLR, TestElem, _>::new(&mut input).unwrap();

    assert_eq!(TestElem(0xABCD), rider.streams.read_next_elem::<IDX_TLR>().unwrap().unwrap());
    assert_eq!(TestElem(0xEF12), rider.streams.read_next_elem::<IDX_TLR>().unwrap().unwrap());
}

#[test]
fn png_riding_data_written_correctly() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut output = DummyBinaryStream::new(data);

    let mut rider = PngRider::<'_, IDX_IMG, IDX_TLR, TestElem, _>::new(&mut output).unwrap();

    rider.streams.write_next_elem::<IDX_TLR>(TestElem(0xAABB)).unwrap();
    rider.streams.write_next_elem::<IDX_TLR>(TestElem(0xFFEE)).unwrap();

    let data = output.get_all();
    assert_eq!(
        &[0xAA, 0xBB, 0xFF, 0xEE],
        &data[data.len()-4..]
    );
}
