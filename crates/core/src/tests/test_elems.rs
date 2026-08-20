use std::{collections::HashSet, hash::Hash, vec};

use super::super::elems::*;

fn slot_to_set<I: Eq + Copy + Hash, Slot: ElemIndexesMatrixSlot<I>>(slot: Slot) -> HashSet<I> {
    let mut res = HashSet::new();
    for elem in slot.iter().unwrap() {
        res.insert(*elem);
    }
    res
}

//////////////////////////////////////////////////////////////
// ---------------- Templated matrix tests ---------------- //
//////////////////////////////////////////////////////////////

fn template_insertion_gives_correct_iteration(mut matrix: impl ElemIndexesMatrix<u8, i32>) {
    matrix.at_mut(0, 0).unwrap().insert(4).unwrap();
    matrix.at_mut(0, 0).unwrap().insert(3).unwrap();
    matrix.at_mut(0, 0).unwrap().insert(5).unwrap();

    matrix.at_mut(4, 3).unwrap().insert(5).unwrap();
    
    matrix.at_mut(255, 255).unwrap().insert(1000).unwrap();
    matrix.at_mut(255, 255).unwrap().insert(2222).unwrap();

    assert_eq!(slot_to_set(matrix.at(0, 0).unwrap()), HashSet::from_iter(vec![3, 4, 5]));
    assert_eq!(slot_to_set(matrix.at(4, 3).unwrap()), HashSet::from_iter(vec![5]));
    assert_eq!(slot_to_set(matrix.at(255, 255).unwrap()), HashSet::from_iter(vec![1000, 2222]));
    assert_eq!(slot_to_set(matrix.at(50, 50).unwrap()), HashSet::from_iter(vec![]));
}

////////////////////////////////////////////////////////////////
// ---------------- RuntimeElemIndexesMatrix ---------------- //
////////////////////////////////////////////////////////////////

#[test]
fn runtime_insertion_gives_correct_iteration() {
   template_insertion_gives_correct_iteration(RuntimeElemIndexesMatrix::new());
}
