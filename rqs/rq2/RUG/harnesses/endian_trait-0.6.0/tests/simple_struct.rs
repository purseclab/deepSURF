/*! Test Endian derivation on normal structs composed of primitives
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;
use std::mem::{
	size_of,
	transmute,
};

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

#[test]
fn flip_struct() {
	let eb = get_struct().to_be();
	let el = get_struct().to_le();
	assert_eq!(size_of::<Struct>(), 16);
	let eb: [u8; 16] = unsafe { transmute(eb) };
	let el: [u8; 16] = unsafe { transmute(el) };

	//  .a
	assert_eq!(eb[0], el[1]);
	assert_eq!(eb[1], el[0]);

	//  .b
	assert_eq!(eb[2], el[3]);
	assert_eq!(eb[3], el[2]);

	//  .c
	assert_eq!(eb[4], el[7]);
	assert_eq!(eb[5], el[6]);
	assert_eq!(eb[6], el[5]);
	assert_eq!(eb[7], el[4]);

	//  .d
	assert_eq!(eb[8], el[15]);
	assert_eq!(eb[9], el[14]);
	assert_eq!(eb[10], el[13]);
	assert_eq!(eb[11], el[12]);
	assert_eq!(eb[12], el[11]);
	assert_eq!(eb[13], el[10]);
	assert_eq!(eb[14], el[9]);
	assert_eq!(eb[15], el[8]);
}

#[test]
fn double_flip() {
	let e = get_struct().to_be().from_be();
	assert_eq!(e, get_struct());

	let e = get_struct().to_le().from_le();
	assert_eq!(e, get_struct());

	//  On BE chips, to_be() goes away; on LE chips, to_le() goes away.
	//  We want to test that flipping twice cancels out, so these four calls
	//  should just flip, then flip again, back to the original.
	let e = get_struct().to_be().to_be().to_le().to_le();
	assert_eq!(e, get_struct());
}
