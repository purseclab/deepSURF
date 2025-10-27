/*! Invalid Code Generation

The macro should successfully emit Rust code that has the correct shape, but
which will then fail to compile.

When the rest of this code is uncommented, compilation must fail. When it is
disabled, compilation must succeed.
!*/

/*
#![compile_fail]

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;

struct Foo {
	a: i32
}

#[derive(Endian)]
struct Bar {
	a: Foo,
	b: i32
}
*/
