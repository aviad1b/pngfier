use std::{collections::HashSet, hash::Hash};

use crate::{chunks::ChunkIndex, elems::{ElemIndexesMatrix, ElemIndexesMatrixSlot, RuntimeElemIndexesMatrix}, streams::dummy::DummyInputElemStream};

use super::super::super::chunks::mapping::{reach::*, reach_utils::*};

fn slot_to_set<I: Eq + Copy + Hash, Slot: ElemIndexesMatrixSlot<I>>(slot: Slot) -> HashSet<I> {
    let mut res = HashSet::new();
    for elem in slot.iter().unwrap() {
        res.insert(*elem);
    }
    res
}

fn paths_src_starts<'a, T>(paths: T) -> HashSet<ChunkIndex>
where
    T: Iterator<Item = &'a Path>
{
    HashSet::from_iter(paths.map(|path| path.src_start))
}

fn paths_from_starts(src_starts: Vec<ChunkIndex>) -> Vec<Path> {
    src_starts.iter().map(|src_start| Path { src_start: *src_start, len: 2 }).collect()
}

#[test]
fn test_init_img_matrix() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();

    init_img_matrix(&mut image, &mut img_matrix).unwrap();

    assert_eq!(slot_to_set(img_matrix.at(b'A', b'A').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'C').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'D').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'E').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'F').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'A').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'B').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'D').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'F').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'A').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'B').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'C').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'E').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'F').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'B').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'C').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'D').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'E').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'F').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'A').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'B').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'C').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'D').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'E').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'B').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'C').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'D').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'E').unwrap()), HashSet::new());
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'F').unwrap()), HashSet::new());
    
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'B').unwrap()), HashSet::from_iter(vec![0, 4, 8]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'C').unwrap()), HashSet::from_iter(vec![1, 9]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'E').unwrap()), HashSet::from_iter(vec![5]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'D').unwrap()), HashSet::from_iter(vec![2, 10]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'A').unwrap()), HashSet::from_iter(vec![3]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'F').unwrap()), HashSet::from_iter(vec![6]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'A').unwrap()), HashSet::from_iter(vec![7]));
}

#[test]
fn test_get_path_starts_vec() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();
    init_img_matrix(&mut image, &mut img_matrix).unwrap();

    let mut data = DummyInputElemStream::new(b"ABEFABCDZABCDDA".to_vec());

    let paths = get_path_starts_vec(&mut data, &img_matrix, 0).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![0, 4, 8]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 1).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![5]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 2).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![6]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 3).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![7]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 4).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![0, 4, 8]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 5).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![1, 9]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 6).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![2, 10]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 7).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 8).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 9).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![0, 4, 8]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 10).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![1, 9]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 11).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![2, 10]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 12).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 13).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![3]));

    let paths = get_path_starts_vec(&mut data, &img_matrix, 14).unwrap();
    assert_eq!(paths_src_starts(paths.iter()), HashSet::from_iter(vec![]));
}

#[test]
fn test_walk_paths() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();
    init_img_matrix(&mut image, &mut img_matrix).unwrap();

    let mut data = DummyInputElemStream::new(b"ABEFABCDZABCDDA".to_vec());
    
    let path = walk_paths(&mut data, &img_matrix, 0, paths_from_starts(vec![0, 4, 8])).unwrap();
    assert_eq!(path, Some(Path { src_start: 4, len: 8 }));

    let path = walk_paths(&mut data, &img_matrix, 1, paths_from_starts(vec![5])).unwrap();
    assert_eq!(path, Some(Path { src_start: 5, len: 7 }));

    let path = walk_paths(&mut data, &img_matrix, 2, paths_from_starts(vec![6])).unwrap();
    assert_eq!(path, Some(Path { src_start: 6, len: 6 }));

    let path = walk_paths(&mut data, &img_matrix, 3, paths_from_starts(vec![7])).unwrap();
    assert_eq!(path, Some(Path { src_start: 7, len: 5 }));

    let path = walk_paths(&mut data, &img_matrix, 4, paths_from_starts(vec![0, 4, 8])).unwrap();
    assert!(
        path == Some(Path { src_start: 0, len: 4 }) || path == Some(Path { src_start: 8, len: 4 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 5, paths_from_starts(vec![1, 9])).unwrap();
    assert!(
        path == Some(Path { src_start: 1, len: 3 }) || path == Some(Path { src_start: 9, len: 3 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 6, paths_from_starts(vec![2, 10])).unwrap();
    assert!(
        path == Some(Path { src_start: 2, len: 2 }) || path == Some(Path { src_start: 10, len: 2 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 7, paths_from_starts(vec![])).unwrap();
    assert_eq!(path, None);

    let path = walk_paths(&mut data, &img_matrix, 8, paths_from_starts(vec![])).unwrap();
    assert_eq!(path, None);

    let path = walk_paths(&mut data, &img_matrix, 9, paths_from_starts(vec![0, 4, 8])).unwrap();
    assert!(
        path == Some(Path { src_start: 0, len: 4 }) || path == Some(Path { src_start: 8, len: 4 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 10, paths_from_starts(vec![1, 9])).unwrap();
    assert!(
        path == Some(Path { src_start: 1, len: 3 }) || path == Some(Path { src_start: 9, len: 3 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 11, paths_from_starts(vec![2, 10])).unwrap();
    assert!(
        path == Some(Path { src_start: 2, len: 2 }) || path == Some(Path { src_start: 10, len: 2 }),
        "Unexpected path: {:?}", path
    );

    let path = walk_paths(&mut data, &img_matrix, 12, paths_from_starts(vec![])).unwrap();
    assert_eq!(path, None);

    let path = walk_paths(&mut data, &img_matrix, 13, paths_from_starts(vec![3])).unwrap();
    assert_eq!(path, Some(Path { src_start: 3, len: 2 }));

    let path = walk_paths(&mut data, &img_matrix, 14, paths_from_starts(vec![])).unwrap();
    assert_eq!(path, None);
}

#[test]
fn reach_mapping_roundtrip() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut data = DummyInputElemStream::new(b"ABEFABCDZABCDDA".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();

    let reach = MatrixBasedReachMapper::new(&mut image, &mut data, &mut img_matrix).unwrap();

    // 	match ABEFABCD, only one candidate
    let info = reach.get(0).unwrap();
    assert_eq!(info.src_start, 4);
    assert_eq!(info.reach, 8);

    // same match as previous, one position later
    let info = reach.get(1).unwrap();
    assert_eq!(info.src_start, 5);
    assert_eq!(info.reach, 8);

    // same match as previous, one position later
    let info = reach.get(2).unwrap();
    assert_eq!(info.src_start, 6);
    assert_eq!(info.reach, 8);

    // same match as previous, one position later. length 2 (no walk)
    let info = reach.get(3).unwrap();
    assert_eq!(info.src_start, 7);
    assert_eq!(info.reach, 8);

    // match ABCD, two potential candidates (tie)
    let info = reach.get(4).unwrap();
    assert!(
        info.src_start == 0 || info.src_start == 8,
        "Unexpected src_start: {} (expected 0 or 8)", info.src_start
    );
    assert_eq!(info.reach, 8);

    // same match as previous, one position later
    let info = reach.get(5).unwrap();
    assert!(
        info.src_start == 1 || info.src_start == 9,
        "Unexpected src_start: {} (expected 1 or 9)", info.src_start
    );
    assert_eq!(info.reach, 8);

    // same match as previous, one position later. length 2 (no walk)
    let info = reach.get(6).unwrap();
    assert!(
        info.src_start == 2 || info.src_start == 10,
        "Unexpected src_start: {} (expected 2 or 10)", info.src_start
    );
    assert_eq!(info.reach, 8);

    // no match
    let info = reach.get(7).unwrap();
    assert!(info.src_start < 0); // no src_start index is applicable
    assert_eq!(info.reach, 7);

    // no match
    let info = reach.get(8).unwrap();
    assert!(info.src_start < 0); // no src_start index is applicable
    assert_eq!(info.reach, 8);

    // match ABCD, two potential candidates (tie)
    let info = reach.get(9).unwrap();
    assert!(
        info.src_start == 0 || info.src_start == 8,
        "Unexpected src_start: {} (expected 0 or 8)", info.src_start
    );
    assert_eq!(info.reach, 13);

    // same match as previous, one position later
    let info = reach.get(10).unwrap();
    assert!(
        info.src_start == 1 || info.src_start == 9,
        "Unexpected src_start: {} (expected 1 or 9)", info.src_start
    );
    assert_eq!(info.reach, 13);

    // same match as previous, one position later. length 2 (no walk)
    let info = reach.get(11).unwrap();
    assert!(
        info.src_start == 2 || info.src_start == 10,
        "Unexpected src_start: {} (expected 2 or 10)", info.src_start
    );
    assert_eq!(info.reach, 13);

    // no match
    let info = reach.get(12).unwrap();
    assert!(info.src_start < 0); // no src_start index is applicable
    assert_eq!(info.reach, 12);

    // match DA, length 2 (no walk) right before end-of-stream
    let info = reach.get(13).unwrap();
    assert_eq!(info.src_start, 3);
    assert_eq!(info.reach, 15);

    // no match (end-of-stream)
    let info = reach.get(14).unwrap();
    assert!(info.src_start < 0); // no src_start index is applicable
    assert_eq!(info.reach, 14);
}

#[test]
fn reach_mapping_gives_correct_length() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut data = DummyInputElemStream::new(b"ABEFABCDZABCDDA".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();

    let reach = MatrixBasedReachMapper::new(&mut image, &mut data, &mut img_matrix).unwrap();

    assert_eq!(reach.len().unwrap(), 15);
}
