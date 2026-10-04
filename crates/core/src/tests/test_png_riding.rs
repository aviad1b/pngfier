use crate::{
    png::riding::PngRider,
    streams::{
        dummy::DummyBinaryStream,
        traits::{InputBinaryStreams, OutputBinaryStreams},
    },
};

const IDX_IMG: usize = 0;
const IDX_TLR: usize = 1;

#[test]
fn png_riding_data_read_correctly() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
        0xAB, 0xCD, 0xEF, 0x12,
    ];

    let mut input = DummyBinaryStream::new(data);

    let mut rider = PngRider::<'_, IDX_IMG, IDX_TLR, _>::new(&mut input).unwrap();

    let mut buff = [0_u8; 2];

    rider.streams.read_bytes::<IDX_TLR>(&mut buff).unwrap();
    assert_eq!(&buff, &[0xAB, 0xCD]);

    rider.streams.read_bytes::<IDX_TLR>(&mut buff).unwrap();
    assert_eq!(&buff, &[0xEF, 0x12]);
}

#[test]
fn png_riding_data_written_correctly() {
    let data: Vec<u8> = vec![
        0x00, 0x01, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
        0x00, 0x00, 0x00, 0x00,
    ];

    let mut output = DummyBinaryStream::new(data);

    let mut rider = PngRider::<'_, IDX_IMG, IDX_TLR, _>::new(&mut output).unwrap();

    rider.streams.write_bytes::<IDX_TLR>(&[0xAA, 0xBB]).unwrap();
    rider.streams.write_bytes::<IDX_TLR>(&[0xFF, 0xEE]).unwrap();

    let data = output.get_all();
    assert_eq!(
        &[0xAA, 0xBB, 0xFF, 0xEE],
        &data[data.len()-4..]
    );
}
