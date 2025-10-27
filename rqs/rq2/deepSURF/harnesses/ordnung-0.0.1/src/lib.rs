//! # Ordnung
//!
//! Fast, vector-based map implementation that preserves insertion order.
//!
//! + Map is implemented as a binary tree over a `Vec` for storage, with only
//!   two extra words per entry for book-keeping on 64-bit architectures.
//! + A fast hash function with good random distribution is used to balance the
//!   tree. Ordnung makes no guarantees that the tree will be perfectly
//!   balanced, but key lookup should be approaching `O(log n)` in most cases.
//! + Tree traversal is always breadth-first and happens over a single
//!   continuous block of memory, which makes it cache friendly.
//! + Iterating over all entries is always `O(n)`, same as `Vec<(K, V)>`.
//! + There are no buckets, so there is no need to re-bucket things when growing
//!   the map.
//!
//! ## When should you use this?
//!
//! + You need to preserve insertion order of the map.
//! + Iterating over the map is very performance sensitive.
//! + Your average map has fewer than 100 entries.
//! + You have no a priori knowledge about the final size of the map when you
//!   start creating it.
//! + Removing items from the map is very, very rare.
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

use core::{mem, slice, fmt};
use core::borrow::Borrow;
use core::num::NonZeroU32;
use core::iter::FromIterator;
use core::cell::Cell;
use core::hash::{Hash, Hasher};
use core::ops::Index;

pub mod compact;

pub use compact::Vec;
// use alloc::vec::Vec;

#[inline]
fn hash_key<H: Hash>(hash: H) -> u64 {
    // let mut hasher = fnv::FnvHasher::default();
    // let mut hasher = rustc_hash::FxHasher::default();
    let mut hasher = ahash::AHasher::default();

    hash.hash(&mut hasher);

    hasher.finish()
}

#[derive(Clone)]
struct Node<K, V> {
    // Key
    pub key: K,

    // Hash of the key
    pub hash: u64,

    // Value stored.
    pub value: V,

    // Store vector index pointing to the `Node` for which `hash` is smaller
    // than that of this `Node`.
    pub left: Cell<Option<NonZeroU32>>,

    // Same as above but for `Node`s with hash larger than this one. If the
    // hash is the same, but keys are different, the lookup will default
    // to the right branch as well.
    pub right: Cell<Option<NonZeroU32>>,
}

impl<K, V> fmt::Debug for Node<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&(&self.key, &self.value, self.left.get(), self.right.get()), f)
    }
}

impl<K, V> PartialEq for Node<K, V>
where
    K: PartialEq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash &&
        self.key == other.key &&
        self.value == other.value
    }
}

impl<K, V> Node<K, V> {
    #[inline]
    const fn new(key: K, value: V, hash: u64) -> Self {
        Node {
            key,
            hash,
            value,
            left: Cell::new(None),
            right: Cell::new(None),
        }
    }
}

// `Cell` isn't `Sync`, but all of our writes are contained and require
// `&mut` access, ergo this is safe.
unsafe impl<K: Sync, V: Sync> Sync for Node<K, V> {}

/// A binary tree implementation of a string -> `JsonValue` map. You normally don't
/// have to interact with instances of `Object`, much more likely you will be
/// using the `JsonValue::Object` variant, which wraps around this struct.
#[derive(Debug, Clone)]
pub struct Map<K, V> {
    store: Vec<Node<K, V>>
}

enum FindResult<'find> {
    Hit(usize),
    Miss(Option<&'find Cell<Option<NonZeroU32>>>),
}

use FindResult::*;

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    /// Create a new `Map`.
    #[inline]
    pub fn new() -> Self {
        Map {
            store: Vec::new()
        }
    }

    /// Create a `Map` with a given capacity
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Map {
            store: Vec::with_capacity(capacity)
        }
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = hash_key(&key);

        match self.find(&key, hash) {
            Hit(idx) => unsafe {
                let slot = &mut self.store.get_unchecked_mut(idx).value;

                Some(core::mem::replace(slot, value))
            },
            Miss(parent) => {
                if let Some(parent) = parent {
                    parent.set(NonZeroU32::new(self.store.len() as u32));
                }

                self.store.push(Node::new(key, value, hash));

                None
            },
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = hash_key(key);

        match self.find(key, hash) {
            Hit(idx) => Some(unsafe { &self.store.get_unchecked(idx).value }),
            Miss(_) => None,
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = hash_key(key);

        match self.find(key, hash) {
            Hit(_) => true,
            Miss(_) => false,
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = hash_key(key);

        match self.find(key, hash) {
            Hit(idx) => Some(unsafe { &mut self.store.get_unchecked_mut(idx).value }),
            Miss(_) => None,
        }
    }

    /// Get a mutable reference to entry at key. Inserts a new entry by
    /// calling `F` if absent.
    // TODO: Replace with entry API
    pub fn get_or_insert<F>(&mut self, key: K, fill: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        let key = key.into();
        let hash = hash_key(&key);

        match self.find(&key, hash) {
            Hit(idx) => &mut self.store[idx].value,
            Miss(parent) => {
                let idx = self.store.len();

                if let Some(parent) = parent {
                    parent.set(NonZeroU32::new(self.store.len() as u32));
                }

                self.store.push(Node::new(key, fill(), hash));

                &mut self.store[idx].value
            },
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = hash_key(key);

        let index = match self.find(key, hash) {
            Hit(idx) => idx,
            Miss(_) => return None,
        };

        // Removing a node would screw the tree badly, it's easier to just
        // recreate it.
        let mut removed = None;
        let capacity = self.store.len();
        let old = mem::replace(&mut self.store, Vec::with_capacity(capacity));

        for (i, Node { key, value, hash, .. }) in old.into_iter().enumerate() {
            if i == index {
                // Rust doesn't like us moving things from `node`, even if
                // it is owned. Replace fixes that.
                removed = Some(value);
            } else {
                // Faster than .insert() since we can avoid hashing
                if let Miss(Some(parent)) = self.find(key.borrow(), hash) {
                    parent.set(NonZeroU32::new(self.store.len() as u32));
                }

                self.store.push(Node::new(key, value, hash));
            }
        }

        removed
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    #[inline]
    pub fn clear(&mut self) {
        self.store.clear();
    }

    #[inline]
    fn find<Q: ?Sized>(&self, key: &Q, hash: u64) -> FindResult
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        if self.len() == 0 {
            return Miss(None);
        }

        let mut idx = 0;

        loop {
            let node = unsafe { self.store.get_unchecked(idx) };

            if hash == node.hash && key == node.key.borrow() {
                return Hit(idx);
            } else if hash < node.hash {
                match node.left.get() {
                    Some(i) => idx = i.get() as usize,
                    None => return Miss(Some(&node.left)),
                }
            } else {
                match node.right.get() {
                    Some(i) => idx = i.get() as usize,
                    None => return Miss(Some(&node.right)),
                }
            }
        }
    }

    /// An iterator visiting all key-value pairs in insertion order.
    /// The iterator element type is `(&K, &V)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// let entries: Vec<_> = map.iter().collect();
    ///
    /// assert_eq!(
    ///     entries,
    ///     &[
    ///         (&"a", &1),
    ///         (&"b", &2),
    ///         (&"c", &3),
    ///     ],
    /// );
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner: self.store.iter()
        }
    }

    /// An iterator visiting all key-value pairs in insertion order, with
    /// mutable references to the values. The iterator element type is
    /// (&K, &mut V).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ordnung::Map;
    ///
    /// let mut map = Map::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// // Update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val *= 2;
    /// }
    ///
    /// // Check if values are doubled
    /// let entries: Vec<_> = map.iter().collect();
    ///
    /// assert_eq!(
    ///     entries,
    ///     &[
    ///         (&"a", &2),
    ///         (&"b", &4),
    ///         (&"c", &6),
    ///     ],
    /// );
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            inner: self.store.iter_mut()
        }
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for Map<K, V>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the HashMap.
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("Key not found in Map")
    }
}

impl<'json, IK, IV, K, V> FromIterator<(IK, IV)> for Map<K, V>
where
    IK: Into<K>,
    IV: Into<V>,
    K: Hash + Eq,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=(IK, IV)>,
    {
        let iter = iter.into_iter();
        let mut map = Map::with_capacity(iter.size_hint().0);

        for (key, value) in iter {
            map.insert(key.into(), value.into());
        }

        map
    }
}

// Because keys can inserted in different order, the safe way to
// compare `Map`s is to iterate over one and check if the other
// has all the same keys.
impl<K, V> PartialEq for Map<K, V>
where
    K: Hash + Eq,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        // Faster than .get() since we can avoid hashing
        for &Node { ref key, ref value, hash, .. } in self.store.iter() {
            if let Hit(idx) = other.find(key, hash) {
                if &other.store[idx].value == value {
                    continue;
                }
            }

            return false;
        }

        true
    }
}

/// An iterator over the entries of a `Map`.
///
/// This struct is created by the [`iter`](./struct.Map.html#method.iter)
/// method on [`Map`](./struct.Map.html). See its documentation for more.
pub struct Iter<'a, K, V> {
    inner: slice::Iter<'a, Node<K, V>>,
}

/// A mutable iterator over the entries of a `Map`.
///
/// This struct is created by the [`iter_mut`](./struct.Map.html#method.iter_mut)
/// method on [`Map`](./struct.Map.html). See its documentation for more.
pub struct IterMut<'a, K, V> {
    inner: slice::IterMut<'a, Node<K, V>>,
}

impl<K, V> Iter<'_, K, V> {
    /// Create an empty iterator that always returns `None`
    pub fn empty() -> Self {
        Iter {
            inner: [].iter()
        }
    }
}

impl<'i, K, V> Iterator for Iter<'i, K, V> {
    type Item = (&'i K, &'i V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| (&node.key, &node.value))
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|node| (&node.key, &node.value))
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> IterMut<'_, K, V> {
    /// Create an empty iterator that always returns `None`
    pub fn empty() -> Self {
        IterMut {
            inner: [].iter_mut()
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| (&node.key, &mut node.value))
    }
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|node| (&node.key, &mut node.value))
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn empty() {
        let map: Map<&str, u64> = Map::new();

        assert_eq!(map.get("foo"), None);
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn simple() {
        let mut map: Map<&str, u64> = Map::new();

        map.insert("foo", 42);

        assert_eq!(map.get("foo"), Some(&42));
        assert_eq!(map.len(), 1);
        assert_eq!(map.is_empty(), false);
    }
}