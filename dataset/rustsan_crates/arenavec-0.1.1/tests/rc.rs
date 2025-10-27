use arenavec::rc::{Arena, SliceVec};
use arenavec::ArenaBacking;

const DEFAULT_CAPACITY: usize = 4096 << 16;

#[test]
fn init_empty() {
    if cfg!(not(miri)) {
        let arena = Arena::init_capacity(ArenaBacking::MemoryMap, DEFAULT_CAPACITY).unwrap();

        let vec: SliceVec<usize> = SliceVec::new(arena.inner());

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }

    {
        let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

        let vec: SliceVec<usize> = SliceVec::new(arena.inner());

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }
}

#[test]
fn init_capacity() {
    let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

    let mut vec = SliceVec::with_capacity(arena.inner(), 10);

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);
}

#[test]
fn init_empty_push() {
    let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

    let mut vec = SliceVec::new(arena.inner());

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);

    vec.push(1);

    assert_eq!(vec.len(), 1);
    assert_eq!(vec.capacity(), 4);

    vec.push(2);
    vec.push(3);
    vec.push(4);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec.capacity(), 4);

    vec.push(5);

    assert_eq!(vec.len(), 5);
    assert_eq!(vec.capacity(), 8);
}

#[test]
fn reserve_and_resize() {
    let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

    let mut vec = SliceVec::new(arena.inner());

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);

    for i in 0..5 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 5);
    assert_eq!(vec.capacity(), 8);

    vec.reserve(2);

    assert_eq!(vec.capacity(), 8);

    vec.reserve(9);

    assert_eq!(vec.capacity(), 16);

    vec.resize(12, 1);

    assert_eq!(vec.len(), 12);
    assert_eq!(vec.capacity(), 16);

    for i in 5..12 {
        assert_eq!(vec[i], 1);
    }
}

#[test]
fn drop() {
    use std::rc::Rc;

    let rc = Rc::new(());
    let arena = Arena::init_capacity(ArenaBacking::SystemAllocation, DEFAULT_CAPACITY).unwrap();

    {
        let mut vec = SliceVec::with_capacity(arena.inner(), 15);

        for _ in 0..20 {
            vec.push(rc.clone());
        }

        vec.resize(10, Rc::new(()));
    }

    assert_eq!(Rc::strong_count(&rc), 1);
}
