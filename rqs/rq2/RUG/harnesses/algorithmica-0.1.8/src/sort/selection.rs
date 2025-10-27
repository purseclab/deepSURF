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
use crate::sort::selection;
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(i32, i32, i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        crate::sort::selection::sort(&mut p0);
        debug_assert_eq!(p0, [1, 2, 3, 4, 5]);
             }
});    }
}
