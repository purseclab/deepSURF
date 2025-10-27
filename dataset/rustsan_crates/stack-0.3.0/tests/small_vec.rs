/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate stack;

use stack::{ArrayVec, SmallVec, Vector};

#[test]
pub fn test_inline() {
    let mut v = SmallVec::<[String; 16]>::new();
    v.push("hello".into());
    v.push("there".into());
    assert_eq!(&*v, &[
        String::from("hello"),
        String::from("there"),
    ][..]);
}

#[test]
pub fn test_spill() {
    let mut v = SmallVec::<[String; 2]>::new();
    v.push("hello".into());
    assert_eq!(v[0], "hello");
    v.push("there".into());
    v.push("burma".into());
    assert_eq!(v[0], "hello");
    v.push("shave".into());
    assert_eq!(&*v, &[
        String::from("hello"),
        String::from("there"),
        String::from("burma"),
        String::from("shave"),
    ][..]);
}

#[test]
pub fn test_double_spill() {
    let mut v = SmallVec::<[String; 2]>::new();
    v.push("hello".into());
    v.push("there".into());
    v.push("burma".into());
    v.push("shave".into());
    v.push("hello".into());
    v.push("there".into());
    v.push("burma".into());
    v.push("shave".into());
    assert_eq!(&*v, &[
        String::from("hello"),
        String::from("there"),
        String::from("burma"),
        String::from("shave"),
        String::from("hello"),
        String::from("there"),
        String::from("burma"),
        String::from("shave"),
    ][..]);
}

#[test]
fn drop_soundness() {
    SmallVec::<[Box<u32>; 2]>::new();
}

#[test]
fn null_optimization() {
    assert!(Some(ArrayVec::<[&u32; 2]>::new()).is_some());
    assert!(Some(SmallVec::<[&u32; 2]>::new()).is_some());
}

#[test]
fn into_iter() {
    let mut v: SmallVec<[u8; 2]> = Vector::new();
    v.push(3);
    assert_eq!(v.iter().cloned().collect::<Vec<_>>(), &[3]);

    v.clear();
    v.push(3);
    v.push(4);
    v.push(5);
    assert_eq!(v.into_iter().collect::<Vec<_>>(), &[3, 4, 5]);
}

#[test]
fn test_capacity() {
    let mut v: SmallVec<[u8; 2]> = Vector::new();
    v.reserve(1);
    assert_eq!(v.capacity(), 2);
    assert!(!v.is_spilled());

    v.reserve_exact(0x100);
    assert!(v.capacity() >= 0x100);

    v.push(0);
    v.push(1);
    v.push(2);
    v.push(3);

    v.shrink_to_fit();
    assert!(v.capacity() < 0x100);
}

#[test]
fn test_truncate() {
    let mut v: SmallVec<[Box<u8>; 8]> = Vector::new();

    for x in 0..8 {
        v.push(Box::new(x));
    }
    v.truncate(4);

    assert_eq!(v.len(), 4);
    assert!(!v.is_spilled());

    assert_eq!(*v.swap_remove(1), 1);
    assert_eq!(*v.remove(1), 3);
    v.insert(1, Box::new(3));

    assert_eq!(&v.iter().map(|v| **v).collect::<Vec<_>>(), &[0, 3, 2]);
}
