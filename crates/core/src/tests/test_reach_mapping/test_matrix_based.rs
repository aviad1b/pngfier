use std::{collections::HashSet, hash::Hash};

use crate::{elems::{ElemIndexesMatrix, ElemIndexesMatrixSlot, RuntimeElemIndexesMatrix}, streams::dummy::DummyInputElemStream};

use super::super::super::chunks::mapping::{reach::*, reach_utils::*};

fn slot_to_set<I: Eq + Copy + Hash, Slot: ElemIndexesMatrixSlot<I>>(slot: Slot) -> HashSet<I> {
    let mut res = HashSet::new();
    for elem in slot.iter().unwrap() {
        res.insert(*elem);
    }
    res
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
    
    assert_eq!(slot_to_set(img_matrix.at(b'A', b'B').unwrap()), HashSet::from_iter(vec![1, 5, 9]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'C').unwrap()), HashSet::from_iter(vec![2, 10]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'B', b'E').unwrap()), HashSet::from_iter(vec![6]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'C', b'D').unwrap()), HashSet::from_iter(vec![3, 11]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'D', b'A').unwrap()), HashSet::from_iter(vec![4]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'E', b'F').unwrap()), HashSet::from_iter(vec![7]));
    
    assert_eq!(slot_to_set(img_matrix.at(b'F', b'A').unwrap()), HashSet::from_iter(vec![8]));
}
