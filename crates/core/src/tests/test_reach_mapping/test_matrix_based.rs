use crate::{elems::{ElemIndexesMatrix, ElemIndexesMatrixSlot, RuntimeElemIndexesMatrix}, streams::dummy::DummyInputElemStream};

use super::super::super::chunks::mapping::{reach::*, reach_utils::*};

#[test]
fn test_init_img_matrix() {
    let mut image = DummyInputElemStream::new(b"ABCDABEFABCD".to_vec());
    let mut img_matrix = RuntimeElemIndexesMatrix::<u8, _>::new();

    init_img_matrix(&mut image, &mut img_matrix).unwrap();

    assert!(Vec::from_iter(img_matrix.at(b'A', b'A').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'A', b'C').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'A', b'D').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'A', b'E').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'A', b'F').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'B', b'A').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'B', b'B').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'B', b'D').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'B', b'F').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'C', b'A').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'C', b'B').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'C', b'C').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'C', b'E').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'C', b'F').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'D', b'B').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'D', b'C').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'D', b'D').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'D', b'E').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'D', b'F').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'E', b'A').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'E', b'B').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'E', b'C').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'E', b'D').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'E', b'E').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'F', b'B').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'F', b'C').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'F', b'D').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'F', b'E').unwrap().iter()).is_empty());
    assert!(Vec::from_iter(img_matrix.at(b'F', b'F').unwrap().iter()).is_empty());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'A', b'B').unwrap().iter()).len(), 3);
    assert!(img_matrix.at(b'A', b'B').unwrap().contains(&1).unwrap());
    assert!(img_matrix.at(b'A', b'B').unwrap().contains(&5).unwrap());
    assert!(img_matrix.at(b'A', b'B').unwrap().contains(&9).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'B', b'C').unwrap().iter()).len(), 2);
    assert!(img_matrix.at(b'B', b'C').unwrap().contains(&2).unwrap());
    assert!(img_matrix.at(b'B', b'C').unwrap().contains(&10).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'B', b'E').unwrap().iter()).len(), 1);
    assert!(img_matrix.at(b'B', b'E').unwrap().contains(&6).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'C', b'D').unwrap().iter()).len(), 2);
    assert!(img_matrix.at(b'C', b'D').unwrap().contains(&3).unwrap());
    assert!(img_matrix.at(b'C', b'D').unwrap().contains(&11).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'D', b'A').unwrap().iter()).len(), 1);
    assert!(img_matrix.at(b'D', b'A').unwrap().contains(&4).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'E', b'F').unwrap().iter()).len(), 1);
    assert!(img_matrix.at(b'E', b'F').unwrap().contains(&7).unwrap());
    
    assert_eq!(Vec::from_iter(img_matrix.at(b'F', b'A').unwrap().iter()).len(), 1);
    assert!(img_matrix.at(b'F', b'A').unwrap().contains(&8).unwrap());
}
