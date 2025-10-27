/*! Test Endian derivation on a generic struct

This test ensures that a generic struct can have Endian derived on it, so long
as it is composed of components which are all themselves Endian. This means that
all generic type parameters must be bound by Endian:

```rust
#[derive(Endian)]
struct ExampleGeneric<A> where A: Endian {
	a: A,
}
```

So long as this bound is satisfied, Endian can be derived on any generic struct.
!*/

#[macro_use]
extern crate endian_trait;

use endian_trait::Endian;

#[derive(Clone, Debug, Endian, PartialEq)]
struct GenericStruct<A, B, C>
where
	A: Endian + PartialEq,
	B: Endian + PartialEq,
	C: Endian + PartialEq,
{
	a: A,
	b: B,
	c: C,
	d: u64,
	e: f64,
	f: char,
	g: bool,
}

#[test]
fn generic_struct() {
	let generic: GenericStruct<_, _, _> = GenericStruct {
		a: 5u32,
		b: -5i32,
		c: '🦀',
		d: 1234567890,
		e: 6.283185307179586,
		f: '🐬',
		g: false,
	};
	let gb = generic.clone().to_be();
	let gl = generic.clone().to_le();

	assert_eq!(gb.from_be(), gl.from_le());
}
