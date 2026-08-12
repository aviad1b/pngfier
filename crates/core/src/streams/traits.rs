use generic_array::{ArrayLength, GenericArray, typenum::U1};

/// Used for types that can be parsed from&to bytes, using a fixed-sized buffer.
pub trait ConstBinParsible {
    /// Size of buffer needed for parsing.
    type BuffSize: ArrayLength;

    /// Parses `Self` from a corresponding buffer of bytes.
    /// 
    /// * `buff` - Buffer to parse `Self` from.
    /// 
    /// Returns parsed `Self` instance.
    /// 
    fn const_bin_parse(buff: &GenericArray<u8, Self::BuffSize>) -> Self;

    /// Parses `self` into a corresponding buffer of bytes.
    /// 
    /// * - `buff` - Buffer to parse into.
    /// 
    fn const_bin_unparse(&self, buff: &mut GenericArray<u8, Self::BuffSize>);
}

impl ConstBinParsible for u8 {
    type BuffSize = U1;

    fn const_bin_parse(buff: &GenericArray<u8, Self::BuffSize>) -> Self {
        buff[0]
    }

    fn const_bin_unparse(&self, buff: &mut GenericArray<u8, Self::BuffSize>) {
        buff[0] = *self;
    }
}

