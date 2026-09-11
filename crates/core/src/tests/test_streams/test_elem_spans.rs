use generic_array::{GenericArray, typenum::{U1, U2}};

use super::super::super::streams::{spans::*, traits::*, dummy::*};

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

///////////////////////////////////////////////////
// ---------------- Single span ---------------- //
///////////////////////////////////////////////////

#[test]
fn single_span_reads_elements_in_order() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]); // elems 1, 2, 3
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(2)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(3)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), None);
}

#[test]
fn single_span_get_size_reports_element_count() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 3);
}

#[test]
fn single_span_get_pos_tracks_elements_read() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_pos::<0>().unwrap(), 0);
	spans.read_next_elem::<0>().unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 1);
	spans.read_next_elem::<0>().unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
}

#[test]
fn single_span_set_pos_seeks_to_element_index() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.set_pos::<0>(2).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(3)));
}

#[test]
fn single_span_rewind_resets_to_start() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.read_next_elem::<0>().unwrap();
	spans.rewind::<0>().unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 0);
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
}

////////////////////////////////////////////////////////////////
// ---------------- Byte-range bounded spans ---------------- //
////////////////////////////////////////////////////////////////

#[test]
fn span_with_explicit_offset_skips_leading_bytes() {
	let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x00, 0x01, 0x00, 0x02]);
	let offsets = opt_array::<U1>(&[Some(2)]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(2)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), None);
}

#[test]
fn span_with_explicit_end_stops_before_trailing_bytes() {
	// stream has 4 elements worth of data; span only covers the first 2 (bytes [0,4)).
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);
	let offsets = opt_array::<U1>(&[Some(0)]);
	let ends = opt_array::<U1>(&[Some(4)]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	assert_eq!(spans.get_size::<0>().unwrap(), 2);
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(2)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), None); // stops at byte_end
}

///////////////////////////////////////////////////////////////////////////
// ---------------- Multiple spans over a shared stream ---------------- //
///////////////////////////////////////////////////////////////////////////

#[test]
fn multiple_spans_over_shared_stream_are_independent() {
	let mut stream = DummyBinaryStream::new(vec![
		0x00, 0x01, 0x00, 0x02, // span0: elems 1, 2
		0x00, 0x03, 0x00, 0x04, // span1: elems 3, 4
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(4)]);
	let ends = opt_array::<U2>(&[Some(4), Some(8)]);
	let mut spans = BinaryElemSpans::<TestElem, _, U2>::new(&mut stream, offsets, ends);

	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
	assert_eq!(spans.read_next_elem::<1>().unwrap(), Some(TestElem(3)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(2)));
	assert_eq!(spans.read_next_elem::<1>().unwrap(), Some(TestElem(4)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), None);
	assert_eq!(spans.read_next_elem::<1>().unwrap(), None);
}

#[test]
fn multiple_spans_track_positions_independently() {
	let mut stream = DummyBinaryStream::new(vec![
		0x00, 0x01, 0x00, 0x02, 0x00, 0x03, // span0: 3 elems
		0x00, 0x09, 0x00, 0x08,             // span1: 2 elems
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(6)]);
	let ends = opt_array::<U2>(&[Some(6), Some(10)]);
	let mut spans = BinaryElemSpans::<TestElem, _, U2>::new(&mut stream, offsets, ends);

	spans.read_next_elem::<0>().unwrap();
	spans.read_next_elem::<0>().unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
	assert_eq!(spans.get_pos::<1>().unwrap(), 0); // untouched

	spans.set_pos::<1>(1).unwrap();
	assert_eq!(spans.read_next_elem::<1>().unwrap(), Some(TestElem(8)));
	assert_eq!(spans.get_pos::<0>().unwrap(), 2); // unaffected by span1's seek
}

//////////////////////////////////////////////
// ---------------- Output ---------------- //
//////////////////////////////////////////////

#[test]
fn write_next_elem_writes_at_correct_offset() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 6]);
	let offsets = opt_array::<U1>(&[Some(2)]); // span starts 2 bytes in
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.write_next_elem::<0>(TestElem(0x0102)).unwrap();
	spans.write_next_elem::<0>(TestElem(0x0304)).unwrap();

	assert_eq!(stream.get_all(), &[0x00, 0x00, 0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn write_next_elem_advances_local_pos() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.write_next_elem::<0>(TestElem(1)).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 1);
	spans.write_next_elem::<0>(TestElem(2)).unwrap();
	assert_eq!(spans.get_pos::<0>().unwrap(), 2);
}

#[test]
fn write_then_read_back_roundtrip() {
	let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.write_next_elem::<0>(TestElem(42)).unwrap();
	spans.write_next_elem::<0>(TestElem(99)).unwrap();
	spans.rewind::<0>().unwrap();

	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(42)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(99)));
}

#[test]
fn truncate_shrinks_span_element_count() {
	let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]); // 3 elems
	let offsets = opt_array::<U1>(&[None]);
	let ends = opt_array::<U1>(&[None]);
	let mut spans = BinaryElemSpans::<TestElem, _, U1>::new(&mut stream, offsets, ends);

	spans.truncate::<0>(1).unwrap();

	assert_eq!(spans.get_size::<0>().unwrap(), 1);
	spans.rewind::<0>().unwrap();
	assert_eq!(spans.read_next_elem::<0>().unwrap(), Some(TestElem(1)));
	assert_eq!(spans.read_next_elem::<0>().unwrap(), None);
}

#[test]
fn truncate_on_one_span_does_not_affect_sibling_span() {
	let mut stream = DummyBinaryStream::new(vec![
		0x00, 0x01, 0x00, 0x02, 0x00, 0x03, // span0: 3 elems, bytes [0,6)
		0x00, 0x09, 0x00, 0x08,             // span1: 2 elems, bytes [6,10)
	]);
	let offsets = opt_array::<U2>(&[Some(0), Some(6)]);
	let ends = opt_array::<U2>(&[Some(6), Some(10)]);
	let mut spans = BinaryElemSpans::<TestElem, _, U2>::new(&mut stream, offsets, ends);

	spans.truncate::<0>(1).unwrap();

	assert_eq!(spans.get_size::<0>().unwrap(), 1);
	assert_eq!(spans.get_size::<1>().unwrap(), 2);
}
