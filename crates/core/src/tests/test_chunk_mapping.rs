use crate::{
    elems::{Elem, RuntimeElemIndexesMatrix},
    streams::dummy::DummyInputElemStream,
};

use super::super::chunks::{ChunkInfo, ChunkSize, mapping::{self::*, reach::*}};

// Total size of Reference/Literal chunks must exactly equal data.len(),
// with no gaps or overlaps, regardless of which chunk types were chosen.
fn total_covered<E: Elem>(chunks: &[ChunkInfo<E>]) -> ChunkSize {
	chunks.iter().map(|c| match c {
		ChunkInfo::Reference { size, .. } => *size,
		ChunkInfo::Literal(elems) => elems.len() as ChunkSize,
	}).sum()
}

fn run_mapper(image_bytes: &[u8], data_bytes: &[u8], min_cap: Option<ChunkSize>,
			  max_cap: Option<ChunkSize>) -> Vec<ChunkInfo<u8>> {
	let mut image = DummyInputElemStream::new(image_bytes.to_vec());
	let mut data = DummyInputElemStream::new(data_bytes.to_vec());
	let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();
	let mut reach = MatrixBasedReachMapper::new(&mut image, &mut data, &mut img_matrix).unwrap();
	let mut mapper = ChunkMapper::new(&mut reach);
	mapper.map_chunks(min_cap, max_cap).unwrap()
}

/////////////////////////
// --- Basic shape --- //
/////////////////////////

#[test]
fn perfect_full_match_is_one_reference_chunk() {
	let chunks = run_mapper(b"ABCD", b"ABCD", None, None);
	assert_eq!(chunks, vec![ChunkInfo::Reference { index: 0, size: 4 }]);
}

#[test]
fn completely_unmatched_data_is_one_literal_chunk() {
	// no byte in `data` appears anywhere in `image`
	let chunks = run_mapper(b"AAAA", b"ZZZZ", None, None);
	assert_eq!(chunks, vec![ChunkInfo::Literal(b"ZZZZ".to_vec())]);
}

#[test]
fn empty_data_produces_no_chunks() {
	let chunks = run_mapper(b"ABCD", b"", None, None);
	assert!(chunks.is_empty());
}

#[test]
fn empty_image_forces_everything_literal() {
	let chunks = run_mapper(b"", b"ABCD", None, None);
	assert_eq!(chunks, vec![ChunkInfo::Literal(b"ABCD".to_vec())]);
}

///////////////////////////////////////////////////////////////////////////////////////////
// --- Testing for specific scenario with a few edge cases (see reach mapping tests) --- //
// image: A B C D A B E F A B C D   (0..12)                                              //
// data:  A B E F A B C D Z A B C D D A  (0..15)                                         //
///////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn mixed_reference_and_literal_matches_expected_shape() {
	let image = b"ABCDABEFABCD";
	let data = b"ABEFABCDZABCDDA";
	let chunks = run_mapper(image, data, None, None);

	// overage must exactly equal data.len() first, regardless of any
	// tie-broken src_start values below
	assert_eq!(total_covered(&chunks), data.len() as ChunkSize);

	// shape: Reference(8) at src 4, Literal("Z"), Reference(4) at src 0 or 8,
	// Reference(2) at src 3
	assert_eq!(chunks.len(), 4);

	assert_eq!(chunks[0], ChunkInfo::Reference { index: 4, size: 8 });
	assert_eq!(chunks[1], ChunkInfo::Literal(vec![b'Z']));

	match &chunks[2] {
		ChunkInfo::Reference { index, size } => {
			assert!(*index == 0 || *index == 8, "unexpected src_start {index}");
			assert_eq!(*size, 4);
		}
		other => panic!("expected Reference, got {other:?}"),
	}

	assert_eq!(chunks[3], ChunkInfo::Reference { index: 3, size: 2 });
}

/////////////////////
// --- min_cap --- //
/////////////////////

#[test]
fn min_cap_rejects_short_matches_in_favor_of_literals() {
	// "AB" occurs in image but the match is only 2 long; reject anything <= 2
	let image = b"XXABYY";
	let data = b"AB";
	let chunks = run_mapper(image, data, Some(2), None);

	// size == min_cap is rejected (strict `<` semantics), so this must fall
	// back to per-element literals, not a Reference
	assert_eq!(chunks, vec![
		ChunkInfo::Literal(vec![b'A']),
		ChunkInfo::Literal(vec![b'B']),
	]);
}

#[test]
fn min_cap_allows_matches_strictly_above_threshold() {
	let image = b"XXABCYY";
	let data = b"ABC";
	let chunks = run_mapper(image, data, Some(2), None);
	assert_eq!(chunks, vec![ChunkInfo::Reference { index: 2, size: 3 }]);
}

#[test]
fn min_cap_boundary_is_exclusive() {
	// size == 3, min_cap == 3: must be rejected (strictly-greater-than semantics)
	let image = b"XXABCYY";
	let data = b"ABC";
	let chunks = run_mapper(image, data, Some(3), None);
	assert!(chunks.iter().all(|c| matches!(c, ChunkInfo::Literal(_))));
	assert_eq!(total_covered(&chunks), 3);
}

/////////////////////
// --- max_cap --- //
/////////////////////

#[test]
fn max_cap_splits_long_matches_into_multiple_references() {
	let image = b"ABCDEFGH";
	let data = b"ABCDEFGH"; // full 8-byte match
	let chunks = run_mapper(image, data, None, Some(3));

	assert_eq!(chunks, vec![
		ChunkInfo::Reference { index: 0, size: 3 },
		ChunkInfo::Reference { index: 3, size: 3 },
		ChunkInfo::Reference { index: 6, size: 2 }, // remainder, not re-checked against min_cap
	]);
	assert_eq!(total_covered(&chunks), 8);
}

#[test]
fn max_cap_exact_multiple_has_no_small_remainder() {
	let image = b"ABCDEF";
	let data = b"ABCDEF";
	let chunks = run_mapper(image, data, None, Some(2));
	assert_eq!(chunks, vec![
		ChunkInfo::Reference { index: 0, size: 2 },
		ChunkInfo::Reference { index: 2, size: 2 },
		ChunkInfo::Reference { index: 4, size: 2 },
	]);
}

#[test]
fn min_cap_and_max_cap_combined() {
	// long match gets split by max_cap; each split piece is allowed through
	// even though individually some pieces would be <= min_cap,
	// since the *whole* run cleared min_cap before splitting 
	let image = b"ABCDEFGH";
	let data = b"ABCDEFGH";
	let chunks = run_mapper(image, data, Some(1), Some(3));
	assert_eq!(chunks, vec![
		ChunkInfo::Reference { index: 0, size: 3 },
		ChunkInfo::Reference { index: 3, size: 3 },
		ChunkInfo::Reference { index: 6, size: 2 },
	]);
}

/////////////////////////////////////////////////
// --- Trailing / end-of-stream edge cases --- //
/////////////////////////////////////////////////

#[test]
fn short_match_ending_exactly_at_data_eof() {
	// match exists but there's nothing left in `data` after the seed pair
	// for walk_paths to extend through - must not be dropped
	let image = b"XXABYY";
	let data = b"ZZAB"; // "AB" is the last two bytes of data
	let chunks = run_mapper(image, data, None, None);
	assert_eq!(total_covered(&chunks), 4);

	// last chunk must be the AB reference, not swallowed by the leading literal(s)
	assert_eq!(chunks.last().unwrap(), &ChunkInfo::Reference { index: 2, size: 2 });
}

#[test]
fn single_final_byte_with_no_pair_is_literal() {
	// the very last byte of `data` can never seed a bigram match, regardless
	// of whether that byte's value exists in `image`
	let image = b"AAAA";
	let data = b"BA"; // trailing 'A' has no `curr` to pair with
	let chunks = run_mapper(image, data, None, None);
	assert_eq!(total_covered(&chunks), 2);
    match chunks.last().unwrap() {
        ChunkInfo::Literal(elems) => assert_eq!(*elems.last().unwrap(), b'A'),
        _ => panic!("Expected last element being 'A'")
    };
}

//////////////////////////////////////////////////////
// --- Bigram-specific vs single-byte existence --- //
//////////////////////////////////////////////////////

#[test]
fn byte_present_but_never_in_this_pair_is_literal() {
	// 'D' exists in image, 'Z' exists nowhere - but even swap 'Z' for a byte
	// that DOES exist individually, just never adjacent to 'D':
	let image = b"ABCD"; // no "DA" bigram here
	let data = b"XXDA"; // "DA" pair never occurs in image
	let chunks = run_mapper(image, data, None, None);
	// 'D' and 'A' both individually exist in `image`, but never as an
	// adjacent pair - must not be treated as a match
	assert!(total_covered(&chunks) == 4);
}

////////////////////////////////////////////////////////////
// --- General invariant, run across everything above --- //
////////////////////////////////////////////////////////////

#[test]
fn coverage_invariant_holds_across_varied_inputs() {
	let cases: &[(&[u8], &[u8])] = &[
		(b"ABCDABEFABCD", b"ABEFABCDZABCDDA"),
		(b"", b"HELLO"),
		(b"HELLO", b""),
		(b"AAAAAAAA", b"AAAAAAAA"),
		(b"XYZ", b"ABCXYZABC"),
	];
	for (image, data) in cases {
		let chunks = run_mapper(image, data, None, None);
		assert_eq!(
			total_covered(&chunks), data.len() as ChunkSize,
			"coverage mismatch for image={image:?} data={data:?}"
		);
	}
}
