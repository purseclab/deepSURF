use std::cmp::{Ord, Ordering};

pub fn sort<T>(list: &mut [T])
where
    T: Ord + Clone,
{
    let l = list.len();
    for i in 1..l {
        let mut j: i32 = (i - 1) as i32;
        let key = list[i].clone();
        while j >= 0 && key < list[j as usize] {
            list.swap(j as usize, (j + 1) as usize);
            j -= 1;
        }
        list[(j + 1) as usize] = key;
    }
}

pub fn sort_by<T, F>(list: &mut [T], f: F)
where
    T: Ord + Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let l = list.len();
    for i in 1..l {
        let mut j: i32 = (i - 1) as i32;
        let key = list[i].clone();
        while j >= 0 {
            if let Ordering::Less = f(&key, &list[j as usize]) {
                list.swap(j as usize, (j + 1) as usize);
                j -= 1;
            } else {
                break;
            }
        }
        list[(j + 1) as usize] = key;
    }
}
