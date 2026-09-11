use std::{collections::HashSet, hash::Hash, io, marker::PhantomData};

use generic_array::{ArrayLength, typenum::{U256, Unsigned}};

use crate::streams::traits::ConstBinParsible;

/// Represents a data element.
/// PNGfier will read & write data (& pngs) one element at a time.
pub trait Elem : Eq + Copy + ConstBinParsible {
    /// Amount of all different possible values for element.
    type N: ArrayLength;

    /// Converts element to an index for an array/matrix/etc.
    fn as_index(&self) -> usize;
}

impl Elem for u8 {
    type N = U256;

    fn as_index(&self) -> usize {
        *self as usize
    }
}

/// A slot in a matrix of index sets mapped by elements.
/// 
/// * `I` - Type of index in set. Must implement `Eq` and `Copy`.
/// 
pub trait ElemIndexesMatrixSlot<I: Eq + Copy> {
    /// Iterates over indexes in the slot's set.
    /// 
    /// Returns iterator, or error if occurred.
    /// 
    fn iter<'a>(&'a self) -> io::Result<impl Iterator<Item = &'a I>> where I: 'a;

    /// Checks if the slot's set contains a given index.
    /// 
    /// * `index` - Index to check if exists in the slot's set.
    /// 
    /// Returns `true`/`false` depending on `index`'s existance, or error if occurred.
    /// 
    fn contains(&self, index: &I) -> io::Result<bool>;
}

/// A mutable slot in a matrix of index sets mapped by elements.
/// 
/// * `I` - Type of index in set. Must implement `Eq` and `Copy`.
/// 
pub trait ElemIndexesMatrixSlotMut<I: Eq + Copy> : ElemIndexesMatrixSlot<I> {
    /// Inserts a new index to the slot's set.
    /// 
    /// * `index` - Index to insert.
    /// 
    /// Returns error if occurred.
    /// 
    fn insert(&mut self, index: I) -> io::Result<()>;
}

/// A matrix of index sets mapped by elements.
/// 
/// * `E` - Element type. Must implement `Elem`.
/// * `I` - Type of index in set. Must implement `Eq` and `Copy`.
/// 
pub trait ElemIndexesMatrix<E: Elem, I: Eq + Copy> {
    /// Type of slot mapped by two elements (row & column).
    type Slot<'a>: ElemIndexesMatrixSlot<I> where Self: 'a;

    /// Type of mutable slot mapped by two elements (row & column).
    type SlotMut<'a>: ElemIndexesMatrixSlotMut<I> where Self: 'a;

    /// Gets slot of indexes set mapped by two given elements (row & column).
    /// 
    /// * `i` - Row element key.
    /// * `j` - Column element key.
    /// 
    /// Returns slot mapped by `i` and `j`, or error if occurred.
    /// 
    fn at(&self, i: E, j: E) -> io::Result<Self::Slot<'_>>;

    /// Gets mutable slot of indexes set mapped by two given elements (row & column).
    /// 
    /// * `i` - Row element key.
    /// * `j` - Column element key.
    /// 
    /// Returns slot mapped by `i` and `j`.
    /// 
    fn at_mut(&mut self, i: E, j: E) -> io::Result<Self::SlotMut<'_>>;
}

/// Implementation of `ElemIndexesMatrixSlot` and `ElemIndexesMatrixSlotMut` that is 
/// based on a runtime hash set.
/// 
/// * `I` - Type of index in set. Must implement `Eq`, `Copy` and `Hash`.
/// 
pub struct RuntimeElemIndexesMatrixSlot<I: Eq + Copy + Hash> {
    indexes: HashSet<I>
}

impl<I: Eq + Copy + Hash> RuntimeElemIndexesMatrixSlot<I> {
    /// Constructs a new instance.
    pub fn new() -> Self {
        Self { indexes: HashSet::new() }
    }
}

impl<I: Eq + Copy + Hash> ElemIndexesMatrixSlot<I> for RuntimeElemIndexesMatrixSlot<I> {
    fn iter<'a>(&'a self) -> io::Result<impl Iterator<Item = &'a I>> where I: 'a {
        Ok(self.indexes.iter())
    }

    fn contains(&self, index: &I) -> io::Result<bool> {
        Ok(self.indexes.contains(index))
    }
}

impl<I: Eq + Copy + Hash> ElemIndexesMatrixSlotMut<I> for RuntimeElemIndexesMatrixSlot<I> {
    fn insert(&mut self, index: I) -> io::Result<()> {
        self.indexes.insert(index);
        Ok(())
    }
}

impl<'s, I: Eq + Copy + Hash> ElemIndexesMatrixSlot<I> for &'s RuntimeElemIndexesMatrixSlot<I> {
    fn iter<'a>(&'a self) -> io::Result<impl Iterator<Item = &'a I>> where I: 'a {
        Ok(self.indexes.iter())
    }

    fn contains(&self, index: &I) -> io::Result<bool> {
        Ok(self.indexes.contains(index))
    }
}

impl<'s, I: Eq + Copy + Hash> ElemIndexesMatrixSlot<I> for &'s mut RuntimeElemIndexesMatrixSlot<I> {
    fn iter<'a>(&'a self) -> io::Result<impl Iterator<Item = &'a I>> where I: 'a {
        Ok(self.indexes.iter())
    }

    fn contains(&self, index: &I) -> io::Result<bool> {
        Ok(self.indexes.contains(index))
    }
}

impl<'s, I: Eq + Copy + Hash> ElemIndexesMatrixSlotMut<I> for &'s mut RuntimeElemIndexesMatrixSlot<I> {
    fn insert(&mut self, index: I) -> io::Result<()> {
        self.indexes.insert(index);
        Ok(())
    }
}

/// Implementation of `ElemIndexesMatrix` that is based on runtime hash sets.
/// 
/// * `E` - Element type. Must implement `Elem`.
/// * `I` - Type of index in set. Must implement `Eq`, `Copy` and `Hash`.
/// 
pub struct RuntimeElemIndexesMatrix<E: Elem, I: Eq + Copy + Hash> {
    matrix: Vec<Vec<RuntimeElemIndexesMatrixSlot<I>>>,
    phantom: PhantomData<E>,
}

impl<E: Elem, I: Eq + Copy + Hash> RuntimeElemIndexesMatrix<E, I> {
    /// Constructs a new instance.
    pub fn new() -> Self {
        Self {
            matrix: (0..E::N::to_usize()).map(|_|
                    (0..E::N::to_usize()).map(|_|
                        RuntimeElemIndexesMatrixSlot::new()
                    ).collect()
                ).collect(),
            phantom: PhantomData,
        }
    }
}

impl<E: Elem, I: Eq + Copy + Hash> ElemIndexesMatrix<E, I> for RuntimeElemIndexesMatrix<E, I> {
    type Slot<'a> = &'a RuntimeElemIndexesMatrixSlot<I> where Self: 'a;
    type SlotMut<'a> = &'a mut RuntimeElemIndexesMatrixSlot<I> where Self: 'a;

    fn at(&self, i: E, j: E) -> io::Result<Self::Slot<'_>> {
        Ok(&self.matrix[i.as_index()][j.as_index()])
    }

    fn at_mut(&mut self, i: E, j: E) -> io::Result<Self::SlotMut<'_>> {
        Ok(&mut self.matrix[i.as_index()][j.as_index()])
    }
}
