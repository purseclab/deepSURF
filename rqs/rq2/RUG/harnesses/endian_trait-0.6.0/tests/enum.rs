/*! Test deriving Endian on an enum

This won't compile, because Endian on an enum doesn't make any sense. Cast it to
an integer repr outside the Endian operations.

If this code is uncommented, it should emit an error message about enums not
being supported. Proc-macro panics are compile-time, not runtime, so this test
cannot be run directly.
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;
use std::mem::{
	size_of,
	transmute,
};

#[test]
fn derive_8() {
	type Raw = [u8; 1];
	#[repr(u8)]
	#[derive(Endian)]
	enum Foo {
		A = 16,
	}
	assert_eq!(size_of::<Foo>(), 1);

	let bytes_be: Raw = [16];
	let bytes_le: Raw = [16];

	let foo_be: Raw = unsafe { transmute(Foo::A.to_be()) };
	let foo_le: Raw = unsafe { transmute(Foo::A.to_le()) };

	assert_eq!(&bytes_be[..], &foo_be[..]);
	assert_eq!(&bytes_le[..], &foo_le[..]);
}

#[test]
fn derive_16() {
	type Raw = [u8; 2];
	#[repr(i16)]
	#[derive(Endian)]
	enum Foo {
		A = 16,
	}
	assert_eq!(size_of::<Foo>(), 2);

	let bytes_be: Raw = [0, 16];
	let bytes_le: Raw = [16, 0];

	let foo_be: Raw = unsafe { transmute(Foo::A.to_be()) };
	let foo_le: Raw = unsafe { transmute(Foo::A.to_le()) };

	assert_eq!(&bytes_be[..], &foo_be[..]);
	assert_eq!(&bytes_le[..], &foo_le[..]);
}

#[test]
fn derive_32() {
	type Raw = [u8; 4];
	#[repr(u32)]
	#[derive(Endian)]
	enum Foo {
		A = 16,
	}
	assert_eq!(size_of::<Foo>(), 4);

	let bytes_be: Raw = [0, 0, 0, 16];
	let bytes_le: Raw = [16, 0, 0, 0];

	let foo_be: Raw = unsafe { transmute(Foo::A.to_be()) };
	let foo_le: Raw = unsafe { transmute(Foo::A.to_le()) };

	assert_eq!(&bytes_be[..], &foo_be[..]);
	assert_eq!(&bytes_le[..], &foo_le[..]);
}

#[test]
fn derive_64() {
	type Raw = [u8; 8];
	#[repr(i64)]
	#[derive(Endian)]
	enum Foo {
		A = 16,
	}
	assert_eq!(size_of::<Foo>(), 8);

	let bytes_be: Raw = [0, 0, 0, 0, 0, 0, 0, 16];
	let bytes_le: Raw = [16, 0, 0, 0, 0, 0, 0, 0];

	let foo_be: Raw = unsafe { transmute(Foo::A.to_be()) };
	let foo_le: Raw = unsafe { transmute(Foo::A.to_le()) };

	assert_eq!(&bytes_be[..], &foo_be[..]);
	assert_eq!(&bytes_le[..], &foo_le[..]);
}

//  Uncomment these to fail the build.
//  If either of these compiles, it is a bug.
/*
#[test]
#[compile_fail]
#[should_panic]
fn derive_c() {
	#[repr(C)]
	#[derive(Endian)]
	enum Foo {
		A, B, C,
	}
}

//  This fails because the body adds size to the discriminant, and the transmute
//  call powering the Endian transform will not succeed.
#[test]
#[compile_fail]
#[should_panic]
fn derive_bodied() {
	#[repr(i16)]
	#[derive(Endian)]
	enum Foo {
		A(i8),
	}
}
*/
