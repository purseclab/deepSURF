# Crate Documentation

**Version:** 0.3.2

**Format Version:** 39

# Module `simple_slab`

Fast and lightweight Slab Allocator.

## Types

### Struct `Slab`

```rust
pub struct Slab<T> {
    pub(crate) capacity: usize,
    pub(crate) len: usize,
    pub(crate) mem: *mut T,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `capacity` | `usize` |  |
| `len` | `usize` |  |
| `mem` | `*mut T` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Slab<T> { /* ... */ }
  ```
  Creates a new Slab

- ```rust
  pub fn with_capacity(capacity: usize) -> Slab<T> { /* ... */ }
  ```
  Creates a new, empty Slab with room for `capacity` elems

- ```rust
  pub fn insert(self: &mut Self, elem: T) { /* ... */ }
  ```
  Inserts a new element into the slab, re-allocating if neccessary.

- ```rust
  pub fn remove(self: &mut Self, offset: usize) -> T { /* ... */ }
  ```
  Removes the element at `offset`.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the slab

- ```rust
  pub fn iter(self: &Self) -> SlabIter<''_, T> { /* ... */ }
  ```
  Returns an iterator over the slab

- ```rust
  pub fn iter_mut(self: &mut Self) -> SlabMutIter<''_, T> { /* ... */ }
  ```
  Returns a mutable iterator over the slab

- ```rust
  pub(crate) fn reallocate(self: &mut Self) { /* ... */ }
  ```
  Reserves capacity * 2 extra space in this slab

##### Trait Implementations

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &<Self as >::Output { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> SlabIter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> SlabMutIter<''a, T> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

### Struct `SlabIter`

```rust
pub struct SlabIter<''a, T: ''a> {
    pub(crate) slab: &''a Slab<T>,
    pub(crate) current_offset: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `slab` | `&''a Slab<T>` |  |
| `current_offset` | `usize` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a T> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `SlabMutIter`

```rust
pub struct SlabMutIter<''a, T: ''a> {
    pub(crate) iter: SlabIter<''a, T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `SlabIter<''a, T>` |  |

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<&''a mut T> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
