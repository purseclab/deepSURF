/*! Implement `Endian` on mutable slices.
!*/

use super::Endian;
use std::ptr;

/// Traverse a slice, performing the `Endian` method on each item in place.
impl<'a, T: Endian> Endian for &'a mut [T] {
	fn from_be(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			ptr::write(elt, ptr::read(elt).from_be());
		} }
		self
	}
	fn from_le(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			ptr::write(elt, ptr::read(elt).from_le());
		} }
		self
	}
	fn to_be(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			ptr::write(elt, ptr::read(elt).to_be());
		} }
		self
	}
	fn to_le(self) -> Self {
		for elt in self.iter_mut() { unsafe {
			ptr::write(elt, ptr::read(elt).to_le());
		} }
		self
	}
}
