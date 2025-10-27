use core::fmt;
use core::fmt::{Formatter, Debug};
use core::ops::{Index, IndexMut};
use core::borrow::Borrow;
use core::iter::IntoIterator;
use core::ptr::{self, NonNull};
use core::mem;
use core::slice;
use core::cmp::Ordering;
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc::vec::Drain;
use alloc::vec::IntoIter;
use crate::iter::*;
use crate::view::*;
use crate::ops::*;
/// DrainRow type alias for future-proofing.
pub type DrainRow<'a, T> = Drain<'a, T>;
/// IntoIter type alias for future-proofing.
pub type IntoIterTooDee<T> = IntoIter<T>;
/// Represents a two-dimensional array.
///
/// Empty arrays will always have dimensions of zero.
#[derive(Clone, Hash, Eq, PartialEq)]
pub struct TooDee<T> {
    data: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}
/// Custom `Default` implementation because `T` does not need to implement `Default`.
/// See rust issue [#26925](https://github.com/rust-lang/rust/issues/26925)
impl<T> Default for TooDee<T> {
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// struct Abc { }
    /// let toodee : TooDee<Abc> = TooDee::default();
    /// ```
    fn default() -> Self {
        TooDee {
            data: Vec::default(),
            num_rows: 0,
            num_cols: 0,
        }
    }
}
impl<T> Index<usize> for TooDee<T> {
    type Output = [T];
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let row = &toodee[3];
    /// assert_eq!(row.len(), 10);
    /// ```
    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.num_cols;
        unsafe { self.data.get_unchecked(start..start + self.num_cols) }
    }
}
impl<T> Index<Coordinate> for TooDee<T> {
    type Output = T;
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee[(1,3)], 0);
    /// ```
    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        unsafe { self.data.get_unchecked(coord.1 * self.num_cols + coord.0) }
    }
}
impl<T> IndexMut<usize> for TooDee<T> {
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut row = &mut toodee[3];
    /// row[0] = 42;
    /// ```
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.num_rows);
        let start = row * self.num_cols;
        unsafe { self.data.get_unchecked_mut(start..start + self.num_cols) }
    }
}
impl<T> IndexMut<Coordinate> for TooDee<T> {
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee[(1,3)], 0);
    /// ```
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(coord.1 < self.num_rows);
        assert!(coord.0 < self.num_cols);
        unsafe { self.data.get_unchecked_mut(coord.1 * self.num_cols + coord.0) }
    }
}
impl<T> TooDeeOps<T> for TooDee<T> {
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee.num_cols(), 10);
    ///
    fn num_cols(&self) -> usize {
        self.num_cols
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee.num_rows(), 5);
    ///
    fn num_rows(&self) -> usize {
        self.num_rows
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee.bounds(), ((0, 0), (10, 5)));
    /// ```
    fn bounds(&self) -> (Coordinate, Coordinate) {
        ((0, 0), (self.num_cols, self.num_rows))
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view((1,2), (8,4));
    /// assert_eq!(view.num_cols(), 7);
    /// assert_eq!(view.num_rows(), 2);
    /// ```
    fn view(&self, start: Coordinate, end: Coordinate) -> TooDeeView<'_, T> {
        TooDeeView::from_toodee(start, end, self)
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut rows = toodee.rows();
    /// assert_eq!(rows.len(), 5);
    /// let r0 = rows.next().unwrap();
    /// assert_eq!(r0.len(), 10);
    /// ```
    fn rows(&self) -> Rows<'_, T> {
        Rows {
            v: &self.data,
            cols: self.num_cols,
            skip_cols: 0,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut col = toodee.col(8);
    /// assert_eq!(col.len(), 5);
    /// ```
    fn col(&self, col: usize) -> Col<'_, T> {
        assert!(col < self.num_cols);
        unsafe {
            Col {
                v: self
                    .data
                    .get_unchecked(col..self.data.len() - self.num_cols + col + 1),
                skip: self.num_cols - 1,
            }
        }
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let row = toodee.get_unchecked_row(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row(&self, row: usize) -> &[T] {
        let start = row * self.num_cols;
        self.data.get_unchecked(start..start + self.num_cols)
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// unsafe {
    ///     assert_eq!(*toodee.get_unchecked((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked(&self, coord: Coordinate) -> &T {
        self.data.get_unchecked(coord.1 * self.num_cols + coord.0)
    }
}
impl<T> TooDeeOpsMut<T> for TooDee<T> {
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let view = toodee.view_mut((1,2), (8,4));
    /// assert_eq!(view.num_cols(), 7);
    /// assert_eq!(view.num_rows(), 2);
    /// ```
    fn view_mut(&mut self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<'_, T> {
        TooDeeViewMut::from_toodee(start, end, self)
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut rows = toodee.rows_mut();
    /// assert_eq!(rows.len(), 5);
    /// let r0 = rows.next().unwrap();
    /// assert_eq!(r0.len(), 10);
    /// ```
    fn rows_mut(&mut self) -> RowsMut<'_, T> {
        RowsMut {
            v: &mut self.data,
            cols: self.num_cols,
            skip_cols: 0,
        }
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// let mut col = toodee.col_mut(8);
    /// assert_eq!(col.len(), 5);
    /// ```
    fn col_mut(&mut self, col: usize) -> ColMut<'_, T> {
        assert!(col < self.num_cols);
        let dlen = self.data.len();
        unsafe {
            ColMut {
                v: self.data.get_unchecked_mut(col..dlen - self.num_cols + col + 1),
                skip: self.num_cols - 1,
            }
        }
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// toodee.fill(42);
    /// assert_eq!(toodee[1][1], 42);
    /// ```
    fn fill<V>(&mut self, fill: V)
    where
        V: Borrow<T>,
        T: Clone,
    {
        let value = fill.borrow();
        for v in self.data.iter_mut() {
            v.clone_from(value);
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
            Ordering::Less => {}
            Ordering::Greater => {
                core::mem::swap(&mut r1, &mut r2);
            }
            Ordering::Equal => {
                return;
            }
        }
        assert!(r2 < self.num_rows);
        let num_cols = self.num_cols;
        unsafe {
            let (first, rest) = self
                .data
                .get_unchecked_mut(r1 * num_cols..)
                .split_at_mut(num_cols);
            let snd_idx = (r2 - r1 - 1) * num_cols;
            let second = rest.get_unchecked_mut(snd_idx..snd_idx + num_cols);
            debug_assert_eq!(first.len(), num_cols);
            debug_assert_eq!(second.len(), num_cols);
            ptr::swap_nonoverlapping(first.as_mut_ptr(), second.as_mut_ptr(), num_cols);
        }
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// unsafe {
    ///     let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    ///     let row = toodee.get_unchecked_row_mut(3);
    ///     assert_eq!(row.len(), 10);
    /// }
    /// ```
    unsafe fn get_unchecked_row_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.num_cols;
        self.data.get_unchecked_mut(start..start + self.num_cols)
    }
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
    /// let mut toodee : TooDee<u32> = TooDee::new(10, 5);
    /// unsafe {
    ///     assert_eq!(*toodee.get_unchecked_mut((1,3)), 0);
    /// }
    /// ```
    unsafe fn get_unchecked_mut(&mut self, coord: Coordinate) -> &mut T {
        self.data.get_unchecked_mut(coord.1 * self.num_cols + coord.0)
    }
}
impl<T> TooDee<T> {
    /// Create a new `TooDee` array of the specified dimensions, and fill it with
    /// the type's default value.
    ///
    /// # Panics
    ///
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee : TooDee<u32> = TooDee::new(10, 5);
    /// assert_eq!(toodee.num_cols(), 10);
    /// assert_eq!(toodee.num_rows(), 5);
    /// assert_eq!(toodee[0][0], 0);
    /// ```
    pub fn new(num_cols: usize, num_rows: usize) -> TooDee<T>
    where
        T: Default + Clone,
    {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let len = num_rows * num_cols;
        let v = vec![T::default(); len];
        TooDee {
            data: v,
            num_cols,
            num_rows,
        }
    }
    /// Create a new `TooDee` array of the specified dimensions, and fill it with
    /// an initial value.
    ///
    /// # Panics
    ///
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let toodee = TooDee::init(10, 5, 42u32);
    /// assert_eq!(toodee.num_cols(), 10);
    /// assert_eq!(toodee.num_rows(), 5);
    /// assert_eq!(toodee[0][0], 42);
    /// ```
    pub fn init(num_cols: usize, num_rows: usize, init_value: T) -> TooDee<T>
    where
        T: Clone,
    {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        let len = num_rows * num_cols;
        let v = vec![init_value; len];
        TooDee {
            data: v,
            num_cols,
            num_rows,
        }
    }
    /// Returns the element capacity of the underlying `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// let v = vec![42u32; 10];
    /// let toodee : TooDee<u32> = TooDee::from_vec(5, 2, v);
    /// assert!(toodee.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    /// Constructs a new, empty `TooDee<T>` with the specified element capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// let toodee : TooDee<u32> = TooDee::with_capacity(50);
    /// assert!(toodee.capacity() >= 50);
    /// ```
    pub fn with_capacity(capacity: usize) -> TooDee<T> {
        TooDee {
            data: Vec::with_capacity(capacity),
            num_cols: 0,
            num_rows: 0,
        }
    }
    /// Reserves the minimum capacity for at least `additional` more elements to be inserted
    /// into the `TooDee<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// let mut toodee : TooDee<u32> = TooDee::default();
    /// toodee.reserve_exact(50);
    /// assert_eq!(toodee.capacity(), 50);
    /// ```
    pub fn reserve_exact(&mut self, capacity: usize) {
        self.data.reserve_exact(capacity);
    }
    /// Reserves capacity for at least `additional` more elements to be inserted
    /// in the given `TooDee<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// let mut toodee : TooDee<u32> = TooDee::default();
    /// toodee.reserve(50);
    /// assert!(toodee.capacity() >= 50);
    /// ```
    pub fn reserve(&mut self, capacity: usize) {
        self.data.reserve(capacity);
    }
    /// Shrinks the capacity of the underlying vector as much as possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::TooDee;
    /// let mut toodee : TooDee<u32> = TooDee::with_capacity(50);
    /// toodee.shrink_to_fit();
    /// assert_eq!(toodee.capacity(), 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }
    /// Create a new `TooDee` array using the provided vector. The vector's length
    /// must match the dimensions of the array.
    ///
    /// # Panics
    ///
    /// Panics if one of the dimensions is zero but the other is non-zero. This
    /// is to enforce the rule that empty arrays have no dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 10];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 2, v);
    /// assert_eq!(toodee.num_cols(), 5);
    /// assert_eq!(toodee.num_rows(), 2);
    /// assert_eq!(toodee[0][0], 42);
    /// ```
    pub fn from_vec(num_cols: usize, num_rows: usize, v: Vec<T>) -> TooDee<T> {
        if num_cols == 0 || num_rows == 0 {
            assert_eq!(num_rows, num_cols);
        }
        assert_eq!(num_cols * num_rows, v.len());
        TooDee {
            data: v,
            num_cols,
            num_rows,
        }
    }
    /// Create a new `TooDee` array using the provided boxed slice. The slice's length
    /// must match the dimensions of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 10];
    /// let mut toodee : TooDee<u32> = TooDee::from_box(5, 2, v.into_boxed_slice());
    /// assert_eq!(toodee.num_cols(), 5);
    /// assert_eq!(toodee.num_rows(), 2);
    /// assert_eq!(toodee[0][0], 42);
    /// ```
    pub fn from_box(num_cols: usize, num_rows: usize, b: Box<[T]>) -> TooDee<T> {
        TooDee::from_vec(num_cols, num_rows, b.into_vec())
    }
    /// Returns a reference to the raw array data
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 10];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 2, v);
    /// assert_eq!(toodee.data()[0], 42);
    /// ```
    pub fn data(&self) -> &[T] {
        &self.data
    }
    /// Returns a mutable reference to the raw array data
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 10];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 2, v);
    /// assert_eq!(toodee.data_mut()[0], 42);
    /// ```
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
    /// Clears the array, removing all values and zeroing the number of columns and rows.
    ///
    /// Note that this method has no effect on the allocated capacity of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 10];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 2, v);
    /// toodee.clear();
    /// assert_eq!(toodee.num_cols(), 0);
    /// assert_eq!(toodee.num_rows(), 0);
    /// assert!(toodee.capacity() >= 10);
    /// ```
    pub fn clear(&mut self) {
        self.num_cols = 0;
        self.num_rows = 0;
        self.data.clear();
    }
    /// Removes the last row from the array and returns it as a `Drain`, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// {
    ///    let drain = toodee.pop_row().unwrap();
    ///    assert_eq!(drain.len(), 5);
    /// }
    /// assert_eq!(toodee.num_cols(), 5);
    /// assert_eq!(toodee.num_rows(), 2);
    /// ```
    pub fn pop_row(&mut self) -> Option<DrainRow<'_, T>> {
        if self.num_rows == 0 { None } else { Some(self.remove_row(self.num_rows - 1)) }
    }
    /// Appends a new row to the array.
    ///
    /// # Panics
    ///
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn push_row<I>(&mut self, data: impl IntoIterator<Item = T, IntoIter = I>)
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        self.insert_row(self.num_rows, data);
    }
    /// Inserts new `data` into the array at the specified `row`
    ///
    /// # Panics
    ///
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn insert_row<I>(
        &mut self,
        index: usize,
        data: impl IntoIterator<Item = T, IntoIter = I>,
    )
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        assert!(index <= self.num_rows);
        let iter = data.into_iter();
        if self.num_rows == 0 {
            self.num_cols = iter.len();
        } else {
            assert_eq!(self.num_cols, iter.len());
        }
        self.reserve(self.num_cols);
        let start = index * self.num_cols;
        let len = self.data.len();
        unsafe {
            let mut p = self.data.as_mut_ptr().add(start);
            ptr::copy(p, p.add(self.num_cols), len - start);
            for e in iter {
                ptr::write(p, e);
                p = p.add(1);
            }
            self.data.set_len(len + self.num_cols);
        }
        self.num_rows += 1;
    }
    /// Removes the specified row from the array and returns it as a `Drain`
    ///
    /// # Panics
    ///
    /// Panics if the specified row index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// {
    ///    let drain = toodee.remove_row(1);
    ///    assert_eq!(drain.len(), 5);
    /// }
    /// assert_eq!(toodee.num_cols(), 5);
    /// assert_eq!(toodee.num_rows(), 2);
    /// ```
    pub fn remove_row(&mut self, index: usize) -> DrainRow<'_, T> {
        assert!(index < self.num_rows);
        let start = index * self.num_cols;
        let drain = self.data.drain(start..start + self.num_cols);
        self.num_rows -= 1;
        if self.num_rows == 0 {
            self.num_cols = 0;
        }
        drain
    }
    /// Removes the last column from the array and returns it as a `Drain`, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// {
    ///    let drain = toodee.pop_col().unwrap();
    ///    assert_eq!(drain.len(), 3);
    /// }
    /// assert_eq!(toodee.num_cols(), 4);
    /// assert_eq!(toodee.num_rows(), 3);
    /// ```
    pub fn pop_col(&mut self) -> Option<DrainCol<'_, T>> {
        if self.num_cols == 0 { None } else { Some(self.remove_col(self.num_cols - 1)) }
    }
    /// Appends a new column to the array.
    ///
    /// # Panics
    ///
    /// Panics if the data's length doesn't match the length of existing rows (if any).
    pub fn push_col<I>(&mut self, data: impl IntoIterator<Item = T, IntoIter = I>)
    where
        I: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator,
    {
        self.insert_col(self.num_cols, data);
    }
    /// Removes the specified column from the array and returns it as a `Drain`
    ///
    /// # Panics
    ///
    /// Panics if the specified column index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use toodee::{TooDee,TooDeeOps};
    /// let v = vec![42u32; 15];
    /// let mut toodee : TooDee<u32> = TooDee::from_vec(5, 3, v);
    /// {
    ///    let drain = toodee.remove_col(1);
    ///    assert_eq!(drain.len(), 3);
    /// }
    /// assert_eq!(toodee.num_cols(), 4);
    /// assert_eq!(toodee.num_rows(), 3);
    /// ```
    pub fn remove_col(&mut self, index: usize) -> DrainCol<'_, T> {
        assert!(index < self.num_cols);
        let v = &mut self.data;
        let num_cols = self.num_cols;
        let slice_len = v.len() - num_cols + 1;
        unsafe {
            v.set_len(0);
            DrainCol {
                iter: Col {
                    skip: num_cols - 1,
                    v: slice::from_raw_parts_mut(v.as_mut_ptr().add(index), slice_len),
                },
                col: index,
                toodee: NonNull::from(self),
            }
        }
    }
    /// Inserts new `data` into the array at the specified `col`.
    ///
    /// # Panics
    ///
    /// Panics if the data's length doesn't match the length of existing columns (if any).
    pub fn insert_col<I>(
        &mut self,
        index: usize,
        data: impl IntoIterator<Item = T, IntoIter = I>,
    )
    where
        I: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator,
    {
        assert!(index <= self.num_cols);
        let iter = data.into_iter();
        if self.num_cols == 0 {
            self.num_rows = iter.len();
        } else {
            assert_eq!(self.num_rows, iter.len());
        }
        self.reserve(self.num_rows);
        let old_len = self.data.len();
        let new_len = old_len + self.num_rows;
        let suffix_len = self.num_cols - index;
        unsafe {
            let p = self.data.as_mut_ptr();
            let mut read_p = p.add(old_len);
            let mut write_p = p.add(new_len);
            for e in iter.rev() {
                read_p = read_p.sub(suffix_len);
                write_p = write_p.sub(suffix_len);
                ptr::copy(read_p, write_p, suffix_len);
                write_p = write_p.sub(1);
                ptr::write(write_p, e);
                read_p = read_p.sub(index);
                write_p = write_p.sub(index);
                ptr::copy(read_p, write_p, index);
            }
            self.data.set_len(new_len);
        }
        self.num_cols += 1;
    }
}
/// Use `Vec`'s `IntoIter` for performance reasons.
///
/// TODO: return type that implements `TooDeeIterator`
impl<T> IntoIterator for TooDee<T> {
    type Item = T;
    type IntoIter = IntoIterTooDee<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<'a, T> IntoIterator for &'a TooDee<T> {
    type Item = &'a T;
    type IntoIter = Cells<'a, T>;
    /// `Cells` is the preferred iterator type here, because it implements `TooDeeIterator`
    fn into_iter(self) -> Self::IntoIter {
        self.cells()
    }
}
impl<'a, T> IntoIterator for &'a mut TooDee<T> {
    type Item = &'a mut T;
    /// `CellsMut` is the preferred iterator type here, because it implements `TooDeeIterator`
    type IntoIter = CellsMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.cells_mut()
    }
}
/// Support conversion into a `Vec`.
impl<T> Into<Vec<T>> for TooDee<T> {
    fn into(self) -> Vec<T> {
        self.data
    }
}
/// Support conversion into a boxed slice.
impl<T> Into<Box<[T]>> for TooDee<T> {
    fn into(self) -> Box<[T]> {
        self.data.into_boxed_slice()
    }
}
impl<T> AsRef<[T]> for TooDee<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}
impl<T> AsMut<[T]> for TooDee<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
/// We can allow immutable access to the underlying `Vec`,
/// mut not mutable access because that could lead to changes
/// in the `Vec`'s length.
impl<T> AsRef<Vec<T>> for TooDee<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.data
    }
}
impl<T> Debug for TooDee<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.rows()).finish()
    }
}
impl<T> From<TooDeeView<'_, T>> for TooDee<T>
where
    T: Clone,
{
    fn from(view: TooDeeView<'_, T>) -> Self {
        let num_cols = view.num_cols();
        let num_rows = view.num_rows();
        let mut v = Vec::with_capacity(num_cols * num_rows);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            data: v,
            num_cols,
            num_rows,
        }
    }
}
impl<T> From<TooDeeViewMut<'_, T>> for TooDee<T>
where
    T: Clone,
{
    fn from(view: TooDeeViewMut<'_, T>) -> Self {
        let num_cols = view.num_cols();
        let num_rows = view.num_rows();
        let mut v = Vec::with_capacity(num_cols * num_rows);
        for r in view.rows() {
            v.extend_from_slice(r);
        }
        TooDee {
            data: v,
            num_cols,
            num_rows,
        }
    }
}
/// Drains a column.
#[derive(Debug)]
pub struct DrainCol<'a, T> {
    /// Current remaining elements to remove
    iter: Col<'a, T>,
    col: usize,
    toodee: NonNull<TooDee<T>>,
}
unsafe impl<T: Sync> Sync for DrainCol<'_, T> {}
unsafe impl<T: Send> Send for DrainCol<'_, T> {}
impl<T> Iterator for DrainCol<'_, T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|elt| unsafe { ptr::read(elt as *const _) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
impl<T> DoubleEndedIterator for DrainCol<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|elt| unsafe { ptr::read(elt as *const _) })
    }
}
impl<T> ExactSizeIterator for DrainCol<'_, T> {}
impl<T> Drop for DrainCol<'_, T> {
    fn drop(&mut self) {
        /// Continues dropping the remaining elements in the `DrainCol`, then repositions the
        /// un-`Drain`ed elements to restore the original `TooDee`.
        struct DropGuard<'r, 'a, T>(&'r mut DrainCol<'a, T>);
        impl<'r, 'a, T> Drop for DropGuard<'r, 'a, T> {
            fn drop(&mut self) {
                self.0.for_each(drop);
                let col = self.0.col;
                unsafe {
                    let toodee = self.0.toodee.as_mut();
                    let vec = &mut toodee.data;
                    let mut dest = vec.as_mut_ptr().add(col);
                    let mut src = dest.add(1);
                    let orig_cols = toodee.num_cols;
                    let new_cols = orig_cols - 1;
                    let num_rows = toodee.num_rows;
                    for _ in 1..num_rows {
                        ptr::copy(src, dest, new_cols);
                        src = src.add(orig_cols);
                        dest = dest.add(new_cols);
                    }
                    ptr::copy(src, dest, orig_cols - col);
                    toodee.num_cols -= 1;
                    if toodee.num_cols == 0 {
                        toodee.num_rows = 0;
                    }
                    vec.set_len(toodee.num_cols * toodee.num_rows);
                }
            }
        }
        while let Some(item) = self.next() {
            let guard = DropGuard(self);
            drop(item);
            mem::forget(guard);
        }
        DropGuard(self);
    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use crate::TooDee;
    use std::default::Default;
    struct Abc {}
    #[test]
    fn test_default() {
        let _rug_st_tests_rug_116_rrrruuuugggg_test_default = 0;
        let toodee: TooDee<Abc> = <TooDee<Abc> as Default>::default();
        let _rug_ed_tests_rug_116_rrrruuuugggg_test_default = 0;
    }
}
#[cfg(test)]
mod tests_rug_120 {
    use super::*;
    use crate::toodee::{TooDee, Coordinate};
    #[test]
    fn test_index_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let coord: Coordinate = (rug_fuzz_2, rug_fuzz_3);
        debug_assert_eq!(toodee[(rug_fuzz_4, rug_fuzz_5)], 0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_121 {
    use super::*;
    use crate::{TooDeeOps, TooDee};
    #[test]
    fn test_num_cols() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(
            < TooDee < u32 > as TooDeeOps < u32 > > ::num_cols(& toodee), 10
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_122 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_num_rows() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(
            < TooDee < u32 > as TooDeeOps < u32 > > ::num_rows(& toodee), 5
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_123 {
    use super::*;
    use crate::{Coordinate, TooDee, TooDeeOps};
    #[test]
    fn test_bounds() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        debug_assert_eq!(
            < TooDee < u32 > as TooDeeOps < u32 > > ::bounds(& toodee), ((0, 0), (10, 5))
        );
             }
});    }
}
#[cfg(test)]
mod tests_rug_124 {
    use super::*;
    use crate::{TooDeeOps, TooDee, TooDeeView};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let start = (rug_fuzz_2, rug_fuzz_3);
        let end = (rug_fuzz_4, rug_fuzz_5);
        let view = toodee.view(start, end);
        debug_assert_eq!(view.num_cols(), 7);
        debug_assert_eq!(view.num_rows(), 2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::TooDeeOps;
    use crate::{TooDeeOpsMut, TooDeeView, TooDee};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        p0.rows();
             }
});    }
}
#[cfg(test)]
mod tests_rug_126 {
    use super::*;
    use crate::{TooDeeOps, TooDeeOpsMut, TooDee};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let p1: usize = rug_fuzz_2;
        p0.col(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_127 {
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let row_index = rug_fuzz_2;
        unsafe {
            let row = toodee.get_unchecked_row(row_index);
            debug_assert_eq!(row.len(), 10);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_128 {
    use super::*;
    use crate::TooDeeOps;
    use crate::{TooDee, TooDeeOpsMut};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let coord = (rug_fuzz_2, rug_fuzz_3);
        unsafe {
            debug_assert_eq!(
                * < TooDee < u32 > as TooDeeOps < u32 > > ::get_unchecked(& toodee,
                coord), 0
            );
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_129 {
    use super::*;
    use crate::{TooDee, TooDeeOpsMut};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let p1 = (rug_fuzz_2, rug_fuzz_3);
        let p2 = (rug_fuzz_4, rug_fuzz_5);
        p0.view_mut(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_130 {
    use super::*;
    use crate::TooDeeOpsMut;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let mut rows = <TooDee<u32> as TooDeeOpsMut<u32>>::rows_mut(&mut toodee);
        debug_assert_eq!(rows.len(), 5);
        let r0 = rows.next().unwrap();
        debug_assert_eq!(r0.len(), 10);
             }
});    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::{TooDeeOpsMut, TooDeeOps};
    #[test]
    fn test_col_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut too_dee: crate::TooDee<u32> = crate::TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let col_index = rug_fuzz_2;
        let mut col_mut = <crate::TooDee<
            u32,
        > as crate::TooDeeOpsMut<u32>>::col_mut(&mut too_dee, col_index);
        debug_assert_eq!(col_mut.len(), 5);
             }
});    }
}
#[cfg(test)]
mod tests_rug_133 {
    use super::*;
    use crate::{TooDee, TooDeeOpsMut, TooDeeOps};
    #[test]
    fn test_swap_rows() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, u32, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1: usize = rug_fuzz_3;
        let mut p2: usize = rug_fuzz_4;
        p0.swap_rows(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_134 {
    use super::*;
    use crate::TooDeeOpsMut;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let p1: usize = rug_fuzz_2;
        unsafe {
            p0.get_unchecked_row_mut(p1);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_135 {
    use super::*;
    use crate::{Coordinate, TooDeeOpsMut, TooDee};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut toodee: TooDee<u32> = TooDee::new(rug_fuzz_0, rug_fuzz_1);
        let p0 = &mut toodee;
        let p1: Coordinate = (rug_fuzz_2, rug_fuzz_3);
        unsafe {
            <TooDee<u32> as TooDeeOpsMut<u32>>::get_unchecked_mut(p0, p1);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_136 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let _ = TooDee::<u32>::new(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_137 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let mut p2: u32 = rug_fuzz_2;
        TooDee::<u32>::init(p1, p0, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_138 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 10];
        let toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        TooDee::<u32>::capacity(&toodee);
             }
});    }
}
#[cfg(test)]
mod tests_rug_139 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_with_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let _ = TooDee::<u32>::with_capacity(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_140 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::default();
        let mut p1: usize = rug_fuzz_0;
        p0.reserve_exact(p1);
        debug_assert_eq!(p0.capacity(), 50);
             }
});    }
}
#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::default();
        let p1: usize = rug_fuzz_0;
        p0.reserve(p1);
        debug_assert!(p0.capacity() >= rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_142 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_shrink_to_fit() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::with_capacity(rug_fuzz_0);
        p0.shrink_to_fit();
        debug_assert_eq!(p0.capacity(), 0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_143 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_from_vec() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        let mut p2: Vec<u32> = vec![rug_fuzz_2, 2, 3, 4, 5, 6];
        let _result = TooDee::<u32>::from_vec(p0, p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_144 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, u32, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let p1: usize = rug_fuzz_1;
        let v53: Box<[u32]> = Box::new([rug_fuzz_2, rug_fuzz_3, rug_fuzz_4]);
        let mut p2 = v53;
        <TooDee<u32>>::from_box(p0, p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 10];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        <TooDee<u32>>::data(&toodee);
             }
});    }
}
#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 10];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        <TooDee<u32>>::data_mut(&mut toodee);
        debug_assert_eq!(toodee.data_mut() [rug_fuzz_2], 42);
             }
});    }
}
#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_clear() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 10];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        <TooDee<u32>>::clear(&mut toodee);
        debug_assert_eq!(toodee.num_cols(), 0);
        debug_assert_eq!(toodee.num_rows(), 0);
        debug_assert!(toodee.capacity() >= rug_fuzz_2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_148 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        <TooDee<u32>>::pop_row(&mut toodee);
             }
});    }
}
#[cfg(test)]
mod tests_rug_149 {
    use super::*;
    use crate::TooDee;
    #[test]
    fn test_push_row() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let p1 = vec![rug_fuzz_3, 2, 3];
        p0.push_row(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_150 {
    use super::*;
    use crate::{TooDee, TooDeeView};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(usize, usize, u32, usize, usize, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let p1: usize = rug_fuzz_3;
        let mut p2: TooDee<u32> = TooDee::init(rug_fuzz_4, rug_fuzz_5, rug_fuzz_6);
        p0.insert_row(p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_151 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_remove_row() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        let index = rug_fuzz_2;
        toodee.remove_row(index);
        debug_assert_eq!(toodee.num_cols(), 5);
        debug_assert_eq!(toodee.num_rows(), 2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_152 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        <TooDee<u32>>::pop_col(&mut toodee);
             }
});    }
}
#[cfg(test)]
mod tests_rug_153 {
    use super::*;
    use crate::{TooDee, TooDeeView};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, u32, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v2: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let mut p1 = vec![rug_fuzz_3, 2, 3].into_iter();
        v2.push_col(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_154 {
    use super::*;
    use crate::{TooDee, TooDeeOps};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let v = vec![42u32; 15];
        let mut toodee: TooDee<u32> = TooDee::from_vec(rug_fuzz_0, rug_fuzz_1, v);
        let index = rug_fuzz_2;
        toodee.remove_col(index);
             }
});    }
}
#[cfg(test)]
mod tests_rug_155 {
    use super::*;
    use crate::{TooDee, TooDeeView};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, u32, usize, u32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut v1: TooDee<u32> = TooDee::init(rug_fuzz_0, rug_fuzz_1, rug_fuzz_2);
        let index = rug_fuzz_3;
        let data = vec![rug_fuzz_4, 2, 3];
        TooDee::<u32>::insert_col(&mut v1, index, data);
             }
});    }
}
