#![feature(test)]

extern crate test;
extern crate id_map;
extern crate bit_set;

use std::collections::{HashMap, HashSet};

use test::Bencher;

use id_map::IdMap;

#[bench]
fn naive(b: &mut Bencher) {
    b.iter(|| {
        let mut counter = 0;
        let mut get_id = |val| {
            let id = counter;
            counter += 1;
            (id, val)
        };
        let mut map: HashMap<usize, u32> = (0..1024).map(&mut get_id).collect();

        let mut to_remove = Vec::new();
        for (&id, &val) in &map {
            if val % 7 == 0 {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            map.remove(&id);
        }

        for &val in map.values() {
            test::black_box(val);
        }

        map.extend((0..512).map(&mut get_id));

        for &val in map.values() {
            test::black_box(val);
        }

        map
    })
}

#[bench]
fn id_map(b: &mut Bencher) {
    b.iter(|| {
        let mut map: IdMap<u32> = (0..1024).collect();

        let mut to_remove = Vec::new();
        for (id, &val) in &map {
            if val % 7 == 0 {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            map.remove(id);
        }

        for &val in map.values() {
            test::black_box(val);
        }

        map.extend((0..512));

        for &val in map.values() {
            test::black_box(val);
        }

        map
    })
}

#[bench]
fn set(b: &mut Bencher) {
    b.iter(|| {
        let mut values: Vec<u32> = (0..1024).collect();
        let mut empty = HashSet::new();

        let mut to_remove = Vec::new();
        for (id, &val) in values.iter().enumerate().filter(|&(id, _)| !empty.contains(&id)) {
            if val % 7 == 0 {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            empty.insert(id);
        }

        for (_, &val) in values.iter().enumerate().filter(|&(id, _)| !empty.contains(&id)) {
            test::black_box(val);
        }

        {
            let mut drain = empty.drain();
            for val in 0..512 {
                if let Some(id) = drain.next() {
                    values[id] = val;
                } else {
                    values.push(val);
                }
            }
        }

        for (_, &val) in values.iter().enumerate().filter(|&(id, _)| !empty.contains(&id)) {
            test::black_box(val);
        }

        (values, empty);
    })
}