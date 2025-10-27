use std::cmp::Ord;
fn quick_sort<T>(list: &mut [T], start: usize, end: usize)
where
    T: Ord + Clone,
{
    if start >= end {
        return;
    }
    let pivot = list[end].clone();
    let mut i = start;
    let mut j = start;
    while j < end {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    list.swap(i, end);
    if i > 0 {
        quick_sort(list, start, i - 1);
    }
    quick_sort(list, i + 1, end);
}
pub fn sort<T>(list: &mut [T])
where
    T: Ord + Clone,
{
    if list.is_empty() || list.len() == 1 {
        return;
    }
    quick_sort(list, 0, list.len() - 1);
}
use crate::sort::quick_sort;
#[cfg(test)]
mod tests_rug_17 {
    use super::*;
    #[test]
    fn test_quick_sort() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(i32, i32, i32, i32, i32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let p1: usize = rug_fuzz_5;
        let p2: usize = rug_fuzz_6;
        quick_sort(&mut p0, p1, p2);
        debug_assert_eq!(p0, [1, 2, 3, 4, 5]);
             }
});    }
}
