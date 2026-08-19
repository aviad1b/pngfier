use super::super::elems::*;

fn slot_to_vec<I: Eq + Copy, Slot: ElemIndexesMatrixSlot<I>>(slot: Slot) -> Vec<I> {
    let mut res = Vec::new();
    for elem in slot.iter().unwrap() {
        res.push(*elem);
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

    assert_eq!(slot_to_vec(matrix.at(0, 0).unwrap()), &[3, 4, 5]);
    assert_eq!(slot_to_vec(matrix.at(4, 3).unwrap()), &[5]);
    assert_eq!(slot_to_vec(matrix.at(255, 255).unwrap()), &[1000, 2222]);
    assert_eq!(slot_to_vec(matrix.at(50, 50).unwrap()), &[]);
}

////////////////////////////////////////////////////////////////
// ---------------- RuntimeElemIndexesMatrix ---------------- //
////////////////////////////////////////////////////////////////

#[test]
fn runtime_insertion_gives_correct_iteration() {
   template_insertion_gives_correct_iteration(RuntimeElemIndexesMatrix::new());
}
