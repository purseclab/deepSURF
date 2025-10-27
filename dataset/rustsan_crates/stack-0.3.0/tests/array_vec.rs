/* Copyright (C) 2015 bluss
 *
 * This software may be modified and distributed under the terms
 * of the MIT license. See the COPYING file for details.
 */

extern crate stack;

use stack::{ArrayVec, Vector};
use std::mem;

#[test]
fn test_simple() {
    use std::ops::Add;

    let mut vec: ArrayVec<[Vec<i32>; 3]> = ArrayVec::new();

    vec.push(vec![1, 2, 3, 4]);
    vec.push(vec![10]);
    vec.push(vec![-1, 13, -2]);

    for elt in &vec {
        assert_eq!(elt.iter().fold(0, Add::add), 10);
    }

    let sum_len = vec.into_iter().map(|x| x.len()).fold(0, Add::add);
    assert_eq!(sum_len, 8);
}

#[test]
fn test_u16_index() {
    const N: usize = 4096;
    let mut vec: ArrayVec<[_; N]> = ArrayVec::new();
    for _ in 0..N {
        assert!(vec.push_cap(1u8).is_ok());
    }
    assert!(vec.push_cap(0).is_err());
    assert_eq!(vec.len(), N);
}

#[test]
fn test_iter() {
    let mut iter = ArrayVec::from([1, 2, 3]).into_iter();
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn test_drop() {
    use std::cell::Cell;

    let flag = &Cell::new(0);

    struct Bump<'a>(&'a Cell<i32>);

    impl<'a> Drop for Bump<'a> {
        fn drop(&mut self) {
            let n = self.0.get();
            self.0.set(n + 1);
        }
    }

    {
        let mut array = ArrayVec::<[Bump; 128]>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
    }
    assert_eq!(flag.get(), 2);

    // test something with the nullable pointer optimization
    flag.set(0);

    {
        let mut array = ArrayVec::<[_; 3]>::new();
        array.push(vec![Bump(flag)]);
        array.push(vec![Bump(flag), Bump(flag)]);
        array.push(vec![]);
        assert!(array.push_cap(vec![Bump(flag)]).is_err());
        assert_eq!(flag.get(), 1);
        drop(array.pop());
        assert_eq!(flag.get(), 1);
        drop(array.pop());
        assert_eq!(flag.get(), 3);
    }

    assert_eq!(flag.get(), 4);

    // test into_inner
    flag.set(0);
    {
        let mut array = ArrayVec::<[_; 3]>::new();
        array.push(Bump(flag));
        array.push(Bump(flag));
        array.push(Bump(flag));
        let inner = array.into_inner();
        assert!(inner.is_ok());
        assert_eq!(flag.get(), 0);
        drop(inner);
        assert_eq!(flag.get(), 3);
    }

}

#[test]
fn test_extend() {
    let mut range = 0..10;

    let mut array: ArrayVec<[_; 5]> = range.by_ref().take(5).collect();
    assert_eq!(&array[..], &[0, 1, 2, 3, 4]);
    assert_eq!(range.next(), Some(5));

    let take = array.capacity() - array.len();
    array.extend(range.by_ref().take(take));
    assert_eq!(range.next(), Some(6));

    let mut array: ArrayVec<[_; 10]> = (0..3).collect();
    assert_eq!(&array[..], &[0, 1, 2]);
    array.extend(3..5);
    assert_eq!(&array[..], &[0, 1, 2, 3, 4]);
}

#[test]
fn test_is_send_sync() {
    let data = ArrayVec::<[Vec<i32>; 5]>::new();
    &data as &Send;
    &data as &Sync;
}

#[test]
#[cfg(feature = "drain")]
fn test_drain() {
    let mut v = ArrayVec::from([0; 8]);
    v.pop();
    v.drain(0..7);
    assert_eq!(&v[..], &[]);

    v.extend(0..);
    v.drain(1..4);
    assert_eq!(&v[..], &[0, 4, 5, 6, 7]);
    let u: ArrayVec<[_; 3]> = v.drain(1..4).rev().collect();
    assert_eq!(&u[..], &[6, 5, 4]);
    assert_eq!(&v[..], &[0, 7]);
    v.drain(..);
    assert_eq!(&v[..], &[]);
}

#[test]
#[should_panic]
#[cfg(feature = "drain")]
fn test_drain_oob() {
    let mut v = ArrayVec::from([0; 8]);
    v.pop();
    v.drain(0..8);
}

#[test]
fn test_insert() {
    let mut v = ArrayVec::from([]);
    assert_eq!(v.push_cap(1), Err(1));
    assert_eq!(v.insert_cap(0, 1), Some(1));

    let mut v = ArrayVec::<[_; 3]>::new();
    v.insert_cap(0, 0);
    v.insert_cap(1, 1);
    v.insert_cap(2, 2);
    v.insert_cap(3, 3);
    assert_eq!(&v[..], &[0, 1, 2]);
    v.insert_cap(1, 9);
    assert_eq!(&v[..], &[0, 9, 1]);

    let mut v = ArrayVec::from([2]);
    assert_eq!(v.insert_cap(1, 1), Some(1));
    assert_eq!(v.insert_cap(2, 1), Some(1));
}

#[test]
fn test_in_option() {
    // Sanity check that we are sound w.r.t Option & non-nullable layout optimization.
    let mut v = Some(ArrayVec::<[&i32; 1]>::new());
    assert!(v.is_some());
    unsafe {
        *v.as_mut().unwrap().get_unchecked_mut(0) = mem::zeroed();
    }
    assert!(v.is_some());
}

#[test]
fn test_into_inner_1() {
    let mut v = ArrayVec::from([1, 2]);
    v.pop();
    let u = v.clone();
    assert_eq!(v.into_inner(), Err(u));
}

#[test]
fn test_into_inner_2() {
    let mut v = ArrayVec::<[String; 4]>::new();
    v.push("a".into());
    v.push("b".into());
    v.push("c".into());
    v.push("d".into());
    assert_eq!(v.into_inner().unwrap(), ["a", "b", "c", "d"]);
}

#[test]
fn test_into_inner_3() {
    let mut v = ArrayVec::<[i32; 4]>::new();
    v.extend(1..5);
    assert_eq!(v.into_inner().unwrap(), [1, 2, 3, 4]);
}
