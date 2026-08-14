use super::super::super::streams::traits::*;
use super::super::super::streams::dummy::*;

////////////////////////////////////////////////
// ---------- DummyInputElemStream ---------- //
////////////////////////////////////////////////

#[test]
fn input_new_starts_at_pos_zero() {
    let mut s = DummyInputElemStream::new(vec![10, 20, 30]);
    assert_eq!(s.get_pos().unwrap(), 0);
    assert_eq!(s.get_size().unwrap(), 3);
}

#[test]
fn input_reads_elements_in_order() {
    let mut s = DummyInputElemStream::new(vec![1, 2, 3]);
    assert_eq!(s.read_next_elem().unwrap(), Some(1));
    assert_eq!(s.read_next_elem().unwrap(), Some(2));
    assert_eq!(s.read_next_elem().unwrap(), Some(3));
}

#[test]
fn input_returns_none_at_end_of_stream() {
    let mut s = DummyInputElemStream::new(vec![1, 2]);
    s.read_next_elem().unwrap();
    s.read_next_elem().unwrap();
    assert_eq!(s.read_next_elem().unwrap(), None);
}

#[test]
fn input_empty_stream_reads_none_immediately() {
    let mut s: DummyInputElemStream<i32> = DummyInputElemStream::new(vec![]);
    assert_eq!(s.get_size().unwrap(), 0);
    assert_eq!(s.read_next_elem().unwrap(), None);
}

#[test]
fn input_pos_advances_with_each_read() {
    let mut s = DummyInputElemStream::new(vec![5, 6, 7]);
    assert_eq!(s.get_pos().unwrap(), 0);
    s.read_next_elem().unwrap();
    assert_eq!(s.get_pos().unwrap(), 1);
    s.read_next_elem().unwrap();
    assert_eq!(s.get_pos().unwrap(), 2);
}

#[test]
fn input_rewind_resets_pos_and_allows_reread() {
    let mut s = DummyInputElemStream::new(vec![1, 2, 3]);
    s.read_next_elem().unwrap();
    s.read_next_elem().unwrap();
    s.rewind().unwrap();
    assert_eq!(s.get_pos().unwrap(), 0);
    assert_eq!(s.read_next_elem().unwrap(), Some(1));
}

#[test]
fn input_set_pos_seeks_to_arbitrary_position() {
    let mut s = DummyInputElemStream::new(vec![10, 20, 30, 40]);
    s.set_pos(2).unwrap();
    assert_eq!(s.get_pos().unwrap(), 2);
    assert_eq!(s.read_next_elem().unwrap(), Some(30));
}

#[test]
fn input_set_pos_past_end_yields_none_on_read() {
    let mut s = DummyInputElemStream::new(vec![1, 2, 3]);
    s.set_pos(100).unwrap();
    assert_eq!(s.read_next_elem().unwrap(), None);
}

/////////////////////////////////////////////////
// ---------- DummyOutputElemStream ---------- //
/////////////////////////////////////////////////

#[test]
fn output_new_starts_at_pos_zero() {
    let mut s = DummyOutputElemStream::new(vec![0; 3]);
    assert_eq!(s.get_pos().unwrap(), 0);
    assert_eq!(s.get_size().unwrap(), 3);
}

#[test]
fn output_writes_elements_in_order() {
    let mut s = DummyOutputElemStream::new(vec![0; 3]);
    s.write_next_elem(11).unwrap();
    s.write_next_elem(22).unwrap();
    s.write_next_elem(33).unwrap();
    assert_eq!(s.get_all(), &[11, 22, 33]);
}

#[test]
fn output_pos_advances_with_each_write() {
    let mut s = DummyOutputElemStream::new(vec![0; 2]);
    s.write_next_elem(1).unwrap();
    assert_eq!(s.get_pos().unwrap(), 1);
    s.write_next_elem(2).unwrap();
    assert_eq!(s.get_pos().unwrap(), 2);
}

#[test]
fn output_write_past_capacity() {
    let mut s = DummyOutputElemStream::new(vec![0; 1]);
    s.write_next_elem(1).unwrap();
    s.write_next_elem(2).unwrap(); // out of initial bounds
}

#[test]
fn output_get_all_reflects_underlying_buffer() {
    let s = DummyOutputElemStream::new(vec![7, 8, 9]);
    assert_eq!(s.get_all(), &[7, 8, 9]);
}

#[test]
fn output_truncate_to_zero_clears_elems() {
    let mut s = DummyOutputElemStream::new(vec![1, 2, 3]);
    s.truncate(0).unwrap();
    assert_eq!(s.get_all(), &[] as &[i32]);
    assert_eq!(s.get_size().unwrap(), 0);
}

#[test]
fn output_truncate_shrinks_elems() {
    let mut s = DummyOutputElemStream::new(vec![1, 2, 3, 4]);
    s.truncate(2).unwrap();
    assert_eq!(s.get_all(), &[1, 2]);
}

#[test]
fn output_truncate_grows_elems() {
    let mut s = DummyOutputElemStream::new(vec![9, 2, 3]);
    s.truncate(5).unwrap();
    assert_eq!(&s.get_all()[0..=2], &[9, 2, 3]);
    assert_eq!(s.get_size().unwrap(), 5);
}

#[test]
fn output_rewind_resets_pos() {
    let mut s = DummyOutputElemStream::new(vec![0; 2]);
    s.write_next_elem(1).unwrap();
    s.rewind().unwrap();
    assert_eq!(s.get_pos().unwrap(), 0);
}

#[test]
fn output_set_pos_seeks_and_overwrites() {
    let mut s = DummyOutputElemStream::new(vec![1, 2, 3]);
    s.set_pos(1).unwrap();
    s.write_next_elem(99).unwrap();
    assert_eq!(s.get_all(), &[1, 99, 3]);
}
