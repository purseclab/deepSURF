use std::cmp::{Ord, Ordering};
use std::fmt::Debug;

unsafe fn get_by_index<T>(list: &[T], index: isize) -> *const T {
    let list_offset = list.as_ptr();
    list_offset.offset(index)
}

fn merge<T: Debug, F>(list: &mut [T], start: usize, mid: usize, end: usize, compare: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let mut left = Vec::with_capacity(mid - start + 1);
    let mut right = Vec::with_capacity(end - mid);
    unsafe {
        let mut start = start;
        while start <= mid {
            left.push(get_by_index(list, start as isize).read());
            start += 1;
        }
        while start <= end {
            right.push(get_by_index(list, start as isize).read());
            start += 1;
        }
    }

    let mut left_index = 0;
    let mut right_index = 0;
    let mut k = start;

    unsafe {
        while left_index < left.len() && right_index < right.len() {
            if compare(&left[left_index], &right[right_index]) {
                list[k] = get_by_index(&left, left_index as isize).read();
                left_index += 1;
            } else {
                list[k] = get_by_index(&right, right_index as isize).read();
                right_index += 1;
            }
            k += 1;
        }

        while left_index < left.len() {
            list[k] = get_by_index(&left, left_index as isize).read();
            left_index += 1;
            k += 1;
        }

        while right_index < right.len() {
            list[k] = get_by_index(&right, right_index as isize).read();
            right_index += 1;
            k += 1;
        }
    }
}

fn merge_sort<T: Debug, F>(list: &mut [T], start: usize, end: usize, f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    if end <= start {
        return;
    }
    let mid = (end - start) / 2 + start;
    merge_sort(list, start, mid, f);
    merge_sort(list, mid + 1, end, f);
    merge(list, start, mid, end, f);
}

pub fn sort<T>(list: &mut [T])
where
    T: Ord + Debug,
{
    if list.is_empty() || list.len() == 1 {
        return;
    }
    merge_sort(list, 0, list.len() - 1, &|a, b| a.lt(b));
}

pub fn sort_by<T, F>(list: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
    T: Debug,
{
    if list.is_empty() || list.len() == 1 {
        return;
    }
    merge_sort(list, 0, list.len() - 1, &|a, b| {
        compare(a, b) == Ordering::Less
    });
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sorting_test() {
        let mut t = [1, 2, 3, 4, 5, 6, 7, 8];
        t.reverse();
        sort(&mut t);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], t);
    }
}
