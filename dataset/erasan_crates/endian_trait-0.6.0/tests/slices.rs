extern crate endian_trait;

use endian_trait::Endian;

use std::mem::transmute;

#[test]
fn slices() {
	let src: [i32; 8] = [
		1, 512, 196608, 67108864, 83886080, 393216, 1792, 8,
	];

	let mut src_be = src;
	let mut src_le = src;

	let src_be: &mut [i32] = &mut src_be;
	src_be.to_be();

	unsafe {
		assert_eq!([0, 0, 0, 1], transmute::<i32, [u8; 4]>(src_be[0]));
		assert_eq!([0, 0, 2, 0], transmute::<i32, [u8; 4]>(src_be[1]));
		assert_eq!([0, 3, 0, 0], transmute::<i32, [u8; 4]>(src_be[2]));
		assert_eq!([4, 0, 0, 0], transmute::<i32, [u8; 4]>(src_be[3]));
		assert_eq!([5, 0, 0, 0], transmute::<i32, [u8; 4]>(src_be[4]));
		assert_eq!([0, 6, 0, 0], transmute::<i32, [u8; 4]>(src_be[5]));
		assert_eq!([0, 0, 7, 0], transmute::<i32, [u8; 4]>(src_be[6]));
		assert_eq!([0, 0, 0, 8], transmute::<i32, [u8; 4]>(src_be[7]));
	}

	let src_le: &mut [i32] = &mut src_le;
	src_le.to_le();

	unsafe {
		assert_eq!([1, 0, 0, 0], transmute::<i32, [u8; 4]>(src_le[0]));
		assert_eq!([0, 2, 0, 0], transmute::<i32, [u8; 4]>(src_le[1]));
		assert_eq!([0, 0, 3, 0], transmute::<i32, [u8; 4]>(src_le[2]));
		assert_eq!([0, 0, 0, 4], transmute::<i32, [u8; 4]>(src_le[3]));
		assert_eq!([0, 0, 0, 5], transmute::<i32, [u8; 4]>(src_le[4]));
		assert_eq!([0, 0, 6, 0], transmute::<i32, [u8; 4]>(src_le[5]));
		assert_eq!([0, 7, 0, 0], transmute::<i32, [u8; 4]>(src_le[6]));
		assert_eq!([8, 0, 0, 0], transmute::<i32, [u8; 4]>(src_le[7]));
	}

	let from_be = src_be.from_be();
	let from_le = src_le.from_le();

	assert_eq!(from_be, from_le);
}
