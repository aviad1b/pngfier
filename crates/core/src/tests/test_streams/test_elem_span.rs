use generic_array::{GenericArray, typenum::U2};

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

////////////////////////////////////////////////////////////////////////
// ---------------- Construction / whole-stream span ---------------- //
////////////////////////////////////////////////////////////////////////

#[test]
fn span_with_no_bounds_covers_whole_stream() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]); // elems 1, 2, 3
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    assert_eq!(span.get_size().unwrap(), 3);
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(2)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(3)));
    assert_eq!(span.read_next_elem().unwrap(), None);
}

///////////////////////////////////////////////////////////////
// ---------------- Stream trait forwarding ---------------- //
///////////////////////////////////////////////////////////////

#[test]
fn get_pos_starts_at_zero_and_advances_on_read() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    assert_eq!(span.get_pos().unwrap(), 0);
    span.read_next_elem().unwrap();
    assert_eq!(span.get_pos().unwrap(), 1);
    span.read_next_elem().unwrap();
    assert_eq!(span.get_pos().unwrap(), 2);
}

#[test]
fn set_pos_seeks_to_element_index() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.set_pos(2).unwrap();
    assert_eq!(span.get_pos().unwrap(), 2);
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(3)));
}

#[test]
fn rewind_resets_to_start() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.read_next_elem().unwrap();
    span.rewind().unwrap();
    assert_eq!(span.get_pos().unwrap(), 0);
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
}

///////////////////////////////////////////////////////////////
// ---------------- Byte-range bounded span ---------------- //
///////////////////////////////////////////////////////////////

#[test]
fn explicit_offset_skips_leading_bytes() {
    let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x00, 0x01, 0x00, 0x02]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, Some(2), None);

    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(2)));
    assert_eq!(span.read_next_elem().unwrap(), None);
}

#[test]
fn explicit_end_stops_before_trailing_bytes() {
    // stream has 4 elements worth of data; span only covers the first 2 (bytes [0,4))
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, Some(0), Some(4));

    assert_eq!(span.get_size().unwrap(), 2);
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(2)));
    assert_eq!(span.read_next_elem().unwrap(), None); // stops at byte_end
}

#[test]
fn explicit_offset_and_end_bound_a_middle_slice() {
    let mut stream = DummyBinaryStream::new(vec![
        0xFF, 0xFF,             // leading junk, excluded
        0x00, 0x01, 0x00, 0x02, // the span: elems 1, 2
        0xFF, 0xFF,             // trailing junk, excluded
    ]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, Some(2), Some(6));

    assert_eq!(span.get_size().unwrap(), 2);
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(2)));
    assert_eq!(span.read_next_elem().unwrap(), None);
}

//////////////////////////////////////////////
// ---------------- Output ---------------- //
//////////////////////////////////////////////

#[test]
fn write_next_elem_writes_at_correct_offset_in_underlying_stream() {
    let mut stream = DummyBinaryStream::new(vec![0x00; 6]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, Some(2), None);

    span.write_next_elem(TestElem(0x0102)).unwrap();
    span.write_next_elem(TestElem(0x0304)).unwrap();

    assert_eq!(stream.get_all(), &[0x00, 0x00, 0x01, 0x02, 0x03, 0x04]);
}

#[test]
fn write_next_elem_advances_pos() {
    let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.write_next_elem(TestElem(1)).unwrap();
    assert_eq!(span.get_pos().unwrap(), 1);
    span.write_next_elem(TestElem(2)).unwrap();
    assert_eq!(span.get_pos().unwrap(), 2);
}

#[test]
fn write_then_rewind_then_read_back_roundtrip() {
    let mut stream = DummyBinaryStream::new(vec![0x00; 4]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.write_next_elem(TestElem(42)).unwrap();
    span.write_next_elem(TestElem(99)).unwrap();
    span.rewind().unwrap();

    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(42)));
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(99)));
}

#[test]
fn truncate_shrinks_span_element_count() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03]); // 3 elems
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.truncate(1).unwrap();

    assert_eq!(span.get_size().unwrap(), 1);
    span.rewind().unwrap();
    assert_eq!(span.read_next_elem().unwrap(), Some(TestElem(1)));
    assert_eq!(span.read_next_elem().unwrap(), None);
}

#[test]
fn truncate_to_zero_empties_span() {
    let mut stream = DummyBinaryStream::new(vec![0x00, 0x01, 0x00, 0x02]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, None, None);

    span.truncate(0).unwrap();

    assert_eq!(span.get_size().unwrap(), 0);
    assert_eq!(span.read_next_elem().unwrap(), None);
}

//////////////////////////////////////////////////////////////
// ---------------- Positions sanity check ---------------- //
//////////////////////////////////////////////////////////////

#[test]
fn span_at_nonzero_offset_reports_local_not_global_pos() {
    // checking that BinaryElemSpan reports *local* element position
    // (relative to its own offset), not the underlying stream's
    // raw byte/element position
    let mut stream = DummyBinaryStream::new(vec![0xFF, 0xFF, 0x00, 0x01, 0x00, 0x02]);
    let mut span = BinaryElemSpan::<TestElem, _>::new(&mut stream, Some(2), None);

    assert_eq!(span.get_pos().unwrap(), 0); // local pos, not byte offset 2
    span.read_next_elem().unwrap();
    assert_eq!(span.get_pos().unwrap(), 1);
}
