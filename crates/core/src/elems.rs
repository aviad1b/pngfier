use std::io;

use generic_array::ArrayLength;

use crate::streams::traits::ConstBinParsible;

/// Represents a data element.
/// PNGfier will read & write data (& pngs) one element at a time.
pub trait Elem : Eq + Copy + ConstBinParsible {
    /// Amount of all different possible values for element.
    type N: ArrayLength;

    /// Converts element to an index for an array/matrix/etc.
    fn as_index(&self) -> usize;
}

/// A slot in a matrix of index sets mapped by elements.
/// 
/// * `I` - Type of index in set. Must satisfy `Eq` and `Copy`.
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
/// * `I` - Type of index in set. Must satisfy `Eq` and `Copy`.
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
/// * `E` - Element type. Must satisfy `Elem`.
/// * `I` - Type of index in set. Must satisfy `Eq` and `Copy`.
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
