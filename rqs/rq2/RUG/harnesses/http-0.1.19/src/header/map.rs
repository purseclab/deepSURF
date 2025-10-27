use std::{fmt, mem, ops, ptr, vec};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
use std::iter::FromIterator;
use std::marker::PhantomData;
use convert::{HttpTryFrom, HttpTryInto};
use Error;
use super::HeaderValue;
use super::name::{HdrName, HeaderName, InvalidHeaderName};
pub use self::as_header_name::AsHeaderName;
pub use self::into_header_name::IntoHeaderName;
/// A set of HTTP headers
///
/// `HeaderMap` is an multimap of [`HeaderName`] to values.
///
/// [`HeaderName`]: struct.HeaderName.html
///
/// # Examples
///
/// Basic usage
///
/// ```
/// # use http::HeaderMap;
/// # use http::header::{CONTENT_LENGTH, HOST, LOCATION};
/// let mut headers = HeaderMap::new();
///
/// headers.insert(HOST, "example.com".parse().unwrap());
/// headers.insert(CONTENT_LENGTH, "123".parse().unwrap());
///
/// assert!(headers.contains_key(HOST));
/// assert!(!headers.contains_key(LOCATION));
///
/// assert_eq!(headers[HOST], "example.com");
///
/// headers.remove(HOST);
///
/// assert!(!headers.contains_key(HOST));
/// ```
#[derive(Clone)]
pub struct HeaderMap<T = HeaderValue> {
    mask: Size,
    indices: Box<[Pos]>,
    entries: Vec<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
    danger: Danger,
}
/// `HeaderMap` entry iterator.
///
/// Yields `(&HeaderName, &value)` tuples. The same header name may be yielded
/// more than once if it has more than one associated value.
#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    inner: IterMut<'a, T>,
}
/// `HeaderMap` mutable entry iterator
///
/// Yields `(&HeaderName, &mut value)` tuples. The same header name may be
/// yielded more than once if it has more than one associated value.
#[derive(Debug)]
pub struct IterMut<'a, T: 'a> {
    map: *mut HeaderMap<T>,
    entry: usize,
    cursor: Option<Cursor>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
/// An owning iterator over the entries of a `HeaderMap`.
///
/// This struct is created by the `into_iter` method on `HeaderMap`.
#[derive(Debug)]
pub struct IntoIter<T> {
    next: Option<usize>,
    entries: vec::IntoIter<Bucket<T>>,
    extra_values: Vec<ExtraValue<T>>,
}
/// An iterator over `HeaderMap` keys.
///
/// Each header name is yielded only once, even if it has more than one
/// associated value.
#[derive(Debug)]
pub struct Keys<'a, T: 'a> {
    inner: ::std::slice::Iter<'a, Bucket<T>>,
}
/// `HeaderMap` value iterator.
///
/// Each value contained in the `HeaderMap` will be yielded.
#[derive(Debug)]
pub struct Values<'a, T: 'a> {
    inner: Iter<'a, T>,
}
/// `HeaderMap` mutable value iterator
#[derive(Debug)]
pub struct ValuesMut<'a, T: 'a> {
    inner: IterMut<'a, T>,
}
/// A drain iterator for `HeaderMap`.
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    idx: usize,
    map: *mut HeaderMap<T>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
/// A view to all values stored in a single entry.
///
/// This struct is returned by `HeaderMap::get_all`.
#[derive(Debug)]
pub struct GetAll<'a, T: 'a> {
    map: &'a HeaderMap<T>,
    index: Option<usize>,
}
/// A view into a single location in a `HeaderMap`, which may be vacant or occupied.
#[derive(Debug)]
pub enum Entry<'a, T: 'a> {
    /// An occupied entry
    Occupied(OccupiedEntry<'a, T>),
    /// A vacant entry
    Vacant(VacantEntry<'a, T>),
}
/// A view into a single empty location in a `HeaderMap`.
///
/// This struct is returned as part of the `Entry` enum.
#[derive(Debug)]
pub struct VacantEntry<'a, T: 'a> {
    map: &'a mut HeaderMap<T>,
    key: HeaderName,
    hash: HashValue,
    probe: usize,
    danger: bool,
}
/// A view into a single occupied location in a `HeaderMap`.
///
/// This struct is returned as part of the `Entry` enum.
#[derive(Debug)]
pub struct OccupiedEntry<'a, T: 'a> {
    map: &'a mut HeaderMap<T>,
    probe: usize,
    index: usize,
}
/// An iterator of all values associated with a single header name.
#[derive(Debug)]
pub struct ValueIter<'a, T: 'a> {
    map: &'a HeaderMap<T>,
    index: usize,
    front: Option<Cursor>,
    back: Option<Cursor>,
}
/// A mutable iterator of all values associated with a single header name.
#[derive(Debug)]
pub struct ValueIterMut<'a, T: 'a> {
    map: *mut HeaderMap<T>,
    index: usize,
    front: Option<Cursor>,
    back: Option<Cursor>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
/// An drain iterator of all values associated with a single header name.
#[derive(Debug)]
pub struct ValueDrain<'a, T: 'a> {
    map: *mut HeaderMap<T>,
    first: Option<T>,
    next: Option<usize>,
    lt: PhantomData<&'a mut HeaderMap<T>>,
}
/// Tracks the value iterator state
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cursor {
    Head,
    Values(usize),
}
/// Type used for representing the size of a HeaderMap value.
///
/// 32,768 is more than enough entries for a single header map. Setting this
/// limit enables using `u16` to represent all offsets, which takes 2 bytes
/// instead of 8 on 64 bit processors.
///
/// Setting this limit is especially benificial for `indices`, making it more
/// cache friendly. More hash codes can fit in a cache line.
///
/// You may notice that `u16` may represent more than 32,768 values. This is
/// true, but 32,768 should be plenty and it allows us to reserve the top bit
/// for future usage.
type Size = usize;
/// This limit falls out from above.
const MAX_SIZE: usize = (1 << 15);
/// An entry in the hash table. This represents the full hash code for an entry
/// as well as the position of the entry in the `entries` vector.
#[derive(Copy, Clone)]
struct Pos {
    index: Size,
    hash: HashValue,
}
/// Hash values are limited to u16 as well. While `fast_hash` and `Hasher`
/// return `usize` hash codes, limiting the effective hash code to the lower 16
/// bits is fine since we know that the `indices` vector will never grow beyond
/// that size.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct HashValue(usize);
/// Stores the data associated with a `HeaderMap` entry. Only the first value is
/// included in this struct. If a header name has more than one associated
/// value, all extra values are stored in the `extra_values` vector. A doubly
/// linked list of entries is maintained. The doubly linked list is used so that
/// removing a value is constant time. This also has the nice property of
/// enabling double ended iteration.
#[derive(Debug, Clone)]
struct Bucket<T> {
    hash: HashValue,
    key: HeaderName,
    value: T,
    links: Option<Links>,
}
/// The head and tail of the value linked list.
#[derive(Debug, Copy, Clone)]
struct Links {
    next: usize,
    tail: usize,
}
/// Node in doubly-linked list of header value entries
#[derive(Debug, Clone)]
struct ExtraValue<T> {
    value: T,
    prev: Link,
    next: Link,
}
/// A header value node is either linked to another node in the `extra_values`
/// list or it points to an entry in `entries`. The entry in `entries` is the
/// start of the list and holds the associated header name.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Link {
    Entry(usize),
    Extra(usize),
}
/// Tracks the header map danger level! This relates to the adaptive hashing
/// algorithm. A HeaderMap starts in the "green" state, when a large number of
/// collisions are detected, it transitions to the yellow state. At this point,
/// the header map will either grow and switch back to the green state OR it
/// will transition to the red state.
///
/// When in the red state, a safe hashing algorithm is used and all values in
/// the header map have to be rehashed.
#[derive(Clone)]
enum Danger {
    Green,
    Yellow,
    Red(RandomState),
}
const DISPLACEMENT_THRESHOLD: usize = 128;
const FORWARD_SHIFT_THRESHOLD: usize = 512;
const LOAD_FACTOR_THRESHOLD: f32 = 0.2;
macro_rules! probe_loop {
    ($label:tt : $probe_var:ident < $len:expr, $body:expr) => {
        debug_assert!($len > 0); $label : loop { if $probe_var < $len { $body $probe_var
        += 1; } else { $probe_var = 0; } }
    };
    ($probe_var:ident < $len:expr, $body:expr) => {
        debug_assert!($len > 0); loop { if $probe_var < $len { $body $probe_var += 1; }
        else { $probe_var = 0; } }
    };
}
macro_rules! insert_phase_one {
    (
        $map:ident, $key:expr, $probe:ident, $pos:ident, $hash:ident, $danger:ident,
        $vacant:expr, $occupied:expr, $robinhood:expr
    ) => {
        { let $hash = hash_elem_using(&$map .danger, &$key); let mut $probe =
        desired_pos($map .mask, $hash); let mut dist = 0; let ret; probe_loop!('probe :
        $probe < $map .indices.len(), { if let Some(($pos, entry_hash)) = $map
        .indices[$probe].resolve() { let their_dist = probe_distance($map .mask,
        entry_hash, $probe); if their_dist < dist { let $danger = dist >=
        FORWARD_SHIFT_THRESHOLD && !$map .danger.is_red(); ret = $robinhood; break
        'probe; } else if entry_hash == $hash && $map .entries[$pos].key == $key { ret =
        $occupied; break 'probe; } } else { let $danger = dist >= FORWARD_SHIFT_THRESHOLD
        && !$map .danger.is_red(); ret = $vacant; break 'probe; } dist += 1; }); ret }
    };
}
impl HeaderMap {
    /// Create an empty `HeaderMap`.
    ///
    /// The map will be created without any capacity. This function will not
    /// allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let map = HeaderMap::new();
    ///
    /// assert!(map.is_empty());
    /// assert_eq!(0, map.capacity());
    /// ```
    pub fn new() -> Self {
        HeaderMap::with_capacity(0)
    }
}
impl<T> HeaderMap<T> {
    /// Create an empty `HeaderMap` with the specified capacity.
    ///
    /// The returned map will allocate internal storage in order to hold about
    /// `capacity` elements without reallocating. However, this is a "best
    /// effort" as there are usage patterns that could cause additional
    /// allocations before `capacity` headers are stored in the map.
    ///
    /// More capacity than requested may be allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let map: HeaderMap<u32> = HeaderMap::with_capacity(10);
    ///
    /// assert!(map.is_empty());
    /// assert_eq!(12, map.capacity());
    /// ```
    pub fn with_capacity(capacity: usize) -> HeaderMap<T> {
        assert!(capacity <= MAX_SIZE, "requested capacity too large");
        if capacity == 0 {
            HeaderMap {
                mask: 0,
                indices: Box::new([]),
                entries: Vec::new(),
                extra_values: Vec::new(),
                danger: Danger::Green,
            }
        } else {
            let raw_cap = to_raw_capacity(capacity).next_power_of_two();
            debug_assert!(raw_cap > 0);
            HeaderMap {
                mask: (raw_cap - 1) as Size,
                indices: vec![Pos::none(); raw_cap].into_boxed_slice(),
                entries: Vec::with_capacity(raw_cap),
                extra_values: Vec::new(),
                danger: Danger::Green,
            }
        }
    }
    /// Returns the number of headers stored in the map.
    ///
    /// This number represents the total number of **values** stored in the map.
    /// This number can be greater than or equal to the number of **keys**
    /// stored given that a single key may have more than one associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{ACCEPT, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// assert_eq!(0, map.len());
    ///
    /// map.insert(ACCEPT, "text/plain".parse().unwrap());
    /// map.insert(HOST, "localhost".parse().unwrap());
    ///
    /// assert_eq!(2, map.len());
    ///
    /// map.append(ACCEPT, "text/html".parse().unwrap());
    ///
    /// assert_eq!(3, map.len());
    /// ```
    pub fn len(&self) -> usize {
        self.entries.len() + self.extra_values.len()
    }
    /// Returns the number of keys stored in the map.
    ///
    /// This number will be less than or equal to `len()` as each key may have
    /// more than one associated value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{ACCEPT, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// assert_eq!(0, map.keys_len());
    ///
    /// map.insert(ACCEPT, "text/plain".parse().unwrap());
    /// map.insert(HOST, "localhost".parse().unwrap());
    ///
    /// assert_eq!(2, map.keys_len());
    ///
    /// map.insert(ACCEPT, "text/html".parse().unwrap());
    ///
    /// assert_eq!(2, map.keys_len());
    /// ```
    pub fn keys_len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    ///
    /// assert!(map.is_empty());
    ///
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.entries.len() == 0
    }
    /// Clears the map, removing all key-value pairs. Keeps the allocated memory
    /// for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// map.clear();
    /// assert!(map.is_empty());
    /// assert!(map.capacity() > 0);
    /// ```
    pub fn clear(&mut self) {
        self.entries.clear();
        self.extra_values.clear();
        self.danger = Danger::Green;
        for e in self.indices.iter_mut() {
            *e = Pos::none();
        }
    }
    /// Returns the number of headers the map can hold without reallocating.
    ///
    /// This number is an approximation as certain usage patterns could cause
    /// additional allocations before the returned capacity is filled.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    ///
    /// assert_eq!(0, map.capacity());
    ///
    /// map.insert(HOST, "hello.world".parse().unwrap());
    /// assert_eq!(6, map.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        usable_capacity(self.indices.len())
    }
    /// Reserves capacity for at least `additional` more headers to be inserted
    /// into the `HeaderMap`.
    ///
    /// The header map may reserve more space to avoid frequent reallocations.
    /// Like with `with_capacity`, this will be a "best effort" to avoid
    /// allocations until `additional` more headers are inserted. Certain usage
    /// patterns could cause additional allocations before the number is
    /// reached.
    ///
    /// # Panics
    ///
    /// Panics if the new allocation size overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// map.reserve(10);
    /// # map.insert(HOST, "bar".parse().unwrap());
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        let cap = self.entries.len().checked_add(additional).expect("reserve overflow");
        if cap > self.indices.len() {
            let cap = cap.next_power_of_two();
            if self.entries.len() == 0 {
                self.mask = cap - 1;
                self.indices = vec![Pos::none(); cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(cap));
            } else {
                self.grow(cap);
            }
        }
    }
    /// Returns a reference to the value associated with the key.
    ///
    /// If there are multiple values associated with the key, then the first one
    /// is returned. Use `get_all` to get all values associated with a given
    /// key. Returns `None` if there are no values associated with the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// assert!(map.get("host").is_none());
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// assert_eq!(map.get(HOST).unwrap(), &"hello");
    /// assert_eq!(map.get("host").unwrap(), &"hello");
    ///
    /// map.append(HOST, "world".parse().unwrap());
    /// assert_eq!(map.get("host").unwrap(), &"hello");
    /// ```
    pub fn get<K>(&self, key: K) -> Option<&T>
    where
        K: AsHeaderName,
    {
        self.get2(&key)
    }
    fn get2<K>(&self, key: &K) -> Option<&T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((_, found)) => {
                let entry = &self.entries[found];
                Some(&entry.value)
            }
            None => None,
        }
    }
    /// Returns a mutable reference to the value associated with the key.
    ///
    /// If there are multiple values associated with the key, then the first one
    /// is returned. Use `entry` to get all values associated with a given
    /// key. Returns `None` if there are no values associated with the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::default();
    /// map.insert(HOST, "hello".to_string());
    /// map.get_mut("host").unwrap().push_str("-world");
    ///
    /// assert_eq!(map.get(HOST).unwrap(), &"hello-world");
    /// ```
    pub fn get_mut<K>(&mut self, key: K) -> Option<&mut T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((_, found)) => {
                let entry = &mut self.entries[found];
                Some(&mut entry.value)
            }
            None => None,
        }
    }
    /// Returns a view of all values associated with a key.
    ///
    /// The returned view does not incur any allocations and allows iterating
    /// the values associated with the key.  See [`GetAll`] for more details.
    /// Returns `None` if there are no values associated with the key.
    ///
    /// [`GetAll`]: struct.GetAll.html
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// map.append(HOST, "goodbye".parse().unwrap());
    ///
    /// let view = map.get_all("host");
    ///
    /// let mut iter = view.iter();
    /// assert_eq!(&"hello", iter.next().unwrap());
    /// assert_eq!(&"goodbye", iter.next().unwrap());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn get_all<K>(&self, key: K) -> GetAll<T>
    where
        K: AsHeaderName,
    {
        GetAll {
            map: self,
            index: key.find(self).map(|(_, i)| i),
        }
    }
    /// Returns true if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// assert!(!map.contains_key(HOST));
    ///
    /// map.insert(HOST, "world".parse().unwrap());
    /// assert!(map.contains_key("host"));
    /// ```
    pub fn contains_key<K>(&self, key: K) -> bool
    where
        K: AsHeaderName,
    {
        key.find(self).is_some()
    }
    /// An iterator visiting all key-value pairs.
    ///
    /// The iteration order is arbitrary, but consistent across platforms for
    /// the same crate version. Each key will be yielded once per associated
    /// value. So, if a key has 3 associated values, it will be yielded 3 times.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// map.append(HOST, "goodbye".parse().unwrap());
    /// map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{:?}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: IterMut {
                map: self as *const _ as *mut _,
                entry: 0,
                cursor: self.entries.first().map(|_| Cursor::Head),
                lt: PhantomData,
            },
        }
    }
    /// An iterator visiting all key-value pairs, with mutable value references.
    ///
    /// The iterator order is arbitrary, but consistent across platforms for the
    /// same crate version. Each key will be yielded once per associated value,
    /// so if a key has 3 associated values, it will be yielded 3 times.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::default();
    ///
    /// map.insert(HOST, "hello".to_string());
    /// map.append(HOST, "goodbye".to_string());
    /// map.insert(CONTENT_LENGTH, "123".to_string());
    ///
    /// for (key, value) in map.iter_mut() {
    ///     value.push_str("-boop");
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            map: self as *mut _,
            entry: 0,
            cursor: self.entries.first().map(|_| Cursor::Head),
            lt: PhantomData,
        }
    }
    /// An iterator visiting all keys.
    ///
    /// The iteration order is arbitrary, but consistent across platforms for
    /// the same crate version. Each key will be yielded only once even if it
    /// has multiple associated values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// map.append(HOST, "goodbye".parse().unwrap());
    /// map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    ///
    /// for key in map.keys() {
    ///     println!("{:?}", key);
    /// }
    /// ```
    pub fn keys(&self) -> Keys<T> {
        Keys { inner: self.entries.iter() }
    }
    /// An iterator visiting all values.
    ///
    /// The iteration order is arbitrary, but consistent across platforms for
    /// the same crate version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// map.append(HOST, "goodbye".parse().unwrap());
    /// map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    ///
    /// for value in map.values() {
    ///     println!("{:?}", value);
    /// }
    /// ```
    pub fn values(&self) -> Values<T> {
        Values { inner: self.iter() }
    }
    /// An iterator visiting all values mutably.
    ///
    /// The iteration order is arbitrary, but consistent across platforms for
    /// the same crate version.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::default();
    ///
    /// map.insert(HOST, "hello".to_string());
    /// map.append(HOST, "goodbye".to_string());
    /// map.insert(CONTENT_LENGTH, "123".to_string());
    ///
    /// for value in map.values_mut() {
    ///     value.push_str("-boop");
    /// }
    /// ```
    pub fn values_mut(&mut self) -> ValuesMut<T> {
        ValuesMut {
            inner: self.iter_mut(),
        }
    }
    /// Clears the map, returning all entries as an iterator.
    ///
    /// The internal memory is kept for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::{CONTENT_LENGTH, HOST};
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(HOST, "hello".parse().unwrap());
    /// map.append(HOST, "goodbye".parse().unwrap());
    /// map.insert(CONTENT_LENGTH, "123".parse().unwrap());
    ///
    /// let mut drain = map.drain();
    ///
    /// let (key, mut vals) = drain.next().unwrap();
    ///
    /// assert_eq!("host", key);
    /// assert_eq!("hello", vals.next().unwrap());
    /// assert_eq!("goodbye", vals.next().unwrap());
    /// assert!(vals.next().is_none());
    ///
    /// let (key, mut vals) = drain.next().unwrap();
    ///
    /// assert_eq!("content-length", key);
    /// assert_eq!("123", vals.next().unwrap());
    /// assert!(vals.next().is_none());
    /// ```
    pub fn drain(&mut self) -> Drain<T> {
        for i in self.indices.iter_mut() {
            *i = Pos::none();
        }
        Drain {
            idx: 0,
            map: self as *mut _,
            lt: PhantomData,
        }
    }
    fn value_iter(&self, idx: Option<usize>) -> ValueIter<T> {
        use self::Cursor::*;
        if let Some(idx) = idx {
            let back = {
                let entry = &self.entries[idx];
                entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
            };
            ValueIter {
                map: self,
                index: idx,
                front: Some(Head),
                back: Some(back),
            }
        } else {
            ValueIter {
                map: self,
                index: ::std::usize::MAX,
                front: None,
                back: None,
            }
        }
    }
    fn value_iter_mut(&mut self, idx: usize) -> ValueIterMut<T> {
        use self::Cursor::*;
        let back = {
            let entry = &self.entries[idx];
            entry.links.map(|l| Values(l.tail)).unwrap_or(Head)
        };
        ValueIterMut {
            map: self as *mut _,
            index: idx,
            front: Some(Head),
            back: Some(back),
            lt: PhantomData,
        }
    }
    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let mut map: HeaderMap<u32> = HeaderMap::default();
    ///
    /// let headers = &[
    ///     "content-length",
    ///     "x-hello",
    ///     "Content-Length",
    ///     "x-world",
    /// ];
    ///
    /// for &header in headers {
    ///     let counter = map.entry(header).unwrap().or_insert(0);
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(map["content-length"], 2);
    /// assert_eq!(map["x-hello"], 1);
    /// ```
    pub fn entry<K>(&mut self, key: K) -> Result<Entry<T>, InvalidHeaderName>
    where
        K: AsHeaderName,
    {
        key.entry(self)
    }
    fn entry2<K>(&mut self, key: K) -> Entry<T>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        self.reserve_one();
        insert_phase_one!(
            self, key, probe, pos, hash, danger, Entry::Vacant(VacantEntry { map : self,
            hash : hash, key : key.into(), probe : probe, danger : danger, }),
            Entry::Occupied(OccupiedEntry { map : self, index : pos, probe : probe, }),
            Entry::Vacant(VacantEntry { map : self, hash : hash, key : key.into(), probe
            : probe, danger : danger, })
        )
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not previously have this key present, then `None` is
    /// returned.
    ///
    /// If the map did have this key present, the new value is associated with
    /// the key and all previous values are removed. **Note** that only a single
    /// one of the previous values is returned. If there are multiple values
    /// that have been previously associated with the key, then the first one is
    /// returned. See `insert_mult` on `OccupiedEntry` for an API that returns
    /// all values.
    ///
    /// The key is not updated, though; this matters for types that can be `==`
    /// without being identical.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
    /// assert!(!map.is_empty());
    ///
    /// let mut prev = map.insert(HOST, "earth".parse().unwrap()).unwrap();
    /// assert_eq!("world", prev);
    /// ```
    pub fn insert<K>(&mut self, key: K, val: T) -> Option<T>
    where
        K: IntoHeaderName,
    {
        key.insert(self, val)
    }
    #[inline]
    fn insert2<K>(&mut self, key: K, value: T) -> Option<T>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        self.reserve_one();
        insert_phase_one!(
            self, key, probe, pos, hash, danger, { drop(danger); let index = self.entries
            .len(); self.insert_entry(hash, key.into(), value); self.indices[probe] =
            Pos::new(index, hash); None }, Some(self.insert_occupied(pos, value)), { self
            .insert_phase_two(key.into(), value, hash, probe, danger); None }
        )
    }
    /// Set an occupied bucket to the given value
    #[inline]
    fn insert_occupied(&mut self, index: usize, value: T) -> T {
        if let Some(links) = self.entries[index].links {
            self.remove_all_extra_values(links.next);
        }
        let entry = &mut self.entries[index];
        mem::replace(&mut entry.value, value)
    }
    fn insert_occupied_mult(&mut self, index: usize, value: T) -> ValueDrain<T> {
        let old;
        let links;
        {
            let entry = &mut self.entries[index];
            old = mem::replace(&mut entry.value, value);
            links = entry.links.take();
        }
        ValueDrain {
            map: self as *mut _,
            first: Some(old),
            next: links.map(|l| l.next),
            lt: PhantomData,
        }
    }
    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not previously have this key present, then `false` is
    /// returned.
    ///
    /// If the map did have this key present, the new value is pushed to the end
    /// of the list of values currently associated with the key. The key is not
    /// updated, though; this matters for types that can be `==` without being
    /// identical.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// assert!(map.insert(HOST, "world".parse().unwrap()).is_none());
    /// assert!(!map.is_empty());
    ///
    /// map.append(HOST, "earth".parse().unwrap());
    ///
    /// let values = map.get_all("host");
    /// let mut i = values.iter();
    /// assert_eq!("world", *i.next().unwrap());
    /// assert_eq!("earth", *i.next().unwrap());
    /// ```
    pub fn append<K>(&mut self, key: K, value: T) -> bool
    where
        K: IntoHeaderName,
    {
        key.append(self, value)
    }
    #[inline]
    fn append2<K>(&mut self, key: K, value: T) -> bool
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        self.reserve_one();
        insert_phase_one!(
            self, key, probe, pos, hash, danger, { drop(danger); let index = self.entries
            .len(); self.insert_entry(hash, key.into(), value); self.indices[probe] =
            Pos::new(index, hash); false }, { append_value(pos, & mut self.entries[pos],
            & mut self.extra_values, value); true }, { self.insert_phase_two(key.into(),
            value, hash, probe, danger); false }
        )
    }
    #[inline]
    fn find<K: ?Sized>(&self, key: &K) -> Option<(usize, usize)>
    where
        K: Hash + Into<HeaderName>,
        HeaderName: PartialEq<K>,
    {
        if self.entries.is_empty() {
            return None;
        }
        let hash = hash_elem_using(&self.danger, key);
        let mask = self.mask;
        let mut probe = desired_pos(mask, hash);
        let mut dist = 0;
        probe_loop!(
            probe < self.indices.len(), { if let Some((i, entry_hash)) = self
            .indices[probe].resolve() { if dist > probe_distance(mask, entry_hash, probe)
            { return None; } else if entry_hash == hash && self.entries[i].key == * key {
            return Some((probe, i)); } } else { return None; } dist += 1; }
        );
    }
    /// phase 2 is post-insert where we forward-shift `Pos` in the indices.
    #[inline]
    fn insert_phase_two(
        &mut self,
        key: HeaderName,
        value: T,
        hash: HashValue,
        probe: usize,
        danger: bool,
    ) -> usize {
        let index = self.entries.len();
        self.insert_entry(hash, key, value);
        let num_displaced = do_insert_phase_two(
            &mut self.indices,
            probe,
            Pos::new(index, hash),
        );
        if danger || num_displaced >= DISPLACEMENT_THRESHOLD {
            self.danger.to_yellow();
        }
        index
    }
    /// Removes a key from the map, returning the value associated with the key.
    ///
    /// Returns `None` if the map does not contain the key. If there are
    /// multiple values associated with the key, then the first one is returned.
    /// See `remove_entry_mult` on `OccupiedEntry` for an API that yields all
    /// values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// let prev = map.remove(HOST).unwrap();
    /// assert_eq!("hello.world", prev);
    ///
    /// assert!(map.remove(HOST).is_none());
    /// ```
    pub fn remove<K>(&mut self, key: K) -> Option<T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((probe, idx)) => {
                if let Some(links) = self.entries[idx].links {
                    self.remove_all_extra_values(links.next);
                }
                let entry = self.remove_found(probe, idx);
                Some(entry.value)
            }
            None => None,
        }
    }
    /// Remove an entry from the map.
    #[inline]
    fn remove_found(&mut self, probe: usize, found: usize) -> Bucket<T> {
        self.indices[probe] = Pos::none();
        let entry = self.entries.swap_remove(found);
        if let Some(entry) = self.entries.get(found) {
            let mut probe = desired_pos(self.mask, entry.hash);
            probe_loop!(
                probe < self.indices.len(), { if let Some((i, _)) = self.indices[probe]
                .resolve() { if i >= self.entries.len() { self.indices[probe] =
                Pos::new(found, entry.hash); break; } } }
            );
            if let Some(links) = entry.links {
                self.extra_values[links.next].prev = Link::Entry(found);
                self.extra_values[links.tail].next = Link::Entry(found);
            }
        }
        if self.entries.len() > 0 {
            let mut last_probe = probe;
            let mut probe = probe + 1;
            probe_loop!(
                probe < self.indices.len(), { if let Some((_, entry_hash)) = self
                .indices[probe].resolve() { if probe_distance(self.mask, entry_hash,
                probe) > 0 { self.indices[last_probe] = self.indices[probe]; self
                .indices[probe] = Pos::none(); } else { break; } } else { break; }
                last_probe = probe; }
            );
        }
        entry
    }
    /// Removes the `ExtraValue` at the given index.
    #[inline]
    fn remove_extra_value(&mut self, idx: usize) -> ExtraValue<T> {
        let prev;
        let next;
        {
            debug_assert!(self.extra_values.len() > idx);
            let extra = &self.extra_values[idx];
            prev = extra.prev;
            next = extra.next;
        }
        match (prev, next) {
            (Link::Entry(prev), Link::Entry(next)) => {
                debug_assert_eq!(prev, next);
                debug_assert!(self.entries.len() > prev);
                self.entries[prev].links = None;
            }
            (Link::Entry(prev), Link::Extra(next)) => {
                debug_assert!(self.entries.len() > prev);
                debug_assert!(self.entries[prev].links.is_some());
                self.entries[prev].links.as_mut().unwrap().next = next;
                debug_assert!(self.extra_values.len() > next);
                self.extra_values[next].prev = Link::Entry(prev);
            }
            (Link::Extra(prev), Link::Entry(next)) => {
                debug_assert!(self.entries.len() > next);
                debug_assert!(self.entries[next].links.is_some());
                self.entries[next].links.as_mut().unwrap().tail = prev;
                debug_assert!(self.extra_values.len() > prev);
                self.extra_values[prev].next = Link::Entry(next);
            }
            (Link::Extra(prev), Link::Extra(next)) => {
                debug_assert!(self.extra_values.len() > next);
                debug_assert!(self.extra_values.len() > prev);
                self.extra_values[prev].next = Link::Extra(next);
                self.extra_values[next].prev = Link::Extra(prev);
            }
        }
        let mut extra = self.extra_values.swap_remove(idx);
        let old_idx = self.extra_values.len();
        if extra.prev == Link::Extra(old_idx) {
            extra.prev = Link::Extra(idx);
        }
        if extra.next == Link::Extra(old_idx) {
            extra.next = Link::Extra(idx);
        }
        if idx != old_idx {
            let next;
            let prev;
            {
                debug_assert!(self.extra_values.len() > idx);
                let moved = &self.extra_values[idx];
                next = moved.next;
                prev = moved.prev;
            }
            match prev {
                Link::Entry(entry_idx) => {
                    debug_assert!(self.entries.len() > entry_idx);
                    debug_assert!(self.entries[entry_idx].links.is_some());
                    let links = self.entries[entry_idx].links.as_mut().unwrap();
                    links.next = idx;
                }
                Link::Extra(extra_idx) => {
                    debug_assert!(self.extra_values.len() > extra_idx);
                    self.extra_values[extra_idx].next = Link::Extra(idx);
                }
            }
            match next {
                Link::Entry(entry_idx) => {
                    debug_assert!(self.entries.len() > entry_idx);
                    debug_assert!(self.entries[entry_idx].links.is_some());
                    let links = self.entries[entry_idx].links.as_mut().unwrap();
                    links.tail = idx;
                }
                Link::Extra(extra_idx) => {
                    debug_assert!(self.extra_values.len() > extra_idx);
                    self.extra_values[extra_idx].prev = Link::Extra(idx);
                }
            }
        }
        debug_assert!(
            { for v in & self.extra_values { assert!(v.next != Link::Extra(old_idx));
            assert!(v.prev != Link::Extra(old_idx)); } true }
        );
        extra
    }
    fn remove_all_extra_values(&mut self, mut head: usize) {
        loop {
            let extra = self.remove_extra_value(head);
            if let Link::Extra(idx) = extra.next {
                head = idx;
            } else {
                break;
            }
        }
    }
    #[inline]
    fn insert_entry(&mut self, hash: HashValue, key: HeaderName, value: T) {
        assert!(self.entries.len() < MAX_SIZE, "header map at capacity");
        self.entries
            .push(Bucket {
                hash: hash,
                key: key,
                value: value,
                links: None,
            });
    }
    fn rebuild(&mut self) {
        'outer: for (index, entry) in self.entries.iter_mut().enumerate() {
            let hash = hash_elem_using(&self.danger, &entry.key);
            let mut probe = desired_pos(self.mask, hash);
            let mut dist = 0;
            entry.hash = hash;
            probe_loop!(
                probe < self.indices.len(), { if let Some((_, entry_hash)) = self
                .indices[probe].resolve() { let their_dist = probe_distance(self.mask,
                entry_hash, probe); if their_dist < dist { break; } } else { self
                .indices[probe] = Pos::new(index, hash); continue 'outer; } dist += 1; }
            );
            do_insert_phase_two(&mut self.indices, probe, Pos::new(index, hash));
        }
    }
    fn reinsert_entry_in_order(&mut self, pos: Pos) {
        if let Some((_, entry_hash)) = pos.resolve() {
            let mut probe = desired_pos(self.mask, entry_hash);
            probe_loop!(
                probe < self.indices.len(), { if self.indices[probe].resolve().is_none()
                { self.indices[probe] = pos; return; } }
            );
        }
    }
    fn reserve_one(&mut self) {
        let len = self.entries.len();
        if self.danger.is_yellow() {
            let load_factor = self.entries.len() as f32 / self.indices.len() as f32;
            if load_factor >= LOAD_FACTOR_THRESHOLD {
                self.danger.to_green();
                let new_cap = self.indices.len() * 2;
                self.grow(new_cap);
            } else {
                self.danger.to_red();
                for index in self.indices.iter_mut() {
                    *index = Pos::none();
                }
                self.rebuild();
            }
        } else if len == self.capacity() {
            if len == 0 {
                let new_raw_cap = 8;
                self.mask = 8 - 1;
                self.indices = vec![Pos::none(); new_raw_cap].into_boxed_slice();
                self.entries = Vec::with_capacity(usable_capacity(new_raw_cap));
            } else {
                let raw_cap = self.indices.len();
                self.grow(raw_cap << 1);
            }
        }
    }
    #[inline]
    fn grow(&mut self, new_raw_cap: usize) {
        let mut first_ideal = 0;
        for (i, pos) in self.indices.iter().enumerate() {
            if let Some((_, entry_hash)) = pos.resolve() {
                if 0 == probe_distance(self.mask, entry_hash, i) {
                    first_ideal = i;
                    break;
                }
            }
        }
        let old_indices = mem::replace(
            &mut self.indices,
            vec![Pos::none(); new_raw_cap].into_boxed_slice(),
        );
        self.mask = new_raw_cap.wrapping_sub(1) as Size;
        for &pos in &old_indices[first_ideal..] {
            self.reinsert_entry_in_order(pos);
        }
        for &pos in &old_indices[..first_ideal] {
            self.reinsert_entry_in_order(pos);
        }
        let more = self.capacity() - self.entries.len();
        self.entries.reserve_exact(more);
    }
}
impl<'a, T> IntoIterator for &'a HeaderMap<T> {
    type Item = (&'a HeaderName, &'a T);
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut HeaderMap<T> {
    type Item = (&'a HeaderName, &'a mut T);
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
impl<T> IntoIterator for HeaderMap<T> {
    type Item = (Option<HeaderName>, T);
    type IntoIter = IntoIter<T>;
    /// Creates a consuming iterator, that is, one that moves keys and values
    /// out of the map in arbitrary order. The map cannot be used after calling
    /// this.
    ///
    /// For each yielded item that has `None` provided for the `HeaderName`,
    /// then the associated header name is the same as that of the previously
    /// yielded item. The first yielded item will have `HeaderName` set.
    ///
    /// # Examples
    ///
    /// Basic usage.
    ///
    /// ```
    /// # use http::header;
    /// # use http::header::*;
    /// let mut map = HeaderMap::new();
    /// map.insert(header::CONTENT_LENGTH, "123".parse().unwrap());
    /// map.insert(header::CONTENT_TYPE, "json".parse().unwrap());
    ///
    /// let mut iter = map.into_iter();
    /// assert_eq!(iter.next(), Some((Some(header::CONTENT_LENGTH), "123".parse().unwrap())));
    /// assert_eq!(iter.next(), Some((Some(header::CONTENT_TYPE), "json".parse().unwrap())));
    /// assert!(iter.next().is_none());
    /// ```
    ///
    /// Multiple values per key.
    ///
    /// ```
    /// # use http::header;
    /// # use http::header::*;
    /// let mut map = HeaderMap::new();
    ///
    /// map.append(header::CONTENT_LENGTH, "123".parse().unwrap());
    /// map.append(header::CONTENT_LENGTH, "456".parse().unwrap());
    ///
    /// map.append(header::CONTENT_TYPE, "json".parse().unwrap());
    /// map.append(header::CONTENT_TYPE, "html".parse().unwrap());
    /// map.append(header::CONTENT_TYPE, "xml".parse().unwrap());
    ///
    /// let mut iter = map.into_iter();
    ///
    /// assert_eq!(iter.next(), Some((Some(header::CONTENT_LENGTH), "123".parse().unwrap())));
    /// assert_eq!(iter.next(), Some((None, "456".parse().unwrap())));
    ///
    /// assert_eq!(iter.next(), Some((Some(header::CONTENT_TYPE), "json".parse().unwrap())));
    /// assert_eq!(iter.next(), Some((None, "html".parse().unwrap())));
    /// assert_eq!(iter.next(), Some((None, "xml".parse().unwrap())));
    /// assert!(iter.next().is_none());
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: None,
            entries: self.entries.into_iter(),
            extra_values: self.extra_values,
        }
    }
}
impl<T> FromIterator<(HeaderName, T)> for HeaderMap<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (HeaderName, T)>,
    {
        let mut map = HeaderMap::default();
        map.extend(iter);
        map
    }
}
/// Try to convert a `HashMap` into a `HeaderMap`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use http::{HttpTryFrom, header::HeaderMap};
///
/// let mut map = HashMap::new();
/// map.insert("X-Custom-Header".to_string(), "my value".to_string());
///
/// let headers: HeaderMap = HttpTryFrom::try_from(&map).expect("valid headers");
/// assert_eq!(headers["X-Custom-Header"], "my value");
/// ```
impl<'a, K, V, T> HttpTryFrom<&'a HashMap<K, V>> for HeaderMap<T>
where
    K: Eq + Hash,
    HeaderName: HttpTryFrom<&'a K>,
    T: HttpTryFrom<&'a V>,
{
    type Error = Error;
    fn try_from(c: &'a HashMap<K, V>) -> Result<Self, Self::Error> {
        c.into_iter()
            .map(|(k, v)| {
                let name = k.http_try_into()?;
                let value = v.http_try_into()?;
                Ok((name, value))
            })
            .collect()
    }
}
impl<T> Extend<(Option<HeaderName>, T)> for HeaderMap<T> {
    /// Extend a `HeaderMap` with the contents of another `HeaderMap`.
    ///
    /// This function expects the yielded items to follow the same structure as
    /// `IntoIter`.
    ///
    /// # Panics
    ///
    /// This panics if the first yielded item does not have a `HeaderName`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::*;
    /// let mut map = HeaderMap::new();
    ///
    /// map.insert(ACCEPT, "text/plain".parse().unwrap());
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// let mut extra = HeaderMap::new();
    ///
    /// extra.insert(HOST, "foo.bar".parse().unwrap());
    /// extra.insert(COOKIE, "hello".parse().unwrap());
    /// extra.append(COOKIE, "world".parse().unwrap());
    ///
    /// map.extend(extra);
    ///
    /// assert_eq!(map["host"], "foo.bar");
    /// assert_eq!(map["accept"], "text/plain");
    /// assert_eq!(map["cookie"], "hello");
    ///
    /// let v = map.get_all("host");
    /// assert_eq!(1, v.iter().count());
    ///
    /// let v = map.get_all("cookie");
    /// assert_eq!(2, v.iter().count());
    /// ```
    fn extend<I: IntoIterator<Item = (Option<HeaderName>, T)>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        let (mut key, mut val) = match iter.next() {
            Some((Some(key), val)) => (key, val),
            Some((None, _)) => panic!("expected a header name, but got None"),
            None => return,
        };
        'outer: loop {
            let mut entry = match self.entry2(key) {
                Entry::Occupied(mut e) => {
                    e.insert(val);
                    e
                }
                Entry::Vacant(e) => e.insert_entry(val),
            };
            'inner: loop {
                match iter.next() {
                    Some((Some(k), v)) => {
                        key = k;
                        val = v;
                        continue 'outer;
                    }
                    Some((None, v)) => {
                        entry.append(v);
                    }
                    None => {
                        return;
                    }
                }
            }
        }
    }
}
impl<T> Extend<(HeaderName, T)> for HeaderMap<T> {
    fn extend<I: IntoIterator<Item = (HeaderName, T)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let reserve = if self.is_empty() {
            iter.size_hint().0
        } else {
            (iter.size_hint().0 + 1) / 2
        };
        self.reserve(reserve);
        for (k, v) in iter {
            self.append(k, v);
        }
    }
}
impl<T: PartialEq> PartialEq for HeaderMap<T> {
    fn eq(&self, other: &HeaderMap<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.keys().all(|key| { self.get_all(key) == other.get_all(key) })
    }
}
impl<T: Eq> Eq for HeaderMap<T> {}
impl<T: fmt::Debug> fmt::Debug for HeaderMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
impl<T> Default for HeaderMap<T> {
    fn default() -> Self {
        HeaderMap::with_capacity(0)
    }
}
impl<'a, K, T> ops::Index<K> for HeaderMap<T>
where
    K: AsHeaderName,
{
    type Output = T;
    /// # Panics
    /// Using the index operator will cause a panic if the header you're querying isn't set.
    #[inline]
    fn index(&self, index: K) -> &T {
        match self.get2(&index) {
            Some(val) => val,
            None => panic!("no entry found for key {:?}", index.as_str()),
        }
    }
}
/// phase 2 is post-insert where we forward-shift `Pos` in the indices.
///
/// returns the number of displaced elements
#[inline]
fn do_insert_phase_two(
    indices: &mut [Pos],
    mut probe: usize,
    mut old_pos: Pos,
) -> usize {
    let mut num_displaced = 0;
    probe_loop!(
        probe < indices.len(), { let pos = & mut indices[probe]; if pos.is_none() { * pos
        = old_pos; break; } else { num_displaced += 1; old_pos = mem::replace(pos,
        old_pos); } }
    );
    num_displaced
}
#[inline]
fn append_value<T>(
    entry_idx: usize,
    entry: &mut Bucket<T>,
    extra: &mut Vec<ExtraValue<T>>,
    value: T,
) {
    match entry.links {
        Some(links) => {
            let idx = extra.len();
            extra
                .push(ExtraValue {
                    value: value,
                    prev: Link::Extra(links.tail),
                    next: Link::Entry(entry_idx),
                });
            extra[links.tail].next = Link::Extra(idx);
            entry.links = Some(Links { tail: idx, ..links });
        }
        None => {
            let idx = extra.len();
            extra
                .push(ExtraValue {
                    value: value,
                    prev: Link::Entry(entry_idx),
                    next: Link::Entry(entry_idx),
                });
            entry.links = Some(Links { next: idx, tail: idx });
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (&'a HeaderName, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_unsafe().map(|(key, ptr)| { (key, unsafe { &*ptr }) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Send for Iter<'a, T> {}
impl<'a, T> IterMut<'a, T> {
    fn next_unsafe(&mut self) -> Option<(&'a HeaderName, *mut T)> {
        use self::Cursor::*;
        if self.cursor.is_none() {
            if (self.entry + 1) >= unsafe { &*self.map }.entries.len() {
                return None;
            }
            self.entry += 1;
            self.cursor = Some(Cursor::Head);
        }
        let entry = unsafe { &(*self.map).entries[self.entry] };
        match self.cursor.unwrap() {
            Head => {
                self.cursor = entry.links.map(|l| Values(l.next));
                Some((&entry.key, &entry.value as *const _ as *mut _))
            }
            Values(idx) => {
                let extra = unsafe { &(*self.map).extra_values[idx] };
                match extra.next {
                    Link::Entry(_) => self.cursor = None,
                    Link::Extra(i) => self.cursor = Some(Values(i)),
                }
                Some((&entry.key, &extra.value as *const _ as *mut _))
            }
        }
    }
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (&'a HeaderName, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.next_unsafe().map(|(key, ptr)| { (key, unsafe { &mut *ptr }) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let map = unsafe { &*self.map };
        debug_assert!(map.entries.len() >= self.entry);
        let lower = map.entries.len() - self.entry;
        (lower, None)
    }
}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}
unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
impl<'a, T> Iterator for Keys<'a, T> {
    type Item = &'a HeaderName;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|b| &b.key)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, T> ExactSizeIterator for Keys<'a, T> {}
impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl<'a, T> Iterator for Drain<'a, T> {
    type Item = (HeaderName, ValueDrain<'a, T>);
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        if idx == unsafe { (*self.map).entries.len() } {
            return None;
        }
        self.idx += 1;
        let key;
        let value;
        let next;
        unsafe {
            let entry = &(*self.map).entries[idx];
            key = ptr::read(&entry.key as *const _);
            value = ptr::read(&entry.value as *const _);
            next = entry.links.map(|l| l.next);
        };
        let values = ValueDrain {
            map: self.map,
            first: Some(value),
            next: next,
            lt: PhantomData,
        };
        Some((key, values))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let lower = unsafe { (*self.map).entries.len() } - self.idx;
        (lower, Some(lower))
    }
}
impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        unsafe {
            let map = &mut *self.map;
            debug_assert!(map.extra_values.is_empty());
            map.entries.set_len(0);
        }
    }
}
unsafe impl<'a, T: Sync> Sync for Drain<'a, T> {}
unsafe impl<'a, T: Send> Send for Drain<'a, T> {}
impl<'a, T> Entry<'a, T> {
    /// Ensures a value is in the entry by inserting the default if empty.
    ///
    /// Returns a mutable reference to the **first** value in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let mut map: HeaderMap<u32> = HeaderMap::default();
    ///
    /// let headers = &[
    ///     "content-length",
    ///     "x-hello",
    ///     "Content-Length",
    ///     "x-world",
    /// ];
    ///
    /// for &header in headers {
    ///     let counter = map.entry(header)
    ///         .expect("valid header names")
    ///         .or_insert(0);
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(map["content-length"], 2);
    /// assert_eq!(map["x-hello"], 1);
    /// ```
    pub fn or_insert(self, default: T) -> &'a mut T {
        use self::Entry::*;
        match self {
            Occupied(e) => e.into_mut(),
            Vacant(e) => e.insert(default),
        }
    }
    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty.
    ///
    /// The default function is not called if the entry exists in the map.
    /// Returns a mutable reference to the **first** value in the entry.
    ///
    /// # Examples
    ///
    /// Basic usage.
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let mut map = HeaderMap::new();
    ///
    /// let res = map.entry("x-hello").unwrap()
    ///     .or_insert_with(|| "world".parse().unwrap());
    ///
    /// assert_eq!(res, "world");
    /// ```
    ///
    /// The default function is not called if the entry exists in the map.
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    ///
    /// let res = map.entry("host")
    ///     .expect("host is a valid string")
    ///     .or_insert_with(|| unreachable!());
    ///
    ///
    /// assert_eq!(res, "world");
    /// ```
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        use self::Entry::*;
        match self {
            Occupied(e) => e.into_mut(),
            Vacant(e) => e.insert(default()),
        }
    }
    /// Returns a reference to the entry's key
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let mut map = HeaderMap::new();
    ///
    /// assert_eq!(map.entry("x-hello").unwrap().key(), "x-hello");
    /// ```
    pub fn key(&self) -> &HeaderName {
        use self::Entry::*;
        match *self {
            Vacant(ref e) => e.key(),
            Occupied(ref e) => e.key(),
        }
    }
}
impl<'a, T> VacantEntry<'a, T> {
    /// Returns a reference to the entry's key
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// let mut map = HeaderMap::new();
    ///
    /// assert_eq!(map.entry("x-hello").unwrap().key().as_str(), "x-hello");
    /// ```
    pub fn key(&self) -> &HeaderName {
        &self.key
    }
    /// Take ownership of the key
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry};
    /// let mut map = HeaderMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("x-hello").unwrap() {
    ///     assert_eq!(v.into_key().as_str(), "x-hello");
    /// }
    /// ```
    pub fn into_key(self) -> HeaderName {
        self.key
    }
    /// Insert the value into the entry.
    ///
    /// The value will be associated with this entry's key. A mutable reference
    /// to the inserted value will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry};
    /// let mut map = HeaderMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("x-hello").unwrap() {
    ///     v.insert("world".parse().unwrap());
    /// }
    ///
    /// assert_eq!(map["x-hello"], "world");
    /// ```
    pub fn insert(self, value: T) -> &'a mut T {
        let index = self
            .map
            .insert_phase_two(
                self.key,
                value.into(),
                self.hash,
                self.probe,
                self.danger,
            );
        &mut self.map.entries[index].value
    }
    /// Insert the value into the entry.
    ///
    /// The value will be associated with this entry's key. The new
    /// `OccupiedEntry` is returned, allowing for further manipulation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::*;
    /// let mut map = HeaderMap::new();
    ///
    /// if let Entry::Vacant(v) = map.entry("x-hello").unwrap() {
    ///     let mut e = v.insert_entry("world".parse().unwrap());
    ///     e.insert("world2".parse().unwrap());
    /// }
    ///
    /// assert_eq!(map["x-hello"], "world2");
    /// ```
    pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        let index = self
            .map
            .insert_phase_two(
                self.key,
                value.into(),
                self.hash,
                self.probe,
                self.danger,
            );
        OccupiedEntry {
            map: self.map,
            index: index,
            probe: self.probe,
        }
    }
}
impl<'a, T: 'a> GetAll<'a, T> {
    /// Returns an iterator visiting all values associated with the entry.
    ///
    /// Values are iterated in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::HeaderMap;
    /// # use http::header::HOST;
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "hello.world".parse().unwrap());
    /// map.append(HOST, "hello.earth".parse().unwrap());
    ///
    /// let values = map.get_all("host");
    /// let mut iter = values.iter();
    /// assert_eq!(&"hello.world", iter.next().unwrap());
    /// assert_eq!(&"hello.earth", iter.next().unwrap());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> ValueIter<'a, T> {
        GetAll {
            map: self.map,
            index: self.index,
        }
            .into_iter()
    }
}
impl<'a, T: PartialEq> PartialEq for GetAll<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
impl<'a, T> IntoIterator for GetAll<'a, T> {
    type Item = &'a T;
    type IntoIter = ValueIter<'a, T>;
    fn into_iter(self) -> ValueIter<'a, T> {
        self.map.value_iter(self.index)
    }
}
impl<'a, 'b: 'a, T> IntoIterator for &'b GetAll<'a, T> {
    type Item = &'a T;
    type IntoIter = ValueIter<'a, T>;
    fn into_iter(self) -> ValueIter<'a, T> {
        self.map.value_iter(self.index)
    }
}
impl<'a, T: 'a> Iterator for ValueIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;
        match self.front {
            Some(Head) => {
                let entry = &self.map.entries[self.index];
                if self.back == Some(Head) {
                    self.front = None;
                    self.back = None;
                } else {
                    match entry.links {
                        Some(links) => {
                            self.front = Some(Values(links.next));
                        }
                        None => unreachable!(),
                    }
                }
                Some(&entry.value)
            }
            Some(Values(idx)) => {
                let extra = &self.map.extra_values[idx];
                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.next {
                        Link::Entry(_) => self.front = None,
                        Link::Extra(i) => self.front = Some(Values(i)),
                    }
                }
                Some(&extra.value)
            }
            None => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match (self.front, self.back) {
            (Some(Cursor::Head), Some(Cursor::Head)) => (1, Some(1)),
            (Some(_), _) => (1, None),
            (None, _) => (0, Some(0)),
        }
    }
}
impl<'a, T: 'a> DoubleEndedIterator for ValueIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;
        match self.back {
            Some(Head) => {
                self.front = None;
                self.back = None;
                Some(&self.map.entries[self.index].value)
            }
            Some(Values(idx)) => {
                let extra = &self.map.extra_values[idx];
                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.prev {
                        Link::Entry(_) => self.back = Some(Head),
                        Link::Extra(idx) => self.back = Some(Values(idx)),
                    }
                }
                Some(&extra.value)
            }
            None => None,
        }
    }
}
impl<'a, T: 'a> Iterator for ValueIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;
        let entry = unsafe { &mut (*self.map).entries[self.index] };
        match self.front {
            Some(Head) => {
                if self.back == Some(Head) {
                    self.front = None;
                    self.back = None;
                } else {
                    match entry.links {
                        Some(links) => {
                            self.front = Some(Values(links.next));
                        }
                        None => unreachable!(),
                    }
                }
                Some(&mut entry.value)
            }
            Some(Values(idx)) => {
                let extra = unsafe { &mut (*self.map).extra_values[idx] };
                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.next {
                        Link::Entry(_) => self.front = None,
                        Link::Extra(i) => self.front = Some(Values(i)),
                    }
                }
                Some(&mut extra.value)
            }
            None => None,
        }
    }
}
impl<'a, T: 'a> DoubleEndedIterator for ValueIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        use self::Cursor::*;
        let entry = unsafe { &mut (*self.map).entries[self.index] };
        match self.back {
            Some(Head) => {
                self.front = None;
                self.back = None;
                Some(&mut entry.value)
            }
            Some(Values(idx)) => {
                let extra = unsafe { &mut (*self.map).extra_values[idx] };
                if self.front == self.back {
                    self.front = None;
                    self.back = None;
                } else {
                    match extra.prev {
                        Link::Entry(_) => self.back = Some(Head),
                        Link::Extra(idx) => self.back = Some(Values(idx)),
                    }
                }
                Some(&mut extra.value)
            }
            None => None,
        }
    }
}
unsafe impl<'a, T: Sync> Sync for ValueIterMut<'a, T> {}
unsafe impl<'a, T: Send> Send for ValueIterMut<'a, T> {}
impl<T> Iterator for IntoIter<T> {
    type Item = (Option<HeaderName>, T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next {
            self
                .next = match self.extra_values[next].next {
                Link::Entry(_) => None,
                Link::Extra(v) => Some(v),
            };
            let value = unsafe { ptr::read(&self.extra_values[next].value) };
            return Some((None, value));
        }
        if let Some(bucket) = self.entries.next() {
            self.next = bucket.links.map(|l| l.next);
            let name = Some(bucket.key);
            let value = bucket.value;
            return Some((name, value));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, _) = self.entries.size_hint();
        (lower, None)
    }
}
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in self.by_ref() {}
        unsafe {
            self.extra_values.set_len(0);
        }
    }
}
impl<'a, T> OccupiedEntry<'a, T> {
    /// Returns a reference to the entry's key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    ///
    /// if let Entry::Occupied(e) = map.entry("host").unwrap() {
    ///     assert_eq!("host", e.key());
    /// }
    /// ```
    pub fn key(&self) -> &HeaderName {
        &self.map.entries[self.index].key
    }
    /// Get a reference to the first value in the entry.
    ///
    /// Values are stored in insertion order.
    ///
    /// # Panics
    ///
    /// `get` panics if there are no values associated with the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     assert_eq!(e.get(), &"hello.world");
    ///
    ///     e.append("hello.earth".parse().unwrap());
    ///
    ///     assert_eq!(e.get(), &"hello.world");
    /// }
    /// ```
    pub fn get(&self) -> &T {
        &self.map.entries[self.index].value
    }
    /// Get a mutable reference to the first value in the entry.
    ///
    /// Values are stored in insertion order.
    ///
    /// # Panics
    ///
    /// `get_mut` panics if there are no values associated with the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::default();
    /// map.insert(HOST, "hello.world".to_string());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     e.get_mut().push_str("-2");
    ///     assert_eq!(e.get(), &"hello.world-2");
    /// }
    /// ```
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.map.entries[self.index].value
    }
    /// Converts the `OccupiedEntry` into a mutable reference to the **first**
    /// value.
    ///
    /// The lifetime of the returned reference is bound to the original map.
    ///
    /// # Panics
    ///
    /// `into_mut` panics if there are no values associated with the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::default();
    /// map.insert(HOST, "hello.world".to_string());
    /// map.append(HOST, "hello.earth".to_string());
    ///
    /// if let Entry::Occupied(e) = map.entry("host").unwrap() {
    ///     e.into_mut().push_str("-2");
    /// }
    ///
    /// assert_eq!("hello.world-2", map["host"]);
    /// ```
    pub fn into_mut(self) -> &'a mut T {
        &mut self.map.entries[self.index].value
    }
    /// Sets the value of the entry.
    ///
    /// All previous values associated with the entry are removed and the first
    /// one is returned. See `insert_mult` for an API that returns all values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "hello.world".parse().unwrap());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     let mut prev = e.insert("earth".parse().unwrap());
    ///     assert_eq!("hello.world", prev);
    /// }
    ///
    /// assert_eq!("earth", map["host"]);
    /// ```
    pub fn insert(&mut self, value: T) -> T {
        self.map.insert_occupied(self.index, value.into())
    }
    /// Sets the value of the entry.
    ///
    /// This function does the same as `insert` except it returns an iterator
    /// that yields all values previously associated with the key.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    /// map.append(HOST, "world2".parse().unwrap());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     let mut prev = e.insert_mult("earth".parse().unwrap());
    ///     assert_eq!("world", prev.next().unwrap());
    ///     assert_eq!("world2", prev.next().unwrap());
    ///     assert!(prev.next().is_none());
    /// }
    ///
    /// assert_eq!("earth", map["host"]);
    /// ```
    pub fn insert_mult(&mut self, value: T) -> ValueDrain<T> {
        self.map.insert_occupied_mult(self.index, value.into())
    }
    /// Insert the value into the entry.
    ///
    /// The new value is appended to the end of the entry's value list. All
    /// previous values associated with the entry are retained.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     e.append("earth".parse().unwrap());
    /// }
    ///
    /// let values = map.get_all("host");
    /// let mut i = values.iter();
    /// assert_eq!("world", *i.next().unwrap());
    /// assert_eq!("earth", *i.next().unwrap());
    /// ```
    pub fn append(&mut self, value: T) {
        let idx = self.index;
        let entry = &mut self.map.entries[idx];
        append_value(idx, entry, &mut self.map.extra_values, value.into());
    }
    /// Remove the entry from the map.
    ///
    /// All values associated with the entry are removed and the first one is
    /// returned. See `remove_entry_mult` for an API that returns all values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    ///
    /// if let Entry::Occupied(e) = map.entry("host").unwrap() {
    ///     let mut prev = e.remove();
    ///     assert_eq!("world", prev);
    /// }
    ///
    /// assert!(!map.contains_key("host"));
    /// ```
    pub fn remove(self) -> T {
        self.remove_entry().1
    }
    /// Remove the entry from the map.
    ///
    /// The key and all values associated with the entry are removed and the
    /// first one is returned. See `remove_entry_mult` for an API that returns
    /// all values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    ///
    /// if let Entry::Occupied(e) = map.entry("host").unwrap() {
    ///     let (key, mut prev) = e.remove_entry();
    ///     assert_eq!("host", key.as_str());
    ///     assert_eq!("world", prev);
    /// }
    ///
    /// assert!(!map.contains_key("host"));
    /// ```
    pub fn remove_entry(self) -> (HeaderName, T) {
        let entry = self.map.remove_found(self.probe, self.index);
        if let Some(links) = entry.links {
            self.map.remove_all_extra_values(links.next);
        }
        (entry.key, entry.value)
    }
    /// Remove the entry from the map.
    ///
    /// The key and all values associated with the entry are removed and
    /// returned.
    pub fn remove_entry_mult(self) -> (HeaderName, ValueDrain<'a, T>) {
        let entry = self.map.remove_found(self.probe, self.index);
        let drain = ValueDrain {
            map: self.map as *mut _,
            first: Some(entry.value),
            next: entry.links.map(|l| l.next),
            lt: PhantomData,
        };
        (entry.key, drain)
    }
    /// Returns an iterator visiting all values associated with the entry.
    ///
    /// Values are iterated in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::new();
    /// map.insert(HOST, "world".parse().unwrap());
    /// map.append(HOST, "earth".parse().unwrap());
    ///
    /// if let Entry::Occupied(e) = map.entry("host").unwrap() {
    ///     let mut iter = e.iter();
    ///     assert_eq!(&"world", iter.next().unwrap());
    ///     assert_eq!(&"earth", iter.next().unwrap());
    ///     assert!(iter.next().is_none());
    /// }
    /// ```
    pub fn iter(&self) -> ValueIter<T> {
        self.map.value_iter(Some(self.index))
    }
    /// Returns an iterator mutably visiting all values associated with the
    /// entry.
    ///
    /// Values are iterated in insertion order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use http::header::{HeaderMap, Entry, HOST};
    /// let mut map = HeaderMap::default();
    /// map.insert(HOST, "world".to_string());
    /// map.append(HOST, "earth".to_string());
    ///
    /// if let Entry::Occupied(mut e) = map.entry("host").unwrap() {
    ///     for e in e.iter_mut() {
    ///         e.push_str("-boop");
    ///     }
    /// }
    ///
    /// let mut values = map.get_all("host");
    /// let mut i = values.iter();
    /// assert_eq!(&"world-boop", i.next().unwrap());
    /// assert_eq!(&"earth-boop", i.next().unwrap());
    /// ```
    pub fn iter_mut(&mut self) -> ValueIterMut<T> {
        self.map.value_iter_mut(self.index)
    }
}
impl<'a, T> IntoIterator for OccupiedEntry<'a, T> {
    type Item = &'a mut T;
    type IntoIter = ValueIterMut<'a, T>;
    fn into_iter(self) -> ValueIterMut<'a, T> {
        self.map.value_iter_mut(self.index)
    }
}
impl<'a, 'b: 'a, T> IntoIterator for &'b OccupiedEntry<'a, T> {
    type Item = &'a T;
    type IntoIter = ValueIter<'a, T>;
    fn into_iter(self) -> ValueIter<'a, T> {
        self.iter()
    }
}
impl<'a, 'b: 'a, T> IntoIterator for &'b mut OccupiedEntry<'a, T> {
    type Item = &'a mut T;
    type IntoIter = ValueIterMut<'a, T>;
    fn into_iter(self) -> ValueIterMut<'a, T> {
        self.iter_mut()
    }
}
impl<'a, T> Iterator for ValueDrain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.first.is_some() {
            self.first.take()
        } else if let Some(next) = self.next {
            let extra = unsafe { &mut (*self.map) }.remove_extra_value(next);
            match extra.next {
                Link::Extra(idx) => self.next = Some(idx),
                Link::Entry(_) => self.next = None,
            }
            Some(extra.value)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        match (&self.first, self.next) {
            (&Some(_), None) => (1, Some(1)),
            (&_, Some(_)) => (1, None),
            (&None, None) => (0, Some(0)),
        }
    }
}
impl<'a, T> Drop for ValueDrain<'a, T> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {}
    }
}
unsafe impl<'a, T: Sync> Sync for ValueDrain<'a, T> {}
unsafe impl<'a, T: Send> Send for ValueDrain<'a, T> {}
impl Pos {
    #[inline]
    fn new(index: usize, hash: HashValue) -> Self {
        Pos {
            index: index as Size,
            hash: hash,
        }
    }
    #[inline]
    fn none() -> Self {
        Pos {
            index: !0,
            hash: HashValue(0),
        }
    }
    #[inline]
    fn is_some(&self) -> bool {
        !self.is_none()
    }
    #[inline]
    fn is_none(&self) -> bool {
        self.index == !0
    }
    #[inline]
    fn resolve(&self) -> Option<(usize, HashValue)> {
        if self.is_some() { Some((self.index, self.hash)) } else { None }
    }
}
impl Danger {
    fn is_red(&self) -> bool {
        match *self {
            Danger::Red(_) => true,
            _ => false,
        }
    }
    fn to_red(&mut self) {
        debug_assert!(self.is_yellow());
        *self = Danger::Red(RandomState::new());
    }
    fn is_yellow(&self) -> bool {
        match *self {
            Danger::Yellow => true,
            _ => false,
        }
    }
    fn to_yellow(&mut self) {
        match *self {
            Danger::Green => {
                *self = Danger::Yellow;
            }
            _ => {}
        }
    }
    fn to_green(&mut self) {
        debug_assert!(self.is_yellow());
        *self = Danger::Green;
    }
}
#[inline]
fn usable_capacity(cap: usize) -> usize {
    cap - cap / 4
}
#[inline]
fn to_raw_capacity(n: usize) -> usize {
    n + n / 3
}
#[inline]
fn desired_pos(mask: Size, hash: HashValue) -> usize {
    (hash.0 & mask)
}
/// The number of steps that `current` is forward of the desired position for hash
#[inline]
fn probe_distance(mask: Size, hash: HashValue, current: usize) -> usize {
    current.wrapping_sub(desired_pos(mask, hash)) & mask
}
fn hash_elem_using<K: ?Sized>(danger: &Danger, k: &K) -> HashValue
where
    K: Hash,
{
    use fnv::FnvHasher;
    const MASK: u64 = (MAX_SIZE as u64) - 1;
    let hash = match *danger {
        Danger::Red(ref hasher) => {
            let mut h = hasher.build_hasher();
            k.hash(&mut h);
            h.finish()
        }
        _ => {
            let mut h = FnvHasher::default();
            k.hash(&mut h);
            h.finish()
        }
    };
    HashValue((hash & MASK) as usize)
}
mod into_header_name {
    use super::{HdrName, HeaderMap, HeaderName};
    /// A marker trait used to identify values that can be used as insert keys
    /// to a `HeaderMap`.
    pub trait IntoHeaderName: Sealed {}
    pub trait Sealed {
        #[doc(hidden)]
        fn insert<T>(self, map: &mut HeaderMap<T>, val: T) -> Option<T>;
        #[doc(hidden)]
        fn append<T>(self, map: &mut HeaderMap<T>, val: T) -> bool;
    }
    impl Sealed for HeaderName {
        #[doc(hidden)]
        #[inline]
        fn insert<T>(self, map: &mut HeaderMap<T>, val: T) -> Option<T> {
            map.insert2(self, val)
        }
        #[doc(hidden)]
        #[inline]
        fn append<T>(self, map: &mut HeaderMap<T>, val: T) -> bool {
            map.append2(self, val)
        }
    }
    impl IntoHeaderName for HeaderName {}
    impl<'a> Sealed for &'a HeaderName {
        #[doc(hidden)]
        #[inline]
        fn insert<T>(self, map: &mut HeaderMap<T>, val: T) -> Option<T> {
            map.insert2(self, val)
        }
        #[doc(hidden)]
        #[inline]
        fn append<T>(self, map: &mut HeaderMap<T>, val: T) -> bool {
            map.append2(self, val)
        }
    }
    impl<'a> IntoHeaderName for &'a HeaderName {}
    impl Sealed for &'static str {
        #[doc(hidden)]
        #[inline]
        fn insert<T>(self, map: &mut HeaderMap<T>, val: T) -> Option<T> {
            HdrName::from_static(self, move |hdr| map.insert2(hdr, val))
        }
        #[doc(hidden)]
        #[inline]
        fn append<T>(self, map: &mut HeaderMap<T>, val: T) -> bool {
            HdrName::from_static(self, move |hdr| map.append2(hdr, val))
        }
    }
    impl IntoHeaderName for &'static str {}
}
mod as_header_name {
    use super::{Entry, HdrName, HeaderMap, HeaderName, InvalidHeaderName};
    /// A marker trait used to identify values that can be used as search keys
    /// to a `HeaderMap`.
    pub trait AsHeaderName: Sealed {}
    pub trait Sealed {
        #[doc(hidden)]
        fn entry<T>(self, map: &mut HeaderMap<T>) -> Result<Entry<T>, InvalidHeaderName>;
        #[doc(hidden)]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)>;
        #[doc(hidden)]
        fn as_str(&self) -> &str;
    }
    impl Sealed for HeaderName {
        #[doc(hidden)]
        #[inline]
        fn entry<T>(
            self,
            map: &mut HeaderMap<T>,
        ) -> Result<Entry<T>, InvalidHeaderName> {
            Ok(map.entry2(self))
        }
        #[doc(hidden)]
        #[inline]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            map.find(self)
        }
        #[doc(hidden)]
        fn as_str(&self) -> &str {
            <HeaderName>::as_str(self)
        }
    }
    impl AsHeaderName for HeaderName {}
    impl<'a> Sealed for &'a HeaderName {
        #[doc(hidden)]
        #[inline]
        fn entry<T>(
            self,
            map: &mut HeaderMap<T>,
        ) -> Result<Entry<T>, InvalidHeaderName> {
            Ok(map.entry2(self))
        }
        #[doc(hidden)]
        #[inline]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            map.find(*self)
        }
        #[doc(hidden)]
        fn as_str(&self) -> &str {
            <HeaderName>::as_str(*self)
        }
    }
    impl<'a> AsHeaderName for &'a HeaderName {}
    impl<'a> Sealed for &'a str {
        #[doc(hidden)]
        #[inline]
        fn entry<T>(
            self,
            map: &mut HeaderMap<T>,
        ) -> Result<Entry<T>, InvalidHeaderName> {
            HdrName::from_bytes(self.as_bytes(), move |hdr| map.entry2(hdr))
        }
        #[doc(hidden)]
        #[inline]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            HdrName::from_bytes(self.as_bytes(), move |hdr| map.find(&hdr))
                .unwrap_or(None)
        }
        #[doc(hidden)]
        fn as_str(&self) -> &str {
            self
        }
    }
    impl<'a> AsHeaderName for &'a str {}
    impl Sealed for String {
        #[doc(hidden)]
        #[inline]
        fn entry<T>(
            self,
            map: &mut HeaderMap<T>,
        ) -> Result<Entry<T>, InvalidHeaderName> {
            self.as_str().entry(map)
        }
        #[doc(hidden)]
        #[inline]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            Sealed::find(&self.as_str(), map)
        }
        #[doc(hidden)]
        fn as_str(&self) -> &str {
            self
        }
    }
    impl AsHeaderName for String {}
    impl<'a> Sealed for &'a String {
        #[doc(hidden)]
        #[inline]
        fn entry<T>(
            self,
            map: &mut HeaderMap<T>,
        ) -> Result<Entry<T>, InvalidHeaderName> {
            self.as_str().entry(map)
        }
        #[doc(hidden)]
        #[inline]
        fn find<T>(&self, map: &HeaderMap<T>) -> Option<(usize, usize)> {
            Sealed::find(*self, map)
        }
        #[doc(hidden)]
        fn as_str(&self) -> &str {
            *self
        }
    }
    impl<'a> AsHeaderName for &'a String {}
}
#[test]
fn test_bounds() {
    fn check_bounds<T: Send + Send>() {}
    check_bounds::<HeaderMap<()>>();
    check_bounds::<Iter<'static, ()>>();
    check_bounds::<IterMut<'static, ()>>();
    check_bounds::<Keys<'static, ()>>();
    check_bounds::<Values<'static, ()>>();
    check_bounds::<ValuesMut<'static, ()>>();
    check_bounds::<Drain<'static, ()>>();
    check_bounds::<GetAll<'static, ()>>();
    check_bounds::<Entry<'static, ()>>();
    check_bounds::<VacantEntry<'static, ()>>();
    check_bounds::<OccupiedEntry<'static, ()>>();
    check_bounds::<ValueIter<'static, ()>>();
    check_bounds::<ValueIterMut<'static, ()>>();
    check_bounds::<ValueDrain<'static, ()>>();
}
#[test]
fn skip_duplicates_during_key_iteration() {
    let mut map = HeaderMap::new();
    map.append("a", HeaderValue::from_static("a"));
    map.append("a", HeaderValue::from_static("b"));
    assert_eq!(map.keys().count(), map.keys_len());
}
#[cfg(test)]
mod tests_rug_1 {
    use super::*;
    use crate::header::map::Pos;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Vec<Pos> = vec![
            Pos::new(rug_fuzz_0, HashValue(rug_fuzz_1)), Pos::none(), Pos::new(2,
            HashValue(456))
        ];
        let p1: usize = rug_fuzz_2;
        let p2: Pos = Pos::new(rug_fuzz_3, HashValue(rug_fuzz_4));
        crate::header::map::do_insert_phase_two(&mut p0, p1, p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_3 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        debug_assert_eq!(crate ::header::map::usable_capacity(p0), 15);
             }
});    }
}
#[cfg(test)]
mod tests_rug_4 {
    use super::*;
    #[test]
    fn test_to_raw_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        debug_assert_eq!(crate ::header::map::to_raw_capacity(p0), 16);
             }
});    }
}
#[cfg(test)]
mod tests_rug_5 {
    use super::*;
    use crate::header::map::{desired_pos, HashValue};
    #[test]
    fn test_rug() {
        let mut p0: usize = 10;
        fn hash_elem_using(value: usize) -> HashValue {
            HashValue(value)
        }
        let mut p1: HashValue = hash_elem_using(100);
        desired_pos(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_7 {
    use super::*;
    use crate::Version;
    use crate::header::map::{Danger, hash_elem_using};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_7_rrrruuuugggg_test_rug = 0;
        let mut p0 = Danger::Green;
        let mut p1 = Version::HTTP_11;
        hash_elem_using(&p0, &p1);
        let _rug_ed_tests_rug_7_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_8 {
    use super::*;
    use crate::HeaderMap;
    #[test]
    fn test_new() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let map: HeaderMap = <HeaderMap>::new();
        debug_assert!(map.is_empty());
        debug_assert_eq!(rug_fuzz_0, map.capacity());
             }
});    }
}
#[cfg(test)]
mod tests_rug_9 {
    use super::*;
    use crate::HeaderMap;
    #[test]
    fn test_with_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let map: HeaderMap<u32> = HeaderMap::<u32>::with_capacity(p0);
        debug_assert!(map.is_empty());
        debug_assert_eq!(rug_fuzz_1, map.capacity());
             }
});    }
}
#[cfg(test)]
mod tests_rug_10 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{ACCEPT, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, &str, &str, usize, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: HeaderMap<_> = HeaderMap::new();
        debug_assert_eq!(rug_fuzz_0, map.len());
        map.insert(ACCEPT, rug_fuzz_1.parse().unwrap());
        map.insert(HOST, rug_fuzz_2.parse().unwrap());
        debug_assert_eq!(rug_fuzz_3, map.len());
        map.append(ACCEPT, rug_fuzz_4.parse().unwrap());
        debug_assert_eq!(rug_fuzz_5, map.len());
             }
});    }
}
#[cfg(test)]
mod tests_rug_11 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{ACCEPT, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(usize, &str, &str, usize, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap = HeaderMap::new();
        debug_assert_eq!(rug_fuzz_0, p0.keys_len());
        p0.insert(ACCEPT, rug_fuzz_1.parse().unwrap());
        p0.insert(HOST, rug_fuzz_2.parse().unwrap());
        debug_assert_eq!(rug_fuzz_3, p0.keys_len());
        p0.insert(ACCEPT, rug_fuzz_4.parse().unwrap());
        debug_assert_eq!(rug_fuzz_5, p0.keys_len());
             }
});    }
}
#[cfg(test)]
mod tests_rug_12 {
    use super::*;
    use crate::{HeaderMap, header::HOST};
    #[test]
    fn test_is_empty() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap<_> = HeaderMap::new();
        debug_assert!(p0.is_empty());
        p0.insert(HOST, rug_fuzz_0.parse().unwrap());
        debug_assert!(! p0.is_empty());
             }
});    }
}
#[cfg(test)]
mod tests_rug_13 {
    use super::*;
    use crate::{HeaderMap, header::HOST};
    #[test]
    fn test_clear() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: HeaderMap<_> = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.clear();
        debug_assert!(map.is_empty());
        debug_assert!(map.capacity() > rug_fuzz_1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_14 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::HOST;
    #[test]
    fn test_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, &str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        debug_assert_eq!(rug_fuzz_0, map.capacity());
        map.insert(HOST, rug_fuzz_1.parse().unwrap());
        debug_assert_eq!(rug_fuzz_2, map.capacity());
             }
});    }
}
#[cfg(test)]
mod tests_rug_15 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::HOST;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        let mut p0 = &mut map;
        let p1: usize = rug_fuzz_1;
        p0.reserve(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_16 {
    use super::*;
    use crate::header::HeaderMap;
    use crate::header::{AsHeaderName, HeaderValue, HOST};
    #[test]
    fn test_get() {
        let _rug_st_tests_rug_16_rrrruuuugggg_test_get = 0;
        let rug_fuzz_0 = "hello";
        let rug_fuzz_1 = "host";
        let rug_fuzz_2 = "host";
        let mut p0 = HeaderMap::new();
        p0.insert(HOST, HeaderValue::from_static(rug_fuzz_0));
        let p1: String = rug_fuzz_1.to_string();
        debug_assert_eq!(
            p0.get(rug_fuzz_2).unwrap(), & HeaderValue::from_static("hello")
        );
        let _rug_ed_tests_rug_16_rrrruuuugggg_test_get = 0;
    }
}
#[cfg(test)]
mod tests_rug_18 {
    use super::*;
    use crate::HeaderMap;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_18_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Header-Name";
        let rug_fuzz_1 = "SampleValue";
        let rug_fuzz_2 = "Header-Name";
        #[derive(Default)]
        struct CustomType;
        let mut p0 = HeaderMap::<CustomType>::default();
        let mut headers = HeaderMap::new();
        let v24 = headers.insert(rug_fuzz_0, HeaderValue::from_static(rug_fuzz_1));
        let p1 = rug_fuzz_2;
        p0.get_mut(p1);
        let _rug_ed_tests_rug_18_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_19 {
    use super::*;
    use crate::header::HeaderMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let headers = HeaderMap::new();
        let key = rug_fuzz_0.to_string();
        headers.get_all(key);
             }
});    }
}
#[cfg(test)]
mod tests_rug_20 {
    use super::*;
    use crate::header::HeaderMap;
    use crate::header::HeaderValue;
    use crate::header::AsHeaderName;
    #[test]
    fn test_contains_key() {
        let _rug_st_tests_rug_20_rrrruuuugggg_test_contains_key = 0;
        let rug_fuzz_0 = "Host";
        let rug_fuzz_1 = "world";
        let rug_fuzz_2 = "Host";
        let mut map = HeaderMap::new();
        map.insert(rug_fuzz_0, HeaderValue::from_static(rug_fuzz_1));
        let key = String::from(rug_fuzz_2);
        let result = map.contains_key(key);
        debug_assert!(result);
        let _rug_ed_tests_rug_20_rrrruuuugggg_test_contains_key = 0;
    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{CONTENT_LENGTH, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap<_> = HeaderMap::new();
        p0.insert(HOST, rug_fuzz_0.parse().unwrap());
        p0.append(HOST, rug_fuzz_1.parse().unwrap());
        p0.insert(CONTENT_LENGTH, rug_fuzz_2.parse().unwrap());
        for (key, value) in p0.iter() {
            println!("{:?}: {:?}", key, value);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_23 {
    use super::*;
    use crate::header::{HeaderMap, CONTENT_LENGTH, HOST};
    #[test]
    fn test_keys() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        map.insert(CONTENT_LENGTH, rug_fuzz_2.parse().unwrap());
        for key in map.keys() {
            println!("{:?}", key);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{CONTENT_LENGTH, HOST};
    #[test]
    fn test_values() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        map.insert(CONTENT_LENGTH, rug_fuzz_2.parse().unwrap());
        let p0 = &map;
        p0.values();
             }
});    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    use crate::{HeaderMap, header::{CONTENT_LENGTH, HOST}};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap<String> = HeaderMap::default();
        p0.insert(HOST, rug_fuzz_0.to_string());
        p0.append(HOST, rug_fuzz_1.to_string());
        p0.insert(CONTENT_LENGTH, rug_fuzz_2.to_string());
        for value in p0.values_mut() {
            value.push_str(rug_fuzz_3);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_26 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{CONTENT_LENGTH, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7)) = <(&str, &str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap<_> = HeaderMap::new();
        p0.insert(HOST, rug_fuzz_0.parse().unwrap());
        p0.append(HOST, rug_fuzz_1.parse().unwrap());
        p0.insert(CONTENT_LENGTH, rug_fuzz_2.parse().unwrap());
        let mut drain = p0.drain();
        let (key, mut vals) = drain.next().unwrap();
        debug_assert_eq!(rug_fuzz_3, key);
        debug_assert_eq!(rug_fuzz_4, vals.next().unwrap());
        debug_assert_eq!(rug_fuzz_5, vals.next().unwrap());
        debug_assert!(vals.next().is_none());
        let (key, mut vals) = drain.next().unwrap();
        debug_assert_eq!(rug_fuzz_6, key);
        debug_assert_eq!(rug_fuzz_7, vals.next().unwrap());
        debug_assert!(vals.next().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_rug_29 {
    use super::*;
    use crate::header::HeaderMap;
    use crate::header::HeaderValue;
    #[test]
    fn test_http_entry() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: HeaderMap<u32> = HeaderMap::default();
        let mut p1 = String::from(rug_fuzz_0);
        p0.entry(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_31 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::HOST;
    use crate::header::HeaderName;
    #[test]
    fn test_header_map_insert() {
        let _rug_st_tests_rug_31_rrrruuuugggg_test_header_map_insert = 0;
        let rug_fuzz_0 = "HOST";
        let rug_fuzz_1 = "world";
        let rug_fuzz_2 = "HOST";
        let rug_fuzz_3 = "earth";
        let rug_fuzz_4 = "world";
        let mut map = HeaderMap::new();
        debug_assert!(
            < HeaderMap < HeaderValue > > ::insert(& mut map,
            HeaderName::from_static(rug_fuzz_0), rug_fuzz_1.parse().unwrap()).is_none()
        );
        debug_assert!(! map.is_empty());
        let mut prev = <HeaderMap<
            HeaderValue,
        >>::insert(
                &mut map,
                HeaderName::from_static(rug_fuzz_2),
                rug_fuzz_3.parse().unwrap(),
            )
            .unwrap();
        debug_assert_eq!(rug_fuzz_4, prev);
        let _rug_ed_tests_rug_31_rrrruuuugggg_test_header_map_insert = 0;
    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::HeaderMap;
    use crate::header::{HeaderName, HOST};
    #[test]
    fn test_append() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        debug_assert!(map.insert(HOST, rug_fuzz_0.parse().unwrap()).is_none());
        debug_assert!(! map.is_empty());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        let values = map.get_all(rug_fuzz_2);
        let mut i = values.iter();
        debug_assert_eq!(rug_fuzz_3, * i.next().unwrap());
        debug_assert_eq!(rug_fuzz_4, * i.next().unwrap());
             }
});    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::header::{HeaderMap, HeaderValue, HOST, AsHeaderName};
    #[test]
    fn test_remove() {
        let _rug_st_tests_rug_39_rrrruuuugggg_test_remove = 0;
        let rug_fuzz_0 = "hello.world";
        let rug_fuzz_1 = "hello.world";
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_static(rug_fuzz_0));
        let key = HOST;
        let prev = headers.remove(key).unwrap();
        debug_assert_eq!(rug_fuzz_1, prev);
        debug_assert!(headers.remove(HOST).is_none());
        let _rug_ed_tests_rug_39_rrrruuuugggg_test_remove = 0;
    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    use crate::header::{HeaderMap, CONTENT_LENGTH, CONTENT_TYPE};
    #[test]
    fn test_into_iter() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = HeaderMap::new();
        p0.insert(CONTENT_LENGTH, rug_fuzz_0.parse().unwrap());
        p0.insert(CONTENT_TYPE, rug_fuzz_1.parse().unwrap());
        let mut iter = <HeaderMap<_> as std::iter::IntoIterator>::into_iter(p0);
        debug_assert_eq!(
            iter.next(), Some((Some(CONTENT_LENGTH), "123".parse().unwrap()))
        );
        debug_assert_eq!(
            iter.next(), Some((Some(CONTENT_TYPE), "json".parse().unwrap()))
        );
        debug_assert!(iter.next().is_none());
             }
});    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use crate::std::iter::Iterator;
    use crate::header::map::Keys;
    #[test]
    fn test_size_hint() {
        let _rug_st_tests_rug_64_rrrruuuugggg_test_size_hint = 0;
        let keys: Keys<'_, u32> = unimplemented!();
        keys.size_hint();
        let _rug_ed_tests_rug_64_rrrruuuugggg_test_size_hint = 0;
    }
}
#[cfg(test)]
mod tests_rug_72 {
    use super::*;
    use crate::HeaderMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6, mut rug_fuzz_7, mut rug_fuzz_8)) = <(&str, &str, &str, &str, &str, u32, u32, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map: HeaderMap<u32> = HeaderMap::default();
        let headers = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2, rug_fuzz_3];
        for &header in headers {
            let counter = map.entry(header).expect(rug_fuzz_4).or_insert(rug_fuzz_5);
            *counter += rug_fuzz_6;
        }
        debug_assert_eq!(map[rug_fuzz_7], 2);
        debug_assert_eq!(map[rug_fuzz_8], 1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_76 {
    use super::*;
    use crate::header::{HeaderMap, HeaderName, Entry};
    #[test]
    fn test_into_key() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        if let Entry::Vacant(v) = map.entry(rug_fuzz_0).unwrap() {
            debug_assert_eq!(v.into_key().as_str(), "x-hello");
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_77 {
    use super::*;
    use crate::header::{HeaderMap, Entry};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(&str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        let key = rug_fuzz_0;
        let value = rug_fuzz_1.parse().unwrap();
        if let Entry::Vacant(v) = map.entry(key).unwrap() {
            v.insert(value);
        }
        debug_assert_eq!(map[key], "world");
             }
});    }
}
#[cfg(test)]
mod tests_rug_78 {
    use super::*;
    use crate::header::{HeaderMap, Entry};
    #[test]
    fn test_insert_entry() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(&str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        if let Entry::Vacant(v) = map.entry(rug_fuzz_0).unwrap() {
            let mut e = v.insert_entry(rug_fuzz_1.parse().unwrap());
            e.insert(rug_fuzz_2.parse().unwrap());
        }
        debug_assert_eq!(map[rug_fuzz_3], "world2");
             }
});    }
}
#[cfg(test)]
mod tests_rug_79 {
    use super::*;
    use crate::{HeaderMap, header::{HeaderName, HeaderValue, HOST}};
    use crate::header::GetAll;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        let values = map.get_all(rug_fuzz_2);
        <GetAll<'_, HeaderValue>>::iter(&values);
             }
});    }
}
#[cfg(test)]
mod tests_rug_88 {
    use super::*;
    use crate::std::iter::Iterator;
    use crate::header::IntoIter;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_88_rrrruuuugggg_test_rug = 0;
        struct TestData {}
        let mut p0: IntoIter<TestData> = unimplemented!();
        p0.next();
        let _rug_ed_tests_rug_88_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_91 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST, HeaderName};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        if let Entry::Occupied(e) = map.entry(rug_fuzz_1).unwrap() {
            debug_assert_eq!(rug_fuzz_2, e.key());
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_92 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        if let Entry::Occupied(mut e) = map.entry(rug_fuzz_1).unwrap() {
            debug_assert_eq!(e.get(), & "hello.world");
            e.append(rug_fuzz_2.parse().unwrap());
            debug_assert_eq!(e.get(), & "hello.world");
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(&str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::default();
        map.insert(HOST, rug_fuzz_0.to_string());
        let p0 = map.entry(rug_fuzz_1).unwrap();
        if let Entry::Occupied(mut e) = p0 {
            e.get_mut().push_str(rug_fuzz_2);
            debug_assert_eq!(e.get(), & "hello.world-2");
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_into_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::default();
        map.insert(HOST, rug_fuzz_0.to_string());
        map.append(HOST, rug_fuzz_1.to_string());
        if let Entry::Occupied(e) = map.entry(rug_fuzz_2).unwrap() {
            e.into_mut().push_str(rug_fuzz_3);
        }
        debug_assert_eq!(rug_fuzz_4, map["host"]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_95 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_insert() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        let mut p0 = map.entry(rug_fuzz_1).unwrap();
        let p1 = rug_fuzz_2.parse().unwrap();
        if let Entry::Occupied(mut e) = p0 {
            let mut prev = e.insert(p1);
            debug_assert_eq!(rug_fuzz_3, prev);
        }
        debug_assert_eq!(rug_fuzz_4, map["host"]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST, ValueDrain};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5, mut rug_fuzz_6)) = <(&str, &str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        if let Entry::Occupied(mut e) = map.entry(rug_fuzz_2).unwrap() {
            let mut prev = e.insert_mult(rug_fuzz_3.parse().unwrap());
            debug_assert_eq!(rug_fuzz_4, prev.next().unwrap());
            debug_assert_eq!(rug_fuzz_5, prev.next().unwrap());
            debug_assert!(prev.next().is_none());
        }
        debug_assert_eq!(rug_fuzz_6, map["host"]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_97 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        if let Entry::Occupied(mut e) = map.entry(rug_fuzz_1).unwrap() {
            e.append(rug_fuzz_2.parse().unwrap());
        }
        let values = map.get_all(rug_fuzz_3);
        let mut i = values.iter();
        debug_assert_eq!(rug_fuzz_4, * i.next().unwrap());
        debug_assert_eq!(rug_fuzz_5, * i.next().unwrap());
             }
});    }
}
#[cfg(test)]
mod tests_rug_98 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST, HeaderValue};
    #[test]
    fn test_remove() {
        let _rug_st_tests_rug_98_rrrruuuugggg_test_remove = 0;
        let rug_fuzz_0 = "world";
        let rug_fuzz_1 = "host";
        let rug_fuzz_2 = "world";
        let rug_fuzz_3 = "host";
        let mut map = HeaderMap::new();
        map.insert(HOST, HeaderValue::from_static(rug_fuzz_0));
        if let Entry::Occupied(e) = map.entry(rug_fuzz_1).unwrap() {
            let prev = e.remove();
            debug_assert_eq!(rug_fuzz_2, prev.to_str().unwrap());
        }
        debug_assert!(! map.contains_key(rug_fuzz_3));
        let _rug_ed_tests_rug_98_rrrruuuugggg_test_remove = 0;
    }
}
#[cfg(test)]
mod tests_rug_99 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_99_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "host";
        let rug_fuzz_1 = "world";
        let rug_fuzz_2 = "host";
        let rug_fuzz_3 = "host";
        let rug_fuzz_4 = "world";
        let rug_fuzz_5 = "host";
        let mut map = HeaderMap::new();
        map.insert(rug_fuzz_0, rug_fuzz_1.parse().unwrap());
        if let Entry::Occupied(e) = map.entry(rug_fuzz_2).unwrap() {
            let (key, mut prev) = e.remove_entry();
            debug_assert_eq!(rug_fuzz_3, key.as_str());
            debug_assert_eq!(rug_fuzz_4, prev);
        }
        debug_assert!(! map.contains_key(rug_fuzz_5));
        let _rug_ed_tests_rug_99_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(&str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::new();
        map.insert(HOST, rug_fuzz_0.parse().unwrap());
        map.append(HOST, rug_fuzz_1.parse().unwrap());
        if let Entry::Occupied(e) = map.entry(rug_fuzz_2).unwrap() {
            let mut iter = e.iter();
            debug_assert_eq!(& rug_fuzz_3, iter.next().unwrap());
            debug_assert_eq!(& rug_fuzz_4, iter.next().unwrap());
            debug_assert!(iter.next().is_none());
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use crate::header::{HeaderMap, Entry, HOST};
    #[test]
    fn test_iter_mut() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4, mut rug_fuzz_5)) = <(&str, &str, &str, &str, &str, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut map = HeaderMap::default();
        map.insert(HOST, rug_fuzz_0.to_string());
        map.append(HOST, rug_fuzz_1.to_string());
        let key = rug_fuzz_2;
        if let Entry::Occupied(mut e) = map.entry(key).unwrap() {
            for e in e.iter_mut() {
                e.push_str(rug_fuzz_3);
            }
        }
        let mut values = map.get_all(key);
        let mut i = values.iter();
        debug_assert_eq!(& rug_fuzz_4, i.next().unwrap());
        debug_assert_eq!(& rug_fuzz_5, i.next().unwrap());
             }
});    }
}
#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::header::map::OccupiedEntry;
    use crate::header::ValueIterMut;
    #[test]
    fn test_into_iter() {
        let _rug_st_tests_rug_103_rrrruuuugggg_test_into_iter = 0;
        let p0: OccupiedEntry<'static, String> = unimplemented!();
        p0.into_iter();
        let _rug_ed_tests_rug_103_rrrruuuugggg_test_into_iter = 0;
    }
}
#[cfg(test)]
mod tests_rug_104 {
    use super::*;
    use crate::std::iter::IntoIterator;
    use crate::header::map::{OccupiedEntry, ValueIter};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_104_rrrruuuugggg_test_rug = 0;
        let mut entry: OccupiedEntry<'static, String> = todo!();
        entry.into_iter();
        let _rug_ed_tests_rug_104_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_110 {
    use super::*;
    use header::map::Pos;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_110_rrrruuuugggg_test_rug = 0;
        Pos::none();
        let _rug_ed_tests_rug_110_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_111 {
    use super::*;
    use crate::header::map::Pos;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_111_rrrruuuugggg_test_rug = 0;
        let p0 = Pos::none();
        debug_assert_eq!(p0.is_some(), false);
        let _rug_ed_tests_rug_111_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_112 {
    use super::*;
    use crate::header::map::Pos;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_112_rrrruuuugggg_test_rug = 0;
        let p0 = Pos::none();
        debug_assert_eq!(p0.is_none(), true);
        let _rug_ed_tests_rug_112_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_113 {
    use super::*;
    use crate::header::map;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_113_rrrruuuugggg_test_rug = 0;
        let mut p0 = map::Pos::none();
        map::Pos::resolve(&p0);
        let _rug_ed_tests_rug_113_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_116 {
    use super::*;
    use header::map::Danger;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_116_rrrruuuugggg_test_rug = 0;
        let p0: Danger = Danger::Yellow;
        debug_assert_eq!(p0.is_yellow(), true);
        let _rug_ed_tests_rug_116_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_117 {
    use super::*;
    use header::map::Danger;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_117_rrrruuuugggg_test_rug = 0;
        let mut p0: Danger = Danger::Green;
        p0.to_yellow();
        let _rug_ed_tests_rug_117_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_123 {
    use super::*;
    use crate::header::map::into_header_name::Sealed;
    use crate::header::map::{HeaderMap, HeaderValue};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_123_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Content-Type";
        let rug_fuzz_1 = "application/json";
        let p0: &'static str = rug_fuzz_0;
        let mut p1: HeaderMap<HeaderValue> = HeaderMap::new();
        let p2: HeaderValue = HeaderValue::from_static(rug_fuzz_1);
        <&'static str>::insert(&p0, &mut p1, p2);
        debug_assert_eq!(p1.len(), 1);
        let _rug_ed_tests_rug_123_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_124 {
    use super::*;
    use crate::header::map::into_header_name::Sealed;
    use crate::header::map::{HeaderMap, HeaderValue};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_124_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Content-Type";
        let rug_fuzz_1 = "application/json";
        let mut p0: &str = rug_fuzz_0;
        let mut p1: HeaderMap<HeaderValue> = HeaderMap::new();
        let mut p2: HeaderValue = HeaderValue::from_static(rug_fuzz_1);
        p0.append(&mut p1, p2);
        let _rug_ed_tests_rug_124_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_125 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use crate::header::{HeaderName, HeaderMap};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_125_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Content-Type";
        let p0 = HeaderName::from_static(rug_fuzz_0);
        let mut p1 = HeaderMap::new();
        <HeaderName as Sealed>::entry(p0, &mut p1);
        let _rug_ed_tests_rug_125_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_127 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use crate::header::name::HeaderName;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_127_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Content-Type";
        let mut p0 = HeaderName::from_static(rug_fuzz_0);
        p0.as_str();
        let _rug_ed_tests_rug_127_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_128 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use crate::header::{HeaderName, HeaderMap};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_128_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "Content-Type";
        let name = HeaderName::from_static(rug_fuzz_0);
        let mut map = HeaderMap::new();
        name.entry(&mut map);
        let _rug_ed_tests_rug_128_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_131 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use crate::header::map::{HdrName, HeaderMap, Entry, InvalidHeaderName};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let header_name_str = rug_fuzz_0;
        let mut header_map: HeaderMap<u8> = HeaderMap::default();
        header_name_str.entry(&mut header_map);
             }
});    }
}
#[cfg(test)]
mod tests_rug_133 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    #[test]
    fn test_as_str() {
        let _rug_st_tests_rug_133_rrrruuuugggg_test_as_str = 0;
        let rug_fuzz_0 = "Sample Header Value";
        let p0: &'static str = rug_fuzz_0;
        <&'static str as Sealed>::as_str(&p0);
        let _rug_ed_tests_rug_133_rrrruuuugggg_test_as_str = 0;
    }
}
#[cfg(test)]
mod tests_rug_135 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use crate::header::map::HeaderMap;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = String::from(rug_fuzz_0);
        let mut p1 = HeaderMap::new();
        p0.find(&p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_136 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use std::string::String;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: String = String::from(rug_fuzz_0);
        <String as Sealed>::as_str(&p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_139 {
    use super::*;
    use crate::header::map::as_header_name::Sealed;
    use std::string::String;
    #[test]
    fn test_as_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let sample_string: String = String::from(rug_fuzz_0);
        let p0: &String = &sample_string;
        p0.as_str();
             }
});    }
}
