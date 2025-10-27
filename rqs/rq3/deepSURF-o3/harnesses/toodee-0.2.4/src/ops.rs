use core::ops::{Index, IndexMut};
use core::cmp::Ordering;
use core::borrow::Borrow;
use core::ptr;

use crate::iter::*;
use crate::view::*;
use crate::flattenexact::*;

/// A (col, row) coordinate in 2D space.
pub type Coordinate = (usize, usize);

/// An iterator over each "cell" in a 2D array
pub type Cells<'a, T> = FlattenExact<Rows<'a, T>>;
/// A mutable iterator over each "cell" in a 2D array
pub type CellsMut<'a, T> = FlattenExact<RowsMut<'a, T>>;

/// Defines operations common to both `TooDee` and `TooDeeView`. Default implementations are provided
/// where possible/practical.
pub trait TooDeeOps<T> : Index<usize, Output=[T]> + Index<Coordinate, Output=T> {
    
    /// The number of columns in the area represented by this object.
    fn num_cols(&self) -> usize;
    /// The number of rows in the area represented by this object.
    fn num_rows(&self) -> usize;
    
    /// Returns the size/dimensions of the current object.
    fn size(&self) -> (usize, usize) {
        (self.num_cols(), self.num_rows())
    }

    /// Returns `true` if the array contains no elements.
    fn is_empty(&self) -> bool {
        self.num_cols() == 0 || self.num_rows() == 0
    }

    /// Returns the bounds of the object's area within the original `TooDee` area (views
    /// are not nested for now).
    fn bounds(&self) -> (Coordinate, Coordinate);
    
    /// Returns a view (or subset) of the current area based on the coordinates provided.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view((1, 1), (9, 4));
    /// assert_eq!(view.num_cols(), 8);
    /// assert_eq!(view.num_rows(), 3);
    /// ```
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T>;
    
    /// Returns an iterator of slices, where each slice represents an entire row.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// let mut sum = 0u32;
    /// for r in toodee.rows() {
    ///     sum += r.iter().sum::<u32>();
    /// }
    /// assert_eq!(sum, 42*50);
    /// ```
    fn rows(&self) -> Rows<'_, T>;
    
    /// Returns an iterator over a single column. Note that the `Col` iterator is indexable.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// let mut sum = 0u32;
    /// for c in toodee.col(1) {
    ///     sum += c;
    /// }
    /// assert_eq!(sum, 42*5);
    /// ```
    fn col(&self, col: usize) -> Col<'_, T>;

    /// Returns an iterator that traverses all cells within the area.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// let mut sum = toodee.cells().sum::<u32>();
    /// assert_eq!(sum, 42*50);
    /// ```
    fn cells(&self) -> Cells<'_, T> {
        FlattenExact::new(self.rows())
    }
    
    /// Returns a row without checking that the row is valid. Generally it's best to use indexing instead, e.g., toodee[row]
    /// 
    /// # Safety
    /// 
    /// This is generally not recommended, use with caution!
    /// Calling this method with an invalid row is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T];

    /// Returns a cell without checking that the cell coordinate is valid. Generally it's best to use indexing instead, e.g., toodee[(col, row)]
    /// 
    /// # Safety
    /// 
    /// This is generally not recommended, use with caution!
    /// Calling this method with an invalid coordinate is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T;

}

/// Defines operations common to both `TooDee` and `TooDeeViewMut`. Default implementations
/// are provided where possible/practical.
pub trait TooDeeOpsMut<T> : TooDeeOps<T> + IndexMut<usize,Output=[T]>  + IndexMut<Coordinate, Output=T> {

    /// Returns a mutable view (or subset) of the current area based on the coordinates provided.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view_mut((1, 1), (9, 4));
    /// assert_eq!(view.num_cols(), 8);
    /// assert_eq!(view.num_rows(), 3);
    /// ```
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T>;
    
    /// Returns a mutable iterator of slices, where each slice represents an entire row.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for (i, r) in toodee.rows_mut().enumerate() {
    ///    r.iter_mut().for_each(|c| *c -= i as u32);
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*50 - 10 - 20 - 30 - 40);
    /// ```
    fn rows_mut(&mut self) -> RowsMut<'_, T>;
    
    /// Returns a mutable iterator over a single column. Note that the `ColMut` iterator is indexable.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for c in toodee.col_mut(4) {
    ///     *c /= 2;
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*45 + 21*5);
    /// ```
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T>;
    
    /// Returns an iterator that traverses all cells within the area.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for c in toodee.cells_mut() {
    ///     *c -= 1;
    /// }
    /// assert_eq!(toodee.cells().sum::<u32>(), 41*50);
    /// ```
    fn cells_mut(&mut self) -> CellsMut<'_, T> {
        FlattenExact::new(self.rows_mut())
    }
    
    /// Fills the entire area with the specified value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// let mut view = toodee.view_mut((1, 1), (9, 4));
    /// view.fill(0);
    /// assert_eq!(toodee.cells().sum::<u32>(), 42*(50 - 8*3));
    /// ```
    fn fill<V>(&mut self, fill: V)
    where
        V: Borrow<T>,
        T: Clone {
        let value = fill.borrow();
        for r in self.rows_mut() {
            for v in r {
                v.clone_from(value);
            }
        }
    }
    
    /// Swap/exchange the data between two columns.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// for c in toodee.col_mut(2) {
    ///     *c = 1;
    /// }
    /// assert_eq!(toodee[(4, 0)], 42);
    /// toodee.swap_cols(2, 4);
    /// assert_eq!(toodee[(4, 0)], 1);
    /// ```
    fn swap_cols(&mut self, c1: usize, c2: usize) {
        let num_cols = self.num_cols();
        assert!(c1 < num_cols);
        assert!(c2 < num_cols);
        for r in self.rows_mut() {
            // The column indices have been checked with asserts (see above), so we can
            // safely access and swap the elements using `get_unchecked_mut`.
            unsafe {
                let pa: *mut T = r.get_unchecked_mut(c1);
                let pb: *mut T = r.get_unchecked_mut(c2);
                ptr::swap(pa, pb);
            }
        }
    }
    
    /// Swap/exchange the data between two rows. Note that this method is overridden in both `TooDee` and `TooDeeOpsMut`.
    /// This implementation remains in place for other types that may wish to implement the trait.
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
    /// toodee.view_mut((0, 0), (10, 5)).swap_rows(0, 2);
    /// assert_eq!(toodee[(0, 2)], 1);
    /// ```
    fn swap_rows(&mut self, mut r1: usize, mut r2: usize) {
        match r1.cmp(&r2) {
            Ordering::Less => {},
            Ordering::Greater => {
                core::mem::swap(&mut r1, &mut r2);
            },
            Ordering::Equal => {
                return;
            }
        }
        assert!(r2 < self.num_rows());
        let mut iter = self.rows_mut();
        let tmp = iter.nth(r1).unwrap();
        tmp.swap_with_slice(iter.nth(r2-r1-1).unwrap());
    }
    
    /// Return the specified rows as mutable slices.
    /// 
    /// # Panics
    ///
    /// Will panic if `r1` and `r2` are equal, or if either row index is out of bounds.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
    /// let (r1, r2) = toodee.row_pair_mut(0, 4);
    /// // do something with the row pair
    /// r1.swap_with_slice(r2);
    /// ```
    fn row_pair_mut(&mut self, r1: usize, r2: usize) -> (&mut [T], &mut [T]) {
        let num_rows = self.num_rows();
        assert!(r1 < num_rows);
        assert!(r2 < num_rows);
        assert!(r1 != r2);
        match r1.cmp(&r2) {
            Ordering::Less => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r1).unwrap();
                (tmp, iter.nth(r2-r1-1).unwrap())
            },
            Ordering::Greater => {
                let mut iter = self.rows_mut();
                let tmp = iter.nth(r2).unwrap();
                (iter.nth(r1-r2-1).unwrap(), tmp)
            },
            Ordering::Equal => {
                unreachable!("r1 != r2");
            },
        }
    }
    
    /// Returns a mutable row without checking that the row is valid. Generally it's best to use indexing instead, e.g., toodee[row]
    /// 
    /// # Safety
    /// 
    /// This is generally not recommended, use with caution!
    /// Calling this method with an invalid row is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn get_unchecked_row_mut(&mut self, row: usize) -> &mut [T];

    /// Returns a mutable cell without checking that the cell coordinate is valid. Generally it's best to use indexing instead, e.g., toodee[(col, row)]
    /// 
    /// # Safety
    /// 
    /// This is generally not recommended, use with caution!
    /// Calling this method with an invalid coordinate is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn get_unchecked_mut(&mut self, coord: Coordinate) -> &mut T;

}

