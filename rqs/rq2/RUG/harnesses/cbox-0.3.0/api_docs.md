# Crate Documentation

**Version:** 0.3.0

**Format Version:** 45

# Module `cbox`

Provides two types, `CSemiBox` and `DisposeRef`

## Types

### Struct `CSemiBox`

A wrapper for pointers made by C that are now partially owned in Rust.

This is necessary to allow owned and borrowed representations of C types
to be represented by the same type as they are in C with little overhead

```rust
pub struct CSemiBox<''a, D> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(ptr: *mut <D as >::RefTo) -> Self { /* ... */ }
  ```
  Wrap the pointer in a `CSemiBox`

- ```rust
  pub unsafe fn as_ptr(self: &Self) -> *mut <D as >::RefTo { /* ... */ }
  ```
  Returns the internal pointer

- ```rust
  pub unsafe fn unwrap(self: Self) -> *mut <D as >::RefTo { /* ... */ }
  ```
  Returns the internal pointer

##### Trait Implementations

- **Send**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(ptr: *mut <D as >::RefTo) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(text: &''a CStr) -> CSemiBox<''a, str> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &D { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```
    Run the destructor

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut D { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &D { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Struct `CBox`

A wrapper for pointers made by C that are now completely owned by Rust, so
they are not limited by any lifetimes.

This is necessary to allow owned and borrowed representations of C types
to be represented by the same type as they are in C with little overhead.

```rust
pub struct CBox<D> {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(ptr: *mut <D as >::RefTo) -> Self { /* ... */ }
  ```
  Wrap the pointer in a `CBox`.

- ```rust
  pub unsafe fn as_ptr(self: &Self) -> *mut <D as >::RefTo { /* ... */ }
  ```
  Returns the internal pointer.

- ```rust
  pub unsafe fn unwrap(self: Self) -> *mut <D as >::RefTo { /* ... */ }
  ```
  Returns the internal pointer.

- ```rust
  pub fn as_semi<''a>(self: &''a Self) -> &CSemiBox<''a, D> { /* ... */ }
  ```
  Returns the box as a 'CSemiBox'.

- ```rust
  pub fn as_semi_mut<''a>(self: &''a mut Self) -> &mut CSemiBox<''a, D> { /* ... */ }
  ```
  Returns the box as a 'CSemiBox'.

##### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &T) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CBox<str> { /* ... */ }
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

  - ```rust
    fn from(text: &''a str) -> CBox<str> { /* ... */ }
    ```
    Copy this text using malloc and strcpy.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Traits

### Trait `DisposeRef`

Implemented by any type of which its reference represents a C pointer that can be disposed.

```rust
pub trait DisposeRef {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Associated Types

- `RefTo`: What a reference to this type represents as a C pointer.

#### Provided Methods

- ```rust
  unsafe fn dispose(ptr: *mut <Self as >::RefTo) { /* ... */ }
  ```
  Destroy the contents at the pointer's location.

#### Implementations

This trait is implemented for the following types:

- `str`

