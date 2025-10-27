# Crate Documentation

**Version:** 0.2.4

**Format Version:** 39

# Module `toodee`


A lightweight two-dimensional wrapper around a `Vec`.

## Modules

## Module `iter`

```rust
pub(crate) mod iter { /* ... */ }
```

### Types

#### Struct `Rows`

An `Iterator` over each row of a `TooDee[View]`, where each row is represented as a slice.

```rust
pub struct Rows<''a, T> {
    pub(crate) v: &''a [T],
    pub(crate) cols: usize,
    pub(crate) skip_cols: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `v` | `&''a [T]` | This reference contains row data at each end. When iterating in either direction the row will<br>be pulled off the end then `skip_cols` elements will be skipped in preparation for reading the<br>next row. |
| `cols` | `usize` |  |
| `skip_cols` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TooDeeIterator**
  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `RowsMut`

A mutable Iterator over each row of a `TooDee[ViewMut]`, where each row is represented as a slice.

```rust
pub struct RowsMut<''a, T> {
    pub(crate) v: &''a mut [T],
    pub(crate) cols: usize,
    pub(crate) skip_cols: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `v` | `&''a mut [T]` | This reference contains row data at each end. When iterating in either direction the row will<br>be pulled off the end then `skip_cols` elements will be skipped in preparation for reading the<br>next row. |
| `cols` | `usize` |  |
| `skip_cols` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TooDeeIterator**
  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
#### Struct `Col`

An iterator over a single column.

```rust
pub struct Col<''a, T> {
    pub(crate) v: &''a [T],
    pub(crate) skip: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `v` | `&''a [T]` |  |
| `skip` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Index**
  - ```rust
    fn index(self: &Self, idx: usize) -> &<Self as >::Output { /* ... */ }
    ```
    # Examples

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ExactSizeIterator**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

#### Struct `ColMut`

A mutable iterator over a single column.

```rust
pub struct ColMut<''a, T> {
    pub(crate) v: &''a mut [T],
    pub(crate) skip: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `v` | `&''a mut [T]` |  |
| `skip` | `usize` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, idx: usize) -> &<Self as >::Output { /* ... */ }
    ```
    # Examples

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, idx: usize) -> &mut <Self as >::Output { /* ... */ }
    ```
    # Examples

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **ExactSizeIterator**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

### Traits

#### Trait `TooDeeIterator`

An `Iterator` that knows how many columns it emits per row.

```rust
pub trait TooDeeIterator: Iterator {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `num_cols`

The number of columns the iterator emits per row



##### Implementations

This trait is implemented for the following types:

- `Rows<''_, T>` with <T>
- `RowsMut<''_, T>` with <T>
- `FlattenExact<I>` with <I>

## Module `view`

```rust
pub(crate) mod view { /* ... */ }
```

### Types

#### Struct `TooDeeView`

Provides a read-only view (or subset) of a `TooDee` array.

```rust
pub struct TooDeeView<''a, T> {
    pub(in ::view) data: &''a [T],
    pub(in ::view) num_cols: usize,
    pub(in ::view) num_rows: usize,
    pub(in ::view) main_cols: usize,
    pub(in ::view) start: Coordinate,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `&''a [T]` |  |
| `num_cols` | `usize` |  |
| `num_rows` | `usize` |  |
| `main_cols` | `usize` |  |
| `start` | `Coordinate` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(num_cols: usize, num_rows: usize, data: &''a [T]) -> TooDeeView<''a, T> { /* ... */ }
  ```
  Create a new `TooDeeViewMut` using the provided slice reference.

- ```rust
  pub(crate) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &''a TooDee<T>) -> TooDeeView<''a, T> { /* ... */ }
  ```
  Used internally by `TooDee` to create a `TooDeeView`.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TooDeeView<''a, T>) -> bool { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TooDeeOps**
  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn num_rows(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn bounds(self: &Self) -> (Coordinate, Coordinate) { /* ... */ }
    ```

  - ```rust
    fn view(self: &Self, start: Coordinate, end: Coordinate) -> TooDeeView<''_, T> { /* ... */ }
    ```

  - ```rust
    fn rows(self: &Self) -> Rows<''_, T> { /* ... */ }
    ```

  - ```rust
    fn col(self: &Self, col: usize) -> Col<''_, T> { /* ... */ }
    ```

  - ```rust
    unsafe fn get_unchecked_row(self: &Self, row: usize) -> &[T] { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked(self: &Self, coord: Coordinate) -> &T { /* ... */ }
    ```
    # Examples

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(view: TooDeeView<''_, T>) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TooDeeView<''a, T> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **StructuralPartialEq**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> TooDeeView<''a, T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Index**
  - ```rust
    fn index(self: &Self, row: usize) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, coord: Coordinate) -> &<Self as >::Output { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

#### Struct `TooDeeViewMut`

Provides a mutable view (or subset), of a `TooDee` array.

```rust
pub struct TooDeeViewMut<''a, T> {
    pub(in ::view) data: &''a mut [T],
    pub(in ::view) num_cols: usize,
    pub(in ::view) num_rows: usize,
    pub(in ::view) main_cols: usize,
    pub(in ::view) start: Coordinate,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `&''a mut [T]` |  |
| `num_cols` | `usize` |  |
| `num_rows` | `usize` |  |
| `main_cols` | `usize` |  |
| `start` | `Coordinate` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(num_cols: usize, num_rows: usize, data: &''a mut [T]) -> TooDeeViewMut<''a, T> { /* ... */ }
  ```
  Create a new `TooDeeViewMut` using the provided mutable slice reference.

- ```rust
  pub(crate) fn from_toodee(start: Coordinate, end: Coordinate, toodee: &''a mut TooDee<T>) -> TooDeeViewMut<''a, T> { /* ... */ }
  ```
  Used internally by `TooDee` to create a `TooDeeViewMut`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **TooDeeOps**
  - ```rust
    fn num_rows(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn bounds(self: &Self) -> (Coordinate, Coordinate) { /* ... */ }
    ```

  - ```rust
    fn view(self: &Self, start: Coordinate, end: Coordinate) -> TooDeeView<''_, T> { /* ... */ }
    ```

  - ```rust
    fn rows(self: &Self) -> Rows<''_, T> { /* ... */ }
    ```

  - ```rust
    fn col(self: &Self, col: usize) -> Col<''_, T> { /* ... */ }
    ```

  - ```rust
    unsafe fn get_unchecked_row(self: &Self, row: usize) -> &[T] { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked(self: &Self, coord: Coordinate) -> &T { /* ... */ }
    ```
    # Examples

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CopyOps**
- **Sync**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, row: usize) -> &mut <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index_mut(self: &mut Self, coord: Coordinate) -> &mut <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TooDeeOpsMut**
  - ```rust
    fn view_mut(self: &mut Self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<''_, T> { /* ... */ }
    ```

  - ```rust
    fn rows_mut(self: &mut Self) -> RowsMut<''_, T> { /* ... */ }
    ```

  - ```rust
    fn col_mut(self: &mut Self, col: usize) -> ColMut<''_, T> { /* ... */ }
    ```

  - ```rust
    fn swap_rows(self: &mut Self, r1: usize, r2: usize) { /* ... */ }
    ```
    Swap/exchange the data between two rows.

  - ```rust
    unsafe fn get_unchecked_row_mut(self: &mut Self, row: usize) -> &mut [T] { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked_mut(self: &mut Self, coord: Coordinate) -> &mut T { /* ... */ }
    ```
    # Examples

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Unpin**
- **Eq**
- **UnwindSafe**
- **Index**
  - ```rust
    fn index(self: &Self, row: usize) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, coord: Coordinate) -> &<Self as >::Output { /* ... */ }
    ```

- **TranslateOps**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(view: TooDeeViewMut<''_, T>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TooDeeViewMut<''a, T>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **SortOps**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> TooDeeView<''a, T> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Functions

#### Function `calculate_view_dimensions`

Checks the proposed view dimensions, and returns the correct cols and rows for view construction.

```rust
pub(in ::view) fn calculate_view_dimensions<T, /* synthetic */ impl TooDeeOps<T>: TooDeeOps<T>>(start: Coordinate, end: Coordinate, toodee: &impl TooDeeOps<T>) -> (usize, usize) { /* ... */ }
```

## Module `ops`

```rust
pub(crate) mod ops { /* ... */ }
```

### Types

#### Type Alias `Coordinate`

A (col, row) coordinate in 2D space.

```rust
pub type Coordinate = (usize, usize);
```

#### Type Alias `Cells`

An iterator over each "cell" in a 2D array

```rust
pub type Cells<''a, T> = FlattenExact<Rows<''a, T>>;
```

#### Type Alias `CellsMut`

A mutable iterator over each "cell" in a 2D array

```rust
pub type CellsMut<''a, T> = FlattenExact<RowsMut<''a, T>>;
```

### Traits

#### Trait `TooDeeOps`

Defines operations common to both `TooDee` and `TooDeeView`. Default implementations are provided
where possible/practical.

```rust
pub trait TooDeeOps<T>: Index<usize, Output = [T]> + Index<Coordinate, Output = T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `num_cols`

The number of columns in the area represented by this object.


- `num_rows`

The number of rows in the area represented by this object.


- `bounds`

Returns the bounds of the object's area within the original `TooDee` area (views
are not nested for now).


- `view`

Returns a view (or subset) of the current area based on the coordinates provided.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps};
let toodee : TooDee<u32> = TooDee::new(10, 5);
let view = toodee.view((1, 1), (9, 4));
assert_eq!(view.num_cols(), 8);
assert_eq!(view.num_rows(), 3);
```


- `rows`

Returns an iterator of slices, where each slice represents an entire row.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps};
let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
let mut sum = 0u32;
for r in toodee.rows() {
    sum += r.iter().sum::<u32>();
}
assert_eq!(sum, 42*50);
```


- `col`

Returns an iterator over a single column. Note that the `Col` iterator is indexable.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps};
let toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
let mut sum = 0u32;
for c in toodee.col(1) {
    sum += c;
}
assert_eq!(sum, 42*5);
```


- `get_unchecked_row`

Returns a row without checking that the row is valid. Generally it's best to use indexing instead, e.g., toodee[row]
 
# Safety
 
This is generally not recommended, use with caution!
Calling this method with an invalid row is *[undefined behavior]* even if the resulting reference is not used.


- `get_unchecked`

Returns a cell without checking that the cell coordinate is valid. Generally it's best to use indexing instead, e.g., toodee[(col, row)]
 
# Safety
 
This is generally not recommended, use with caution!
Calling this method with an invalid coordinate is *[undefined behavior]* even if the resulting reference is not used.



##### Provided Methods

- ```rust
  fn size(self: &Self) -> (usize, usize) { /* ... */ }
  ```
  Returns the size/dimensions of the current object.

- ```rust
  fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the array contains no elements.

- ```rust
  fn cells(self: &Self) -> Cells<''_, T> { /* ... */ }
  ```
  Returns an iterator that traverses all cells within the area.

##### Implementations

This trait is implemented for the following types:

- `TooDeeView<''a, T>` with <''a, T>
- `TooDeeViewMut<''a, T>` with <''a, T>
- `TooDee<T>` with <T>

#### Trait `TooDeeOpsMut`

Defines operations common to both `TooDee` and `TooDeeViewMut`. Default implementations
are provided where possible/practical.

```rust
pub trait TooDeeOpsMut<T>: TooDeeOps<T> + IndexMut<usize, Output = [T]> + IndexMut<Coordinate, Output = T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `view_mut`

Returns a mutable view (or subset) of the current area based on the coordinates provided.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
let mut toodee : TooDee<u32> = TooDee::new(10, 5);
let view = toodee.view_mut((1, 1), (9, 4));
assert_eq!(view.num_cols(), 8);
assert_eq!(view.num_rows(), 3);
```


- `rows_mut`

Returns a mutable iterator of slices, where each slice represents an entire row.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
for (i, r) in toodee.rows_mut().enumerate() {
   r.iter_mut().for_each(|c| *c -= i as u32);
}
assert_eq!(toodee.cells().sum::<u32>(), 42*50 - 10 - 20 - 30 - 40);
```


- `col_mut`

Returns a mutable iterator over a single column. Note that the `ColMut` iterator is indexable.
 
# Examples
 
```
use toodee::{TooDee,TooDeeOps,TooDeeOpsMut};
let mut toodee : TooDee<u32> = TooDee::init(10, 5, 42u32);
for c in toodee.col_mut(4) {
    *c /= 2;
}
assert_eq!(toodee.cells().sum::<u32>(), 42*45 + 21*5);
```


- `get_unchecked_row_mut`

Returns a mutable row without checking that the row is valid. Generally it's best to use indexing instead, e.g., toodee[row]
 
# Safety
 
This is generally not recommended, use with caution!
Calling this method with an invalid row is *[undefined behavior]* even if the resulting reference is not used.


- `get_unchecked_mut`

Returns a mutable cell without checking that the cell coordinate is valid. Generally it's best to use indexing instead, e.g., toodee[(col, row)]
 
# Safety
 
This is generally not recommended, use with caution!
Calling this method with an invalid coordinate is *[undefined behavior]* even if the resulting reference is not used.



##### Provided Methods

- ```rust
  fn cells_mut(self: &mut Self) -> CellsMut<''_, T> { /* ... */ }
  ```
  Returns an iterator that traverses all cells within the area.

- ```rust
  fn fill<V>(self: &mut Self, fill: V)
where
    V: Borrow<T>,
    T: Clone { /* ... */ }
  ```
  Fills the entire area with the specified value.

- ```rust
  fn swap_cols(self: &mut Self, c1: usize, c2: usize) { /* ... */ }
  ```
  Swap/exchange the data between two columns.

- ```rust
  fn swap_rows(self: &mut Self, r1: usize, r2: usize) { /* ... */ }
  ```
  Swap/exchange the data between two rows. Note that this method is overridden in both `TooDee` and `TooDeeOpsMut`.

- ```rust
  fn row_pair_mut(self: &mut Self, r1: usize, r2: usize) -> (&mut [T], &mut [T]) { /* ... */ }
  ```
  Return the specified rows as mutable slices.

##### Implementations

This trait is implemented for the following types:

- `TooDeeViewMut<''a, T>` with <''a, T>
- `TooDee<T>` with <T>

## Module `toodee`

```rust
pub(crate) mod toodee { /* ... */ }
```

### Types

#### Type Alias `DrainRow`

DrainRow type alias for future-proofing.

```rust
pub type DrainRow<''a, T> = alloc::vec::Drain<''a, T>;
```

#### Type Alias `IntoIterTooDee`

IntoIter type alias for future-proofing.

```rust
pub type IntoIterTooDee<T> = alloc::vec::IntoIter<T>;
```

#### Struct `TooDee`

Represents a two-dimensional array.
 
Empty arrays will always have dimensions of zero.

```rust
pub struct TooDee<T> {
    pub(in ::toodee) data: alloc::vec::Vec<T>,
    pub(in ::toodee) num_rows: usize,
    pub(in ::toodee) num_cols: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `alloc::vec::Vec<T>` |  |
| `num_rows` | `usize` |  |
| `num_cols` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(num_cols: usize, num_rows: usize) -> TooDee<T>
where
    T: Default + Clone { /* ... */ }
  ```
  Create a new `TooDee` array of the specified dimensions, and fill it with

- ```rust
  pub fn init(num_cols: usize, num_rows: usize, init_value: T) -> TooDee<T>
where
    T: Clone { /* ... */ }
  ```
  Create a new `TooDee` array of the specified dimensions, and fill it with

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the element capacity of the underlying `Vec`.

- ```rust
  pub fn with_capacity(capacity: usize) -> TooDee<T> { /* ... */ }
  ```
  Constructs a new, empty `TooDee<T>` with the specified element capacity.

- ```rust
  pub fn reserve_exact(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Reserves the minimum capacity for at least `additional` more elements to be inserted

- ```rust
  pub fn reserve(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Reserves capacity for at least `additional` more elements to be inserted

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrinks the capacity of the underlying vector as much as possible.

- ```rust
  pub fn from_vec(num_cols: usize, num_rows: usize, v: Vec<T>) -> TooDee<T> { /* ... */ }
  ```
  Create a new `TooDee` array using the provided vector. The vector's length

- ```rust
  pub fn from_box(num_cols: usize, num_rows: usize, b: Box<[T]>) -> TooDee<T> { /* ... */ }
  ```
  Create a new `TooDee` array using the provided boxed slice. The slice's length

- ```rust
  pub fn data(self: &Self) -> &[T] { /* ... */ }
  ```
  Returns a reference to the raw array data

- ```rust
  pub fn data_mut(self: &mut Self) -> &mut [T] { /* ... */ }
  ```
  Returns a mutable reference to the raw array data

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the array, removing all values and zeroing the number of columns and rows.

- ```rust
  pub fn pop_row(self: &mut Self) -> Option<DrainRow<''_, T>> { /* ... */ }
  ```
  Removes the last row from the array and returns it as a `Drain`, or `None` if it is empty.

- ```rust
  pub fn push_row<I, /* synthetic */ impl IntoIterator<Item = T, IntoIter = I>: IntoIterator<Item = T, IntoIter = I>>(self: &mut Self, data: impl IntoIterator<Item = T, IntoIter = I>)
where
    I: Iterator<Item = T> + ExactSizeIterator { /* ... */ }
  ```
  Appends a new row to the array.

- ```rust
  pub fn insert_row<I, /* synthetic */ impl IntoIterator<Item = T, IntoIter = I>: IntoIterator<Item = T, IntoIter = I>>(self: &mut Self, index: usize, data: impl IntoIterator<Item = T, IntoIter = I>)
where
    I: Iterator<Item = T> + ExactSizeIterator { /* ... */ }
  ```
  Inserts new `data` into the array at the specified `row`

- ```rust
  pub fn remove_row(self: &mut Self, index: usize) -> DrainRow<''_, T> { /* ... */ }
  ```
  Removes the specified row from the array and returns it as a `Drain`

- ```rust
  pub fn pop_col(self: &mut Self) -> Option<DrainCol<''_, T>> { /* ... */ }
  ```
  Removes the last column from the array and returns it as a `Drain`, or `None` if it is empty.

- ```rust
  pub fn push_col<I, /* synthetic */ impl IntoIterator<Item = T, IntoIter = I>: IntoIterator<Item = T, IntoIter = I>>(self: &mut Self, data: impl IntoIterator<Item = T, IntoIter = I>)
where
    I: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator { /* ... */ }
  ```
  Appends a new column to the array.

- ```rust
  pub fn remove_col(self: &mut Self, index: usize) -> DrainCol<''_, T> { /* ... */ }
  ```
  Removes the specified column from the array and returns it as a `Drain`

- ```rust
  pub fn insert_col<I, /* synthetic */ impl IntoIterator<Item = T, IntoIter = I>: IntoIterator<Item = T, IntoIter = I>>(self: &mut Self, index: usize, data: impl IntoIterator<Item = T, IntoIter = I>)
where
    I: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator { /* ... */ }
  ```
  Inserts new `data` into the array at the specified `col`.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```
    `Cells` is the preferred iterator type here, because it implements `TooDeeIterator`

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **UnwindSafe**
- **Freeze**
- **Index**
  - ```rust
    fn index(self: &Self, row: usize) -> &<Self as >::Output { /* ... */ }
    ```
    # Examples

  - ```rust
    fn index(self: &Self, coord: Coordinate) -> &<Self as >::Output { /* ... */ }
    ```
    # Examples

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    # Examples

- **SortOps**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TooDee<T>) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, row: usize) -> &mut <Self as >::Output { /* ... */ }
    ```
    # Examples

  - ```rust
    fn index_mut(self: &mut Self, coord: Coordinate) -> &mut <Self as >::Output { /* ... */ }
    ```
    # Examples

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TranslateOps**
- **CopyOps**
  - ```rust
    fn copy_from_slice(self: &mut Self, src: &[T])
where
    T: Copy { /* ... */ }
    ```

  - ```rust
    fn clone_from_slice(self: &mut Self, src: &[T])
where
    T: Clone { /* ... */ }
    ```

  - ```rust
    fn copy_from_toodee</* synthetic */ impl TooDeeOps<T>: TooDeeOps<T>>(self: &mut Self, src: &impl TooDeeOps<T>)
where
    T: Copy { /* ... */ }
    ```

  - ```rust
    fn clone_from_toodee</* synthetic */ impl TooDeeOps<T>: TooDeeOps<T>>(self: &mut Self, src: &impl TooDeeOps<T>)
where
    T: Clone { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> Vec<T> { /* ... */ }
    ```

  - ```rust
    fn into(self: Self) -> Box<[T]> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[T] { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Vec<T> { /* ... */ }
    ```

- **TooDeeOps**
  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```
    # Examples

  - ```rust
    fn num_rows(self: &Self) -> usize { /* ... */ }
    ```
    # Examples

  - ```rust
    fn bounds(self: &Self) -> (Coordinate, Coordinate) { /* ... */ }
    ```
    # Examples

  - ```rust
    fn view(self: &Self, start: Coordinate, end: Coordinate) -> TooDeeView<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    fn rows(self: &Self) -> Rows<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    fn col(self: &Self, col: usize) -> Col<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked_row(self: &Self, row: usize) -> &[T] { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked(self: &Self, coord: Coordinate) -> &T { /* ... */ }
    ```
    # Examples

- **TooDeeOpsMut**
  - ```rust
    fn view_mut(self: &mut Self, start: Coordinate, end: Coordinate) -> TooDeeViewMut<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    fn rows_mut(self: &mut Self) -> RowsMut<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    fn col_mut(self: &mut Self, col: usize) -> ColMut<''_, T> { /* ... */ }
    ```
    # Examples

  - ```rust
    fn fill<V>(self: &mut Self, fill: V)
where
    V: Borrow<T>,
    T: Clone { /* ... */ }
    ```
    # Examples

  - ```rust
    fn swap_rows(self: &mut Self, r1: usize, r2: usize) { /* ... */ }
    ```
    Swap/exchange the data between two rows.

  - ```rust
    unsafe fn get_unchecked_row_mut(self: &mut Self, row: usize) -> &mut [T] { /* ... */ }
    ```
    # Examples

  - ```rust
    unsafe fn get_unchecked_mut(self: &mut Self, coord: Coordinate) -> &mut T { /* ... */ }
    ```
    # Examples

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TooDee<T> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(view: TooDeeView<''_, T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(view: TooDeeViewMut<''_, T>) -> Self { /* ... */ }
    ```

#### Struct `DrainCol`

Drains a column.

```rust
pub struct DrainCol<''a, T> {
    pub(in ::toodee) iter: Col<''a, T>,
    pub(in ::toodee) col: usize,
    pub(in ::toodee) toodee: core::ptr::NonNull<TooDee<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `Col<''a, T>` | Current remaining elements to remove |
| `col` | `usize` |  |
| `toodee` | `core::ptr::NonNull<TooDee<T>>` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **ExactSizeIterator**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<T> { /* ... */ }
    ```

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `flattenexact`

**Attributes:**

- `#![forbid(unsafe_code)]`
- `#![allow(missing_debug_implementations)]`

```rust
pub(crate) mod flattenexact { /* ... */ }
```

### Types

#### Struct `FlattenExact`

An iterator that behaves like `core::iter::adapters::Flatten` but has the added advantage of implementing
`ExactSizeIterator` (we know how many cells there are per row in a `TooDee` array).

```rust
pub struct FlattenExact<I> {
    pub(in ::flattenexact) iter: I,
    pub(in ::flattenexact) frontiter: Option<<<I as >::Item as IntoIterator>::IntoIter>,
    pub(in ::flattenexact) backiter: Option<<<I as >::Item as IntoIterator>::IntoIter>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `I` |  |
| `frontiter` | `Option<<<I as >::Item as IntoIterator>::IntoIter>` |  |
| `backiter` | `Option<<<I as >::Item as IntoIterator>::IntoIter>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new(iter: I) -> FlattenExact<I> { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TooDeeIterator**
  - ```rust
    fn num_cols(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<<I as >::Item as IntoIterator>::Item> { /* ... */ }
    ```

  - ```rust
    fn nth(self: &mut Self, n: usize) -> Option<<<I as >::Item as IntoIterator>::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn last(self: Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn fold<Acc, Fold>(self: Self, init: Acc, fold: Fold) -> Acc
where
    Fold: FnMut(Acc, <Self as >::Item) -> Acc { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ExactSizeIterator**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<<I as >::Item as IntoIterator>::Item> { /* ... */ }
    ```

  - ```rust
    fn nth_back(self: &mut Self, n: usize) -> Option<<<I as >::Item as IntoIterator>::Item> { /* ... */ }
    ```

  - ```rust
    fn rfold<Acc, Fold>(self: Self, init: Acc, fold: Fold) -> Acc
where
    Fold: FnMut(Acc, <Self as >::Item) -> Acc { /* ... */ }
    ```

- **RefUnwindSafe**
## Module `sort`

**Attributes:**

- `#[cfg(feature = "sort")]`

```rust
pub(crate) mod sort { /* ... */ }
```

### Traits

#### Trait `SortOps`

Provides sorting capabilities to two-dimensional arrays. Sorting of the rows and columns
is performed in-place, and care is taken to minimise row/col swaps. This is achieved by
sorting the row/col and original index pair, then repositioning the rows/columns once the
new sort order has been determined.

```rust
pub trait SortOps<T>: TooDeeOpsMut<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn sort_row_ord<F>(self: &mut Self, row: usize)
where
    T: Ord { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.

- ```rust
  fn sort_unstable_row_ord<F>(self: &mut Self, row: usize)
where
    T: Ord { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row, using the natural ordering.

- ```rust
  fn sort_by_row<F>(self: &mut Self, row: usize, compare: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.

- ```rust
  fn sort_unstable_by_row<F>(self: &mut Self, row: usize, compare: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row using the provided compare function.

- ```rust
  fn sort_by_row_key<B, F>(self: &mut Self, row: usize, f: F)
where
    B: Ord,
    F: FnMut(&T) -> B { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row using a key

- ```rust
  fn sort_unstable_by_row_key<B, F>(self: &mut Self, row: usize, f: F)
where
    B: Ord,
    F: FnMut(&T) -> B { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific row using a key

- ```rust
  fn sort_col_ord<F>(self: &mut Self, col: usize)
where
    T: Ord { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific column using the natural ordering.

- ```rust
  fn sort_by_col<F>(self: &mut Self, col: usize, compare: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on in a specific column.

- ```rust
  fn sort_unstable_by_col<F>(self: &mut Self, col: usize, compare: F)
where
    F: FnMut(&T, &T) -> Ordering { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on in a specific column.

- ```rust
  fn sort_by_col_key<B, F>(self: &mut Self, col: usize, f: F)
where
    B: Ord,
    F: FnMut(&T) -> B { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific column using a key

- ```rust
  fn sort_unstable_by_col_key<B, F>(self: &mut Self, col: usize, f: F)
where
    B: Ord,
    F: FnMut(&T) -> B { /* ... */ }
  ```
  Sort the entire two-dimensional array by comparing elements on a specific column using a key

##### Implementations

This trait is implemented for the following types:

- `O` with <T, O>

### Functions

#### Function `build_swap_trace`

Common re-indexing logic used internally by the `SortOps` trait.

```rust
pub(in ::sort) fn build_swap_trace(ordering: &mut [(usize, usize)]) -> &mut [(usize, usize)] { /* ... */ }
```

#### Function `sorted_box_to_ordering`

Use some unsafeness to coerce a [(usize, &T)] into a [(usize, usize)]. The `Box` is consumed,
meaning that we "unborrow" the &T values.

```rust
pub(in ::sort) fn sorted_box_to_ordering<T>(sorted: alloc::boxed::Box<[(usize, &T)]>) -> alloc::boxed::Box<[(usize, usize)]> { /* ... */ }
```

## Module `tests_sort`

**Attributes:**

- `#[cfg(feature = "sort")]`

```rust
pub(crate) mod tests_sort { /* ... */ }
```

## Module `translate`

**Attributes:**

- `#[cfg(feature = "translate")]`

```rust
pub(crate) mod translate { /* ... */ }
```

### Traits

#### Trait `TranslateOps`

Provides implementations for translate (also known as scroll) operations, and other internal data
movement operations such as flipping.

```rust
pub trait TranslateOps<T>: TooDeeOpsMut<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn translate_with_wrap(self: &mut Self, mid: Coordinate) { /* ... */ }
  ```
  Translate (or scroll) the entire area. The `mid` coordinate will be moved to (0, 0), and

- ```rust
  fn flip_rows(self: &mut Self) { /* ... */ }
  ```
  Flips (or mirrors) the rows.

- ```rust
  fn flip_cols(self: &mut Self) { /* ... */ }
  ```
  Flips (or mirrors) the columns.

##### Implementations

This trait is implemented for the following types:

- `O` with <T, O>

## Module `tests_translate`

**Attributes:**

- `#[cfg(feature = "translate")]`

```rust
pub(crate) mod tests_translate { /* ... */ }
```

## Module `copy`

**Attributes:**

- `#[cfg(feature = "copy")]`
- `#![forbid(unsafe_code)]`

```rust
pub(crate) mod copy { /* ... */ }
```

### Traits

#### Trait `CopyOps`

Provides basic copying operations for `TooDee` structures.

```rust
pub trait CopyOps<T>: TooDeeOpsMut<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Provided Methods

- ```rust
  fn copy_from_slice(self: &mut Self, src: &[T])
where
    T: Copy { /* ... */ }
  ```
  Copies data from another slice into this area. The source slice's length

- ```rust
  fn clone_from_slice(self: &mut Self, src: &[T])
where
    T: Clone { /* ... */ }
  ```
  Clones data from another slice into this area. The source slice's length

- ```rust
  fn copy_from_toodee</* synthetic */ impl TooDeeOps<T>: TooDeeOps<T>>(self: &mut Self, src: &impl TooDeeOps<T>)
where
    T: Copy { /* ... */ }
  ```
  Copies data from another `TooDeeOps` object into this one. The source and

- ```rust
  fn clone_from_toodee</* synthetic */ impl TooDeeOps<T>: TooDeeOps<T>>(self: &mut Self, src: &impl TooDeeOps<T>)
where
    T: Clone { /* ... */ }
  ```
  Copies data from another `TooDeeOps` object into this one. The source and

- ```rust
  fn copy_within(self: &mut Self, src: (Coordinate, Coordinate), dest: Coordinate)
where
    T: Copy { /* ... */ }
  ```
  Copies the `src` area (top-left to bottom-right) to a destination area. `dest` specifies

##### Implementations

This trait is implemented for the following types:

- `TooDeeViewMut<''_, T>` with <T>
- `TooDee<T>` with <T>

## Module `tests_copy`

**Attributes:**

- `#[cfg(feature = "copy")]`

```rust
pub(crate) mod tests_copy { /* ... */ }
```

## Module `tests`

```rust
pub(crate) mod tests { /* ... */ }
```

## Module `tests_iter`

```rust
pub(crate) mod tests_iter { /* ... */ }
```

## Re-exports

### Re-export `crate::sort::*`

**Attributes:**

- `#[cfg(feature = "sort")]`

```rust
pub use crate::sort::*;
```

### Re-export `crate::translate::*`

**Attributes:**

- `#[cfg(feature = "translate")]`

```rust
pub use crate::translate::*;
```

### Re-export `crate::copy::*`

**Attributes:**

- `#[cfg(feature = "copy")]`

```rust
pub use crate::copy::*;
```

### Re-export `crate::iter::*`

```rust
pub use crate::iter::*;
```

### Re-export `crate::view::*`

```rust
pub use crate::view::*;
```

### Re-export `crate::ops::*`

```rust
pub use crate::ops::*;
```

### Re-export `crate::toodee::*`

```rust
pub use crate::toodee::*;
```

### Re-export `crate::flattenexact::*`

```rust
pub use crate::flattenexact::*;
```

