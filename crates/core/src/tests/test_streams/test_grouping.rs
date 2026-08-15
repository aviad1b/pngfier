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
	let mut grouped = GroupedElemStreams::<i32, U2, _>::new(arr);

	assert_eq!(grouped.read_next_elem::<0>().unwrap(), Some(1));
	assert_eq!(grouped.read_next_elem::<1>().unwrap(), Some(10));
	assert_eq!(grouped.read_next_elem::<0>().unwrap(), Some(2));
	assert_eq!(grouped.read_next_elem::<1>().unwrap(), Some(20));
}
