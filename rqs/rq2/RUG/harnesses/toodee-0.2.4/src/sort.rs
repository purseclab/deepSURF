extern crate alloc;
use alloc::boxed::Box;
use core::cmp::Ordering;
use core::slice;
use core::ptr;
use crate::ops::*;
/// Common re-indexing logic used internally by the `SortOps` trait.
fn build_swap_trace(ordering: &mut [(usize, usize)]) -> &mut [(usize, usize)] {
    let len = ordering.len();
    for idx in 0..len {
        unsafe {
            let v = ordering.get_unchecked(idx).0;
            ordering.get_unchecked_mut(v).1 = idx;
        }
    }
    let mut swap_count = 0;
    for i in 0..len {
        unsafe {
            let (other, inv_i) = *ordering.get_unchecked(i);
            if i != other {
                *ordering.get_unchecked_mut(swap_count) = (i, other);
                swap_count += 1;
                if inv_i > i {
                    ordering.get_unchecked_mut(inv_i).0 = other;
                    ordering.get_unchecked_mut(other).1 = inv_i;
                }
            }
        }
    }
    &mut ordering[..swap_count]
}
/// Use some unsafeness to coerce a [(usize, &T)] into a [(usize, usize)]. The `Box` is consumed,
/// meaning that we "unborrow" the &T values.
fn sorted_box_to_ordering<T>(sorted: Box<[(usize, &T)]>) -> Box<[(usize, usize)]> {
    debug_assert_eq!(core::mem::size_of::<& T > (), core::mem::size_of::< usize > ());
    let len = sorted.len();
    let p = Box::into_raw(sorted);
    unsafe {
        let p2 = slice::from_raw_parts_mut(p as *mut (usize, usize), len);
        Box::from_raw(p2)
    }
}
/// Provides sorting capabilities to two-dimensional arrays. Sorting of the rows and columns
/// is performed in-place, and care is taken to minimise row/col swaps. This is achieved by
/// sorting the row/col and original index pair, then repositioning the rows/columns once the
/// new sort order has been determined.
pub trait SortOps<T>: TooDeeOpsMut<T> {
    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is stable.
    fn sort_row_ord<F>(&mut self, row: usize)
    where
        T: Ord,
    {
        self.sort_by_row(row, T::cmp);
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is unstable.
    fn sort_unstable_row_ord<F>(&mut self, row: usize)
    where
        T: Ord,
    {
        self.sort_unstable_by_row(row, T::cmp);
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.
    /// This sort is stable.
    fn sort_by_row<F>(&mut self, row: usize, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        assert!(row < self.num_rows());
        let mut sort_data: Box<[(usize, &T)]> = self[row]
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();
        sort_data.sort_by(|i, j| compare(i.1, j.1));
        let mut ordering = sorted_box_to_ordering(sort_data);
        let swap_trace = build_swap_trace(&mut ordering);
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                unsafe {
                    let pa: *mut T = r.get_unchecked_mut(i.0);
                    let pb: *mut T = r.get_unchecked_mut(i.1);
                    ptr::swap(pa, pb);
                }
            }
        }
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.
    /// This sort is unstable.
    fn sort_unstable_by_row<F>(&mut self, row: usize, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        assert!(row < self.num_rows());
        let mut sort_data: Box<[(usize, &T)]> = self[row]
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();
        sort_data.sort_unstable_by(|i, j| compare(i.1, j.1));
        let mut ordering = sorted_box_to_ordering(sort_data);
        let swap_trace = build_swap_trace(&mut ordering);
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                unsafe {
                    let pa: *mut T = r.get_unchecked_mut(i.0);
                    let pb: *mut T = r.get_unchecked_mut(i.1);
                    ptr::swap(pa, pb);
                }
            }
        }
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific row using a key
    /// extraction function.
    /// This sort is stable.
    fn sort_by_row_key<B, F>(&mut self, row: usize, mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_by_row(row, |a, b| f(a).cmp(&f(b)));
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific row using a key
    /// extraction function.
    /// This sort is unstable.
    fn sort_unstable_by_row_key<B, F>(&mut self, row: usize, mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_unstable_by_row(row, |a, b| f(a).cmp(&f(b)));
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific column using the natural ordering.
    /// This sort is stable.
    fn sort_col_ord<F>(&mut self, col: usize)
    where
        T: Ord,
    {
        self.sort_by_col(col, T::cmp);
    }
    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is stable.
    fn sort_by_col<F>(&mut self, col: usize, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        assert!(col < self.num_cols());
        let mut sort_data: Box<[(usize, &T)]> = self
            .col(col)
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();
        sort_data.sort_by(|i, j| compare(i.1, j.1));
        let mut ordering = sorted_box_to_ordering(sort_data);
        let swap_trace = build_swap_trace(&mut ordering);
        for i in swap_trace.iter() {
            self.swap_rows(i.0, i.1);
        }
    }
    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is unstable.
    fn sort_unstable_by_col<F>(&mut self, col: usize, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        assert!(col < self.num_cols());
        let mut sort_data: Box<[(usize, &T)]> = self
            .col(col)
            .enumerate()
            .map(|(i, v)| (i, v))
            .collect();
        sort_data.sort_unstable_by(|i, j| compare(i.1, j.1));
        let mut ordering = sorted_box_to_ordering(sort_data);
        let swap_trace = build_swap_trace(&mut ordering);
        for i in swap_trace.iter() {
            self.swap_rows(i.0, i.1);
        }
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific column using a key
    /// extraction function.
    /// This sort is stable.
    fn sort_by_col_key<B, F>(&mut self, col: usize, mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_by_row(col, |a, b| f(a).cmp(&f(b)));
    }
    /// Sort the entire two-dimensional array by comparing elements on a specific column using a key
    /// extraction function.
    /// This sort is unstable.
    fn sort_unstable_by_col_key<B, F>(&mut self, col: usize, mut f: F)
    where
        B: Ord,
        F: FnMut(&T) -> B,
    {
        self.sort_unstable_by_row(col, |a, b| f(a).cmp(&f(b)));
    }
}
impl<T, O> SortOps<T> for O
where
    O: TooDeeOpsMut<T>,
{}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(usize, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: &mut [(usize, usize)] = &mut [
            (rug_fuzz_0, rug_fuzz_1),
            (rug_fuzz_2, rug_fuzz_3),
            (rug_fuzz_4, rug_fuzz_5),
            (rug_fuzz_6, rug_fuzz_7),
        ];
        crate::sort::build_swap_trace(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use core::mem;
    use std::slice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, i32, usize, i32, usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Box<[(usize, &i32)]> = Box::new([
            (rug_fuzz_0, &rug_fuzz_1),
            (rug_fuzz_2, &rug_fuzz_3),
            (rug_fuzz_4, &rug_fuzz_5),
        ]);
        crate::sort::sorted_box_to_ordering(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::{TooDeeOpsMut, SortOps, TooDee};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12, mut rug_fuzz_13, mut rug_fuzz_14, mut rug_fuzz_15, mut rug_fuzz_16, mut rug_fuzz_17, mut rug_fuzz_18)) = <(usize, usize, i32, usize, usize, i32, usize, usize, i32, usize, usize, i32, usize, usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut toodee: TooDee<i32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        toodee[(rug_fuzz_3, rug_fuzz_4)] = rug_fuzz_5;
        toodee[(rug_fuzz_6, rug_fuzz_7)] = rug_fuzz_8;
        toodee[(rug_fuzz_9, rug_fuzz_10)] = rug_fuzz_11;
        let row_index = rug_fuzz_12;
        SortOps::sort_unstable_row_ord::<
            fn(&i32, &i32) -> Ordering,
        >(&mut toodee, row_index);
        debug_assert_eq!(toodee[(rug_fuzz_13, rug_fuzz_14)], 1);
        debug_assert_eq!(toodee[(rug_fuzz_15, rug_fuzz_16)], 2);
        debug_assert_eq!(toodee[(rug_fuzz_17, rug_fuzz_18)], 3);
             }
});    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, sort::SortOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, u32, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        toodee.fill(rug_fuzz_2);
        let row = rug_fuzz_3;
        let f = |x: &u32| x * rug_fuzz_4;
        toodee.sort_by_row_key(row, f);
             }
});    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::{TooDee, TooDeeOpsMut, Coordinate};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, i32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<i32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        p0.fill(rug_fuzz_2);
        let p1: usize = rug_fuzz_3;
        let mut p2 = |v: &i32| v.abs();
        p0.sort_unstable_by_row_key(p1, &mut p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use core::mem;
    use core::ptr;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, RowsMut, SortOps};
    struct ConcreteType {
        data: u32,
    }
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let p1: usize = rug_fuzz_2;
        let p2 = |a: &u32, b: &u32| { a.cmp(b) };
        p0.sort_unstable_by_col(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::{TooDee, TooDeeOps, TooDeeOpsMut, SortOps};
    #[test]
    fn test_sort_unstable_by_col_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8, mut rug_fuzz_9, mut rug_fuzz_10, mut rug_fuzz_11, mut rug_fuzz_12)) = <(usize, usize, u32, usize, usize, u32, usize, usize, u32, usize, usize, u32, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        p0.fill(rug_fuzz_2);
        p0[(rug_fuzz_3, rug_fuzz_4)] = rug_fuzz_5;
        p0[(rug_fuzz_6, rug_fuzz_7)] = rug_fuzz_8;
        p0[(rug_fuzz_9, rug_fuzz_10)] = rug_fuzz_11;
        let p1: usize = rug_fuzz_12;
        let mut extract_key = |&val: &u32| val;
        crate::sort::SortOps::sort_unstable_by_col_key(&mut p0, p1, &mut extract_key);
             }
});    }
}
