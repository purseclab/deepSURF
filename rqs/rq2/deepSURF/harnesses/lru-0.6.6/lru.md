# Crate Documentation

**Version:** 0.6.6

**Format Version:** 39

# Module `lru`

An implementation of a LRU cache. The cache supports `get`, `get_mut`, `put`,
and `pop` operations, all of which are O(1). This crate was heavily influenced
by the [LRU Cache implementation in an earlier version of Rust's std::collections crate](https://doc.rust-lang.org/0.12.0/std/collections/lru_cache/struct.LruCache.html).

## Example

```rust,no_run
extern crate lru;

use lru::LruCache;

fn main() {
        let mut cache = LruCache::new(2);
        cache.put("apple", 3);
        cache.put("banana", 2);

        assert_eq!(*cache.get(&"apple").unwrap(), 3);
        assert_eq!(*cache.get(&"banana").unwrap(), 2);
        assert!(cache.get(&"pear").is_none());

        assert_eq!(cache.put("banana", 4), Some(2));
        assert_eq!(cache.put("pear", 5), None);

        assert_eq!(*cache.get(&"pear").unwrap(), 5);
        assert_eq!(*cache.get(&"banana").unwrap(), 4);
        assert!(cache.get(&"apple").is_none());

        {
            let v = cache.get_mut(&"banana").unwrap();
            *v = 6;
        }

        assert_eq!(*cache.get(&"banana").unwrap(), 6);
}
```

## Types

### Struct `LruEntry`

```rust
pub(crate) struct LruEntry<K, V> {
    pub(crate) key: mem::MaybeUninit<K>,
    pub(crate) val: mem::MaybeUninit<V>,
    pub(crate) prev: *mut LruEntry<K, V>,
    pub(crate) next: *mut LruEntry<K, V>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `mem::MaybeUninit<K>` |  |
| `val` | `mem::MaybeUninit<V>` |  |
| `prev` | `*mut LruEntry<K, V>` |  |
| `next` | `*mut LruEntry<K, V>` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn new(key: K, val: V) -> Self { /* ... */ }
  ```

- ```rust
  pub(crate) fn new_sigil() -> Self { /* ... */ }
  ```

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **RefUnwindSafe**
### Type Alias `DefaultHasher`

**Attributes:**

- `#[cfg(feature = "hashbrown")]`

```rust
pub type DefaultHasher = hashbrown::hash_map::DefaultHashBuilder;
```

### Struct `LruCache`

An LRU Cache

```rust
pub struct LruCache<K, V, S = DefaultHasher> {
    pub(crate) map: hashbrown::HashMap<KeyRef<K>, alloc::boxed::Box<LruEntry<K, V>>, S>,
    pub(crate) cap: usize,
    pub(crate) head: *mut LruEntry<K, V>,
    pub(crate) tail: *mut LruEntry<K, V>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `hashbrown::HashMap<KeyRef<K>, alloc::boxed::Box<LruEntry<K, V>>, S>` |  |
| `cap` | `usize` |  |
| `head` | `*mut LruEntry<K, V>` |  |
| `tail` | `*mut LruEntry<K, V>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new(cap: usize) -> LruCache<K, V> { /* ... */ }
  ```
  Creates a new LRU Cache that holds at most `cap` items.

- ```rust
  pub fn unbounded() -> LruCache<K, V> { /* ... */ }
  ```
  Creates a new LRU Cache that never automatically evicts items.

- ```rust
  pub fn with_hasher(cap: usize, hash_builder: S) -> LruCache<K, V, S> { /* ... */ }
  ```
  Creates a new LRU Cache that holds at most `cap` items and

- ```rust
  pub fn unbounded_with_hasher(hash_builder: S) -> LruCache<K, V, S> { /* ... */ }
  ```
  Creates a new LRU Cache that never automatically evicts items and

- ```rust
  pub(crate) fn construct(cap: usize, map: HashMap<KeyRef<K>, Box<LruEntry<K, V>>, S>) -> LruCache<K, V, S> { /* ... */ }
  ```
  Creates a new LRU Cache with the given capacity.

- ```rust
  pub fn put(self: &mut Self, k: K, v: V) -> Option<V> { /* ... */ }
  ```
  Puts a key-value pair into cache. If the key already exists in the cache, then it updates

- ```rust
  pub fn get<''a, Q>(self: &''a mut Self, k: &Q) -> Option<&''a V>
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a reference to the value of the key in the cache or `None` if it is not

- ```rust
  pub fn get_mut<''a, Q>(self: &''a mut Self, k: &Q) -> Option<&''a mut V>
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value of the key in the cache or `None` if it

- ```rust
  pub fn peek<''a, Q>(self: &''a Self, k: &Q) -> Option<&''a V>
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a reference to the value corresponding to the key in the cache or `None` if it is

- ```rust
  pub fn peek_mut<''a, Q>(self: &''a mut Self, k: &Q) -> Option<&''a mut V>
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a mutable reference to the value corresponding to the key in the cache or `None`

- ```rust
  pub fn peek_lru<''a>(self: &Self) -> Option<(&''a K, &''a V)> { /* ... */ }
  ```
  Returns the value corresponding to the least recently used item or `None` if the

- ```rust
  pub fn contains<Q>(self: &Self, k: &Q) -> bool
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Returns a bool indicating whether the given key is in the cache. Does not update the

- ```rust
  pub fn pop<Q>(self: &mut Self, k: &Q) -> Option<V>
where
    KeyRef<K>: Borrow<Q>,
    Q: Hash + Eq + ?Sized { /* ... */ }
  ```
  Removes and returns the value corresponding to the key from the cache or

- ```rust
  pub fn pop_lru(self: &mut Self) -> Option<(K, V)> { /* ... */ }
  ```
  Removes and returns the key and value corresponding to the least recently

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of key-value pairs that are currently in the the cache.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns a bool indicating whether the cache is empty or not.

- ```rust
  pub fn cap(self: &Self) -> usize { /* ... */ }
  ```
  Returns the maximum number of key-value pairs the cache can hold.

- ```rust
  pub fn resize(self: &mut Self, cap: usize) { /* ... */ }
  ```
  Resizes the cache. If the new capacity is smaller than the size of the current

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the contents of the cache.

- ```rust
  pub fn iter<''a>(self: &Self) -> Iter<''a, K, V> { /* ... */ }
  ```
  An iterator visiting all entries in most-recently used order. The iterator element type is

- ```rust
  pub fn iter_mut<''a>(self: &mut Self) -> IterMut<''a, K, V> { /* ... */ }
  ```
  An iterator visiting all entries in most-recently-used order, giving a mutable reference on

- ```rust
  pub(crate) fn remove_last(self: &mut Self) -> Option<Box<LruEntry<K, V>>> { /* ... */ }
  ```

- ```rust
  pub(crate) fn detach(self: &mut Self, node: *mut LruEntry<K, V>) { /* ... */ }
  ```

- ```rust
  pub(crate) fn attach(self: &mut Self, node: *mut LruEntry<K, V>) { /* ... */ }
  ```

##### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a, K, V> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, K, V> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
### Struct `Iter`

An iterator over the entries of a `LruCache`.

This `struct` is created by the [`iter`] method on [`LruCache`][`LruCache`]. See its
documentation for more.

[`iter`]: struct.LruCache.html#method.iter
[`LruCache`]: struct.LruCache.html

```rust
pub struct Iter<''a, K: ''a, V: ''a> {
    pub(crate) len: usize,
    pub(crate) ptr: *const LruEntry<K, V>,
    pub(crate) end: *const LruEntry<K, V>,
    pub(crate) phantom: core::marker::PhantomData<&''a K>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `len` | `usize` |  |
| `ptr` | `*const LruEntry<K, V>` |  |
| `end` | `*const LruEntry<K, V>` |  |
| `phantom` | `core::marker::PhantomData<&''a K>` |  |

#### Implementations

##### Trait Implementations

- **Sync**
- **ExactSizeIterator**
- **Unpin**
- **FusedIterator**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(&''a K, &''a V)> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Iter<''a, K, V> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(&''a K, &''a V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **UnwindSafe**
### Struct `IterMut`

An iterator over mutables entries of a `LruCache`.

This `struct` is created by the [`iter_mut`] method on [`LruCache`][`LruCache`]. See its
documentation for more.

[`iter_mut`]: struct.LruCache.html#method.iter_mut
[`LruCache`]: struct.LruCache.html

```rust
pub struct IterMut<''a, K: ''a, V: ''a> {
    pub(crate) len: usize,
    pub(crate) ptr: *mut LruEntry<K, V>,
    pub(crate) end: *mut LruEntry<K, V>,
    pub(crate) phantom: core::marker::PhantomData<&''a K>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `len` | `usize` |  |
| `ptr` | `*mut LruEntry<K, V>` |  |
| `end` | `*mut LruEntry<K, V>` |  |
| `phantom` | `core::marker::PhantomData<&''a K>` |  |

#### Implementations

##### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **UnwindSafe**
- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<(&''a K, &''a mut V)> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

  - ```rust
    fn count(self: Self) -> usize { /* ... */ }
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

- **ExactSizeIterator**
- **FusedIterator**
- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<(&''a K, &''a mut V)> { /* ... */ }
    ```

