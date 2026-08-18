use std::io;

use bitstream_io::{BigEndian, BitRead, BitWrite};

use super::super::super::streams::{spans::*, traits::*, dummy::*};

////////////////////////////////////////////////////////////////////////
// ---------------- Construction / whole-stream span ---------------- //
////////////////////////////////////////////////////////////////////////

#[test]
fn span_with_no_bounds_covers_whole_stream() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	assert_eq!(span.get_size().unwrap(), 4);
	let mut buf = [0x00u8; 4];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03, 0x04]);
}

///////////////////////////////////////////////////////////////
// ---------------- Stream trait forwarding ---------------- //
///////////////////////////////////////////////////////////////

#[test]
fn get_pos_starts_at_zero_and_advances_on_read() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	assert_eq!(span.get_pos().unwrap(), 0);
	let mut buf = [0x00u8; 2];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(span.get_pos().unwrap(), 2);
}

#[test]
fn set_pos_seeks_locally() {
	let mut stream = DummyBinaryStream::new(vec![0x0A, 0x0B, 0x0C, 0x0D]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	span.set_pos(2).unwrap();
	assert_eq!(span.get_pos().unwrap(), 2);
	let mut buf = [0x00u8; 2];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x0C, 0x0D]);
}

#[test]
fn rewind_resets_to_local_zero() {
	let mut stream = DummyBinaryStream::new(vec![0x05, 0x06, 0x07]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	let mut buf = [0x00u8; 1];
	span.read_bytes(&mut buf).unwrap();
	span.rewind().unwrap();
	assert_eq!(span.get_pos().unwrap(), 0);
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x05]);
}

#[test]
fn read_past_bound_errors() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	let mut buf = [0x00u8; 5]; // more than the 2 bytes available
	let result = span.read_bytes(&mut buf);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
}

// ---------------- Byte-range bounded span ----------------

#[test]
fn explicit_offset_skips_leading_bytes() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x01, 0x02, 0x03]);
	let mut span = BinarySpan::new(&mut stream, Some(2), None);

	assert_eq!(span.get_size().unwrap(), 3);
	let mut buf = [0x00u8; 3];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03]);
}

#[test]
fn explicit_end_stops_before_trailing_bytes() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
	let mut span = BinarySpan::new(&mut stream, Some(0), Some(3)); // covers [0,3)

	assert_eq!(span.get_size().unwrap(), 3);
	let mut buf = [0x00u8; 3];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02, 0x03]);

	// eading beyond the span's end must fail even though the
	// underlying stream still has more bytes
	let mut extra = [0u8; 1];
	let result = span.read_bytes(&mut extra);
	assert!(result.is_err());
}

#[test]
fn explicit_offset_and_end_bound_a_middle_slice() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x01, 0x02, 0xFF, 0xFF]);
	let mut span = BinarySpan::new(&mut stream, Some(2), Some(4));

	assert_eq!(span.get_size().unwrap(), 2);
	let mut buf = [0x00u8; 2];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02]);
}

//////////////////////////////////////////////
// ---------------- Output ---------------- //
//////////////////////////////////////////////

#[test]
fn write_bytes_writes_at_correct_global_offset() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 6]);
	let mut span = BinarySpan::new(&mut stream, Some(2), None);

	span.write_bytes(&[0x01, 0x02]).unwrap();
	span.write_bytes(&[0x03, 0x04]).unwrap();

	assert_eq!(stream.get_all(), &[0x00, 0x00, 0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn write_bytes_advances_pos() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	span.write_bytes(&[0x01, 0x02]).unwrap();
	assert_eq!(span.get_pos().unwrap(), 2);
}

#[test]
fn write_then_rewind_then_read_back_roundtrip() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	span.write_bytes(&[0x11, 0x99]).unwrap();
	span.rewind().unwrap();

	let mut buf = [0x00u8; 2];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x11, 0x99]);
}

#[test]
fn truncate_shrinks_span_size() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03, 0x04]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	span.truncate(2).unwrap();

	assert_eq!(span.get_size().unwrap(), 2);
	span.rewind().unwrap();
	let mut buf = [0x00u8; 2];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(buf, [0x01, 0x02]);
}

#[test]
fn truncate_to_zero_empties_span() {
	let mut stream = DummyBinaryStream::new(vec![0x01, 0x02, 0x03]);
	let mut span = BinarySpan::new(&mut stream, None, None);

	span.truncate(0).unwrap();

	assert_eq!(span.get_size().unwrap(), 0);
	let mut buf = [0x00u8; 1];
	let result = span.read_bytes(&mut buf);
	assert!(result.is_err());
}

/////////////////////////////////////////////////////////
// ---------------- Bit reader/writer ---------------- //
/////////////////////////////////////////////////////////

#[test]
fn bits_reader_reads_from_span_offset_not_stream_start() {
	// leading junk byte, then 0b10110000 at the span's start
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0b10110000]);
	let mut span = BinarySpan::new(&mut stream, Some(1), None);

	let mut reader = span.obtain_bits_reader(BigEndian).unwrap();
	let bit0: u8 = reader.read(1).unwrap();
	let bit1: u8 = reader.read(1).unwrap();
	let bit2: u8 = reader.read(1).unwrap();
	assert_eq!((bit0, bit1, bit2), (1, 0, 1));
}

#[test]
fn bits_writer_writes_within_span_bounds() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0x00]);
	let mut span = BinarySpan::new(&mut stream, Some(1), None);

	{
		let mut writer = span.obtain_bits_writer(BigEndian).unwrap();
		writer.write(4, 0b1010u8).unwrap();
		writer.write(4, 0b0101u8).unwrap();
		writer.byte_align().unwrap();
	}

	// The leading junk byte (span offset) must remain untouched.
	assert_eq!(stream.get_all()[0], 0xFF);
	assert_eq!(stream.get_all()[1], 0b10100101);
}

//////////////////////////////////////////////////////////////
// ---------------- Positions sanity check ---------------- //
//////////////////////////////////////////////////////////////

#[test]
fn span_at_nonzero_offset_reports_local_not_global_pos() {
    // checking that BinarySpan reports *local* position
    // (relative to its own offset), not the underlying stream's
    // global position
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x01, 0x02, 0x03]);
	let mut span = BinarySpan::new(&mut stream, Some(2), None);

	assert_eq!(span.get_pos().unwrap(), 0); // local pos, not byte offset 2
	let mut buf = [0x00u8; 1];
	span.read_bytes(&mut buf).unwrap();
	assert_eq!(span.get_pos().unwrap(), 1);
}
