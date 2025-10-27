# Crate Documentation

**Version:** 0.3.0

**Format Version:** 39

# Module `qwutils`

## Modules

## Module `imp`

```rust
pub mod imp { /* ... */ }
```

### Modules

## Module `vec`

```rust
pub mod vec { /* ... */ }
```

### Traits

#### Trait `VecExt`

```rust
pub trait VecExt<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `push_option`
- `grow_to_with`
- `grow_to`
- `grow_to_default`
- `insert_slice_copy`
- `insert_slice_clone`
- `extend_from_slice_copy`

##### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>

## Module `range`

```rust
pub mod range { /* ... */ }
```

## Module `result`

```rust
pub mod result { /* ... */ }
```

### Traits

#### Trait `ResultNonDebugUnwrap`

```rust
pub trait ResultNonDebugUnwrap<T, E> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `expect_nodebug`
- `expect_err_nodebug`
- `unwrap_nodebug`
- `unwrap_err_nodebug`

##### Implementations

This trait is implemented for the following types:

- `Result<T, E>` with <T, E>

### Functions

#### Function `unwrap_failed`

**Attributes:**

- `#[inline(never)]`
- `#[cold]`

```rust
pub(in ::imp::result) fn unwrap_failed<E>(msg: &str) -> never { /* ... */ }
```

## Module `boolext`

```rust
pub mod boolext { /* ... */ }
```

### Traits

#### Trait `BoolExtOption`

```rust
pub trait BoolExtOption {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `option`
- `result`

##### Provided Methods

- ```rust
  fn map<U, /* synthetic */ impl FnOnce() -> U: FnOnce() -> U>(self: &Self, f: impl FnOnce() -> U) -> Option<U> { /* ... */ }
  ```

- ```rust
  fn map_or<U, /* synthetic */ impl FnOnce() -> U: FnOnce() -> U>(self: &Self, default: U, f: impl FnOnce() -> U) -> U { /* ... */ }
  ```

- ```rust
  fn map_or_else<U, /* synthetic */ impl FnOnce() -> U: FnOnce() -> U, /* synthetic */ impl FnOnce() -> U: FnOnce() -> U>(self: &Self, default: impl FnOnce() -> U, f: impl FnOnce() -> U) -> U { /* ... */ }
  ```

- ```rust
  fn map_or_err<T, E, /* synthetic */ impl FnOnce() -> T: FnOnce() -> T, /* synthetic */ impl FnOnce() -> E: FnOnce() -> E>(self: &Self, f: impl FnOnce() -> T, e: impl FnOnce() -> E) -> Result<T, E> { /* ... */ }
  ```

##### Implementations

This trait is implemented for the following types:

- `bool`

## Module `option`

```rust
pub mod option { /* ... */ }
```

### Traits

#### Trait `OptionExt`

```rust
pub trait OptionExt<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `with`
- `with_mut`
- `with_if`
- `with_mut_if`
- `with_mut_if_saturating`
- `add_to`
- `sub_to`
- `mul_to`
- `div_to`
- `add_to_lossy`
- `sub_to_lossy`
- `mul_to_lossy`
- `div_to_lossy`
- `add_to_if`
- `sub_to_if`
- `mul_to_if`
- `div_to_if`

##### Implementations

This trait is implemented for the following types:

- `Option<T>` with <T>

### Functions

#### Function `flatten`

**Attributes:**

- `#[inline]`

```rust
pub(in ::imp::option) fn flatten<T>(i: Option<Option<T>>) -> Option<T> { /* ... */ }
```

## Module `tuple`

```rust
pub mod tuple { /* ... */ }
```

### Traits

#### Trait `AsTuple`

```rust
pub trait AsTuple {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Dest`

###### Required Methods

- `as_tuple`

##### Implementations

This trait is implemented for the following types:

- `[T; 32]` with <T>
- `[T; 31]` with <T>
- `[T; 30]` with <T>
- `[T; 29]` with <T>
- `[T; 28]` with <T>
- `[T; 27]` with <T>
- `[T; 26]` with <T>
- `[T; 25]` with <T>
- `[T; 24]` with <T>
- `[T; 23]` with <T>
- `[T; 22]` with <T>
- `[T; 21]` with <T>
- `[T; 20]` with <T>
- `[T; 19]` with <T>
- `[T; 18]` with <T>
- `[T; 17]` with <T>
- `[T; 16]` with <T>
- `[T; 15]` with <T>
- `[T; 14]` with <T>
- `[T; 13]` with <T>
- `[T; 12]` with <T>
- `[T; 11]` with <T>
- `[T; 10]` with <T>
- `[T; 9]` with <T>
- `[T; 8]` with <T>
- `[T; 7]` with <T>
- `[T; 6]` with <T>
- `[T; 5]` with <T>
- `[T; 4]` with <T>
- `[T; 3]` with <T>
- `[T; 2]` with <T>
- `[T; 1]` with <T>

#### Trait `AsArray`

```rust
pub trait AsArray {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Dest`

###### Required Methods

- `as_array`

##### Implementations

This trait is implemented for the following types:

- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T, T)` with <T>
- `(T, T, T, T, T)` with <T>
- `(T, T, T, T)` with <T>
- `(T, T, T)` with <T>
- `(T, T)` with <T>
- `(T)` with <T>

#### Trait `TupleFns`

```rust
pub trait TupleFns<T>
where
    T: ''static {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `avg`

##### Implementations

This trait is implemented for the following types:

- `[T; 32]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 31]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 30]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 29]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 28]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 27]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 26]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 25]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 24]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 23]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 22]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 21]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 20]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 19]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 18]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 17]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 16]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 15]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 14]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 13]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 12]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 11]` with <T>
- `(T, T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 10]` with <T>
- `(T, T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 9]` with <T>
- `(T, T, T, T, T, T, T, T, T)` with <T>
- `[T; 8]` with <T>
- `(T, T, T, T, T, T, T, T)` with <T>
- `[T; 7]` with <T>
- `(T, T, T, T, T, T, T)` with <T>
- `[T; 6]` with <T>
- `(T, T, T, T, T, T)` with <T>
- `[T; 5]` with <T>
- `(T, T, T, T, T)` with <T>
- `[T; 4]` with <T>
- `(T, T, T, T)` with <T>
- `[T; 3]` with <T>
- `(T, T, T)` with <T>
- `[T; 2]` with <T>
- `(T, T)` with <T>
- `[T; 1]` with <T>
- `(T)` with <T>

### Macros

#### Macro `impl_arr`

```rust
pub(crate) macro_rules! impl_arr {
    /* macro_rules! impl_arr {
    {$n:expr;$t:ident $($ts:ident)*;$l:ident $($ls:ident)*} => { ... };
    {$n:expr;;} => { ... };
} */
}
```

## Module `numext`

```rust
pub mod numext { /* ... */ }
```

### Traits

#### Trait `DivOrNop`

```rust
pub trait DivOrNop: Sized + Copy {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `div_or_nop`

##### Implementations

This trait is implemented for the following types:

- `u8`
- `i8`
- `u16`
- `i16`
- `u32`
- `i32`
- `u64`
- `i64`
- `u128`
- `i128`
- `usize`
- `isize`

### Macros

#### Macro `impl_don`

```rust
pub(crate) macro_rules! impl_don {
    /* macro_rules! impl_don {
    ($t:ty;$($tt:ty);+) => { ... };
    ($t:ty) => { ... };
} */
}
```

### Re-exports

#### Re-export `vec::*`

```rust
pub use vec::*;
```

#### Re-export `range::*`

```rust
pub use range::*;
```

#### Re-export `boolinator::*`

```rust
pub use boolinator::*;
```

#### Re-export `result::*`

```rust
pub use result::*;
```

#### Re-export `boolext::*`

```rust
pub use boolext::*;
```

#### Re-export `option::*`

```rust
pub use option::*;
```

#### Re-export `tuple::*`

```rust
pub use tuple::*;
```

#### Re-export `numext::*`

```rust
pub use numext::*;
```

## Module `refc`

```rust
pub mod refc { /* ... */ }
```

### Modules

## Module `imp`

```rust
pub(in ::refc) mod imp { /* ... */ }
```

### Traits

#### Trait `RefClonable`

Clone function but only does cheap ref-cloning like rc

```rust
pub trait RefClonable {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `refc`

##### Implementations

This trait is implemented for the following types:

- `std::sync::Arc<T>` with <T>
- `std::rc::Rc<T>` with <T>
- `Box<T>` with <T>
- `ArcSlice<T>` with <T>

## Module `arc_slice`

```rust
pub mod arc_slice { /* ... */ }
```

### Modules

## Module `imp`

```rust
pub mod imp { /* ... */ }
```

### Functions

#### Function `slice_slice`

**Attributes:**

- `#[inline]`

```rust
pub(in ::arc_slice::imp) fn slice_slice<S>(range: &std::ops::Range<usize>, slice: S) -> std::ops::Range<usize>
where
    S: RangeBounds<usize> { /* ... */ }
```

### Types

#### Struct `ArcSlice`

Slice backed by Arc<Vec<T>>

Cow mechanisms, minimizes clones

```rust
pub struct ArcSlice<T> {
    pub(in ::arc_slice) inner: std::sync::Arc<Vec<T>>,
    pub(in ::arc_slice) slice: std::ops::Range<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::sync::Arc<Vec<T>>` |  |
| `slice` | `std::ops::Range<usize>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn slice<S>(self: &Self, range: S) -> Self
where
    S: RangeBounds<usize> { /* ... */ }
  ```
  slice of the current slice

- ```rust
  pub fn extract(self: &Self) -> Vec<T>
where
    T: Clone { /* ... */ }
  ```
  will always allocate and clone

- ```rust
  pub fn extracted(self: &Self) -> Self
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn extract_with_capacity(self: &Self, capacity: usize) -> Vec<T>
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn extracted_with_capacity(self: &Self, capacity: usize) -> Self
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn is_unsliced(self: &Self) -> bool { /* ... */ }
  ```
  whether this slice vievs the complete backing vec

- ```rust
  pub fn compact(self: &mut Self) -> bool { /* ... */ }
  ```
  Minimize memory usage.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```

- ```rust
  pub fn swap_remove(self: &mut Self, index: usize) -> T
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn remove(self: &mut Self, index: usize) -> T
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    T: Clone,
    F: FnMut(&T) -> bool { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, index: usize, element: T)
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn insert_slice(self: &mut Self, index: usize, s: &[T])
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn split_at(self: &mut Self, at: usize) -> (Self, Self) { /* ... */ }
  ```

- ```rust
  pub fn split_off(self: &mut Self, at: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn resize_with<F>(self: &mut Self, new_len: usize, f: F)
where
    T: Clone,
    F: FnMut() -> T { /* ... */ }
  ```

- ```rust
  pub fn resize<F>(self: &mut Self, new_len: usize, value: T)
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn resize_default<F>(self: &mut Self, new_len: usize)
where
    T: Clone + Default { /* ... */ }
  ```

- ```rust
  pub fn push(self: &mut Self, v: T)
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn pop(self: &mut Self) -> Option<T>
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn append(self: &mut Self, other: &mut Vec<T>)
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn extend_from_slice(self: &mut Self, other: &[T])
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn _make_mut(self: &mut Self) -> (&mut Vec<T>, &mut Range<usize>)
where
    T: Clone { /* ... */ }
  ```
  mutably access the mutable Vec inside

- ```rust
  pub fn _make_mut_with_capacity(self: &mut Self, capacity: usize) -> (&mut Vec<T>, &mut Range<usize>)
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn _make_mut_extracted(self: &mut Self) -> &mut Vec<T>
where
    T: Clone { /* ... */ }
  ```

- ```rust
  pub fn _make_mut_extracted_with_capacity(self: &mut Self, capacity: usize) -> &mut Vec<T>
where
    T: Clone { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Self { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut [T] { /* ... */ }
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
    fn into(self: Self) -> Vec<T> { /* ... */ }
    ```

  - ```rust
    fn into(self: Self) -> Vec<T> { /* ... */ }
    ```

- **Eq**
- **RefClonable**
  - ```rust
    fn refc(self: &Self) -> Self { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> slice::Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> slice::IterMut<''a, T> { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_vectored(self: &mut Self, bufs: &[IoSlice<''_>]) -> io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn write_all(self: &mut Self, buf: &[u8]) -> io::Result<()> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> io::Result<()> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &O) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[T] { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Receiver**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[T] { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Extend**
  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

  - ```rust
    fn extend<I>(self: &mut Self, iter: I)
where
    I: IntoIterator<Item = &''a T> { /* ... */ }
    ```

- **Sync**
- **AsMut**
  - ```rust
    fn as_mut(self: &mut Self) -> &mut [T] { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(v: Vec<T>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(v: Arc<Vec<T>>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(v: &[T]) -> Self { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> std::cmp::Ordering { /* ... */ }
    ```

- **FromInto**
  - ```rust
    fn qfrom(t: U) -> T { /* ... */ }
    ```

  - ```rust
    fn qinto(self: Self) -> U { /* ... */ }
    ```

## Module `static_stor`

**Attributes:**

- `#[macro_use]`

```rust
pub mod static_stor { /* ... */ }
```

## Module `scoped`

```rust
pub mod scoped { /* ... */ }
```

### Modules

## Module `imp`

```rust
pub mod imp { /* ... */ }
```

### Modules

## Module `scoped`

```rust
pub(in ::scoped::imp) mod scoped { /* ... */ }
```

## Module `interior`

```rust
pub(in ::scoped::imp) mod interior { /* ... */ }
```

## Module `macros`

```rust
pub mod macros { /* ... */ }
```

### Traits

#### Trait `ScopedMut`

a type which inner type T can be accessed scoped
 
use impl_scoped_mut!(T) if a implementation is missing

```rust
pub trait ScopedMut {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `T`

###### Required Methods

- `access`
- `access_mut`

##### Implementations

This trait is implemented for the following types:

- `&''a mut S` with <''a, S, T>
- `Box<S>` with <''a, S, T>
- `std::borrow::Cow<''_, S>` with <S, T>
- `std::cell::RefCell<S>` with <S, T>
- `std::sync::RwLock<S>` with <S, T>
- `std::rc::Rc<S>` with <S, T>
- `std::sync::Arc<S>` with <S, T>

#### Trait `Interior`

like ScopedMut, but explict with interior mutability

```rust
pub trait Interior {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `T`

###### Required Methods

- `interior_access`
- `interior_access_mut`

##### Implementations

This trait is implemented for the following types:

- `std::cell::RefCell<S>` with <S, T>
- `std::sync::RwLock<S>` with <S, T>
- `&''a C` with <''a, T, C>
- `&''a mut C` with <''a, T, C>
- `Box<C>` with <''a, T, C>
- `std::rc::Rc<C>` with <T, C>
- `std::sync::Arc<C>` with <T, C>

## Module `if_type`

```rust
pub mod if_type { /* ... */ }
```

### Functions

#### Function `if_type`

**Attributes:**

- `#[inline]`

```rust
pub fn if_type<Specific: ''static, T: ''static, /* synthetic */ impl FnOnce() -> Specific: FnOnce() -> Specific>(f: impl FnOnce() -> Specific) -> Option<T> { /* ... */ }
```

## Module `not_empty`

```rust
pub mod not_empty { /* ... */ }
```

### Traits

#### Trait `NotEmpty`

```rust
pub trait NotEmpty: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `_is_empty`

##### Provided Methods

- ```rust
  fn not_empty(self: Self) -> Option<Self> { /* ... */ }
  ```

##### Implementations

This trait is implemented for the following types:

- `&Vec<T>` with <T>
- `&mut Vec<T>` with <T>
- `Vec<T>` with <T>
- `&str`
- `&mut str`
- `&String`
- `&mut String`
- `String`
- `&std::collections::hash_set::HashSet<T>` with <T>
- `&mut std::collections::hash_set::HashSet<T>` with <T>
- `std::collections::hash_set::HashSet<T>` with <T>
- `&std::collections::hash_map::HashMap<K, V>` with <K, V>
- `&mut std::collections::hash_map::HashMap<K, V>` with <K, V>
- `std::collections::hash_map::HashMap<K, V>` with <K, V>

## Module `from_into`

```rust
pub mod from_into { /* ... */ }
```

### Traits

#### Trait `FromInto`

```rust
pub trait FromInto<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `qfrom`
- `qinto`

##### Implementations

This trait is implemented for the following types:

- `T` with <T, U>

## Module `macros`

```rust
pub mod macros { /* ... */ }
```

### Types

#### Struct `Test`

```rust
pub(in ::macros) struct Test {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Test { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BitAnd**
  - ```rust
    fn bitand(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **FromInto**
  - ```rust
    fn qfrom(t: U) -> T { /* ... */ }
    ```

  - ```rust
    fn qinto(self: Self) -> U { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **Rem**
  - ```rust
    fn rem(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **RefUnwindSafe**
- **BitXor**
  - ```rust
    fn bitxor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: Test) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &Test) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &mut Test) { /* ... */ }
    ```

- **Add**
  - ```rust
    fn add(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test) -> <Self as >::Output { /* ... */ }
    ```

#### Struct `Test2`

```rust
pub(in ::macros) struct Test2 {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **BitAnd**
  - ```rust
    fn bitand(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitand(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Test2 { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Add**
  - ```rust
    fn add(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn add(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn mul(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn sub_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **DivAssign**
  - ```rust
    fn div_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn div_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn bitand_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shl(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn shr(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitxor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn bitor_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn bitxor_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Rem**
  - ```rust
    fn rem(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn rem(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ShrAssign**
  - ```rust
    fn shr_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn shr_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromInto**
  - ```rust
    fn qfrom(t: U) -> T { /* ... */ }
    ```

  - ```rust
    fn qinto(self: Self) -> U { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sub**
  - ```rust
    fn sub(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn sub(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **MulAssign**
  - ```rust
    fn mul_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn mul_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **Div**
  - ```rust
    fn div(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn div(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **RemAssign**
  - ```rust
    fn rem_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn rem_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn bitor(self: Self, r: &mut Test2) -> <Self as >::Output { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **ShlAssign**
  - ```rust
    fn shl_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn shl_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

- **AddAssign**
  - ```rust
    fn add_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: Test2) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &Test2) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

  - ```rust
    fn add_assign(self: &mut Self, r: &mut Test2) { /* ... */ }
    ```

## Macros

### Macro `create_static_stor`

**Attributes:**

- `#[macro_export]`

generate a static global stor which is read fast but updated slow using cow technologies
create_static_stor!(VISIBILITY NAME: TYPE) where T: ?Sized + Clone + Default + Send + Sync;
TODO FIXME minimum required visibility is pub(super)
stor type must implement Default and Clone
generates a pub (TODO: visibility options) module with with and with_mut fns

```rust
pub macro_rules! create_static_stor {
    /* macro_rules! create_static_stor {
    ($name:ident: $t:ty) => { ... };
    ($name:ident: $t:ty = $i:expr) => { ... };
    ($v:vis $name:ident: $t:ty) => { ... };
    ($v:vis $name:ident: $t:ty = $i:expr) => { ... };
} */
}
```

### Macro `impl_scoped_mut`

**Attributes:**

- `#[macro_export]`

implement ScopedMut for a type

```rust
pub macro_rules! impl_scoped_mut {
    /* macro_rules! impl_scoped_mut {
    ($t:ty) => { ... };
} */
}
```

### Macro `impl_scoped_mut_inner`

**Attributes:**

- `#[macro_export]`

impl ScopedMut for T {
    impl_scoped_mut_inner!(T);
}

```rust
pub macro_rules! impl_scoped_mut_inner {
    /* macro_rules! impl_scoped_mut_inner {
    ($t:ty) => { ... };
} */
}
```

### Macro `opion`

**Attributes:**

- `#[macro_export]`

macro for compact operator implementations

```rust
pub macro_rules! opion {
    /* macro_rules! opion {
    ($op:tt($l:ty, $r:ty) |$li:ident,move $ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,$r:ty) |$li:ident,&mut $ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,$r:ty) |$li:ident,&$ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,move $r:ty) |$li:ident,$ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,&mut $r:ty) |$li:ident,$ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,&$r:ty) |$li:ident,$ri:ident| $f:block) => { ... };
    ($op:tt($l:ty,$r:ty) |$li:ident,$ri:ident| $f:block) => { ... };
} */
}
```

### Macro `_opion_grid`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! _opion_grid {
    /* macro_rules! _opion_grid {
    (+,$($t:tt)+) => { ... };
    (-,$($t:tt)+) => { ... };
    (*,$($t:tt)+) => { ... };
    (/,$($t:tt)+) => { ... };
    (&,$($t:tt)+) => { ... };
    (|,$($t:tt)+) => { ... };
    (^,$($t:tt)+) => { ... };
    (%,$($t:tt)+) => { ... };
    (<<,$($t:tt)+) => { ... };
    (>>,$($t:tt)+) => { ... };
    (add,$($t:tt)+) => { ... };
    (sub,$($t:tt)+) => { ... };
    (mul,$($t:tt)+) => { ... };
    (div,$($t:tt)+) => { ... };
    (bitand,$($t:tt)+) => { ... };
    (bitor,$($t:tt)+) => { ... };
    (bitxor,$($t:tt)+) => { ... };
    (rem,$($t:tt)+) => { ... };
    (shl,$($t:tt)+) => { ... };
    (shr,$($t:tt)+) => { ... };
} */
}
```

### Macro `_opion_inner`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! _opion_inner {
    /* macro_rules! _opion_inner {
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,&mut $r:ty,$li:ident,$ri:ident,$f:block) => { ... };
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,&$r:ty,$li:ident,$ri:ident,$f:block) => { ... };
    ($op:ident,$fn:ident,$op_assign:ident,$fn_assign:ident,$l:ty,$r:ty,$li:ident,$ri:ident,$f:block) => { ... };
} */
}
```

## Re-exports

### Re-export `imp::*`

```rust
pub use imp::*;
```

### Re-export `refc::*`

```rust
pub use refc::*;
```

### Re-export `if_type::*`

```rust
pub use if_type::*;
```

### Re-export `not_empty::*`

```rust
pub use not_empty::*;
```

