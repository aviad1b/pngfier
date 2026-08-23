use generic_array::{GenericArray, typenum::U2};

use crate::streams::{dummy::{DummyBinaryStream, DummyOutputElemStream}, grouping::GroupedBinaryStreams, traits::Stream};

use super::super::chunks::{ChunkInfo, storage::*};

#[test]
fn chunks_storage_round_trip() {
    let chunks = vec![
        ChunkInfo::Literal(vec![0x04, 0x03, 0x05]),
        ChunkInfo::Reference { index: 0, size: 5 },
        ChunkInfo::Literal(vec![0x08]),
        ChunkInfo::Literal(vec![0x09]),
    ];

    let img = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

    let expected = vec![
        0x04, 0x03, 0x05,
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0x08, 0x09,
    ];

    let widths = ChunkInfoWidths { is_literal: 1, size: 15, index: 16 };
    let mut img_stream = DummyBinaryStream::new(img);
    let mut key_stream = DummyBinaryStream::new(vec![]);

    {
        let mut streams = GroupedBinaryStreams::<'_, U2, _>::new(
            GenericArray::from_array([&mut img_stream, &mut key_stream])
        );

        let chunks = &mut chunks.iter();
        let mut writer = ChunksWriter::<'_, '_, '_, 0, 1, _, _, _>::new(
            widths, chunks, &mut streams
        );
        writer.write().unwrap();
    }

    let mut output = DummyOutputElemStream::new(Vec::<u8>::new());
    {
        img_stream.rewind().unwrap();
        key_stream.rewind().unwrap();
        let mut streams = GroupedBinaryStreams::<'_, U2, _>::new(
            GenericArray::from_array([&mut img_stream, &mut key_stream])
        );

        let mut reader = ChunksReader::<'_, '_, 0, 1, _, _, _>::new(
            widths, &mut streams, &mut output
        );
        reader.extract_all().unwrap();
    }

    assert_eq!(
        output.get_all(), expected,
        "Bad output. Key: {:?}", key_stream.get_all()
    );
}
