extern crate alloc;
use alloc::boxed::Box;
use core::cmp::Ordering;
use core::slice;
use core::ptr;

use crate::ops::*;

/// Common re-indexing logic used internally by the `SortOps` trait.
fn build_swap_trace(ordering : &mut [(usize,usize)]) ->  &mut [(usize,usize)]
{
    let len = ordering.len();
    
    // Create a reverse lookup
    for idx in 0..len {
        unsafe {
            // We know 0 <= idx < ordering.len(), so we don't need to check the indexing.
            let v = ordering.get_unchecked(idx).0;
            // It's less trivial to figure out that 0 <= v <= ordering.len() - the input
            // array is created by sorted_box_to_ordering()
            ordering.get_unchecked_mut(v).1 = idx;
        }
    }
    
    let mut swap_count = 0;
    
    // Build a swap trace that will shuffle everything into the right position.
    for i in 0..len {
        // Used get_unchecked for the same reason as above
        unsafe {
            let (other, inv_i) = *ordering.get_unchecked(i);
            if i != other {
                // we re-use the ordering slice to store the swap trace
                *ordering.get_unchecked_mut(swap_count) = (i, other);
                swap_count += 1;
                if inv_i > i {
                    ordering.get_unchecked_mut(inv_i).0 = other;
                    ordering.get_unchecked_mut(other).1 = inv_i;
                }
            }
        }
    }
    
    // Only return the portion of the slice containing the swap trace.
    &mut ordering[..swap_count]
}

/// Use some unsafeness to coerce a [(usize, &T)] into a [(usize, usize)]. The `Box` is consumed,
/// meaning that we "unborrow" the &T values.
fn sorted_box_to_ordering<T>(sorted: Box<[(usize, &T)]>) -> Box<[(usize,usize)]> {
    debug_assert_eq!(core::mem::size_of::<&T>(), core::mem::size_of::<usize>());
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
pub trait SortOps<T> : TooDeeOpsMut<T> {

    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is stable.
    fn sort_row_ord<F>(&mut self, row: usize) where T : Ord {
        self.sort_by_row(row, T::cmp);
    }
    
    /// Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.
    /// This sort is unstable.
    fn sort_unstable_row_ord<F>(&mut self, row: usize) where T : Ord {
        self.sort_unstable_by_row(row, T::cmp);
    }

    /// Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.
    /// This sort is stable.
    fn sort_by_row<F>(&mut self, row: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(row < self.num_rows());
        
        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().map(|(i, v)| (i, v)).collect();
        
        sort_data.sort_by(|i, j| compare(i.1, j.1));
        
        // Build up a "trace" of column swaps to apply
        
        let mut ordering = sorted_box_to_ordering(sort_data);
        
        let swap_trace = build_swap_trace(&mut ordering);
        
        // Apply the swap trace to each row. For larger arrays, this approach is faster than applying swap_cols() directly.
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                // The swap indices will definitely be within the expected range,
                // so we can use `get_unchecked_mut` here
                unsafe {
                    let pa: *mut T = r.get_unchecked_mut(i.0);
                    let pb: *mut T = r.get_unchecked_mut(i.1);
                    ptr::swap(pa, pb);
                }
//                r.swap(i.0, i.1);
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

        let mut sort_data : Box<[(usize, &T)]> = self[row].iter().enumerate().map(|(i, v)| (i, v)).collect();
        
        sort_data.sort_unstable_by(|i, j| compare(i.1, j.1));

        // Build up a "trace" of column swaps to apply

        let mut ordering = sorted_box_to_ordering(sort_data);

        let swap_trace = build_swap_trace(&mut ordering);
        
        // Apply the swap trace to each row. For larger arrays, this approach is faster than applying swap_cols() directly.
        for r in self.rows_mut() {
            for i in swap_trace.iter() {
                // The swap indices will definitely be within the expected range,
                // so we can use `get_unchecked_mut` here
                unsafe {
                    let pa: *mut T = r.get_unchecked_mut(i.0);
                    let pb: *mut T = r.get_unchecked_mut(i.1);
                    ptr::swap(pa, pb);
                }
//                r.swap(i.0, i.1);
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
    fn sort_col_ord<F>(&mut self, col: usize) where T : Ord {
        self.sort_by_col(col, T::cmp);
    }
    
    /// Sort the entire two-dimensional array by comparing elements on in a specific column.
    /// This sort is stable.
    fn sort_by_col<F>(&mut self, col: usize, mut compare: F)
        where
        F: FnMut(&T, &T) -> Ordering, 
    {
        assert!(col < self.num_cols());
        
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().map(|(i, v)| (i, v)).collect();

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
        let mut sort_data : Box<[(usize, &T)]> = self.col(col).enumerate().map(|(i, v)| (i, v)).collect();

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

impl<T, O> SortOps<T> for O where O : TooDeeOpsMut<T> {}
