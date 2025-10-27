pub fn subset_util<T>(
    arr: &[T],
    st: usize,
    end: usize,
    reserve: &mut Vec<T>,
    subsets: &mut Vec<Vec<T>>,
) where
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
            find_all_subset(&v),
            vec![
                vec!["A"],
                vec!["A", "B"],
                vec!["A", "B", "C"],
                vec!["A", "C"],
                vec!["B"],
                vec!["B", "C"],
                vec!["C"]
            ]
        );
    }

    #[test]
    fn subset_test_int() {
        let v = vec![1, 2, 3];
        assert_eq!(
            find_all_subset(&v),
            vec![
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
    }
}
