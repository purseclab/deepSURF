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
    merge_sort(list, 0, list.len() - 1, &|a, b| { compare(a, b) == Ordering::Less });
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
use super::*;
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::sort::merge_sort;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v6: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let index: isize = rug_fuzz_5;
        unsafe {
            let ptr = crate::sort::merge_sort::get_by_index(&v6, index);
            debug_assert_eq!(* ptr, 3);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::sort::merge_sort::merge;
    use std::fmt::Debug;
    #[test]
    fn test_rug() {
        let mut v6: [i32; 5] = [1, 2, 3, 4, 5];
        let start: usize = 0;
        let mid: usize = 2;
        let end: usize = 4;
        fn compare_fn<T: Debug>(a: &T, b: &T) -> bool {
            true
        }
        merge(&mut v6, start, mid, end, &compare_fn);
    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::sort::merge_sort::merge_sort;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v6: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let p0: &mut [i32] = &mut v6;
        let p1: usize = rug_fuzz_5;
        let p2: usize = rug_fuzz_6;
        let f = |a: &i32, b: &i32| a < b;
        merge_sort(p0, p1, p2, &f);
             }
});    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::search::binary;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v6: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        crate::sort::merge_sort::sort(&mut v6);
             }
});    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::sort::merge_sort::sort_by;
    use std::cmp::Ordering;
    use std::fmt::Debug;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v6: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let compare = |a: &i32, b: &i32| -> Ordering { a.cmp(b) };
        sort_by(&mut v6, &compare);
             }
});    }
}
