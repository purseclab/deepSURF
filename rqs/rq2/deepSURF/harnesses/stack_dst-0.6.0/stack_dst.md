# Crate Documentation

**Version:** 0.6.0

**Format Version:** 39

# Module `stack_dst`

Support for storing dynamically-sized types on the stack

The `Value` type provides a fixed size (7 word in the current version) buffer in which a trait object
or array can be stored, without resorting to a heap allocation.

# Examples
## An unboxed any
As a quick example - The following wraps a 64-bit integer up in an inline DST using the Any trait.

```rust
use std::any::Any;
use stack_dst::Value;

let dst = Value::<dyn Any>::new_stable(1234u64, |p| p as _).ok().expect("Integer did not fit in allocation");
println!("dst as u64 = {:?}", dst.downcast_ref::<u64>());
println!("dst as i8 = {:?}", dst.downcast_ref::<i8>());
```
 
## Stack-allocated closure!
The following snippet shows how small (`'static`) closures can be returned using this crate

```rust
# fn main() {
use stack_dst::Value;
 
fn make_closure(value: u64) -> Value<dyn FnMut()->String> {
    Value::new_stable(move || format!("Hello there! value={}", value), |p| p as _)
        .ok().expect("Closure doesn't fit")
}
let mut closure = make_closure(666);
assert_eq!( (&mut *closure)(), "Hello there! value=666" );
# }
```

# Features
## `std` (default)
Enables the use of the standard library as a dependency
## `alloc` (default)
Provides the `StackDstA::new_or_boxed` method (if `unsize` feature is active too)
## `unsize` (optional)
Uses the nightly feature `unsize` to provide a more egonomic API (no need for the `|p| p` closures)


## Modules

## Module `value`

Single DST stored inline



```rust
pub(crate) mod value { /* ... */ }
```

### Modules

## Module `trait_impls`

```rust
pub(in ::value) mod trait_impls { /* ... */ }
```

### Types

#### Type Alias `Value`

Stack-allocated DST (using a default size)

```rust
pub type Value<T> = ValueA<T, [usize; 9]>;
```

#### Struct `ValueA`

Stack-allocated dynamically sized type

`T` is the unsized type contaned.
`D` is the buffer used to hold the unsized type (both data and metadata).

```rust
pub struct ValueA<T: ?Sized, D: ::DataBuf> {
    pub(in ::value) _align: [u64; 0],
    pub(in ::value) _pd: marker::PhantomData<T>,
    pub(in ::value) data: D,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_align` | `[u64; 0]` |  |
| `_pd` | `marker::PhantomData<T>` |  |
| `data` | `D` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_stable<U, F: FnOnce(&U) -> &T>(val: U, get_ref: F) -> Result<ValueA<T, D>, U> { /* ... */ }
  ```
  Construct a stack-based DST (without needing `Unsize`)

- ```rust
  pub unsafe fn new_raw(info: &[usize], data: *mut (), size: usize) -> Option<ValueA<T, D>> { /* ... */ }
  ```
  UNSAFE: `data` must point to `size` bytes, which shouldn't be freed if `Some` is returned

- ```rust
  pub(in ::value) unsafe fn as_ptr(self: &Self) -> *mut T { /* ... */ }
  ```
  Obtain raw pointer to the contained data

###### Trait Implementations

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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &T { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **Send**
- **Future**
  - ```rust
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<''_>) -> task::Poll<<Self as >::Output> { /* ... */ }
    ```

- **Receiver**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

### Constants and Statics

#### Constant `DEFAULT_SIZE`

8 data words, plus one metadata

```rust
pub const DEFAULT_SIZE: usize = _;
```

## Module `stack`

 



```rust
pub(crate) mod stack { /* ... */ }
```

### Types

#### Struct `StackA`

A fixed-capacity stack that can contain dynamically-sized types

Uses an array of usize as a backing store for a First-In, Last-Out stack
of items that can unsize to `T`.

Note: Each item in the stack takes at least one `usize` (to store the metadata)

```rust
pub struct StackA<T: ?Sized, D: ::DataBuf> {
    pub(in ::stack) _pd: marker::PhantomData<*const T>,
    pub(in ::stack) next_ofs: usize,
    pub(in ::stack) data: D,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_pd` | `marker::PhantomData<*const T>` |  |
| `next_ofs` | `usize` |  |
| `data` | `D` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> StackA<T, D> { /* ... */ }
  ```
  Construct a new (empty) stack

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Tests if the stack is empty

- ```rust
  pub(in ::stack) fn meta_words() -> usize { /* ... */ }
  ```

- ```rust
  pub(in ::stack) fn push_inner(self: &mut Self, fat_ptr: &T) -> Result<&mut [usize], ()> { /* ... */ }
  ```

- ```rust
  pub fn push_stable<U, F: FnOnce(&U) -> &T>(self: &mut Self, v: U, f: F) -> Result<(), U> { /* ... */ }
  ```
  Push a value at the top of the stack (without using `Unsize`)

- ```rust
  pub(in ::stack) fn top_raw(self: &Self) -> Option<*mut T> { /* ... */ }
  ```

- ```rust
  pub fn top(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns a pointer to the top item on the stack

- ```rust
  pub fn top_mut(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Returns a pointer to the top item on the stack (unique/mutable)

- ```rust
  pub fn pop(self: &mut Self) { /* ... */ }
  ```
  Pop the top item off the stack

- ```rust
  pub fn push_str(self: &mut Self, v: &str) -> Result<(), ()> { /* ... */ }
  ```
  Push the contents of a string slice as an item onto the stack

- ```rust
  pub fn push_cloned(self: &mut Self, v: &[T]) -> Result<(), ()> { /* ... */ }
  ```
  Pushes a set of items (cloning out of the input slice)

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Traits

### Trait `DataBuf`

Trait used to represent a data buffer, typically you'll passs a `[usize; N]` array.

```rust
pub trait DataBuf: Copy + Default + AsMut<[usize]> + AsRef<[usize]> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Implementations

This trait is implemented for the following types:

- `T` with <T: Copy + Default + AsMut<[usize]> + AsRef<[usize]>>

## Functions

### Function `ptr_as_slice`

Obtain mutable access to a pointer's words

```rust
pub(crate) fn ptr_as_slice<T>(ptr: &mut T) -> &mut [usize] { /* ... */ }
```

### Function `make_fat_ptr`

Re-construct a fat pointer

```rust
pub(crate) unsafe fn make_fat_ptr<T: ?Sized>(data_ptr: usize, meta_vals: &[usize]) -> *mut T { /* ... */ }
```

### Function `round_to_words`

```rust
pub(crate) fn round_to_words(len: usize) -> usize { /* ... */ }
```

## Re-exports

### Re-export `ValueA`

```rust
pub use value::ValueA;
```

### Re-export `Value`

```rust
pub use value::Value;
```

### Re-export `StackA`

```rust
pub use stack::StackA;
```

