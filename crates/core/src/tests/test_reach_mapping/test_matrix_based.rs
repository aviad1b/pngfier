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
