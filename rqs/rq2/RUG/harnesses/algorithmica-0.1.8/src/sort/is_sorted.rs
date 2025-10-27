use std::cmp::Ord;
pub fn is_sorted<T>(list: &[T]) -> bool
where
    T: Ord,
{
    if list.is_empty() {
        return true;
    }
    let mut previous = &list[0];
    for current in list.iter().skip(1) {
        if previous > current {
            return false;
        }
        previous = current;
    }
    true
}
pub fn is_sorted_desc<T>(list: &[T]) -> bool
where
    T: Ord,
{
    if list.is_empty() {
        return true;
    }
    let mut previous = &list[0];
    for current in list.iter().skip(1) {
        if previous < current {
            return false;
        }
        previous = current;
    }
    true
}
pub fn is_sorted_by<T, F>(list: &[T], f: F) -> bool
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    if list.is_empty() {
        return true;
    }
    let mut previous = &list[0];
    for current in list.iter().skip(1) {
        if f(previous, current) {
            return false;
        }
        previous = current;
    }
    true
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::sort::is_sorted::is_sorted;
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
        debug_assert_eq!(is_sorted(& v6), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::sort::is_sorted::is_sorted_desc;
    use crate::search::binary;
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
        debug_assert_eq!(is_sorted_desc(& p0), true);
             }
});    }
}
