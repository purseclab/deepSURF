# Crate Documentation

**Version:** 0.7.0

**Format Version:** 39

# Module `string_interner`

Caches strings efficiently, with minimal memory footprint and associates them with unique symbols.
These symbols allow constant time comparisons and look-ups to the underlying interned strings.

### Example: Interning & Symbols

```
use string_interner::StringInterner;

let mut interner = StringInterner::default();
let sym0 = interner.get_or_intern("Elephant");
let sym1 = interner.get_or_intern("Tiger");
let sym2 = interner.get_or_intern("Horse");
let sym3 = interner.get_or_intern("Tiger");
assert_ne!(sym0, sym1);
assert_ne!(sym0, sym2);
assert_ne!(sym1, sym2);
assert_eq!(sym1, sym3); // same!
```

### Example: Creation by `FromIterator`

```
# use string_interner::StringInterner;
let interner = vec!["Elephant", "Tiger", "Horse", "Tiger"]
	.into_iter()
	.collect::<StringInterner>();
```

### Example: Look-up

```
# use string_interner::StringInterner;
let mut interner = StringInterner::default();
let sym = interner.get_or_intern("Banana");
assert_eq!(interner.resolve(sym), Some("Banana"));
```

### Example: Iteration

```
# use string_interner::StringInterner;
let interner = vec!["Earth", "Water", "Fire", "Air"]
	.into_iter()
	.collect::<StringInterner>();
for (sym, str) in interner {
	// iteration code here!
}
```

## Modules

## Module `serde_impl`

**Attributes:**

- `#[cfg(feature = "serde_support")]`

```rust
pub(crate) mod serde_impl { /* ... */ }
```

### Types

#### Struct `StringInternerVisitor`

```rust
pub(in ::serde_impl) struct StringInternerVisitor<Sym, H> {
    pub(in ::serde_impl) mark: marker::PhantomData<(Sym, H)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mark` | `marker::PhantomData<(Sym, H)>` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
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

- **Send**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Visitor**
  - ```rust
    fn expecting(self: &Self, formatter: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn visit_seq<A>(self: Self, seq: A) -> Result<<Self as >::Value, <A as >::Error>
where
    A: SeqAccess<''de> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Expected**
  - ```rust
    fn fmt(self: &Self, formatter: &mut Formatter<''_>) -> Result<(), Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Types

### Struct `Sym`

Symbol type used by the `DefaultStringInterner`.

# Note

This special symbol type has a memory footprint of 32 bits
and allows for certain space optimizations such as using it within an option: `Option<Sym>`

```rust
pub struct Sym(pub(crate) std::num::NonZeroU32);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::num::NonZeroU32` |  |

#### Implementations

##### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Sym) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sym { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Sym) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Eq**
- **Symbol**
  - ```rust
    fn from_usize(val: usize) -> Self { /* ... */ }
    ```
    Creates a `Sym` from the given `usize`.

  - ```rust
    fn to_usize(self: Self) -> usize { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Sym) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `InternalStrRef`

Internal reference to `str` used only within the `StringInterner` itself
to encapsulate the unsafe behaviour of interior references.

```rust
pub(crate) struct InternalStrRef(pub(crate) *const str);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `*const str` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn from_str(val: &str) -> Self { /* ... */ }
  ```
  Creates an InternalStrRef from a str.

- ```rust
  pub(crate) fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Reinterprets this InternalStrRef as a str.

##### Trait Implementations

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InternalStrRef { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InternalStrRef) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(val: T) -> Self { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **UnwindSafe**
### Type Alias `DefaultStringInterner`

`StringInterner` that uses `Sym` as its underlying symbol type.

```rust
pub type DefaultStringInterner = StringInterner<Sym>;
```

### Struct `StringInterner`

Caches strings efficiently, with minimal memory footprint and associates them with unique symbols.
These symbols allow constant time comparisons and look-ups to the underlying interned strings.

```rust
pub struct StringInterner<S, H = std::collections::hash_map::RandomState> {
    pub(crate) map: std::collections::HashMap<InternalStrRef, S, H>,
    pub(crate) values: Vec<Box<str>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `std::collections::HashMap<InternalStrRef, S, H>` |  |
| `values` | `Vec<Box<str>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> StringInterner<S, RandomState> { /* ... */ }
  ```
  Creates a new empty `StringInterner`.

- ```rust
  pub fn with_capacity(cap: usize) -> Self { /* ... */ }
  ```
  Creates a new `StringInterner` with the given initial capacity.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements the `StringInterner` can hold without reallocating.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserves capacity for at least `additional` more elements to be interned into `self`.

- ```rust
  pub fn with_hasher(hash_builder: H) -> StringInterner<S, H> { /* ... */ }
  ```
  Creates a new empty `StringInterner` with the given hasher.

- ```rust
  pub fn with_capacity_and_hasher(cap: usize, hash_builder: H) -> StringInterner<S, H> { /* ... */ }
  ```
  Creates a new empty `StringInterner` with the given initial capacity and the given hasher.

- ```rust
  pub fn get_or_intern<T>(self: &mut Self, val: T) -> S
where
    T: Into<String> + AsRef<str> { /* ... */ }
  ```
  Interns the given value.

- ```rust
  pub(crate) fn intern<T>(self: &mut Self, new_val: T) -> S
where
    T: Into<String> + AsRef<str> { /* ... */ }
  ```
  Interns the given value and ignores collissions.

- ```rust
  pub(crate) fn make_symbol(self: &Self) -> S { /* ... */ }
  ```
  Creates a new symbol for the current state of the interner.

- ```rust
  pub fn resolve(self: &Self, symbol: S) -> Option<&str> { /* ... */ }
  ```
  Returns the string slice associated with the given symbol if available,

- ```rust
  pub unsafe fn resolve_unchecked(self: &Self, symbol: S) -> &str { /* ... */ }
  ```
  Returns the string associated with the given symbol.

- ```rust
  pub fn get<T>(self: &Self, val: T) -> Option<S>
where
    T: AsRef<str> { /* ... */ }
  ```
  Returns the symbol associated with the given string for this interner

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of uniquely interned strings within this interner.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the string interner holds no elements.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, S> { /* ... */ }
  ```
  Returns an iterator over the interned strings.

- ```rust
  pub fn iter_values(self: &Self) -> Values<''_, S> { /* ... */ }
  ```
  Returns an iterator over all intern indices and their associated strings.

- ```rust
  pub fn shrink_to_fit(self: &mut Self) { /* ... */ }
  ```
  Shrinks the capacity of the interner as much as possible.

##### Trait Implementations

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deserialize**
  - ```rust
    fn deserialize<D>(deserializer: D) -> Result<StringInterner<Sym, H>, <D as >::Error>
where
    D: Deserializer<''de> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DeserializeOwned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StringInterner<S, H> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, rhs: &Self) -> bool { /* ... */ }
    ```

- **Serialize**
  - ```rust
    fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: Serializer { /* ... */ }
    ```

- **Send**
- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Iter`

Iterator over the pairs of associated symbols and interned strings for a `StringInterner`.

```rust
pub struct Iter<''a, S> {
    pub(crate) iter: iter::Enumerate<slice::Iter<''a, Box<str>>>,
    pub(crate) mark: marker::PhantomData<S>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `iter::Enumerate<slice::Iter<''a, Box<str>>>` |  |
| `mark` | `marker::PhantomData<S>` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn new<H>(interner: &''a StringInterner<S, H>) -> Self
where
    H: BuildHasher { /* ... */ }
  ```
  Creates a new iterator for the given StringIterator over pairs of

##### Trait Implementations

- **Freeze**
- **Sync**
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

- **UnwindSafe**
- **RefUnwindSafe**
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
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Struct `Values`

Iterator over the interned strings of a `StringInterner`.

```rust
pub struct Values<''a, S> {
    pub(crate) iter: slice::Iter<''a, Box<str>>,
    pub(crate) mark: marker::PhantomData<S>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `slice::Iter<''a, Box<str>>` |  |
| `mark` | `marker::PhantomData<S>` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn new<H>(interner: &''a StringInterner<S, H>) -> Self
where
    H: BuildHasher { /* ... */ }
  ```
  Creates a new iterator for the given StringIterator over its interned strings.

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `IntoIter`

Iterator over the pairs of associated symbol and strings.

Consumes the `StringInterner` upon usage.

```rust
pub struct IntoIter<S> {
    pub(crate) iter: iter::Enumerate<vec::IntoIter<Box<str>>>,
    pub(crate) mark: marker::PhantomData<S>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `iter` | `iter::Enumerate<vec::IntoIter<Box<str>>>` |  |
| `mark` | `marker::PhantomData<S>` |  |

#### Implementations

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
## Traits

### Trait `Symbol`

Types implementing this trait are able to act as symbols for string interners.

Symbols are returned by `StringInterner::get_or_intern` and allow look-ups of the
original string contents with `StringInterner::resolve`.

# Note

Optimal symbols allow for efficient comparisons and have a small memory footprint.

```rust
pub trait Symbol: Copy + Ord + Eq {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `from_usize`: Creates a symbol from a `usize`.
- `to_usize`: Returns the `usize` representation of `self`.

#### Implementations

This trait is implemented for the following types:

- `Sym`
- `usize`

