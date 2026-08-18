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
