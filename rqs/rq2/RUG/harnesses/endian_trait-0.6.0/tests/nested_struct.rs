/*! Test Endian derivation on a struct composed of other structs
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;
use std::mem::transmute;

#[repr(C)]
#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
struct Struct {
	a: u16,
	b: u16,
	c: u32,
	d: u64,
}

fn get_struct() -> Struct {
	Struct {
		a: 0x1234,
		b: 0x5678,
		c: 0x9ABCDEF0,
		d: 0x123456789ABCDEF0,
	}
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
struct Nested {
	a: u64,
	b: Struct,
}

fn get_nested() -> Nested {
	Nested {
		a: 0xabad1dea0c05fefe,
		b: get_struct(),
	}
}

#[test]
fn flip_nested_struct() {
	let nb = get_nested().to_be();
	let nl = get_nested().to_le();

	let nb: [u8; 24] = unsafe { transmute(nb) };
	let nl: [u8; 24] = unsafe { transmute(nl) };

	//  .a
	assert_eq!(nb[0], nl[7]);
	assert_eq!(nb[1], nl[6]);
	assert_eq!(nb[2], nl[5]);
	assert_eq!(nb[3], nl[4]);
	assert_eq!(nb[4], nl[3]);
	assert_eq!(nb[5], nl[2]);
	assert_eq!(nb[6], nl[1]);
	assert_eq!(nb[7], nl[0]);

	//  .b.a
	assert_eq!(nb[8], nl[9]);
	assert_eq!(nb[9], nl[8]);

	//  .b.b
	assert_eq!(nb[10], nl[11]);
	assert_eq!(nb[11], nl[10]);

	//  .b.b
	assert_eq!(nb[12], nl[15]);
	assert_eq!(nb[13], nl[14]);
	assert_eq!(nb[14], nl[13]);
	assert_eq!(nb[15], nl[12]);

	//  .b.b
	assert_eq!(nb[16], nl[23]);
	assert_eq!(nb[17], nl[22]);
	assert_eq!(nb[18], nl[21]);
	assert_eq!(nb[19], nl[20]);
	assert_eq!(nb[20], nl[19]);
	assert_eq!(nb[21], nl[18]);
	assert_eq!(nb[22], nl[17]);
	assert_eq!(nb[23], nl[16]);
}
