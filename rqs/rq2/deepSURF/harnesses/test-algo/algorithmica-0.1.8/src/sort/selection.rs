use std::cmp::{Ord, Ordering};

pub fn sort<T>(list: &mut [T])
where
    T: Ord,
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut min_index = i;
        for j in i + 1..n {
            if list[min_index] > list[j] {
                min_index = j;
            }
        }
        if i != min_index {
            list.swap(i, min_index);
        }
    }
}

pub fn sort_by<T, F>(list: &mut [T], f: F)
where
    T: Ord,
    F: Fn(&T, &T) -> Ordering,
{
    let n = list.len();
    for i in 0..n - 1 {
        let mut min_index = i;
        for j in i + 1..n {
            if let Ordering::Greater = f(&list[min_index], &list[j]) {
                min_index = j;
            }
        }
        if i != min_index {
            list.swap(i, min_index);
        }
    }
}
