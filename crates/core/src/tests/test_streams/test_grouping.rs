use generic_array::GenericArray;
use generic_array::typenum::U2;

use super::super::super::streams::grouping::*;
use super::super::super::streams::traits::*;
use super::super::super::streams::dummy::*;

//////////////////////////////////////////////////////
// ---------- GroupedElemStreams (input) ---------- //
//////////////////////////////////////////////////////

#[test]
fn grouped_elem_streams_read_from_correct_index() {
	let mut s0 = DummyInputElemStream::new(vec![1, 2, 3]);
	let mut s1 = DummyInputElemStream::new(vec![10, 20, 30]);
	let arr: GenericArray<&mut DummyInputElemStream<i32>, U2> =
		GenericArray::from_iter([&mut s0, &mut s1]);
	let mut grouped = GroupedElemStreams::new(arr);

	assert_eq!(grouped.read_next_elem::<0>().unwrap(), Some(1));
	assert_eq!(grouped.read_next_elem::<1>().unwrap(), Some(10));
	assert_eq!(grouped.read_next_elem::<0>().unwrap(), Some(2));
	assert_eq!(grouped.read_next_elem::<1>().unwrap(), Some(20));
}

#[test]
fn grouped_elem_streams_stream_methods_are_per_index() {
	let mut s0 = DummyInputElemStream::new(vec![1, 2, 3]);
	let mut s1 = DummyInputElemStream::new(vec![10, 20]);
	let arr: GenericArray<&mut DummyInputElemStream<i32>, U2> =
		GenericArray::from_iter([&mut s0, &mut s1]);
	let mut grouped = GroupedElemStreams::new(arr);

	assert_eq!(grouped.get_size::<0>().unwrap(), 3);
	assert_eq!(grouped.get_size::<1>().unwrap(), 2);

	grouped.set_pos::<0>(2).unwrap();
	assert_eq!(grouped.get_pos::<0>().unwrap(), 2);
	assert_eq!(grouped.get_pos::<1>().unwrap(), 0); // <1> should be unaffected from <0>

	assert_eq!(grouped.read_next_elem::<0>().unwrap(), Some(3));

	grouped.rewind::<0>().unwrap();
	assert_eq!(grouped.get_pos::<0>().unwrap(), 0);
}

///////////////////////////////////////////////////////
// ---------- GroupedElemStreams (output) ---------- //
///////////////////////////////////////////////////////

#[test]
fn grouped_elem_streams_write_to_correct_index() {
	let mut s0 = DummyOutputElemStream::new(vec![0; 2]);
	let mut s1 = DummyOutputElemStream::new(vec![0; 2]);
	let arr: GenericArray<&mut DummyOutputElemStream<i32>, U2> =
		GenericArray::from_iter([&mut s0, &mut s1]);
	let mut grouped = GroupedElemStreams::new(arr);

	grouped.write_next_elem::<0>(11).unwrap();
	grouped.write_next_elem::<1>(99).unwrap();
	grouped.write_next_elem::<0>(22).unwrap();

	assert_eq!(s0.get_all(), &[11, 22]);
	assert_eq!(s1.get_all(), &[99, 0]);
}

#[test]
fn grouped_elem_streams_truncate_only_affects_target_index() {
	let mut s0 = DummyOutputElemStream::new(vec![1, 2, 3]);
	let mut s1 = DummyOutputElemStream::new(vec![4, 5, 6]);
	let arr: GenericArray<&mut DummyOutputElemStream<i32>, U2> =
		GenericArray::from_iter([&mut s0, &mut s1]);
	let mut grouped = GroupedElemStreams::new(arr);

	grouped.truncate::<0>(1).unwrap();

	assert_eq!(s0.get_all(), &[1]);
	assert_eq!(s1.get_all(), &[4, 5, 6]);
}

///////////////////////////////////////////////
// ---------- UngroupedElemStream ---------- //
///////////////////////////////////////////////

#[test]
fn ungrouped_elem_stream_reads_only_its_index() {
	let mut s0 = DummyInputElemStream::new(vec![1, 2]);
	let mut s1 = DummyInputElemStream::new(vec![100, 200]);
	let arr: GenericArray<&mut DummyInputElemStream<i32>, U2> =
		GenericArray::from_iter([&mut s0, &mut s1]);
	let mut grouped = GroupedElemStreams::new(arr);

	let mut ungrouped_0 = UngroupedElemStream::<0, _, _, _>::new(&mut grouped);
	assert_eq!(ungrouped_0.read_next_elem().unwrap(), Some(1));
	assert_eq!(ungrouped_0.read_next_elem().unwrap(), Some(2));
	assert_eq!(ungrouped_0.read_next_elem().unwrap(), None);
}
