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
use crate::sort::insertion;
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
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
        crate::sort::insertion::sort(&mut v6);
        debug_assert_eq!(v6, [1, 2, 3, 4, 5]);
             }
});    }
}
