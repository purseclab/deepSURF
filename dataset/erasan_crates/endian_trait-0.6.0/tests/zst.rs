/*! Test Endian on a zero-sized type

Let's make an abomination unto the Lord and compiler

It doesn't make any sense from a mechanical perspective to define Endian on a
zero sized type because zero sized types don't ever exist. However, it is
permissible to define all kinds of things that look at or use the byte repr on
ZSTs because, since ZSTs **have no byte repr**, we just use it as abstract
logic. ZSTs are all logically Eq since they are all non-existent in the same
way. ZSTs can also claim to implement byte reorder methods, and the compiler
will just erase them all during monomorphization.

¯\_(ツ)_/¯ 🤷
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;
use std::mem::size_of;

#[derive(Clone, Copy, Debug, Endian, PartialEq, Eq)]
struct Zst;

#[derive(Clone, Copy, Debug, Endian, PartialEq, Eq)]
struct ZsTuple();

#[derive(Clone, Copy, Debug, Endian, PartialEq, Eq)]
struct ComplexZeroType {
	a: Zst,
	b: ZsTuple,
}

#[test]
fn zst() {
	let z: Zst = Zst;
	let z2: Zst = z.to_be();
	assert_eq!(size_of::<Zst>(), 0);
	assert_eq!(z.to_le(), z2);

	let zt: ZsTuple = ZsTuple();
	let zt2: ZsTuple = zt.to_be();
	assert_eq!(size_of::<ZsTuple>(), 0);
	assert_eq!(zt.to_le(), zt2);

	let czt: ComplexZeroType = ComplexZeroType {
		a: z,
		b: zt,
	};
	assert_eq!(size_of::<ComplexZeroType>(), 0);
	assert_eq!(czt.to_be(), czt.to_le());
}
