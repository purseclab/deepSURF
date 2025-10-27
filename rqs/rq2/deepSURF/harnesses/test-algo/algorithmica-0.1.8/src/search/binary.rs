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
    !(list.is_empty() == true) && binary_search_util(list, element, 0, (list.len() - 1) as isize)
}
