/*! Test Endian derivation on a tuple
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;
use std::mem::{
	size_of,
	transmute,
};

//  Time to get freaky
//  This is necessary because my test process is transmuting to a byte array and
//  I don't want the compiler meddling with what I think the tuple looks like in
//  memory. Plus, testing Endian on a packed repr is probably a good idea.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Endian, Eq, PartialEq)]
struct Tuple(u64, i32, u16, i8);

fn get_tuple() -> Tuple {
	Tuple(
		0x0123456789ABCDEF,
		0x02468ACD,
		0x1337,
		0x5A,
	)
}

#[test]
fn flip_tuple() {
	let tb = get_tuple().to_be();
	let tl = get_tuple().to_le();
	assert_eq!(size_of::<Tuple>(), 15);
	let tb: [u8; 15] = unsafe { transmute(tb) };
	let tl: [u8; 15] = unsafe { transmute(tl) };

	//  .0
	assert_eq!(tb[0], tl[7]);
	assert_eq!(tb[1], tl[6]);
	assert_eq!(tb[2], tl[5]);
	assert_eq!(tb[3], tl[4]);
	assert_eq!(tb[4], tl[3]);
	assert_eq!(tb[5], tl[2]);
	assert_eq!(tb[6], tl[1]);
	assert_eq!(tb[7], tl[0]);

	//  .1
	assert_eq!(tb[8], tl[11]);
	assert_eq!(tb[9], tl[10]);
	assert_eq!(tb[10], tl[9]);
	assert_eq!(tb[11], tl[8]);

	//  .2
	assert_eq!(tb[12], tl[13]);
	assert_eq!(tb[13], tl[12]);

	//  .3
	//  This should be idempotent, since a single byte has no endian
	assert_eq!(tb[14], tl[14]);
}
