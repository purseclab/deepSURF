use std::cmp::PartialOrd;
fn binary_search_util<T>(list: &[T], element: &T, start: isize, end: isize) -> bool
where
    T: PartialOrd,
{
    if end < start {
        return false;
    }
    let mid = start + (end - start) / 2;
    if &list[mid as usize] == element {
        return true;
    }
    if &list[mid as usize] > &element {
        return binary_search_util(list, element, start, mid - 1);
    }
    binary_search_util(list, element, mid + 1, end)
}
pub fn search<T>(list: &[T], element: &T) -> bool
where
    T: PartialOrd,
{
    !(list.is_empty() == true)
        && binary_search_util(list, element, 0, (list.len() - 1) as isize)
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    use crate::search::binary::binary_search_util;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(i32, i32, i32, i32, i32, i32, isize, isize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &[i32] = &[
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let mut p1: &i32 = &rug_fuzz_5;
        let mut p2: isize = rug_fuzz_6;
        let mut p3: isize = rug_fuzz_7;
        binary_search_util(p0, p1, p2, p3);
             }
});    }
}
