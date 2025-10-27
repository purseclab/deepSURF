/*! Endian conversion trait

This trait declares methods that perform endian conversions on data types. For
the primitives, which are essentially atomic in structure, this conversion is
simple: flip all their bytes around. This conversion is also defined as inherent
methods on the integral primitives, so `Endian::from_be(n: i32)` is equivalent
to `::std::i32::from_be(n: i32)`
!*/

#![deny(missing_docs)]

//  Re-export the custom-derive so that users don't need two crates explicitly.
#[allow(unused_imports)]
#[macro_use]
extern crate endian_trait_derive;
pub use endian_trait_derive::*;

/// Convert a type from one endian order to another.
///
/// The standard implementation of this trait is simply to call the methods on
/// the component members of a data type which are themselves `Endian`, until the
/// call stack bottoms out at one of Rust's primitives.
pub trait Endian {
	/// Converts from host endian to big-endian order.
	///
	/// On big-endian platforms, this is a no-op and should be compiled out.
	fn to_be(self) -> Self;

	/// Converts from host endian to little-endian order.
	///
	/// On little-endian platforms, this is a no-op and should be compiled out.
	fn to_le(self) -> Self;

	/// Converts from big-endian order to host endian.
	///
	/// On big-endian platforms, this is a no-op and should be compiled out.
	fn from_be(self) -> Self;

	/// Converts from little-endian order to host endian.
	///
	/// On little-endian platforms, this is a no-op and should be compiled out.
	fn from_le(self) -> Self;
}

/// Implementing Endian on the integer primitives just means delegating to their
/// inherent methods. As there are many integer primitives, this macro kills the
/// needless code duplication.
macro_rules! implendian {
	( $( $t:tt ),* ) => { $(
		impl Endian for $t {
			#[inline(always)]
			fn from_be(self) -> Self {
				$t::from_be(self)
			}
			#[inline(always)]
			fn from_le(self) -> Self {
				$t::from_le(self)
			}
			#[inline(always)]
			fn to_be(self) -> Self {
				$t::to_be(self)
			}
			#[inline(always)]
			fn to_le(self) -> Self {
				$t::to_le(self)
			}
		}
	)* };
}

/// Implement Endian on the floats by flipping their byte repr.
///
/// The to_ conversions use bare transmute, as the result may wind up looking
/// invalid on the host architecture when used in floating-point contexts. The
/// from_ conversions use Rust's from_bits functions, as the final result must
/// be a valid local floating-point number.
///
/// The to/from _bits() APIs on f32/64 were stabilized in Rust 1.20, and thus
/// this code cannot be run on Rust version below that.
//  TODO: Figure out how to isolate this so that older Rust versions can use the
//  crate?
macro_rules! implendian_f {
	( $( $t:tt ),* ) => { $(
		impl Endian for $t {
			fn from_be(self) -> Self {
				Self::from_bits(self.to_bits().from_be())
			}
			fn from_le(self) -> Self {
				Self::from_bits(self.to_bits().from_le())
			}
			fn to_be(self) -> Self {
				Self::from_bits(self.to_bits().to_be())
			}
			fn to_le(self) -> Self {
				Self::from_bits(self.to_bits().to_le())
			}
		}
	)* };
}

/// Implement on `bool`.
///
/// `bool` is always one byte, and single bytes don`t have endian order.
impl Endian for bool {
	fn from_be(self) -> Self { self }
	fn from_le(self) -> Self { self }
	fn to_be(self) -> Self { self }
	fn to_le(self) -> Self { self }
}

/// Implement on `char`.
///
/// `char` is four bytes wide. Delegate to `u32`'s implementation and transmute.
/// This is safe ONLY IF THE CONVERSION MAKES LOGICAL SENSE
/// `char` is Unicode codepoints, NOT integers, so not all values of `u32` are
/// valid values of `char`.
/// The `to_` functions will emit potentially invalid `char` values, and this is
/// to be expected. The `from_` functions, however, will panic if they are about
/// to emit an invalid `char` byte value.
impl Endian for char {
	/// Attempts to create a local `char` from a big-endian value.
	///
	/// This function WILL panic if the local value exceeds the maximum Unicode
	/// Scalar Value permissible.
	fn from_be(self) -> Self {
		let flip: u32 = (self as u32).from_be();
		if flip > ::std::char::MAX as u32 {
			panic!("A `char` cannot have a value of {:X}", flip);
		}
		unsafe { ::std::mem::transmute(flip) }
	}

	/// Attempts to create a local `char` from a little-endian value.
	///
	/// This function WILL panic if the local value exceeds the maximum Unicode
	/// Scalar Value permissible.
	fn from_le(self) -> Self {
		let flip: u32 = (self as u32).from_le();
		if flip > ::std::char::MAX as u32 {
			panic!("A `char` cannot have a value of {:X}", flip);
		}
		unsafe { ::std::mem::transmute(flip) }
	}

	/// Converts a local `char` to big-endian.
	///
	/// This may result in a byte value that is not a valid Unicode Scalar Value
	/// and the result of this transform should be passed into a `from_be()`
	/// before using it in anything that requires `char` semantics.
	fn to_be(self) -> Self {
		unsafe { ::std::mem::transmute((self as u32).to_be()) }
	}

	/// Converts a local `char` to little-endian.
	///
	/// This may result in a byte value that is not a valid Unicode Scalar Value
	/// and the result of this transform should be passed into a `from_le()`
	/// before using it in anything that requires `char` semantics.
	fn to_le(self) -> Self {
		unsafe { ::std::mem::transmute((self as u32).to_le()) }
	}
}

//  Implement on the integer primitives
implendian!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

//  Implement on floats
implendian_f!(f32, f64);

#[cfg(feature = "arrays")]
mod arrays;

mod slices;
