extern crate stack;

use stack::{ArrayVec, SmallVec, Vector, Addressable};

#[test]
fn addressable() {
    fn addressable<T: Vector<Item=u32> + Addressable<u32>>(_: T) { }

    addressable(SmallVec::<[u32; 4]>::new());
    addressable(ArrayVec::<[u32; 4]>::new());
}
