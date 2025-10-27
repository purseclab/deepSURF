pub fn subset_util<T>(
    arr: &[T],
    st: usize,
    end: usize,
    reserve: &mut Vec<T>,
    subsets: &mut Vec<Vec<T>>,
)
where
    T: Clone,
{
    for index in st..end {
        reserve.push(arr[index].clone());
        subsets.push(reserve.clone());
        subset_util(&arr, index + 1, end, reserve, subsets);
        reserve.pop();
    }
}
/// This method will give all subsets of a set which is cloneable
/// pub fn find_all_subset<T>(arr: &[T]) -> Vec<Vec<T>> where  T: Clone
///
/// # Examples
/// ```rust
/// use algorithmica::subset::find_all_subset;
/// let v = vec![1, 2, 3];
/// assert_eq!(
///            find_all_subset(&v),
///            vec![
///                vec![1],
///                vec![1, 2],
///                vec![1, 2, 3],
///                vec![1, 3],
///                vec![2],
///                vec![2, 3],
///                vec![3]
///            ]
///        );
/// ```
pub fn find_all_subset<T>(arr: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut subsets = vec![];
    let mut reserve = vec![];
    subset_util(arr, 0, arr.len(), &mut reserve, &mut subsets);
    subsets
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn subset_test_string() {
        let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        assert_eq!(
            find_all_subset(& v), vec![vec!["A"], vec!["A", "B"], vec!["A", "B", "C"],
            vec!["A", "C"], vec!["B"], vec!["B", "C"], vec!["C"]]
        );
    }
    #[test]
    fn subset_test_int() {
        let v = vec![1, 2, 3];
        assert_eq!(
            find_all_subset(& v), vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3],
            vec![2], vec![2, 3], vec![3]]
        );
    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::subset::subset_util;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(i32, i32, i32, i32, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v6: [i32; 5] = [
            rug_fuzz_0,
            rug_fuzz_1,
            rug_fuzz_2,
            rug_fuzz_3,
            rug_fuzz_4,
        ];
        let p0: &[i32] = &v6;
        let p1: usize = rug_fuzz_5;
        let p2: usize = v6.len();
        let mut p3: Vec<i32> = Vec::new();
        let mut p4: Vec<Vec<i32>> = Vec::new();
        subset_util(p0, p1, p2, &mut p3, &mut p4);
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    use crate::subset::find_all_subset;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v6: [i32; 3] = [rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        find_all_subset(&v6);
             }
});    }
}
