/* Copyright (C) 2015 John Hodge
 *
 * This software may be modified and distributed under the terms
 * of the MIT license. See the COPYING file for details.
 */

#[macro_use]
extern crate stack;

use stack::{SmallVec, ArrayVec, SmallDST};

#[test]
// A trivial check that ensures that methods are correctly called
fn trivial_type() {
    let val: SmallDST<PartialEq<u32>> = small_dst!(1234u32);
    assert!( *val == 1234 );
    assert!( *val != 1233 );
}

#[test]
// Create an instance with a Drop implementation, and ensure the drop handler fires when destructed
// This also ensures that lifetimes are correctly handled
fn ensure_drop() {
    use std::cell::Cell;
    #[derive(Debug)]
    struct Struct<'a>(&'a Cell<usize>);
    impl<'a> Drop for Struct<'a> { fn drop(&mut self) { let v = self.0.get(); self.0.set(v + 1); } }
    
    let flag = Cell::new(0);
    let val: SmallDST<::std::fmt::Debug> = small_dst!(Struct(&flag));
    assert!(flag.get() == 0);
    drop(val);
    assert!(flag.get() == 1);
}

#[test]
fn many_instances() {
    trait TestTrait {
        fn get_value(&self) -> u32;
    }
    
    #[inline(never)]
    fn instance_one() -> SmallDST<TestTrait> {
        struct OneStruct(u32);
        impl TestTrait for OneStruct {
            fn get_value(&self) -> u32 { self.0 }
        }
        small_dst!(OneStruct(12345))
    }
    
    #[inline(never)]
    fn instance_two() -> SmallDST<TestTrait> {
        struct TwoStruct;
        impl TestTrait for TwoStruct {
            fn get_value(&self) -> u32 { 54321 }
        }
        small_dst!(TwoStruct)
    }
    
    let i1 = instance_one();
    let i2 = instance_two();
    assert_eq!(i1.get_value(), 12345);
    assert_eq!(i2.get_value(), 54321);
}

#[test]
fn closure() {
    let v1 = 1234u64;
    let c: SmallDST<Fn() -> String> = small_dst!(|| format!("{}", v1));
    assert_eq!(c(), "1234");
}

#[test]
fn undersize() {
    use std::any::Any;
    let _: SmallDST<Any, ArrayVec<[usize; 8]>> = small_dst!([0usize; 7]);
}

#[test]
#[should_panic]
fn oversize() {
    use std::any::Any;
    let _: SmallDST<Any, ArrayVec<[usize; 8]>> = small_dst!([0usize; 8]);
}

#[test]
fn spill() {
    use std::any::Any;
    let dst: SmallDST<Any, SmallVec<[usize; 8]>> = small_dst!([0usize; 8]);
    unsafe {
        assert!(dst.data().is_spilled());
    }
}

#[test]
fn sized() {
    let value = SmallDST::<_, ArrayVec<[usize; 1]>>::from(12345usize);
    assert_eq!(*value, 12345);
}
