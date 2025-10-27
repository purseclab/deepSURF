# Crate Documentation

**Version:** 0.5.2

**Format Version:** 39

# Module `nano_arena`

## Modules

## Module `split`

```rust
pub(crate) mod split { /* ... */ }
```

### Types

#### Struct `ArenaSplit`

```rust
pub struct ArenaSplit<''a, T> {
    pub(crate) selected: super::Idx,
    pub(crate) arena: &''a mut super::Arena<T>,
    pub(crate) __type: std::marker::PhantomData<T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `selected` | `super::Idx` |  |
| `arena` | `&''a mut super::Arena<T>` |  |
| `__type` | `std::marker::PhantomData<T>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get<I: Borrow<Idx>>(self: &Self, index: I) -> Option<&T> { /* ... */ }
  ```

- ```rust
  pub fn get_mut<I: Borrow<Idx>>(self: &mut Self, index: I) -> Option<&mut T> { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **RefUnwindSafe**
## Types

### Struct `IdxInner`

```rust
pub(crate) struct IdxInner {
    pub(crate) index: std::sync::atomic::AtomicUsize,
    pub(crate) removed: std::sync::atomic::AtomicBool,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `index` | `std::sync::atomic::AtomicUsize` |  |
| `removed` | `std::sync::atomic::AtomicBool` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn index(self: &Self) -> Option<usize> { /* ... */ }
  ```

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Idx`

```rust
pub struct Idx {
    pub(crate) inner: std::sync::Arc<IdxInner>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::sync::Arc<IdxInner>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn value(self: &Self) -> Option<usize> { /* ... */ }
  ```

##### Trait Implementations

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Idx { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, rhs: &Idx) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, formatter: &mut std::fmt::Formatter<''_>) -> Result<(), std::fmt::Error> { /* ... */ }
    ```

### Struct `Arena`

```rust
pub struct Arena<T> {
    pub(crate) values: Vec<(std::sync::Arc<IdxInner>, T)>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `values` | `Vec<(std::sync::Arc<IdxInner>, T)>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Arena<T> { /* ... */ }
  ```

- ```rust
  pub fn with_capacity(capacity: usize) -> Arena<T> { /* ... */ }
  ```

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn alloc_with_idx<F: FnOnce(Idx) -> T>(self: &mut Self, func: F) -> Idx { /* ... */ }
  ```

- ```rust
  pub fn alloc_with<F: FnOnce() -> T>(self: &mut Self, func: F) -> Idx { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, value: T) -> Idx { /* ... */ }
  ```

- ```rust
  pub fn alloc(self: &mut Self, value: T) -> Idx { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn get_idx_at_index(self: &Self, index: usize) -> Option<Idx> { /* ... */ }
  ```

- ```rust
  pub fn split_at<''a, I: Borrow<Idx>>(self: &''a mut Self, selected: I) -> Option<(&mut T, ArenaSplit<''a, T>)> { /* ... */ }
  ```

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```

- ```rust
  pub fn retain<F: FnMut(&T) -> bool>(self: &mut Self, f: F) { /* ... */ }
  ```

- ```rust
  pub fn entries<''a>(self: &''a Self) -> Entries<''a, T> { /* ... */ }
  ```

- ```rust
  pub fn entries_mut<''a>(self: &''a mut Self) -> EntriesMut<''a, T> { /* ... */ }
  ```

- ```rust
  pub fn iter_mut<''a>(self: &''a mut Self) -> IterMut<''a, T> { /* ... */ }
  ```

- ```rust
  pub fn iter<''a>(self: &''a Self) -> Iter<''a, T> { /* ... */ }
  ```

- ```rust
  pub fn to_vec(self: Self) -> Vec<T> { /* ... */ }
  ```

- ```rust
  pub(crate) fn remove_index(self: &mut Self, index: usize) -> T { /* ... */ }
  ```

- ```rust
  pub fn remove<I: Borrow<Idx>>(self: &mut Self, index: I) -> T { /* ... */ }
  ```

- ```rust
  pub(crate) fn swap_index(self: &mut Self, a: usize, b: usize) { /* ... */ }
  ```

- ```rust
  pub fn swap<A: Borrow<Idx>, B: Borrow<Idx>>(self: &mut Self, a: A, b: B) { /* ... */ }
  ```

- ```rust
  pub fn position<F: Fn(&T) -> bool>(self: &Self, func: F) -> Option<Idx> { /* ... */ }
  ```

- ```rust
  pub fn apply_ordering<I: Borrow<Idx>>(self: &mut Self, ordering: &Vec<I>) { /* ... */ }
  ```

- ```rust
  pub(crate) fn swap_remove_index(self: &mut Self, index: usize) -> (Arc<IdxInner>, T) { /* ... */ }
  ```

- ```rust
  pub fn swap_remove<I: Borrow<Idx>>(self: &mut Self, index: I) -> T { /* ... */ }
  ```

- ```rust
  pub fn get<I: Borrow<Idx>>(self: &Self, index: I) -> Option<&T> { /* ... */ }
  ```

- ```rust
  pub fn get_mut<I: Borrow<Idx>>(self: &mut Self, index: I) -> Option<&mut T> { /* ... */ }
  ```

##### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

  - ```rust
    fn into(self: Self) -> Vec<T> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Struct `IterMut`

```rust
pub struct IterMut<''a, T> {
    pub(crate) iterator: std::iter::Map<std::slice::IterMut<''a, (std::sync::Arc<IdxInner>, T)>, &''a dyn Fn(&mut (std::sync::Arc<IdxInner>, T)) -> &mut T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iterator` | `std::iter::Map<std::slice::IterMut<''a, (std::sync::Arc<IdxInner>, T)>, &''a dyn Fn(&mut (std::sync::Arc<IdxInner>, T)) -> &mut T>` |  |

#### Implementations

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Struct `Iter`

```rust
pub struct Iter<''a, T> {
    pub(crate) iterator: std::iter::Map<std::slice::Iter<''a, (std::sync::Arc<IdxInner>, T)>, &''a dyn Fn(&(std::sync::Arc<IdxInner>, T)) -> &T>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iterator` | `std::iter::Map<std::slice::Iter<''a, (std::sync::Arc<IdxInner>, T)>, &''a dyn Fn(&(std::sync::Arc<IdxInner>, T)) -> &T>` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Sync**
### Struct `EntriesMut`

```rust
pub struct EntriesMut<''a, T> {
    pub(crate) iterator: std::slice::IterMut<''a, (std::sync::Arc<IdxInner>, T)>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iterator` | `std::slice::IterMut<''a, (std::sync::Arc<IdxInner>, T)>` |  |

#### Implementations

##### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
### Struct `Entries`

```rust
pub struct Entries<''a, T> {
    pub(crate) iterator: std::slice::Iter<''a, (std::sync::Arc<IdxInner>, T)>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iterator` | `std::slice::Iter<''a, (std::sync::Arc<IdxInner>, T)>` |  |

#### Implementations

##### Trait Implementations

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

## Functions

### Function `choose_second_member_of_tuple_mut`

**Attributes:**

- `#[inline]`

```rust
pub(crate) fn choose_second_member_of_tuple_mut<A, B>((_, value): &mut (A, B)) -> &mut B { /* ... */ }
```

### Function `choose_second_member_of_tuple_ref`

**Attributes:**

- `#[inline]`

```rust
pub(crate) fn choose_second_member_of_tuple_ref<A, B>((_, value): &(A, B)) -> &B { /* ... */ }
```

### Function `create_idx`

**Attributes:**

- `#[inline]`

```rust
pub(crate) fn create_idx(index: usize) -> std::sync::Arc<IdxInner> { /* ... */ }
```

## Constants and Statics

### Constant `DEFAULT_CAPACITY`

```rust
pub(crate) const DEFAULT_CAPACITY: usize = 4;
```

