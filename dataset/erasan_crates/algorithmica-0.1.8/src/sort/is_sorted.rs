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
