use std::io;

use bitstream_io::{BigEndian, BitRead, BitWrite};
use generic_array::typenum::{U1, U2};

use crate::{
    obtain_bits_reader,
    obtain_bits_writer,
    return_bits_reader,
    return_bits_writer,
};

use super::super::super::streams::{spans::*, traits::*, dummy::*};

/////////////////////////////////////////////////////////////////
// ---------------- Single span, whole stream ---------------- //
/////////////////////////////////////////////////////////////////

#[test]
fn single_span_no_bounds_covers_whole_stream() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 4);
	let mut buf = [0x00u8; 4];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn single_span_get_pos_starts_at_zero_and_advances() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_pos::<0>().unwrap(), 0);
	let mut buf = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
}

#[test]
fn single_span_set_pos_seeks_locally() {
	let mut stream = DummyBinaryStream::new(vec![0x0A, 0x0B, 0x0C, 0x0D]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	spans.set_pos::<0>(2).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
	let mut buf = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x0C, 0x0D]);
}

#[test]
fn single_span_rewind_resets_to_local_zero() {
	let mut stream = DummyBinaryStream::new(vec![0x05, 0x06, 0x07]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	let mut buf = [0x00u8; 1];
	spans.read_bytes::<0>(&mut buf).unwrap();
	spans.rewind::<0>().unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 0);
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x05]);
}

#[test]
fn single_span_read_past_bound_errors() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	let mut buf = [0x00u8; 5]; // more than the 2 bytes available
	let result = spans.read_bytes::<0>(&mut buf);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
}

///////////////////////////////////////////////////////////////
// ---------------- Byte-range bounded span ---------------- //
///////////////////////////////////////////////////////////////

#[test]
fn span_with_explicit_offset_skips_leading_bytes() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x01, 0x02, 0x03]);
	let offsets = opt_array::<U1>(&[Some(2)]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 3);
	let mut buf = [0x00u8; 3];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03]);
}

#[test]
fn span_with_explicit_end_stops_before_trailing_bytes() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
	let offsets = opt_array::<U1>(&[Some(0)]);
	let ends = opt_array::<U1>(&[Some(3)]); // covers bytes [0,3)
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 3);
	let mut buf = [0x00u8; 3];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03]);

	// reading beyond the span's end must fail even though the
	// underlying stream still has more bytes.
	let mut extra = [0x00u8; 1];
	let result = spans.read_bytes::<0>(&mut extra);
	assert!(result.is_err());
}

#[test]
fn span_with_offset_and_end_bounds_a_middle_slice() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x01, 0x02, 0xFF, 0xFF]);
	let offsets = opt_array::<U1>(&[Some(2)]);
	let ends = opt_array::<U1>(&[Some(4)]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 2);
	let mut buf = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02]);
}

///////////////////////////////////////////////////////////////////////////
// ---------------- Multiple spans over a shared stream ---------------- //
///////////////////////////////////////////////////////////////////////////

#[test]
fn multiple_spans_are_independent() {
	let mut stream = DummyBinaryStream::new(vec![
		0x01, 0x02, 0x03, 0x04, // span0: bytes [0,4)
		0x09, 0x08, 0x07,       // span1: bytes [4,7)
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(4)]);
	let ends = opt_array::<U2>(&[Some(4), Some(7)]);
	let mut spans = BinarySpans::<_, U2>::new(&mut stream, offsets, ends);

	let mut buf0a = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf0a).unwrap();
	assert_eq!(buf0a, [0x01, 0x02]);

	let mut buf1 = [0x00u8; 2];
	spans.read_bytes::<1>(&mut buf1).unwrap();
	assert_eq!(buf1, [0x09, 0x08]);

	// continuing span0 must resume where it left off, unaffected by
	// the interleaved read on span1
	let mut buf0b = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf0b).unwrap();
	assert_eq!(buf0b, [0x03, 0x04]);
}

#[test]
fn multiple_spans_track_positions_independently() {
	let mut stream = DummyBinaryStream::new(vec![
		0x01, 0x02, 0x03, 0x04, 0x05, // span0: 5 bytes
		0x09, 0x08, 0x07,             // span1: 3 bytes
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(5)]);
	let ends = opt_array::<U2>(&[Some(5), Some(8)]);
	let mut spans = BinarySpans::<_, U2>::new(&mut stream, offsets, ends);

	spans.set_pos::<0>(3).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 3);
	assert_eq!(spans.get_pos::<1>().unwrap(), 0); // untouched

	spans.set_pos::<1>(2).unwrap();
	let mut buf = [0x00u8; 1];
	spans.read_bytes::<1>(&mut buf).unwrap();
	assert_eq!(buf, [0x07]);
	assert_eq!(spans.get_pos::<0>().unwrap(), 3); // unaffected by span1's seek
}

//////////////////////////////////////////////
// ---------------- Output ---------------- //
//////////////////////////////////////////////

#[test]
fn write_bytes_writes_at_correct_global_offset() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 6]);
	let offsets = opt_array::<U1>(&[Some(2)]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	spans.write_bytes::<0>(&[0x01, 0x02]).unwrap();
	spans.write_bytes::<0>(&[0x03, 0x04]).unwrap();

	assert_eq!(stream.get_all(), &[0x00, 0x00, 0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn write_bytes_advances_local_pos() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	spans.write_bytes::<0>(&[0x01, 0x02]).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
}

#[test]
fn write_then_rewind_then_read_back_roundtrip() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	spans.write_bytes::<0>(&[0x11, 0x99]).unwrap();
	spans.rewind::<0>().unwrap();

	let mut buf = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x11, 0x99]);
}

#[test]
fn writes_on_two_spans_do_not_clobber_each_other() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 6]);
	let offsets = opt_array::<U2>(&[Some(0), Some(3)]);
	let ends = opt_array::<U2>(&[Some(3), Some(6)]);
	let mut spans = BinarySpans::<_, U2>::new(&mut stream, offsets, ends);

	spans.write_bytes::<0>(&[0x01, 0x01, 0x01]).unwrap();
	spans.write_bytes::<1>(&[0x02, 0x02, 0x02]).unwrap();

	assert_eq!(stream.get_all(), &[0x01, 0x01, 0x01, 0x02, 0x02, 0x02]);
}

#[test]
fn truncate_shrinks_span_size() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	spans.truncate::<0>(2).unwrap();

	assert_eq!(spans.get_size::<0>().unwrap(), 2);
	spans.rewind::<0>().unwrap();
	let mut buf = [0x00u8; 2];
	spans.read_bytes::<0>(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02]);
}

#[test]
fn truncate_on_one_span_does_not_affect_sibling_span() {
	let mut stream = DummyBinaryStream::new(vec![
		0x01, 0x02, 0x03, 0x04, // span0: bytes [0,4)
		0x09, 0x08, 0x07, 0x06, // span1: bytes [4,8)
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(4)]);
	let ends = opt_array::<U2>(&[Some(4), Some(8)]);
	let mut spans = BinarySpans::<_, U2>::new(&mut stream, offsets, ends);

	spans.truncate::<0>(1).unwrap();

	assert_eq!(spans.get_size::<0>().unwrap(), 1);
	assert_eq!(spans.get_size::<1>().unwrap(), 4);
}

/////////////////////////////////////////////////////////
// ---------------- Bit reader/writer ---------------- //
/////////////////////////////////////////////////////////

#[test]
fn bits_reader_reads_from_span_offset_not_stream_start() {
	// leading junk byte, then 0b10110000 at the span's start
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0b10110000]);
	let offsets = opt_array::<U1>(&[Some(1)]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	let mut reader = obtain_bits_reader!(spans, BigEndian, 0).unwrap();
	let bit0: u8 = reader.read(1).unwrap();
	let bit1: u8 = reader.read(1).unwrap();
	let bit2: u8 = reader.read(1).unwrap();
	return_bits_reader!(reader, spans, 0).unwrap();
	assert_eq!((bit0, bit1, bit2), (1, 0, 1));
}

#[test]
fn bits_writer_writes_within_span_bounds() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0x00]);
	let offsets = opt_array::<U1>(&[Some(1)]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinarySpans::<_, U1>::new(&mut stream, offsets, ends);

	{
		let mut writer = obtain_bits_writer!(spans, BigEndian, 0).unwrap();
		writer.write(0x04, 0b1010u8).unwrap();
		writer.write(0x04, 0b0101u8).unwrap();
		return_bits_writer!(writer, spans, 0).unwrap();
	}

	// the leading junk byte (span offset) must remain untouched.
	assert_eq!(stream.get_all()[0], 0xFF);
	assert_eq!(stream.get_all()[1], 0b10100101);
}
