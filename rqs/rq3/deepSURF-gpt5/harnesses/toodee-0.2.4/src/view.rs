use core::fmt;
use core::fmt::{ Formatter, Debug };
use core::ops::{Index, IndexMut};
use core::cmp::Ordering;
use core::ptr;

use crate::toodee::*;
use crate::ops::*;
use crate::iter::*;

/// Checks the proposed view dimensions, and returns the correct cols and rows for view construction.
fn calculate_view_dimensions<T>(start: Coordinate, end: Coordinate, toodee: &impl TooDeeOps<T>) -> (usize, usize) {
    assert!(end.0 >= start.0);
    assert!(end.1 >= start.1);
    assert!(end.0 <= toodee.num_cols());
    assert!(end.1 <= toodee.num_rows());
    let mut num_cols = end.0 - start.0;
    let mut num_rows = end.1 - start.1;
    // zero out dimensions for empty arrays
    if num_cols == 0 || num_rows == 0 {
        num_cols = 0;
        num_rows = 0;
    }
    (num_cols, num_rows)
}

/// Provides a read-only view (or subset) of a `TooDee` array.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TooDeeView<'a, T> {
    data: &'a [T],
    num_cols: usize,
    num_rows: usize,
    main_cols: usize,
    start: Coordinate,
}

impl<'a, T> TooDeeView<'a, T> {

    /// Create a new `TooDeeViewMut` using the provided slice reference.
    /// 
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    /// 
    /// Panics if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::TooDeeView;
    /// let data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    /// let view = TooDeeView::new(4, 3, &data);
    /// ```
    pub fn new(num_cols: usize, num_rows: usize, data: &'a [T]) -> TooDeeView<'a, T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        TooDeeView {
            data : &data[..size],
            start: (0, 0),
            num_cols,
            num_rows,
            main_cols : num_cols,
        }
    }
    
    /// Used internally by `TooDee` to create a `TooDeeView`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a TooDee<T>) -> TooDeeView<'a, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, toodee);
        let main_cols = toodee.num_cols();
        let data_start = start.1 * main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * main_cols + num_cols
            }
        };
        unsafe {
            TooDeeView {
                data: toodee.data().get_unchecked(data_start..data_start + data_len),
                start,
                num_cols,
                num_rows,
                main_cols,
            }
        }
    }

}

impl<'a, T> TooDeeOps<T> for TooDeeView<'a, T>
{
    #[inline]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline]
    fn num_rows(&self) -> usize {
        self.num_rows
    }
    
    #[inline]
    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);
        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };

        unsafe {
            TooDeeView {
                data: self.data.get_unchecked(data_start..data_start + data_len),
                start: (self.start.0 + start.0, self.start.1 + start.1),
                num_cols,
                num_rows,
                main_cols : self.main_cols,
            }
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        Rows {
            v : &self.data,
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
        }
    }
    
    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
        unsafe {
            Col {
                v : self.data.get_unchecked(start..end),
                skip : self.main_cols - 1,
            }
        }
    }

    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let view = toodee.view((0,0), (10,5));
    ///     let row = view.get_unchecked_row(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * self.main_cols;
        self.data.get_unchecked(start..start + self.num_cols)
    }

    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
    }

}

impl<'a, T> Index<usize> for TooDeeView<'a, T> {

    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.main_cols;
        unsafe {
            self.data.get_unchecked(start..start + self.num_cols)
        }
    }
}

impl<'a, T> Index<Coordinate> for TooDeeView<'a, T> {

    type Output = T;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
        }
    }
}


/// Provides a mutable view (or subset), of a `TooDee` array.
#[derive(Hash, Eq, PartialEq)]
pub struct TooDeeViewMut<'a, T> {
    data: &'a mut [T],
    num_cols: usize,
    num_rows: usize,
    main_cols: usize,
    start: Coordinate,
}


impl<'a, T> TooDeeViewMut<'a, T> {

    /// Create a new `TooDeeViewMut` using the provided mutable slice reference.
    /// 
    /// # Panics
    /// 
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    /// 
    /// Panics if the slice's length is not sufficient to represent
    /// the desired array dimensions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::TooDeeViewMut;
    /// let mut data = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    /// let view_mut = TooDeeViewMut::new(4, 3, &mut data);
    /// ```
    pub fn new(num_cols: usize, num_rows: usize, data: &'a mut [T]) -> TooDeeViewMut<'a, T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let size = num_cols * num_rows;
        assert!(size <= data.len());
        unsafe {
            TooDeeViewMut {
                data : data.get_unchecked_mut(..size),
                start: (0, 0),
                num_cols,
                num_rows,
                main_cols : num_cols,
            }
        }
    }

    /// Used internally by `TooDee` to create a `TooDeeViewMut`.
    pub(super) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &'a mut TooDee<T>) -> TooDeeViewMut<'a, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, toodee);
        let main_cols = toodee.num_cols();
        let data_start = start.1 * main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * main_cols + num_cols
            }
        };
        unsafe {
            TooDeeViewMut {
                data: toodee.data_mut().get_unchecked_mut(data_start..data_start + data_len),
                start,
                num_cols,
                num_rows,
                main_cols,
            }
        }
    }

}


impl<'a, T> TooDeeOps<T> for TooDeeViewMut<'a,T> {

    #[inline]
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    #[inline]
    fn num_cols(&self) -> usize {
        self.num_cols
    }

    #[inline]
    fn bounds(&self) -> (Coordinate, Coordinate) {
        (self.start, (self.start.0 + self.num_cols, self.start.1 + self.num_rows))
    }
    
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);
        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };
        
        TooDeeView {
            data: &self.data[data_start..data_start + data_len],
            start: (self.start.0 + start.0, self.start.1 + start.1),
            num_cols,
            num_rows,
            main_cols : self.main_cols,
        }
    }

    fn rows(&self) -> Rows<'_, T> {
        Rows {
            v : &self.data,
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
        }
    }

    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
        unsafe {
            Col {
                v : self.data.get_unchecked(start..end),
                skip : self.main_cols - 1,
            }
        }
    }

    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let mut view = toodee.view_mut((0,0), (10,5));
    ///     let row = view.get_unchecked_row(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * self.main_cols;
        self.data.get_unchecked(start..start + self.num_cols)
    }
    
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut view = toodee.view_mut((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
    }

}

impl<'a, T> TooDeeOpsMut<T> for TooDeeViewMut<'a,T> {

    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        let (num_cols, num_rows) = calculate_view_dimensions(start, end, self);

        let data_start = start.1 * self.main_cols + start.0;
        let data_len = {
            if num_rows == 0 {
                0
            } else {
                (num_rows - 1) * self.main_cols + num_cols
            }
        };

        unsafe {
            TooDeeViewMut {
                data: self.data.get_unchecked_mut(data_start..data_start + data_len),
                start: (self.start.0 + start.0, self.start.1 + start.1),
                num_cols,
                num_rows,
                main_cols : self.main_cols,
            }
        }
    }
    
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            v : &mut self.data,
            cols : self.num_cols,
            skip_cols : self.main_cols - self.num_cols,
        }
    }

    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        assert!(col < self.num_cols);
        let start = col;
        let end = {
            if self.num_rows == 0 {
                start
            } else {
                start + (self.num_rows - 1) * self.main_cols + 1
            }
        };
        unsafe {
            ColMut {
                v : self.data.get_unchecked_mut(start..end),
                skip : self.main_cols - 1,
            }
        }
    }

    /// Swap/exchange the data between two rows.
    /// 
    /// # Panics
    /// 
    /// Panics if either row index is out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// toodee[0].iter_mut().for_each(|v| *v = 1);
    /// assert_eq!(toodee[(0, 2)], 42);
    /// toodee.swap_rows(0, 2);
    /// assert_eq!(toodee[(0, 2)], 1);
    /// ```
    fn swap_rows(&mut self, mut r1: usize, mut r2: usize) {
        match r1.cmp(&r2) {
            Ordering::Less => {},
            Ordering::Greater => {
                // force r1 < r2
                core::mem::swap(&mut r1, &mut r2);
            },
            Ordering::Equal => {
                // swapping a row with itself
                return;
            }
        }
        assert!(r2 < self.num_rows);
        let num_cols = self.num_cols;
        unsafe {
            let (first, rest) = self.data.get_unchecked_mut(r1 * self.main_cols..).split_at_mut(num_cols);
            let snd_idx = (r2 - r1) * self.main_cols - num_cols;
            let second = rest.get_unchecked_mut(snd_idx..snd_idx + num_cols);
            // Both slices are guaranteed to have the same length
            debug_assert_eq!(first.len(), num_cols);
            debug_assert_eq!(second.len(), num_cols);
            // We know that the two slices will not overlap because r1 != r2, and we used split_at_mut()
            ptr::swap_nonoverlapping(first.as_mut_ptr(), second.as_mut_ptr(), num_cols);
        }
    }

    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let mut view = toodee.view_mut((0,0), (10,5));
    ///     let row = view.get_unchecked_row_mut(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.main_cols;
        self.data.get_unchecked_mut(start..start + self.num_cols)
    }

    
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut view = toodee.view_mut((0,0), (10,5));
    /// unsafe {
    ///     assert_eq!(*view.get_unchecked_mut((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked_mut(&mut self, coord: Coordinate) -> &mut T {
        self.data.get_unchecked_mut(coord.1 * self.main_cols + coord.0)
    }

}

impl<'a, T> Index<usize> for TooDeeViewMut<'a, T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.main_cols;
        unsafe {
            &self.data.get_unchecked(start..start + self.num_cols)
        }
    }
}

impl<'a, T> Index<Coordinate> for TooDeeViewMut<'a, T> {
    type Output = T;
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked(coord.1 * self.main_cols + coord.0)
        }
    }
}

impl<'a, T> IndexMut<usize> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.main_cols;
        unsafe {
            self.data.get_unchecked_mut(start..start + self.num_cols)
        }
    }
}

impl<'a, T> IndexMut<Coordinate> for TooDeeViewMut<'a, T> {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        // can access the element unchecked because the above assertions hold
        unsafe {
            self.data.get_unchecked_mut(coord.1 * self.main_cols + coord.0)
        }
    }
}

impl<'a, T> Into<TooDeeView<'a, T>> for TooDeeViewMut<'a, T> {
    fn into(self) -> TooDeeView<'a, T> {
        TooDeeView {
            data:      self.data,
            start:     self.start,
            num_cols:  self.num_cols,
            num_rows:  self.num_rows,
            main_cols: self.main_cols,
        }
    }
}

impl<'a, T> IntoIterator for &'a TooDeeView<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, T> IntoIterator for &'a TooDeeViewMut<'a, T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}

impl<'a, T> IntoIterator for &'a mut TooDeeViewMut<'a, T> {
    type Item = &'a mut T;
    type IntoIter = CellsMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells_mut()
    }
}

impl<T> Debug for TooDeeView<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}

impl<T> Debug for TooDeeViewMut<'_, T> where T : Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}
