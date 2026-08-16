use generic_array::{GenericArray, typenum::U2};

use super::super::super::streams::{spans::*, traits::*, dummy::*};

/// Parsible element used for testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestElem(u16);

impl ConstBinParsible for TestElem {
	type BuffSize = U2;

	fn const_bin_parse(buff: &GenericArray<u8, U2>) -> Self {
		TestElem(u16::from_be_bytes([buff[0], buff[1]]))
	}

	fn const_bin_unparse(&self, buff: &mut GenericArray<u8, U2>) {
		let bytes = self.0.to_be_bytes();
		buff[0] = bytes[0];
		buff[1] = bytes[1];
	}
}
