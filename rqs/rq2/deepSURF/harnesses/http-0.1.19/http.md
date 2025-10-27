# Crate Documentation

**Version:** 0.1.19

**Format Version:** 39

# Module `http`

A general purpose library of common HTTP types

This crate is a general purpose library for common types found when working
with the HTTP protocol. You'll find `Request` and `Response` types for
working as either a client or a server as well as all of their components.
Notably you'll find `Uri` for what a `Request` is requesting, a `Method`
for how it's being requested, a `StatusCode` for what sort of response came
back, a `Version` for how this was communicated, and
`HeaderName`/`HeaderValue` definitions to get grouped in a `HeaderMap` to
work with request/response headers.

You will notably *not* find an implementation of sending requests or
spinning up a server in this crate. It's intended that this crate is the
"standard library" for HTTP clients and servers without dictating any
particular implementation. Note that this crate is still early on in its
lifecycle so the support libraries that integrate with the `http` crate are
a work in progress! Stay tuned and we'll be sure to highlight crates here
in the future.

## Requests and Responses

Perhaps the main two types in this crate are the `Request` and `Response`
types. A `Request` could either be constructed to get sent off as a client
or it can also be received to generate a `Response` for a server. Similarly
as a client a `Response` is what you get after sending a `Request`, whereas
on a server you'll be manufacturing a `Response` to send back to the client.

Each type has a number of accessors for the component fields. For as a
server you might want to inspect a requests URI to dispatch it:

```
use http::{Request, Response};

fn response(req: Request<()>) -> http::Result<Response<()>> {
    match req.uri().path() {
        "/" => index(req),
        "/foo" => foo(req),
        "/bar" => bar(req),
        _ => not_found(req),
    }
}
# fn index(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn foo(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn bar(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
# fn not_found(_req: Request<()>) -> http::Result<Response<()>> { panic!() }
```

On a `Request` you'll also find accessors like `method` to return a
`Method` and `headers` to inspect the various headers. A `Response`
has similar methods for headers, the status code, etc.

In addition to getters, request/response types also have mutable accessors
to edit the request/response:

```
use http::{HeaderValue, Response, StatusCode};
use http::header::CONTENT_TYPE;

fn add_server_headers<T>(response: &mut Response<T>) {
    response.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));
    *response.status_mut() = StatusCode::OK;
}
```

And finally, one of the most important aspects of requests/responses, the
body! The `Request` and `Response` types in this crate are *generic* in
what their body is. This allows downstream libraries to use different
representations such as `Request<Vec<u8>>`, `Response<impl Read>`,
`Request<impl Stream<Item = Vec<u8>, Error = _>>`, or even
`Response<MyCustomType>` where the custom type was deserialized from JSON.

The body representation is intentionally flexible to give downstream
libraries maximal flexibility in implementing the body as appropriate.

## HTTP Headers

Another major piece of functionality in this library is HTTP header
interpretation and generation. The `HeaderName` type serves as a way to
define header *names*, or what's to the left of the colon. A `HeaderValue`
conversely is the header *value*, or what's to the right of a colon.

For example, if you have an HTTP request that looks like:

```http
GET /foo HTTP/1.1
Accept: text/html
```

Then `"Accept"` is a `HeaderName` while `"text/html"` is a `HeaderValue`.
Each of these is a dedicated type to allow for a number of interesting
optimizations and to also encode the static guarantees of each type. For
example a `HeaderName` is always a valid `&str`, but a `HeaderValue` may
not be valid UTF-8.

The most common header names are already defined for you as constant values
in the `header` module of this crate. For example:

```
use http::header::{self, HeaderName};

let name: HeaderName = header::ACCEPT;
assert_eq!(name.as_str(), "accept");
```

You can, however, also parse header names from strings:

```
use http::header::{self, HeaderName};

let name = "Accept".parse::<HeaderName>().unwrap();
assert_eq!(name, header::ACCEPT);
```

Header values can be created from string literals through the `from_static`
function:

```
use http::HeaderValue;

let value = HeaderValue::from_static("text/html");
assert_eq!(value.as_bytes(), b"text/html");
```

And header values can also be parsed like names:

```
use http::HeaderValue;

let value = "text/html";
let value = value.parse::<HeaderValue>().unwrap();
```

Most HTTP requests and responses tend to come with more than one header, so
it's not too useful to just work with names and values only! This crate also
provides a `HeaderMap` type which is a specialized hash map for keys as
`HeaderName` and generic values. This type, like header names, is optimized
for common usage but should continue to scale with your needs over time.

# URIs

Each HTTP `Request` has an associated URI with it. This may just be a path
like `/index.html` but it could also be an absolute URL such as
`https://www.rust-lang.org/index.html`. A `URI` has a number of accessors to
interpret it:

```
use http::Uri;

let uri = "https://www.rust-lang.org/index.html".parse::<Uri>().unwrap();

assert_eq!(uri.scheme_str(), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/index.html");
assert_eq!(uri.query(), None);
```

## Modules

## Module `header`

HTTP header types

The module provides [`HeaderName`], [`HeaderMap`], and a number of types
used for interacting with `HeaderMap`. These types allow representing both
HTTP/1 and HTTP/2 headers.

# `HeaderName`

The `HeaderName` type represents both standard header names as well as
custom header names. The type handles the case insensitive nature of header
names and is used as the key portion of `HeaderMap`. Header names are
normalized to lower case. In other words, when creating a `HeaderName` with
a string, even if upper case characters are included, when getting a string
representation of the `HeaderName`, it will be all lower case. This allows
for faster `HeaderMap` comparison operations.

The internal representation is optimized to efficiently handle the cases
most commonly encountered when working with HTTP. Standard header names are
special cased and are represented internally as an enum. Short custom
headers will be stored directly in the `HeaderName` struct and will not
incur any allocation overhead, however longer strings will require an
allocation for storage.

## Limitations

`HeaderName` has a max length of 32,768 for header names. Attempting to
parse longer names will result in a panic.

# `HeaderMap`

`HeaderMap` is a map structure of header names highly optimized for use
cases common with HTTP. It is a [multimap] structure, where each header name
may have multiple associated header values. Given this, some of the APIs
diverge from [`HashMap`].

## Overview

Just like `HashMap` in Rust's stdlib, `HeaderMap` is based on [Robin Hood
hashing]. This algorithm tends to reduce the worst case search times in the
table and enables high load factors without seriously affecting performance.
Internally, keys and values are stored in vectors. As such, each insertion
will not incur allocation overhead. However, once the underlying vector
storage is full, a larger vector must be allocated and all values copied.

## Deterministic ordering

Unlike Rust's `HashMap`, values in `HeaderMap` are deterministically
ordered. Roughly, values are ordered by insertion. This means that a
function that deterministically operates on a header map can rely on the
iteration order to remain consistent across processes and platforms.

## Adaptive hashing

`HeaderMap` uses an adaptive hashing strategy in order to efficiently handle
most common cases. All standard headers have statically computed hash values
which removes the need to perform any hashing of these headers at runtime.
The default hash function emphasizes performance over robustness. However,
`HeaderMap` detects high collision rates and switches to a secure hash
function in those events. The threshold is set such that only denial of
service attacks should trigger it.

## Limitations

`HeaderMap` can store a maximum of 32,768 headers (header name / value
pairs). Attempting to insert more will result in a panic.

[`HeaderName`]: struct.HeaderName.html
[`HeaderMap`]: struct.HeaderMap.html
[multimap]: https://en.wikipedia.org/wiki/Multimap
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[Robin Hood hashing]: https://en.wikipedia.org/wiki/Hash_table#Robin_Hood_hashing

```rust
pub mod header { /* ... */ }
```

### Modules

## Module `map`

```rust
pub(in ::header) mod map { /* ... */ }
```

### Modules

## Module `into_header_name`

```rust
pub(in ::header::map) mod into_header_name { /* ... */ }
```

### Traits

#### Trait `IntoHeaderName`

A marker trait used to identify values that can be used as insert keys
to a `HeaderMap`.

```rust
pub trait IntoHeaderName: Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `super::name::HeaderName`
- `&''a super::name::HeaderName` with <''a>
- `&''static str`

#### Trait `Sealed`

```rust
pub trait Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `super::name::HeaderName`
- `&''a super::name::HeaderName` with <''a>
- `&''static str`

## Module `as_header_name`

```rust
pub(in ::header::map) mod as_header_name { /* ... */ }
```

### Traits

#### Trait `AsHeaderName`

A marker trait used to identify values that can be used as search keys
to a `HeaderMap`.

```rust
pub trait AsHeaderName: Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `super::name::HeaderName`
- `&''a super::name::HeaderName` with <''a>
- `&''a str` with <''a>
- `String`
- `&''a String` with <''a>

#### Trait `Sealed`

```rust
pub trait Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `super::name::HeaderName`
- `&''a super::name::HeaderName` with <''a>
- `&''a str` with <''a>
- `String`
- `&''a String` with <''a>

### Types

#### Struct `HeaderMap`

A set of HTTP headers

`HeaderMap` is an multimap of [`HeaderName`] to values.

[`HeaderName`]: struct.HeaderName.html

# Examples

Basic usage

```
# use http::HeaderMap;
# use http::header::{CONTENT_LENGTH, HOST, LOCATION};
let mut headers = HeaderMap::new();

headers.insert(HOST, "example.com".parse().unwrap());
headers.insert(CONTENT_LENGTH, "123".parse().unwrap());

assert!(headers.contains_key(HOST));
assert!(!headers.contains_key(LOCATION));

assert_eq!(headers[HOST], "example.com");

headers.remove(HOST);

assert!(!headers.contains_key(HOST));
```

```rust
pub struct HeaderMap<T = super::HeaderValue> {
    pub(in ::header::map) mask: usize,
    pub(in ::header::map) indices: Box<[Pos]>,
    pub(in ::header::map) entries: Vec<Bucket<T>>,
    pub(in ::header::map) extra_values: Vec<ExtraValue<T>>,
    pub(in ::header::map) danger: Danger,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `mask` | `usize` |  |
| `indices` | `Box<[Pos]>` |  |
| `entries` | `Vec<Bucket<T>>` |  |
| `extra_values` | `Vec<ExtraValue<T>>` |  |
| `danger` | `Danger` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create an empty `HeaderMap`.

- ```rust
  pub fn with_capacity(capacity: usize) -> HeaderMap<T> { /* ... */ }
  ```
  Create an empty `HeaderMap` with the specified capacity.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of headers stored in the map.

- ```rust
  pub fn keys_len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of keys stored in the map.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the map contains no elements.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the map, removing all key-value pairs. Keeps the allocated memory

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of headers the map can hold without reallocating.

- ```rust
  pub fn reserve(self: &mut Self, additional: usize) { /* ... */ }
  ```
  Reserves capacity for at least `additional` more headers to be inserted

- ```rust
  pub fn get<K>(self: &Self, key: K) -> Option<&T>
where
    K: AsHeaderName { /* ... */ }
  ```
  Returns a reference to the value associated with the key.

- ```rust
  pub(in ::header::map) fn get2<K>(self: &Self, key: &K) -> Option<&T>
where
    K: AsHeaderName { /* ... */ }
  ```

- ```rust
  pub fn get_mut<K>(self: &mut Self, key: K) -> Option<&mut T>
where
    K: AsHeaderName { /* ... */ }
  ```
  Returns a mutable reference to the value associated with the key.

- ```rust
  pub fn get_all<K>(self: &Self, key: K) -> GetAll<''_, T>
where
    K: AsHeaderName { /* ... */ }
  ```
  Returns a view of all values associated with a key.

- ```rust
  pub fn contains_key<K>(self: &Self, key: K) -> bool
where
    K: AsHeaderName { /* ... */ }
  ```
  Returns true if the map contains a value for the specified key.

- ```rust
  pub fn iter(self: &Self) -> Iter<''_, T> { /* ... */ }
  ```
  An iterator visiting all key-value pairs.

- ```rust
  pub fn iter_mut(self: &mut Self) -> IterMut<''_, T> { /* ... */ }
  ```
  An iterator visiting all key-value pairs, with mutable value references.

- ```rust
  pub fn keys(self: &Self) -> Keys<''_, T> { /* ... */ }
  ```
  An iterator visiting all keys.

- ```rust
  pub fn values(self: &Self) -> Values<''_, T> { /* ... */ }
  ```
  An iterator visiting all values.

- ```rust
  pub fn values_mut(self: &mut Self) -> ValuesMut<''_, T> { /* ... */ }
  ```
  An iterator visiting all values mutably.

- ```rust
  pub fn drain(self: &mut Self) -> Drain<''_, T> { /* ... */ }
  ```
  Clears the map, returning all entries as an iterator.

- ```rust
  pub(in ::header::map) fn value_iter(self: &Self, idx: Option<usize>) -> ValueIter<''_, T> { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn value_iter_mut(self: &mut Self, idx: usize) -> ValueIterMut<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn entry<K>(self: &mut Self, key: K) -> Result<Entry<''_, T>, InvalidHeaderName>
where
    K: AsHeaderName { /* ... */ }
  ```
  Gets the given key's corresponding entry in the map for in-place

- ```rust
  pub(in ::header::map) fn entry2<K>(self: &mut Self, key: K) -> Entry<''_, T>
where
    K: Hash + Into<HeaderName>,
    HeaderName: PartialEq<K> { /* ... */ }
  ```

- ```rust
  pub fn insert<K>(self: &mut Self, key: K, val: T) -> Option<T>
where
    K: IntoHeaderName { /* ... */ }
  ```
  Inserts a key-value pair into the map.

- ```rust
  pub(in ::header::map) fn insert2<K>(self: &mut Self, key: K, value: T) -> Option<T>
where
    K: Hash + Into<HeaderName>,
    HeaderName: PartialEq<K> { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn insert_occupied(self: &mut Self, index: usize, value: T) -> T { /* ... */ }
  ```
  Set an occupied bucket to the given value

- ```rust
  pub(in ::header::map) fn insert_occupied_mult(self: &mut Self, index: usize, value: T) -> ValueDrain<''_, T> { /* ... */ }
  ```

- ```rust
  pub fn append<K>(self: &mut Self, key: K, value: T) -> bool
where
    K: IntoHeaderName { /* ... */ }
  ```
  Inserts a key-value pair into the map.

- ```rust
  pub(in ::header::map) fn append2<K>(self: &mut Self, key: K, value: T) -> bool
where
    K: Hash + Into<HeaderName>,
    HeaderName: PartialEq<K> { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn find<K>(self: &Self, key: &K) -> Option<(usize, usize)>
where
    K: Hash + Into<HeaderName> + ?Sized,
    HeaderName: PartialEq<K> { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn insert_phase_two(self: &mut Self, key: HeaderName, value: T, hash: HashValue, probe: usize, danger: bool) -> usize { /* ... */ }
  ```
  phase 2 is post-insert where we forward-shift `Pos` in the indices.

- ```rust
  pub fn remove<K>(self: &mut Self, key: K) -> Option<T>
where
    K: AsHeaderName { /* ... */ }
  ```
  Removes a key from the map, returning the value associated with the key.

- ```rust
  pub(in ::header::map) fn remove_found(self: &mut Self, probe: usize, found: usize) -> Bucket<T> { /* ... */ }
  ```
  Remove an entry from the map.

- ```rust
  pub(in ::header::map) fn remove_extra_value(self: &mut Self, idx: usize) -> ExtraValue<T> { /* ... */ }
  ```
  Removes the `ExtraValue` at the given index.

- ```rust
  pub(in ::header::map) fn remove_all_extra_values(self: &mut Self, head: usize) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn insert_entry(self: &mut Self, hash: HashValue, key: HeaderName, value: T) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn rebuild(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn reinsert_entry_in_order(self: &mut Self, pos: Pos) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn reserve_one(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn grow(self: &mut Self, new_raw_cap: usize) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HeaderMap<T>) -> bool { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **FromIterator**
  - ```rust
    fn from_iter<I>(iter: I) -> Self
where
    I: IntoIterator<Item = (HeaderName, T)> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> Iter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IterMut<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> IntoIter<T> { /* ... */ }
    ```
    Creates a consuming iterator, that is, one that moves keys and values

- **HttpTryFrom**
- **Index**
  - ```rust
    fn index(self: &Self, index: K) -> &T { /* ... */ }
    ```
    # Panics

- **Extend**
  - ```rust
    fn extend<I: IntoIterator<Item = (Option<HeaderName>, T)>>(self: &mut Self, iter: I) { /* ... */ }
    ```
    Extend a `HeaderMap` with the contents of another `HeaderMap`.

  - ```rust
    fn extend<I: IntoIterator<Item = (HeaderName, T)>>(self: &mut Self, iter: I) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sealed**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HeaderMap<T> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `Iter`

`HeaderMap` entry iterator.

Yields `(&HeaderName, &value)` tuples. The same header name may be yielded
more than once if it has more than one associated value.

```rust
pub struct Iter<''a, T: ''a> {
    pub(in ::header::map) inner: IterMut<''a, T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `IterMut<''a, T>` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `IterMut`

`HeaderMap` mutable entry iterator

Yields `(&HeaderName, &mut value)` tuples. The same header name may be
yielded more than once if it has more than one associated value.

```rust
pub struct IterMut<''a, T: ''a> {
    pub(in ::header::map) map: *mut HeaderMap<T>,
    pub(in ::header::map) entry: usize,
    pub(in ::header::map) cursor: Option<Cursor>,
    pub(in ::header::map) lt: std::marker::PhantomData<&''a mut HeaderMap<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `*mut HeaderMap<T>` |  |
| `entry` | `usize` |  |
| `cursor` | `Option<Cursor>` |  |
| `lt` | `std::marker::PhantomData<&''a mut HeaderMap<T>>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::header::map) fn next_unsafe(self: &mut Self) -> Option<(&''a HeaderName, *mut T)> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `IntoIter`

An owning iterator over the entries of a `HeaderMap`.

This struct is created by the `into_iter` method on `HeaderMap`.

```rust
pub struct IntoIter<T> {
    pub(in ::header::map) next: Option<usize>,
    pub(in ::header::map) entries: vec::IntoIter<Bucket<T>>,
    pub(in ::header::map) extra_values: Vec<ExtraValue<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next` | `Option<usize>` |  |
| `entries` | `vec::IntoIter<Bucket<T>>` |  |
| `extra_values` | `Vec<ExtraValue<T>>` |  |

##### Implementations

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Keys`

An iterator over `HeaderMap` keys.

Each header name is yielded only once, even if it has more than one
associated value.

```rust
pub struct Keys<''a, T: ''a> {
    pub(in ::header::map) inner: ::std::slice::Iter<''a, Bucket<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `::std::slice::Iter<''a, Bucket<T>>` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **ExactSizeIterator**
#### Struct `Values`

`HeaderMap` value iterator.

Each value contained in the `HeaderMap` will be yielded.

```rust
pub struct Values<''a, T: ''a> {
    pub(in ::header::map) inner: Iter<''a, T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Iter<''a, T>` |  |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ValuesMut`

`HeaderMap` mutable value iterator

```rust
pub struct ValuesMut<''a, T: ''a> {
    pub(in ::header::map) inner: IterMut<''a, T>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `IterMut<''a, T>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Drain`

A drain iterator for `HeaderMap`.

```rust
pub struct Drain<''a, T: ''a> {
    pub(in ::header::map) idx: usize,
    pub(in ::header::map) map: *mut HeaderMap<T>,
    pub(in ::header::map) lt: std::marker::PhantomData<&''a mut HeaderMap<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `idx` | `usize` |  |
| `map` | `*mut HeaderMap<T>` |  |
| `lt` | `std::marker::PhantomData<&''a mut HeaderMap<T>>` |  |

##### Implementations

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
#### Struct `GetAll`

A view to all values stored in a single entry.

This struct is returned by `HeaderMap::get_all`.

```rust
pub struct GetAll<''a, T: ''a> {
    pub(in ::header::map) map: &''a HeaderMap<T>,
    pub(in ::header::map) index: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `&''a HeaderMap<T>` |  |
| `index` | `Option<usize>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn iter(self: &Self) -> ValueIter<''a, T> { /* ... */ }
  ```
  Returns an iterator visiting all values associated with the entry.

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> ValueIter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> ValueIter<''a, T> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Send**
#### Enum `Entry`

A view into a single location in a `HeaderMap`, which may be vacant or occupied.

```rust
pub enum Entry<''a, T: ''a> {
    Occupied(OccupiedEntry<''a, T>),
    Vacant(VacantEntry<''a, T>),
}
```

##### Variants

###### `Occupied`

An occupied entry

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `OccupiedEntry<''a, T>` |  |

###### `Vacant`

A vacant entry

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `VacantEntry<''a, T>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn or_insert(self: Self, default: T) -> &''a mut T { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the default if empty.

- ```rust
  pub fn or_insert_with<F: FnOnce() -> T>(self: Self, default: F) -> &''a mut T { /* ... */ }
  ```
  Ensures a value is in the entry by inserting the result of the default

- ```rust
  pub fn key(self: &Self) -> &HeaderName { /* ... */ }
  ```
  Returns a reference to the entry's key

###### Trait Implementations

- **Sync**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `VacantEntry`

A view into a single empty location in a `HeaderMap`.

This struct is returned as part of the `Entry` enum.

```rust
pub struct VacantEntry<''a, T: ''a> {
    pub(in ::header::map) map: &''a mut HeaderMap<T>,
    pub(in ::header::map) key: super::name::HeaderName,
    pub(in ::header::map) hash: HashValue,
    pub(in ::header::map) probe: usize,
    pub(in ::header::map) danger: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `&''a mut HeaderMap<T>` |  |
| `key` | `super::name::HeaderName` |  |
| `hash` | `HashValue` |  |
| `probe` | `usize` |  |
| `danger` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn key(self: &Self) -> &HeaderName { /* ... */ }
  ```
  Returns a reference to the entry's key

- ```rust
  pub fn into_key(self: Self) -> HeaderName { /* ... */ }
  ```
  Take ownership of the key

- ```rust
  pub fn insert(self: Self, value: T) -> &''a mut T { /* ... */ }
  ```
  Insert the value into the entry.

- ```rust
  pub fn insert_entry(self: Self, value: T) -> OccupiedEntry<''a, T> { /* ... */ }
  ```
  Insert the value into the entry.

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **Send**
- **Sync**
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

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `OccupiedEntry`

A view into a single occupied location in a `HeaderMap`.

This struct is returned as part of the `Entry` enum.

```rust
pub struct OccupiedEntry<''a, T: ''a> {
    pub(in ::header::map) map: &''a mut HeaderMap<T>,
    pub(in ::header::map) probe: usize,
    pub(in ::header::map) index: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `&''a mut HeaderMap<T>` |  |
| `probe` | `usize` |  |
| `index` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn key(self: &Self) -> &HeaderName { /* ... */ }
  ```
  Returns a reference to the entry's key.

- ```rust
  pub fn get(self: &Self) -> &T { /* ... */ }
  ```
  Get a reference to the first value in the entry.

- ```rust
  pub fn get_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Get a mutable reference to the first value in the entry.

- ```rust
  pub fn into_mut(self: Self) -> &''a mut T { /* ... */ }
  ```
  Converts the `OccupiedEntry` into a mutable reference to the **first**

- ```rust
  pub fn insert(self: &mut Self, value: T) -> T { /* ... */ }
  ```
  Sets the value of the entry.

- ```rust
  pub fn insert_mult(self: &mut Self, value: T) -> ValueDrain<''_, T> { /* ... */ }
  ```
  Sets the value of the entry.

- ```rust
  pub fn append(self: &mut Self, value: T) { /* ... */ }
  ```
  Insert the value into the entry.

- ```rust
  pub fn remove(self: Self) -> T { /* ... */ }
  ```
  Remove the entry from the map.

- ```rust
  pub fn remove_entry(self: Self) -> (HeaderName, T) { /* ... */ }
  ```
  Remove the entry from the map.

- ```rust
  pub fn remove_entry_mult(self: Self) -> (HeaderName, ValueDrain<''a, T>) { /* ... */ }
  ```
  Remove the entry from the map.

- ```rust
  pub fn iter(self: &Self) -> ValueIter<''_, T> { /* ... */ }
  ```
  Returns an iterator visiting all values associated with the entry.

- ```rust
  pub fn iter_mut(self: &mut Self) -> ValueIterMut<''_, T> { /* ... */ }
  ```
  Returns an iterator mutably visiting all values associated with the

###### Trait Implementations

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> ValueIterMut<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> ValueIter<''a, T> { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> ValueIterMut<''a, T> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ValueIter`

An iterator of all values associated with a single header name.

```rust
pub struct ValueIter<''a, T: ''a> {
    pub(in ::header::map) map: &''a HeaderMap<T>,
    pub(in ::header::map) index: usize,
    pub(in ::header::map) front: Option<Cursor>,
    pub(in ::header::map) back: Option<Cursor>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `&''a HeaderMap<T>` |  |
| `index` | `usize` |  |
| `front` | `Option<Cursor>` |  |
| `back` | `Option<Cursor>` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ValueIterMut`

A mutable iterator of all values associated with a single header name.

```rust
pub struct ValueIterMut<''a, T: ''a> {
    pub(in ::header::map) map: *mut HeaderMap<T>,
    pub(in ::header::map) index: usize,
    pub(in ::header::map) front: Option<Cursor>,
    pub(in ::header::map) back: Option<Cursor>,
    pub(in ::header::map) lt: std::marker::PhantomData<&''a mut HeaderMap<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `*mut HeaderMap<T>` |  |
| `index` | `usize` |  |
| `front` | `Option<Cursor>` |  |
| `back` | `Option<Cursor>` |  |
| `lt` | `std::marker::PhantomData<&''a mut HeaderMap<T>>` |  |

##### Implementations

###### Trait Implementations

- **DoubleEndedIterator**
  - ```rust
    fn next_back(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ValueDrain`

An drain iterator of all values associated with a single header name.

```rust
pub struct ValueDrain<''a, T: ''a> {
    pub(in ::header::map) map: *mut HeaderMap<T>,
    pub(in ::header::map) first: Option<T>,
    pub(in ::header::map) next: Option<usize>,
    pub(in ::header::map) lt: std::marker::PhantomData<&''a mut HeaderMap<T>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `*mut HeaderMap<T>` |  |
| `first` | `Option<T>` |  |
| `next` | `Option<usize>` |  |
| `lt` | `std::marker::PhantomData<&''a mut HeaderMap<T>>` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<T> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
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

- **Sync**
- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `Cursor`

Tracks the value iterator state

```rust
pub(in ::header::map) enum Cursor {
    Head,
    Values(usize),
}
```

##### Variants

###### `Head`

###### `Values`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cursor { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Cursor) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **Freeze**
#### Type Alias `Size`

Type used for representing the size of a HeaderMap value.

32,768 is more than enough entries for a single header map. Setting this
limit enables using `u16` to represent all offsets, which takes 2 bytes
instead of 8 on 64 bit processors.

Setting this limit is especially benificial for `indices`, making it more
cache friendly. More hash codes can fit in a cache line.

You may notice that `u16` may represent more than 32,768 values. This is
true, but 32,768 should be plenty and it allows us to reserve the top bit
for future usage.

```rust
pub(in ::header::map) type Size = usize;
```

#### Struct `Pos`

An entry in the hash table. This represents the full hash code for an entry
as well as the position of the entry in the `entries` vector.

```rust
pub(in ::header::map) struct Pos {
    pub(in ::header::map) index: usize,
    pub(in ::header::map) hash: HashValue,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `index` | `usize` |  |
| `hash` | `HashValue` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::header::map) fn new(index: usize, hash: HashValue) -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn none() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn is_some(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn is_none(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn resolve(self: &Self) -> Option<(usize, HashValue)> { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Pos { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Copy**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `HashValue`

Hash values are limited to u16 as well. While `fast_hash` and `Hasher`
return `usize` hash codes, limiting the effective hash code to the lower 16
bits is fine since we know that the `indices` vector will never grow beyond
that size.

```rust
pub(in ::header::map) struct HashValue(pub(in ::header::map) usize);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HashValue { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HashValue) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `Bucket`

Stores the data associated with a `HeaderMap` entry. Only the first value is
included in this struct. If a header name has more than one associated
value, all extra values are stored in the `extra_values` vector. A doubly
linked list of entries is maintained. The doubly linked list is used so that
removing a value is constant time. This also has the nice property of
enabling double ended iteration.

```rust
pub(in ::header::map) struct Bucket<T> {
    pub(in ::header::map) hash: HashValue,
    pub(in ::header::map) key: super::name::HeaderName,
    pub(in ::header::map) value: T,
    pub(in ::header::map) links: Option<Links>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `hash` | `HashValue` |  |
| `key` | `super::name::HeaderName` |  |
| `value` | `T` |  |
| `links` | `Option<Links>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Bucket<T> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
#### Struct `Links`

The head and tail of the value linked list.

```rust
pub(in ::header::map) struct Links {
    pub(in ::header::map) next: usize,
    pub(in ::header::map) tail: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next` | `usize` |  |
| `tail` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Links { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `ExtraValue`

Node in doubly-linked list of header value entries

```rust
pub(in ::header::map) struct ExtraValue<T> {
    pub(in ::header::map) value: T,
    pub(in ::header::map) prev: Link,
    pub(in ::header::map) next: Link,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `value` | `T` |  |
| `prev` | `Link` |  |
| `next` | `Link` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExtraValue<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `Link`

A header value node is either linked to another node in the `extra_values`
list or it points to an entry in `entries`. The entry in `entries` is the
start of the list and holds the associated header name.

```rust
pub(in ::header::map) enum Link {
    Entry(usize),
    Extra(usize),
}
```

##### Variants

###### `Entry`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Extra`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
- **Eq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Link) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Link { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `Danger`

Tracks the header map danger level! This relates to the adaptive hashing
algorithm. A HeaderMap starts in the "green" state, when a large number of
collisions are detected, it transitions to the yellow state. At this point,
the header map will either grow and switch back to the green state OR it
will transition to the red state.

When in the red state, a safe hashing algorithm is used and all values in
the header map have to be rehashed.

```rust
pub(in ::header::map) enum Danger {
    Green,
    Yellow,
    Red(std::collections::hash_map::RandomState),
}
```

##### Variants

###### `Green`

###### `Yellow`

###### `Red`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::collections::hash_map::RandomState` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::header::map) fn is_red(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn to_red(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn is_yellow(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn to_yellow(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(in ::header::map) fn to_green(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Danger { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

### Functions

#### Function `do_insert_phase_two`

**Attributes:**

- `#[inline]`

phase 2 is post-insert where we forward-shift `Pos` in the indices.

returns the number of displaced elements

```rust
pub(in ::header::map) fn do_insert_phase_two(indices: &mut [Pos], probe: usize, old_pos: Pos) -> usize { /* ... */ }
```

#### Function `append_value`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::map) fn append_value<T>(entry_idx: usize, entry: &mut Bucket<T>, extra: &mut Vec<ExtraValue<T>>, value: T) { /* ... */ }
```

#### Function `usable_capacity`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::map) fn usable_capacity(cap: usize) -> usize { /* ... */ }
```

#### Function `to_raw_capacity`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::map) fn to_raw_capacity(n: usize) -> usize { /* ... */ }
```

#### Function `desired_pos`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::map) fn desired_pos(mask: usize, hash: HashValue) -> usize { /* ... */ }
```

#### Function `probe_distance`

**Attributes:**

- `#[inline]`

The number of steps that `current` is forward of the desired position for hash

```rust
pub(in ::header::map) fn probe_distance(mask: usize, hash: HashValue, current: usize) -> usize { /* ... */ }
```

#### Function `hash_elem_using`

```rust
pub(in ::header::map) fn hash_elem_using<K>(danger: &Danger, k: &K) -> HashValue
where
    K: Hash + ?Sized { /* ... */ }
```

### Constants and Statics

#### Constant `MAX_SIZE`

This limit falls out from above.

```rust
pub(in ::header::map) const MAX_SIZE: usize = _;
```

#### Constant `DISPLACEMENT_THRESHOLD`

```rust
pub(in ::header::map) const DISPLACEMENT_THRESHOLD: usize = 128;
```

#### Constant `FORWARD_SHIFT_THRESHOLD`

```rust
pub(in ::header::map) const FORWARD_SHIFT_THRESHOLD: usize = 512;
```

#### Constant `LOAD_FACTOR_THRESHOLD`

```rust
pub(in ::header::map) const LOAD_FACTOR_THRESHOLD: f32 = 0.2;
```

### Macros

#### Macro `probe_loop`

```rust
pub(crate) macro_rules! probe_loop {
    /* macro_rules! probe_loop {
    ($label:tt: $probe_var: ident < $len: expr, $body: expr) => { ... };
    ($probe_var: ident < $len: expr, $body: expr) => { ... };
} */
}
```

#### Macro `insert_phase_one`

```rust
pub(crate) macro_rules! insert_phase_one {
    /* macro_rules! insert_phase_one {
    ($map:ident,
     $key:expr,
     $probe:ident,
     $pos:ident,
     $hash:ident,
     $danger:ident,
     $vacant:expr,
     $occupied:expr,
     $robinhood:expr) => { ... };
} */
}
```

### Re-exports

#### Re-export `AsHeaderName`

```rust
pub use self::as_header_name::AsHeaderName;
```

#### Re-export `IntoHeaderName`

```rust
pub use self::into_header_name::IntoHeaderName;
```

## Module `name`

```rust
pub(in ::header) mod name { /* ... */ }
```

### Types

#### Struct `HeaderName`

Represents an HTTP header field name

Header field names identify the header. Header sets may include multiple
headers with the same name. The HTTP specification defines a number of
standard headers, but HTTP messages may include non-standard header names as
well as long as they adhere to the specification.

`HeaderName` is used as the [`HeaderMap`] key. Constants are available for
all standard header names in the [`header`] module.

# Representation

`HeaderName` represents standard header names using an `enum`, as such they
will not require an allocation for storage. All custom header names are
lower cased upon conversion to a `HeaderName` value. This avoids the
overhead of dynamically doing lower case conversion during the hash code
computation and the comparison operation.

[`HeaderMap`]: struct.HeaderMap.html
[`header`]: index.html

```rust
pub struct HeaderName {
    pub(in ::header::name) inner: Repr<Custom>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Repr<Custom>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> { /* ... */ }
  ```
  Converts a slice of bytes to an HTTP header name.

- ```rust
  pub fn from_lowercase(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> { /* ... */ }
  ```
  Converts a slice of bytes to an HTTP header name.

- ```rust
  pub fn from_static(src: &''static str) -> HeaderName { /* ... */ }
  ```
  Converts a static string to a HTTP header name.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns a `str` representation of the header.

###### Trait Implementations

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HeaderName) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a HeaderName) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderName) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```
    Performs a case-insensitive comparison of the string against the header

  - ```rust
    fn eq(self: &Self, other: &HeaderName) -> bool { /* ... */ }
    ```
    Performs a case-insensitive comparison of the string against the header

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```
    Performs a case-insensitive comparison of the string against the header

  - ```rust
    fn eq(self: &Self, other: &HeaderName) -> bool { /* ... */ }
    ```
    Performs a case-insensitive comparison of the string against the header

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sealed**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **AsHeaderName**
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
    fn from(src: &''a HeaderName) -> HeaderName { /* ... */ }
    ```

  - ```rust
    fn from(name: HeaderName) -> Bytes { /* ... */ }
    ```

  - ```rust
    fn from(h: HeaderName) -> HeaderValue { /* ... */ }
    ```

- **IntoHeaderName**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HeaderName { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<HeaderName, InvalidHeaderName> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &str { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **HttpTryFrom**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

#### Struct `HdrName`

```rust
pub struct HdrName<''a> {
    pub(in ::header::name) inner: Repr<MaybeLower<''a>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Repr<MaybeLower<''a>>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::header::name) fn custom(buf: &''a [u8], lower: bool) -> HdrName<''a> { /* ... */ }
  ```

- ```rust
  pub fn from_bytes<F, U>(hdr: &[u8], f: F) -> Result<U, InvalidHeaderName>
where
    F: FnOnce(HdrName<''_>) -> U { /* ... */ }
  ```

- ```rust
  pub fn from_static<F, U>(hdr: &''static str, f: F) -> U
where
    F: FnOnce(HdrName<''_>) -> U { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(hdr: StandardHeader) -> HdrName<''a> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `Repr`

```rust
pub(in ::header::name) enum Repr<T> {
    Standard(StandardHeader),
    Custom(T),
}
```

##### Variants

###### `Standard`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `StandardHeader` |  |

###### `Custom`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **StructuralPartialEq**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Repr<T> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Repr<T>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `Custom`

```rust
pub(in ::header::name) struct Custom(pub(in ::header::name) byte_str::ByteStr);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `byte_str::ByteStr` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Custom) -> bool { /* ... */ }
    ```

- **Sync**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Custom { /* ... */ }
    ```

- **StructuralPartialEq**
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
    fn from(Custom: Custom) -> Bytes { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
#### Struct `MaybeLower`

```rust
pub(in ::header::name) struct MaybeLower<''a> {
    pub(in ::header::name) buf: &''a [u8],
    pub(in ::header::name) lower: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |
| `lower` | `bool` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MaybeLower<''a> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, hasher: &mut H) { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `InvalidHeaderName`

A possible error when converting a `HeaderName` from another type.

```rust
pub struct InvalidHeaderName {
    pub(in ::header::name) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::header::name) fn new() -> InvalidHeaderName { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

  - ```rust
    fn from(err: header::InvalidHeaderName) -> Error { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `InvalidHeaderNameBytes`

A possible error when converting a `HeaderName` from another type.

```rust
pub struct InvalidHeaderNameBytes(pub(in ::header::name) InvalidHeaderName);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InvalidHeaderName` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: header::InvalidHeaderNameBytes) -> Error { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `StandardHeader`

```rust
pub(in ::header::name) enum StandardHeader {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    AcceptRanges,
    AccessControlAllowCredentials,
    AccessControlAllowHeaders,
    AccessControlAllowMethods,
    AccessControlAllowOrigin,
    AccessControlExposeHeaders,
    AccessControlMaxAge,
    AccessControlRequestHeaders,
    AccessControlRequestMethod,
    Age,
    Allow,
    AltSvc,
    Authorization,
    CacheControl,
    Connection,
    ContentDisposition,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentRange,
    ContentSecurityPolicy,
    ContentSecurityPolicyReportOnly,
    ContentType,
    Cookie,
    Dnt,
    Date,
    Etag,
    Expect,
    Expires,
    Forwarded,
    From,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    LastModified,
    Link,
    Location,
    MaxForwards,
    Origin,
    Pragma,
    ProxyAuthenticate,
    ProxyAuthorization,
    PublicKeyPins,
    PublicKeyPinsReportOnly,
    Range,
    Referer,
    ReferrerPolicy,
    Refresh,
    RetryAfter,
    SecWebSocketAccept,
    SecWebSocketExtensions,
    SecWebSocketKey,
    SecWebSocketProtocol,
    SecWebSocketVersion,
    Server,
    SetCookie,
    StrictTransportSecurity,
    Te,
    Trailer,
    TransferEncoding,
    UserAgent,
    Upgrade,
    UpgradeInsecureRequests,
    Vary,
    Via,
    Warning,
    WwwAuthenticate,
    XContentTypeOptions,
    XDnsPrefetchControl,
    XFrameOptions,
    XXssProtection,
}
```

##### Variants

###### `Accept`

###### `AcceptCharset`

###### `AcceptEncoding`

###### `AcceptLanguage`

###### `AcceptRanges`

###### `AccessControlAllowCredentials`

###### `AccessControlAllowHeaders`

###### `AccessControlAllowMethods`

###### `AccessControlAllowOrigin`

###### `AccessControlExposeHeaders`

###### `AccessControlMaxAge`

###### `AccessControlRequestHeaders`

###### `AccessControlRequestMethod`

###### `Age`

###### `Allow`

###### `AltSvc`

###### `Authorization`

###### `CacheControl`

###### `Connection`

###### `ContentDisposition`

###### `ContentEncoding`

###### `ContentLanguage`

###### `ContentLength`

###### `ContentLocation`

###### `ContentRange`

###### `ContentSecurityPolicy`

###### `ContentSecurityPolicyReportOnly`

###### `ContentType`

###### `Cookie`

###### `Dnt`

###### `Date`

###### `Etag`

###### `Expect`

###### `Expires`

###### `Forwarded`

###### `From`

###### `Host`

###### `IfMatch`

###### `IfModifiedSince`

###### `IfNoneMatch`

###### `IfRange`

###### `IfUnmodifiedSince`

###### `LastModified`

###### `Link`

###### `Location`

###### `MaxForwards`

###### `Origin`

###### `Pragma`

###### `ProxyAuthenticate`

###### `ProxyAuthorization`

###### `PublicKeyPins`

###### `PublicKeyPinsReportOnly`

###### `Range`

###### `Referer`

###### `ReferrerPolicy`

###### `Refresh`

###### `RetryAfter`

###### `SecWebSocketAccept`

###### `SecWebSocketExtensions`

###### `SecWebSocketKey`

###### `SecWebSocketProtocol`

###### `SecWebSocketVersion`

###### `Server`

###### `SetCookie`

###### `StrictTransportSecurity`

###### `Te`

###### `Trailer`

###### `TransferEncoding`

###### `UserAgent`

###### `Upgrade`

###### `UpgradeInsecureRequests`

###### `Vary`

###### `Via`

###### `Warning`

###### `WwwAuthenticate`

###### `XContentTypeOptions`

###### `XDnsPrefetchControl`

###### `XFrameOptions`

###### `XXssProtection`

##### Implementations

###### Methods

- ```rust
  pub(in ::header::name) fn as_str(self: &Self) -> &''static str { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StandardHeader) -> bool { /* ... */ }
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

  - ```rust
    fn from(hdr: StandardHeader) -> HdrName<''a> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StandardHeader { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `parse_hdr`

**Attributes:**

- `#[cfg(any(not(debug_assertions), not(target_arch = "wasm32")))]`

This version is best under optimized mode, however in a wasm debug compile,
the `eq` macro expands to 1 + 1 + 1 + 1... and wasm explodes when this chain gets too long
See https://github.com/DenisKolodin/yew/issues/478

```rust
pub(in ::header::name) fn parse_hdr<''a>(data: &''a [u8], b: &''a mut [u8; 64], table: &[u8; 256]) -> Result<HdrName<''a>, InvalidHeaderName> { /* ... */ }
```

#### Function `eq_ignore_ascii_case`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::name) fn eq_ignore_ascii_case(lower: &[u8], s: &[u8]) -> bool { /* ... */ }
```

### Constants and Statics

#### Constant `ACCEPT`

Advertises which content types the client is able to understand.

The Accept request HTTP header advertises which content types, expressed
as MIME types, the client is able to understand. Using content
negotiation, the server then selects one of the proposals, uses it and
informs the client of its choice with the Content-Type response header.
Browsers set adequate values for this header depending of the context
where the request is done: when fetching a CSS stylesheet a different
value is set for the request than when fetching an image, video or a
script.

```rust
pub const ACCEPT: HeaderName = _;
```

#### Constant `ACCEPT_CHARSET`

Advertises which character set the client is able to understand.

The Accept-Charset request HTTP header advertises which character set
the client is able to understand. Using content negotiation, the server
then selects one of the proposals, uses it and informs the client of its
choice within the Content-Type response header. Browsers usually don't
set this header as the default value for each content type is usually
correct and transmitting it would allow easier fingerprinting.

If the server cannot serve any matching character set, it can
theoretically send back a 406 (Not Acceptable) error code. But, for a
better user experience, this is rarely done and the more common way is
to ignore the Accept-Charset header in this case.

```rust
pub const ACCEPT_CHARSET: HeaderName = _;
```

#### Constant `ACCEPT_ENCODING`

Advertises which content encoding the client is able to understand.

The Accept-Encoding request HTTP header advertises which content
encoding, usually a compression algorithm, the client is able to
understand. Using content negotiation, the server selects one of the
proposals, uses it and informs the client of its choice with the
Content-Encoding response header.

Even if both the client and the server supports the same compression
algorithms, the server may choose not to compress the body of a
response, if the identity value is also acceptable. Two common cases
lead to this:

* The data to be sent is already compressed and a second compression
won't lead to smaller data to be transmitted. This may the case with
some image formats;

* The server is overloaded and cannot afford the computational overhead
induced by the compression requirement. Typically, Microsoft recommends
not to compress if a server use more than 80 % of its computational
power.

As long as the identity value, meaning no encryption, is not explicitly
forbidden, by an identity;q=0 or a *;q=0 without another explicitly set
value for identity, the server must never send back a 406 Not Acceptable
error.

```rust
pub const ACCEPT_ENCODING: HeaderName = _;
```

#### Constant `ACCEPT_LANGUAGE`

Advertises which languages the client is able to understand.

The Accept-Language request HTTP header advertises which languages the
client is able to understand, and which locale variant is preferred.
Using content negotiation, the server then selects one of the proposals,
uses it and informs the client of its choice with the Content-Language
response header. Browsers set adequate values for this header according
their user interface language and even if a user can change it, this
happens rarely (and is frown upon as it leads to fingerprinting).

This header is a hint to be used when the server has no way of
determining the language via another way, like a specific URL, that is
controlled by an explicit user decision. It is recommended that the
server never overrides an explicit decision. The content of the
Accept-Language is often out of the control of the user (like when
traveling and using an Internet Cafe in a different country); the user
may also want to visit a page in another language than the locale of
their user interface.

If the server cannot serve any matching language, it can theoretically
send back a 406 (Not Acceptable) error code. But, for a better user
experience, this is rarely done and more common way is to ignore the
Accept-Language header in this case.

```rust
pub const ACCEPT_LANGUAGE: HeaderName = _;
```

#### Constant `ACCEPT_RANGES`

Marker used by the server to advertise partial request support.

The Accept-Ranges response HTTP header is a marker used by the server to
advertise its support of partial requests. The value of this field
indicates the unit that can be used to define a range.

In presence of an Accept-Ranges header, the browser may try to resume an
interrupted download, rather than to start it from the start again.

```rust
pub const ACCEPT_RANGES: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_ALLOW_CREDENTIALS`

Preflight response indicating if the response to the request can be
exposed to the page.

The Access-Control-Allow-Credentials response header indicates whether
or not the response to the request can be exposed to the page. It can be
exposed when the true value is returned; it can't in other cases.

Credentials are cookies, authorization headers or TLS client
certificates.

When used as part of a response to a preflight request, this indicates
whether or not the actual request can be made using credentials. Note
that simple GET requests are not preflighted, and so if a request is
made for a resource with credentials, if this header is not returned
with the resource, the response is ignored by the browser and not
returned to web content.

The Access-Control-Allow-Credentials header works in conjunction with
the XMLHttpRequest.withCredentials property or with the credentials
option in the Request() constructor of the Fetch API. Credentials must
be set on both sides (the Access-Control-Allow-Credentials header and in
the XHR or Fetch request) in order for the CORS request with credentials
to succeed.

```rust
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_ALLOW_HEADERS`

Preflight response indicating permitted HTTP headers.

The Access-Control-Allow-Headers response header is used in response to
a preflight request to indicate which HTTP headers will be available via
Access-Control-Expose-Headers when making the actual request.

The simple headers, Accept, Accept-Language, Content-Language,
Content-Type (but only with a MIME type of its parsed value (ignoring
parameters) of either application/x-www-form-urlencoded,
multipart/form-data, or text/plain), are always available and don't need
to be listed by this header.

This header is required if the request has an
Access-Control-Request-Headers header.

```rust
pub const ACCESS_CONTROL_ALLOW_HEADERS: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_ALLOW_METHODS`

Preflight header response indicating permitted access methods.

The Access-Control-Allow-Methods response header specifies the method or
methods allowed when accessing the resource in response to a preflight
request.

```rust
pub const ACCESS_CONTROL_ALLOW_METHODS: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_ALLOW_ORIGIN`

Indicates whether the response can be shared with resources with the
given origin.

```rust
pub const ACCESS_CONTROL_ALLOW_ORIGIN: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_EXPOSE_HEADERS`

Indicates which headers can be exposed as part of the response by
listing their names.

```rust
pub const ACCESS_CONTROL_EXPOSE_HEADERS: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_MAX_AGE`

Indicates how long the results of a preflight request can be cached.

```rust
pub const ACCESS_CONTROL_MAX_AGE: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_REQUEST_HEADERS`

Informs the server which HTTP headers will be used when an actual
request is made.

```rust
pub const ACCESS_CONTROL_REQUEST_HEADERS: HeaderName = _;
```

#### Constant `ACCESS_CONTROL_REQUEST_METHOD`

Informs the server know which HTTP method will be used when the actual
request is made.

```rust
pub const ACCESS_CONTROL_REQUEST_METHOD: HeaderName = _;
```

#### Constant `AGE`

Indicates the time in seconds the object has been in a proxy cache.

The Age header is usually close to zero. If it is Age: 0, it was
probably just fetched from the origin server; otherwise It is usually
calculated as a difference between the proxy's current date and the Date
general header included in the HTTP response.

```rust
pub const AGE: HeaderName = _;
```

#### Constant `ALLOW`

Lists the set of methods support by a resource.

This header must be sent if the server responds with a 405 Method Not
Allowed status code to indicate which request methods can be used. An
empty Allow header indicates that the resource allows no request
methods, which might occur temporarily for a given resource, for
example.

```rust
pub const ALLOW: HeaderName = _;
```

#### Constant `ALT_SVC`

Advertises the availability of alternate services to clients.

```rust
pub const ALT_SVC: HeaderName = _;
```

#### Constant `AUTHORIZATION`

Contains the credentials to authenticate a user agent with a server.

Usually this header is included after the server has responded with a
401 Unauthorized status and the WWW-Authenticate header.

```rust
pub const AUTHORIZATION: HeaderName = _;
```

#### Constant `CACHE_CONTROL`

Specifies directives for caching mechanisms in both requests and
responses.

Caching directives are unidirectional, meaning that a given directive in
a request is not implying that the same directive is to be given in the
response.

```rust
pub const CACHE_CONTROL: HeaderName = _;
```

#### Constant `CONNECTION`

Controls whether or not the network connection stays open after the
current transaction finishes.

If the value sent is keep-alive, the connection is persistent and not
closed, allowing for subsequent requests to the same server to be done.

Except for the standard hop-by-hop headers (Keep-Alive,
Transfer-Encoding, TE, Connection, Trailer, Upgrade, Proxy-Authorization
and Proxy-Authenticate), any hop-by-hop headers used by the message must
be listed in the Connection header, so that the first proxy knows he has
to consume them and not to forward them further. Standard hop-by-hop
headers can be listed too (it is often the case of Keep-Alive, but this
is not mandatory.

```rust
pub const CONNECTION: HeaderName = _;
```

#### Constant `CONTENT_DISPOSITION`

Indicates if the content is expected to be displayed inline.

In a regular HTTP response, the Content-Disposition response header is a
header indicating if the content is expected to be displayed inline in
the browser, that is, as a Web page or as part of a Web page, or as an
attachment, that is downloaded and saved locally.

In a multipart/form-data body, the HTTP Content-Disposition general
header is a header that can be used on the subpart of a multipart body
to give information about the field it applies to. The subpart is
delimited by the boundary defined in the Content-Type header. Used on
the body itself, Content-Disposition has no effect.

The Content-Disposition header is defined in the larger context of MIME
messages for e-mail, but only a subset of the possible parameters apply
to HTTP forms and POST requests. Only the value form-data, as well as
the optional directive name and filename, can be used in the HTTP
context.

```rust
pub const CONTENT_DISPOSITION: HeaderName = _;
```

#### Constant `CONTENT_ENCODING`

Used to compress the media-type.

When present, its value indicates what additional content encoding has
been applied to the entity-body. It lets the client know, how to decode
in order to obtain the media-type referenced by the Content-Type header.

It is recommended to compress data as much as possible and therefore to
use this field, but some types of resources, like jpeg images, are
already compressed.  Sometimes using additional compression doesn't
reduce payload size and can even make the payload longer.

```rust
pub const CONTENT_ENCODING: HeaderName = _;
```

#### Constant `CONTENT_LANGUAGE`

Used to describe the languages intended for the audience.

This header allows a user to differentiate according to the users' own
preferred language. For example, if "Content-Language: de-DE" is set, it
says that the document is intended for German language speakers
(however, it doesn't indicate the document is written in German. For
example, it might be written in English as part of a language course for
German speakers).

If no Content-Language is specified, the default is that the content is
intended for all language audiences. Multiple language tags are also
possible, as well as applying the Content-Language header to various
media types and not only to textual documents.

```rust
pub const CONTENT_LANGUAGE: HeaderName = _;
```

#### Constant `CONTENT_LENGTH`

Indicates the size fo the entity-body.

The header value must be a decimal indicating the number of octets sent
to the recipient.

```rust
pub const CONTENT_LENGTH: HeaderName = _;
```

#### Constant `CONTENT_LOCATION`

Indicates an alternate location for the returned data.

The principal use case is to indicate the URL of the resource
transmitted as the result of content negotiation.

Location and Content-Location are different: Location indicates the
target of a redirection (or the URL of a newly created document), while
Content-Location indicates the direct URL to use to access the resource,
without the need of further content negotiation. Location is a header
associated with the response, while Content-Location is associated with
the entity returned.

```rust
pub const CONTENT_LOCATION: HeaderName = _;
```

#### Constant `CONTENT_RANGE`

Indicates where in a full body message a partial message belongs.

```rust
pub const CONTENT_RANGE: HeaderName = _;
```

#### Constant `CONTENT_SECURITY_POLICY`

Allows controlling resources the user agent is allowed to load for a
given page.

With a few exceptions, policies mostly involve specifying server origins
and script endpoints. This helps guard against cross-site scripting
attacks (XSS).

```rust
pub const CONTENT_SECURITY_POLICY: HeaderName = _;
```

#### Constant `CONTENT_SECURITY_POLICY_REPORT_ONLY`

Allows experimenting with policies by monitoring their effects.

The HTTP Content-Security-Policy-Report-Only response header allows web
developers to experiment with policies by monitoring (but not enforcing)
their effects. These violation reports consist of JSON documents sent
via an HTTP POST request to the specified URI.

```rust
pub const CONTENT_SECURITY_POLICY_REPORT_ONLY: HeaderName = _;
```

#### Constant `CONTENT_TYPE`

Used to indicate the media type of the resource.

In responses, a Content-Type header tells the client what the content
type of the returned content actually is. Browsers will do MIME sniffing
in some cases and will not necessarily follow the value of this header;
to prevent this behavior, the header X-Content-Type-Options can be set
to nosniff.

In requests, (such as POST or PUT), the client tells the server what
type of data is actually sent.

```rust
pub const CONTENT_TYPE: HeaderName = _;
```

#### Constant `COOKIE`

Contains stored HTTP cookies previously sent by the server with the
Set-Cookie header.

The Cookie header might be omitted entirely, if the privacy setting of
the browser are set to block them, for example.

```rust
pub const COOKIE: HeaderName = _;
```

#### Constant `DNT`

Indicates the client's tracking preference.

This header lets users indicate whether they would prefer privacy rather
than personalized content.

```rust
pub const DNT: HeaderName = _;
```

#### Constant `DATE`

Contains the date and time at which the message was originated.

```rust
pub const DATE: HeaderName = _;
```

#### Constant `ETAG`

Identifier for a specific version of a resource.

This header allows caches to be more efficient, and saves bandwidth, as
a web server does not need to send a full response if the content has
not changed. On the other side, if the content has changed, etags are
useful to help prevent simultaneous updates of a resource from
overwriting each other ("mid-air collisions").

If the resource at a given URL changes, a new Etag value must be
generated. Etags are therefore similar to fingerprints and might also be
used for tracking purposes by some servers. A comparison of them allows
to quickly determine whether two representations of a resource are the
same, but they might also be set to persist indefinitely by a tracking
server.

```rust
pub const ETAG: HeaderName = _;
```

#### Constant `EXPECT`

Indicates expectations that need to be fulfilled by the server in order
to properly handle the request.

The only expectation defined in the specification is Expect:
100-continue, to which the server shall respond with:

* 100 if the information contained in the header is sufficient to cause
an immediate success,

* 417 (Expectation Failed) if it cannot meet the expectation; or any
other 4xx status otherwise.

For example, the server may reject a request if its Content-Length is
too large.

No common browsers send the Expect header, but some other clients such
as cURL do so by default.

```rust
pub const EXPECT: HeaderName = _;
```

#### Constant `EXPIRES`

Contains the date/time after which the response is considered stale.

Invalid dates, like the value 0, represent a date in the past and mean
that the resource is already expired.

If there is a Cache-Control header with the "max-age" or "s-max-age"
directive in the response, the Expires header is ignored.

```rust
pub const EXPIRES: HeaderName = _;
```

#### Constant `FORWARDED`

Contains information from the client-facing side of proxy servers that
is altered or lost when a proxy is involved in the path of the request.

The alternative and de-facto standard versions of this header are the
X-Forwarded-For, X-Forwarded-Host and X-Forwarded-Proto headers.

This header is used for debugging, statistics, and generating
location-dependent content and by design it exposes privacy sensitive
information, such as the IP address of the client. Therefore the user's
privacy must be kept in mind when deploying this header.

```rust
pub const FORWARDED: HeaderName = _;
```

#### Constant `FROM`

Contains an Internet email address for a human user who controls the
requesting user agent.

If you are running a robotic user agent (e.g. a crawler), the From
header should be sent, so you can be contacted if problems occur on
servers, such as if the robot is sending excessive, unwanted, or invalid
requests.

```rust
pub const FROM: HeaderName = _;
```

#### Constant `HOST`

Specifies the domain name of the server and (optionally) the TCP port
number on which the server is listening.

If no port is given, the default port for the service requested (e.g.,
"80" for an HTTP URL) is implied.

A Host header field must be sent in all HTTP/1.1 request messages. A 400
(Bad Request) status code will be sent to any HTTP/1.1 request message
that lacks a Host header field or contains more than one.

```rust
pub const HOST: HeaderName = _;
```

#### Constant `IF_MATCH`

Makes a request conditional based on the E-Tag.

For GET and HEAD methods, the server will send back the requested
resource only if it matches one of the listed ETags. For PUT and other
non-safe methods, it will only upload the resource in this case.

The comparison with the stored ETag uses the strong comparison
algorithm, meaning two files are considered identical byte to byte only.
This is weakened when the  W/ prefix is used in front of the ETag.

There are two common use cases:

* For GET and HEAD methods, used in combination with an Range header, it
can guarantee that the new ranges requested comes from the same resource
than the previous one. If it doesn't match, then a 416 (Range Not
Satisfiable) response is returned.

* For other methods, and in particular for PUT, If-Match can be used to
prevent the lost update problem. It can check if the modification of a
resource that the user wants to upload will not override another change
that has been done since the original resource was fetched. If the
request cannot be fulfilled, the 412 (Precondition Failed) response is
returned.

```rust
pub const IF_MATCH: HeaderName = _;
```

#### Constant `IF_MODIFIED_SINCE`

Makes a request conditional based on the modification date.

The If-Modified-Since request HTTP header makes the request conditional:
the server will send back the requested resource, with a 200 status,
only if it has been last modified after the given date. If the request
has not been modified since, the response will be a 304 without any
body; the Last-Modified header will contain the date of last
modification. Unlike If-Unmodified-Since, If-Modified-Since can only be
used with a GET or HEAD.

When used in combination with If-None-Match, it is ignored, unless the
server doesn't support If-None-Match.

The most common use case is to update a cached entity that has no
associated ETag.

```rust
pub const IF_MODIFIED_SINCE: HeaderName = _;
```

#### Constant `IF_NONE_MATCH`

Makes a request conditional based on the E-Tag.

The If-None-Match HTTP request header makes the request conditional. For
GET and HEAD methods, the server will send back the requested resource,
with a 200 status, only if it doesn't have an ETag matching the given
ones. For other methods, the request will be processed only if the
eventually existing resource's ETag doesn't match any of the values
listed.

When the condition fails for GET and HEAD methods, then the server must
return HTTP status code 304 (Not Modified). For methods that apply
server-side changes, the status code 412 (Precondition Failed) is used.
Note that the server generating a 304 response MUST generate any of the
following header fields that would have been sent in a 200 (OK) response
to the same request: Cache-Control, Content-Location, Date, ETag,
Expires, and Vary.

The comparison with the stored ETag uses the weak comparison algorithm,
meaning two files are considered identical not only if they are
identical byte to byte, but if the content is equivalent. For example,
two pages that would differ only by the date of generation in the footer
would be considered as identical.

When used in combination with If-Modified-Since, it has precedence (if
the server supports it).

There are two common use cases:

* For `GET` and `HEAD` methods, to update a cached entity that has an associated ETag.
* For other methods, and in particular for `PUT`, `If-None-Match` used with
the `*` value can be used to save a file not known to exist,
guaranteeing that another upload didn't happen before, losing the data
of the previous put; this problems is the variation of the lost update
problem.

```rust
pub const IF_NONE_MATCH: HeaderName = _;
```

#### Constant `IF_RANGE`

Makes a request conditional based on range.

The If-Range HTTP request header makes a range request conditional: if
the condition is fulfilled, the range request will be issued and the
server sends back a 206 Partial Content answer with the appropriate
body. If the condition is not fulfilled, the full resource is sent back,
with a 200 OK status.

This header can be used either with a Last-Modified validator, or with
an ETag, but not with both.

The most common use case is to resume a download, to guarantee that the
stored resource has not been modified since the last fragment has been
received.

```rust
pub const IF_RANGE: HeaderName = _;
```

#### Constant `IF_UNMODIFIED_SINCE`

Makes the request conditional based on the last modification date.

The If-Unmodified-Since request HTTP header makes the request
conditional: the server will send back the requested resource, or accept
it in the case of a POST or another non-safe method, only if it has not
been last modified after the given date. If the request has been
modified after the given date, the response will be a 412 (Precondition
Failed) error.

There are two common use cases:

* In conjunction non-safe methods, like POST, it can be used to
implement an optimistic concurrency control, like done by some wikis:
editions are rejected if the stored document has been modified since the
original has been retrieved.

* In conjunction with a range request with a If-Range header, it can be
used to ensure that the new fragment requested comes from an unmodified
document.

```rust
pub const IF_UNMODIFIED_SINCE: HeaderName = _;
```

#### Constant `LAST_MODIFIED`

Content-Types that are acceptable for the response.

```rust
pub const LAST_MODIFIED: HeaderName = _;
```

#### Constant `LINK`

Allows the server to point an interested client to another resource
containing metadata about the requested resource.

```rust
pub const LINK: HeaderName = _;
```

#### Constant `LOCATION`

Indicates the URL to redirect a page to.

The Location response header indicates the URL to redirect a page to. It
only provides a meaning when served with a 3xx status response.

The HTTP method used to make the new request to fetch the page pointed
to by Location depends of the original method and of the kind of
redirection:

* If 303 (See Also) responses always lead to the use of a GET method,
307 (Temporary Redirect) and 308 (Permanent Redirect) don't change the
method used in the original request;

* 301 (Permanent Redirect) and 302 (Found) doesn't change the method
most of the time, though older user-agents may (so you basically don't
know).

All responses with one of these status codes send a Location header.

Beside redirect response, messages with 201 (Created) status also
include the Location header. It indicates the URL to the newly created
resource.

Location and Content-Location are different: Location indicates the
target of a redirection (or the URL of a newly created resource), while
Content-Location indicates the direct URL to use to access the resource
when content negotiation happened, without the need of further content
negotiation. Location is a header associated with the response, while
Content-Location is associated with the entity returned.

```rust
pub const LOCATION: HeaderName = _;
```

#### Constant `MAX_FORWARDS`

Indicates the max number of intermediaries the request should be sent
through.

```rust
pub const MAX_FORWARDS: HeaderName = _;
```

#### Constant `ORIGIN`

Indicates where a fetch originates from.

It doesn't include any path information, but only the server name. It is
sent with CORS requests, as well as with POST requests. It is similar to
the Referer header, but, unlike this header, it doesn't disclose the
whole path.

```rust
pub const ORIGIN: HeaderName = _;
```

#### Constant `PRAGMA`

HTTP/1.0 header usually used for backwards compatibility.

The Pragma HTTP/1.0 general header is an implementation-specific header
that may have various effects along the request-response chain. It is
used for backwards compatibility with HTTP/1.0 caches where the
Cache-Control HTTP/1.1 header is not yet present.

```rust
pub const PRAGMA: HeaderName = _;
```

#### Constant `PROXY_AUTHENTICATE`

Defines the authentication method that should be used to gain access to
a proxy.

Unlike `www-authenticate`, the `proxy-authenticate` header field applies
only to the next outbound client on the response chain. This is because
only the client that chose a given proxy is likely to have the
credentials necessary for authentication. However, when multiple proxies
are used within the same administrative domain, such as office and
regional caching proxies within a large corporate network, it is common
for credentials to be generated by the user agent and passed through the
hierarchy until consumed. Hence, in such a configuration, it will appear
as if Proxy-Authenticate is being forwarded because each proxy will send
the same challenge set.

The `proxy-authenticate` header is sent along with a `407 Proxy
Authentication Required`.

```rust
pub const PROXY_AUTHENTICATE: HeaderName = _;
```

#### Constant `PROXY_AUTHORIZATION`

Contains the credentials to authenticate a user agent to a proxy server.

This header is usually included after the server has responded with a
407 Proxy Authentication Required status and the Proxy-Authenticate
header.

```rust
pub const PROXY_AUTHORIZATION: HeaderName = _;
```

#### Constant `PUBLIC_KEY_PINS`

Associates a specific cryptographic public key with a certain server.

This decreases the risk of MITM attacks with forged certificates. If one
or several keys are pinned and none of them are used by the server, the
browser will not accept the response as legitimate, and will not display
it.

```rust
pub const PUBLIC_KEY_PINS: HeaderName = _;
```

#### Constant `PUBLIC_KEY_PINS_REPORT_ONLY`

Sends reports of pinning violation to the report-uri specified in the
header.

Unlike `Public-Key-Pins`, this header still allows browsers to connect
to the server if the pinning is violated.

```rust
pub const PUBLIC_KEY_PINS_REPORT_ONLY: HeaderName = _;
```

#### Constant `RANGE`

Indicates the part of a document that the server should return.

Several parts can be requested with one Range header at once, and the
server may send back these ranges in a multipart document. If the server
sends back ranges, it uses the 206 Partial Content for the response. If
the ranges are invalid, the server returns the 416 Range Not Satisfiable
error. The server can also ignore the Range header and return the whole
document with a 200 status code.

```rust
pub const RANGE: HeaderName = _;
```

#### Constant `REFERER`

Contains the address of the previous web page from which a link to the
currently requested page was followed.

The Referer header allows servers to identify where people are visiting
them from and may use that data for analytics, logging, or optimized
caching, for example.

```rust
pub const REFERER: HeaderName = _;
```

#### Constant `REFERRER_POLICY`

Governs which referrer information should be included with requests
made.

```rust
pub const REFERRER_POLICY: HeaderName = _;
```

#### Constant `REFRESH`

Informs the web browser that the current page or frame should be
refreshed.

```rust
pub const REFRESH: HeaderName = _;
```

#### Constant `RETRY_AFTER`

The Retry-After response HTTP header indicates how long the user agent
should wait before making a follow-up request. There are two main cases
this header is used:

* When sent with a 503 (Service Unavailable) response, it indicates how
long the service is expected to be unavailable.

* When sent with a redirect response, such as 301 (Moved Permanently),
it indicates the minimum time that the user agent is asked to wait
before issuing the redirected request.

```rust
pub const RETRY_AFTER: HeaderName = _;
```

#### Constant `SEC_WEBSOCKET_ACCEPT`

The |Sec-WebSocket-Accept| header field is used in the WebSocket
opening handshake. It is sent from the server to the client to
confirm that the server is willing to initiate the WebSocket
connection.

```rust
pub const SEC_WEBSOCKET_ACCEPT: HeaderName = _;
```

#### Constant `SEC_WEBSOCKET_EXTENSIONS`

The |Sec-WebSocket-Extensions| header field is used in the WebSocket
opening handshake. It is initially sent from the client to the
server, and then subsequently sent from the server to the client, to
agree on a set of protocol-level extensions to use for the duration
of the connection.

```rust
pub const SEC_WEBSOCKET_EXTENSIONS: HeaderName = _;
```

#### Constant `SEC_WEBSOCKET_KEY`

The |Sec-WebSocket-Key| header field is used in the WebSocket opening
handshake. It is sent from the client to the server to provide part
of the information used by the server to prove that it received a
valid WebSocket opening handshake. This helps ensure that the server
does not accept connections from non-WebSocket clients (e.g., HTTP
clients) that are being abused to send data to unsuspecting WebSocket
servers.

```rust
pub const SEC_WEBSOCKET_KEY: HeaderName = _;
```

#### Constant `SEC_WEBSOCKET_PROTOCOL`

The |Sec-WebSocket-Protocol| header field is used in the WebSocket
opening handshake. It is sent from the client to the server and back
from the server to the client to confirm the subprotocol of the
connection.  This enables scripts to both select a subprotocol and be
sure that the server agreed to serve that subprotocol.

```rust
pub const SEC_WEBSOCKET_PROTOCOL: HeaderName = _;
```

#### Constant `SEC_WEBSOCKET_VERSION`

The |Sec-WebSocket-Version| header field is used in the WebSocket
opening handshake.  It is sent from the client to the server to
indicate the protocol version of the connection.  This enables
servers to correctly interpret the opening handshake and subsequent
data being sent from the data, and close the connection if the server
cannot interpret that data in a safe manner.

```rust
pub const SEC_WEBSOCKET_VERSION: HeaderName = _;
```

#### Constant `SERVER`

Contains information about the software used by the origin server to
handle the request.

Overly long and detailed Server values should be avoided as they
potentially reveal internal implementation details that might make it
(slightly) easier for attackers to find and exploit known security
holes.

```rust
pub const SERVER: HeaderName = _;
```

#### Constant `SET_COOKIE`

Used to send cookies from the server to the user agent.

```rust
pub const SET_COOKIE: HeaderName = _;
```

#### Constant `STRICT_TRANSPORT_SECURITY`

Tells the client to communicate with HTTPS instead of using HTTP.

```rust
pub const STRICT_TRANSPORT_SECURITY: HeaderName = _;
```

#### Constant `TE`

Informs the server of transfer encodings willing to be accepted as part
of the response.

See also the Transfer-Encoding response header for more details on
transfer encodings. Note that chunked is always acceptable for HTTP/1.1
recipients and you that don't have to specify "chunked" using the TE
header. However, it is useful for setting if the client is accepting
trailer fields in a chunked transfer coding using the "trailers" value.

```rust
pub const TE: HeaderName = _;
```

#### Constant `TRAILER`

Allows the sender to include additional fields at the end of chunked
messages.

```rust
pub const TRAILER: HeaderName = _;
```

#### Constant `TRANSFER_ENCODING`

Specifies the form of encoding used to safely transfer the entity to the
client.

`transfer-encoding` is a hop-by-hop header, that is applying to a
message between two nodes, not to a resource itself. Each segment of a
multi-node connection can use different `transfer-encoding` values. If
you want to compress data over the whole connection, use the end-to-end
header `content-encoding` header instead.

When present on a response to a `HEAD` request that has no body, it
indicates the value that would have applied to the corresponding `GET`
message.

```rust
pub const TRANSFER_ENCODING: HeaderName = _;
```

#### Constant `USER_AGENT`

Contains a string that allows identifying the requesting client's
software.

```rust
pub const USER_AGENT: HeaderName = _;
```

#### Constant `UPGRADE`

Used as part of the exchange to upgrade the protocol.

```rust
pub const UPGRADE: HeaderName = _;
```

#### Constant `UPGRADE_INSECURE_REQUESTS`

Sends a signal to the server expressing the client’s preference for an
encrypted and authenticated response.

```rust
pub const UPGRADE_INSECURE_REQUESTS: HeaderName = _;
```

#### Constant `VARY`

Determines how to match future requests with cached responses.

The `vary` HTTP response header determines how to match future request
headers to decide whether a cached response can be used rather than
requesting a fresh one from the origin server. It is used by the server
to indicate which headers it used when selecting a representation of a
resource in a content negotiation algorithm.

The `vary` header should be set on a 304 Not Modified response exactly
like it would have been set on an equivalent 200 OK response.

```rust
pub const VARY: HeaderName = _;
```

#### Constant `VIA`

Added by proxies to track routing.

The `via` general header is added by proxies, both forward and reverse
proxies, and can appear in the request headers and the response headers.
It is used for tracking message forwards, avoiding request loops, and
identifying the protocol capabilities of senders along the
request/response chain.

```rust
pub const VIA: HeaderName = _;
```

#### Constant `WARNING`

General HTTP header contains information about possible problems with
the status of the message.

More than one `warning` header may appear in a response. Warning header
fields can in general be applied to any message, however some warn-codes
are specific to caches and can only be applied to response messages.

```rust
pub const WARNING: HeaderName = _;
```

#### Constant `WWW_AUTHENTICATE`

Defines the authentication method that should be used to gain access to
a resource.

```rust
pub const WWW_AUTHENTICATE: HeaderName = _;
```

#### Constant `X_CONTENT_TYPE_OPTIONS`

Marker used by the server to indicate that the MIME types advertised in
the `content-type` headers should not be changed and be followed.

This allows to opt-out of MIME type sniffing, or, in other words, it is
a way to say that the webmasters knew what they were doing.

This header was introduced by Microsoft in IE 8 as a way for webmasters
to block content sniffing that was happening and could transform
non-executable MIME types into executable MIME types. Since then, other
browsers have introduced it, even if their MIME sniffing algorithms were
less aggressive.

Site security testers usually expect this header to be set.

```rust
pub const X_CONTENT_TYPE_OPTIONS: HeaderName = _;
```

#### Constant `X_DNS_PREFETCH_CONTROL`

Controls DNS prefetching.

The `x-dns-prefetch-control` HTTP response header controls DNS
prefetching, a feature by which browsers proactively perform domain name
resolution on both links that the user may choose to follow as well as
URLs for items referenced by the document, including images, CSS,
JavaScript, and so forth.

This prefetching is performed in the background, so that the DNS is
likely to have been resolved by the time the referenced items are
needed. This reduces latency when the user clicks a link.

```rust
pub const X_DNS_PREFETCH_CONTROL: HeaderName = _;
```

#### Constant `X_FRAME_OPTIONS`

Indicates whether or not a browser should be allowed to render a page in
a frame.

Sites can use this to avoid clickjacking attacks, by ensuring that their
content is not embedded into other sites.

The added security is only provided if the user accessing the document
is using a browser supporting `x-frame-options`.

```rust
pub const X_FRAME_OPTIONS: HeaderName = _;
```

#### Constant `X_XSS_PROTECTION`

Stop pages from loading when an XSS attack is detected.

The HTTP X-XSS-Protection response header is a feature of Internet
Explorer, Chrome and Safari that stops pages from loading when they
detect reflected cross-site scripting (XSS) attacks. Although these
protections are largely unnecessary in modern browsers when sites
implement a strong Content-Security-Policy that disables the use of
inline JavaScript ('unsafe-inline'), they can still provide protections
for users of older web browsers that don't yet support CSP.

```rust
pub const X_XSS_PROTECTION: HeaderName = _;
```

#### Constant `HEADER_CHARS`

Valid header name characters

```not_rust
      field-name     = token
      separators     = "(" | ")" | "<" | ">" | "@"
                     | "," | ";" | ":" | "\" | <">
                     | "/" | "[" | "]" | "?" | "="
                     | "{" | "}" | SP | HT
      token          = 1*tchar
      tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
                     / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
                     / DIGIT / ALPHA
                     ; any VCHAR, except delimiters
```

```rust
pub(in ::header::name) const HEADER_CHARS: [u8; 256] = _;
```

#### Constant `HEADER_CHARS_H2`

```rust
pub(in ::header::name) const HEADER_CHARS_H2: [u8; 256] = _;
```

### Macros

#### Macro `standard_headers`

```rust
pub(crate) macro_rules! standard_headers {
    /* macro_rules! standard_headers {
    (
        $(
            $(#[$docs:meta])*
            ($konst:ident, $upcase:ident, $name:expr);
        )+
    ) => { ... };
} */
}
```

#### Macro `eq`

**Attributes:**

- `#[cfg(any(not(debug_assertions), not(target_arch = "wasm32")))]`

```rust
pub(crate) macro_rules! eq {
    /* macro_rules! eq {
    (($($cmp:expr,)*) $v:ident[$n:expr] ==) => { ... };
    (($($cmp:expr,)*) $v:ident[$n:expr] == $a:tt $($rest:tt)*) => { ... };
    ($v:ident == $($rest:tt)+) => { ... };
    ($v:ident[$n:expr] == $($rest:tt)+) => { ... };
} */
}
```

## Module `value`

```rust
pub(in ::header) mod value { /* ... */ }
```

### Types

#### Struct `HeaderValue`

Represents an HTTP header field value.

In practice, HTTP header field values are usually valid ASCII. However, the
HTTP spec allows for a header value to contain opaque bytes as well. In this
case, the header field value is not able to be represented as a string.

To handle this, the `HeaderValue` is useable as a type and can be compared
with strings and implements `Debug`. A `to_str` fn is provided that returns
an `Err` if the header value contains non visible ascii characters.

```rust
pub struct HeaderValue {
    pub(in ::header::value) inner: bytes::Bytes,
    pub(in ::header::value) is_sensitive: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `bytes::Bytes` |  |
| `is_sensitive` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_static(src: &''static str) -> HeaderValue { /* ... */ }
  ```
  Convert a static string to a `HeaderValue`.

- ```rust
  pub fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue> { /* ... */ }
  ```
  Attempt to convert a string to a `HeaderValue`.

- ```rust
  pub fn from_name(name: HeaderName) -> HeaderValue { /* ... */ }
  ```
  Converts a HeaderName into a HeaderValue

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue> { /* ... */ }
  ```
  Attempt to convert a byte slice to a `HeaderValue`.

- ```rust
  pub fn from_shared(src: Bytes) -> Result<HeaderValue, InvalidHeaderValueBytes> { /* ... */ }
  ```
  Attempt to convert a `Bytes` buffer to a `HeaderValue`.

- ```rust
  pub unsafe fn from_shared_unchecked(src: Bytes) -> HeaderValue { /* ... */ }
  ```
  Convert a `Bytes` directly into a `HeaderValue` without validating.

- ```rust
  pub(in ::header::value) fn try_from<T: AsRef<[u8]> + Into<Bytes>>(src: T) -> Result<HeaderValue, InvalidHeaderValue> { /* ... */ }
  ```

- ```rust
  pub fn to_str(self: &Self) -> Result<&str, ToStrError> { /* ... */ }
  ```
  Yields a `&str` slice if the `HeaderValue` only contains visible ASCII

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of `self`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the `HeaderValue` has a length of zero bytes.

- ```rust
  pub fn as_bytes(self: &Self) -> &[u8] { /* ... */ }
  ```
  Converts a `HeaderValue` to a byte slice.

- ```rust
  pub fn set_sensitive(self: &mut Self, val: bool) { /* ... */ }
  ```
  Mark that the header value represents sensitive information.

- ```rust
  pub fn is_sensitive(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the value represents sensitive data.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<HeaderValue, <Self as >::Err> { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Self) -> cmp::Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &[u8]) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a T) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &HeaderValue) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> HeaderValue { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &[u8]) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a T) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &HeaderValue) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **HttpTryFrom**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sealed**
- **Send**
- **Unpin**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(h: HeaderName) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: u16) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: i16) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: u32) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: i32) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: u64) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: i64) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: usize) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(num: isize) -> HeaderValue { /* ... */ }
    ```

  - ```rust
    fn from(value: HeaderValue) -> Bytes { /* ... */ }
    ```

  - ```rust
    fn from(t: &''a HeaderValue) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `InvalidHeaderValue`

A possible error when converting a `HeaderValue` from a string or byte
slice.

```rust
pub struct InvalidHeaderValue {
    pub(in ::header::value) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Trait Implementations

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: header::InvalidHeaderValue) -> Error { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Send**
- **RefUnwindSafe**
#### Struct `InvalidHeaderValueBytes`

A possible error when converting a `HeaderValue` from a string or byte
slice.

```rust
pub struct InvalidHeaderValueBytes(pub(in ::header::value) InvalidHeaderValue);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InvalidHeaderValue` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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
    fn from(err: header::InvalidHeaderValueBytes) -> Error { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
#### Struct `ToStrError`

A possible error when converting a `HeaderValue` to a string representation.

Header field values may contain opaque bytes, in which case it is not
possible to represent the value as a string.

```rust
pub struct ToStrError {
    pub(in ::header::value) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `is_visible_ascii`

```rust
pub(in ::header::value) fn is_visible_ascii(b: u8) -> bool { /* ... */ }
```

#### Function `is_valid`

**Attributes:**

- `#[inline]`

```rust
pub(in ::header::value) fn is_valid(b: u8) -> bool { /* ... */ }
```

### Macros

#### Macro `from_integers`

```rust
pub(crate) macro_rules! from_integers {
    /* macro_rules! from_integers {
    ($($name:ident: $t:ident => $max_len:expr),*) => { ... };
} */
}
```

### Constants and Statics

#### Constant `MAX_HEADER_NAME_LEN`

Maximum length of a header name

Generally, 64kb for a header name is WAY too much than would ever be needed
in practice. Restricting it to this size enables using `u16` values to
represent offsets when dealing with header names.

```rust
pub(in ::header) const MAX_HEADER_NAME_LEN: usize = _;
```

### Re-exports

#### Re-export `HeaderMap`

```rust
pub use self::map::HeaderMap;
```

#### Re-export `AsHeaderName`

```rust
pub use self::map::AsHeaderName;
```

#### Re-export `IntoHeaderName`

```rust
pub use self::map::IntoHeaderName;
```

#### Re-export `Iter`

```rust
pub use self::map::Iter;
```

#### Re-export `IterMut`

```rust
pub use self::map::IterMut;
```

#### Re-export `Keys`

```rust
pub use self::map::Keys;
```

#### Re-export `Values`

```rust
pub use self::map::Values;
```

#### Re-export `ValuesMut`

```rust
pub use self::map::ValuesMut;
```

#### Re-export `Drain`

```rust
pub use self::map::Drain;
```

#### Re-export `GetAll`

```rust
pub use self::map::GetAll;
```

#### Re-export `Entry`

```rust
pub use self::map::Entry;
```

#### Re-export `VacantEntry`

```rust
pub use self::map::VacantEntry;
```

#### Re-export `OccupiedEntry`

```rust
pub use self::map::OccupiedEntry;
```

#### Re-export `ValueIter`

```rust
pub use self::map::ValueIter;
```

#### Re-export `ValueIterMut`

```rust
pub use self::map::ValueIterMut;
```

#### Re-export `ValueDrain`

```rust
pub use self::map::ValueDrain;
```

#### Re-export `IntoIter`

```rust
pub use self::map::IntoIter;
```

#### Re-export `HeaderName`

```rust
pub use self::name::HeaderName;
```

#### Re-export `InvalidHeaderName`

```rust
pub use self::name::InvalidHeaderName;
```

#### Re-export `InvalidHeaderNameBytes`

```rust
pub use self::name::InvalidHeaderNameBytes;
```

#### Re-export `HeaderValue`

```rust
pub use self::value::HeaderValue;
```

#### Re-export `InvalidHeaderValue`

```rust
pub use self::value::InvalidHeaderValue;
```

#### Re-export `InvalidHeaderValueBytes`

```rust
pub use self::value::InvalidHeaderValueBytes;
```

#### Re-export `ToStrError`

```rust
pub use self::value::ToStrError;
```

#### Re-export `ACCEPT`

```rust
pub use self::name::ACCEPT;
```

#### Re-export `ACCEPT_CHARSET`

```rust
pub use self::name::ACCEPT_CHARSET;
```

#### Re-export `ACCEPT_ENCODING`

```rust
pub use self::name::ACCEPT_ENCODING;
```

#### Re-export `ACCEPT_LANGUAGE`

```rust
pub use self::name::ACCEPT_LANGUAGE;
```

#### Re-export `ACCEPT_RANGES`

```rust
pub use self::name::ACCEPT_RANGES;
```

#### Re-export `ACCESS_CONTROL_ALLOW_CREDENTIALS`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_CREDENTIALS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_HEADERS`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_HEADERS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_METHODS`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_METHODS;
```

#### Re-export `ACCESS_CONTROL_ALLOW_ORIGIN`

```rust
pub use self::name::ACCESS_CONTROL_ALLOW_ORIGIN;
```

#### Re-export `ACCESS_CONTROL_EXPOSE_HEADERS`

```rust
pub use self::name::ACCESS_CONTROL_EXPOSE_HEADERS;
```

#### Re-export `ACCESS_CONTROL_MAX_AGE`

```rust
pub use self::name::ACCESS_CONTROL_MAX_AGE;
```

#### Re-export `ACCESS_CONTROL_REQUEST_HEADERS`

```rust
pub use self::name::ACCESS_CONTROL_REQUEST_HEADERS;
```

#### Re-export `ACCESS_CONTROL_REQUEST_METHOD`

```rust
pub use self::name::ACCESS_CONTROL_REQUEST_METHOD;
```

#### Re-export `AGE`

```rust
pub use self::name::AGE;
```

#### Re-export `ALLOW`

```rust
pub use self::name::ALLOW;
```

#### Re-export `ALT_SVC`

```rust
pub use self::name::ALT_SVC;
```

#### Re-export `AUTHORIZATION`

```rust
pub use self::name::AUTHORIZATION;
```

#### Re-export `CACHE_CONTROL`

```rust
pub use self::name::CACHE_CONTROL;
```

#### Re-export `CONNECTION`

```rust
pub use self::name::CONNECTION;
```

#### Re-export `CONTENT_DISPOSITION`

```rust
pub use self::name::CONTENT_DISPOSITION;
```

#### Re-export `CONTENT_ENCODING`

```rust
pub use self::name::CONTENT_ENCODING;
```

#### Re-export `CONTENT_LANGUAGE`

```rust
pub use self::name::CONTENT_LANGUAGE;
```

#### Re-export `CONTENT_LENGTH`

```rust
pub use self::name::CONTENT_LENGTH;
```

#### Re-export `CONTENT_LOCATION`

```rust
pub use self::name::CONTENT_LOCATION;
```

#### Re-export `CONTENT_RANGE`

```rust
pub use self::name::CONTENT_RANGE;
```

#### Re-export `CONTENT_SECURITY_POLICY`

```rust
pub use self::name::CONTENT_SECURITY_POLICY;
```

#### Re-export `CONTENT_SECURITY_POLICY_REPORT_ONLY`

```rust
pub use self::name::CONTENT_SECURITY_POLICY_REPORT_ONLY;
```

#### Re-export `CONTENT_TYPE`

```rust
pub use self::name::CONTENT_TYPE;
```

#### Re-export `COOKIE`

```rust
pub use self::name::COOKIE;
```

#### Re-export `DNT`

```rust
pub use self::name::DNT;
```

#### Re-export `DATE`

```rust
pub use self::name::DATE;
```

#### Re-export `ETAG`

```rust
pub use self::name::ETAG;
```

#### Re-export `EXPECT`

```rust
pub use self::name::EXPECT;
```

#### Re-export `EXPIRES`

```rust
pub use self::name::EXPIRES;
```

#### Re-export `FORWARDED`

```rust
pub use self::name::FORWARDED;
```

#### Re-export `FROM`

```rust
pub use self::name::FROM;
```

#### Re-export `HOST`

```rust
pub use self::name::HOST;
```

#### Re-export `IF_MATCH`

```rust
pub use self::name::IF_MATCH;
```

#### Re-export `IF_MODIFIED_SINCE`

```rust
pub use self::name::IF_MODIFIED_SINCE;
```

#### Re-export `IF_NONE_MATCH`

```rust
pub use self::name::IF_NONE_MATCH;
```

#### Re-export `IF_RANGE`

```rust
pub use self::name::IF_RANGE;
```

#### Re-export `IF_UNMODIFIED_SINCE`

```rust
pub use self::name::IF_UNMODIFIED_SINCE;
```

#### Re-export `LAST_MODIFIED`

```rust
pub use self::name::LAST_MODIFIED;
```

#### Re-export `LINK`

```rust
pub use self::name::LINK;
```

#### Re-export `LOCATION`

```rust
pub use self::name::LOCATION;
```

#### Re-export `MAX_FORWARDS`

```rust
pub use self::name::MAX_FORWARDS;
```

#### Re-export `ORIGIN`

```rust
pub use self::name::ORIGIN;
```

#### Re-export `PRAGMA`

```rust
pub use self::name::PRAGMA;
```

#### Re-export `PROXY_AUTHENTICATE`

```rust
pub use self::name::PROXY_AUTHENTICATE;
```

#### Re-export `PROXY_AUTHORIZATION`

```rust
pub use self::name::PROXY_AUTHORIZATION;
```

#### Re-export `PUBLIC_KEY_PINS`

```rust
pub use self::name::PUBLIC_KEY_PINS;
```

#### Re-export `PUBLIC_KEY_PINS_REPORT_ONLY`

```rust
pub use self::name::PUBLIC_KEY_PINS_REPORT_ONLY;
```

#### Re-export `RANGE`

```rust
pub use self::name::RANGE;
```

#### Re-export `REFERER`

```rust
pub use self::name::REFERER;
```

#### Re-export `REFERRER_POLICY`

```rust
pub use self::name::REFERRER_POLICY;
```

#### Re-export `REFRESH`

```rust
pub use self::name::REFRESH;
```

#### Re-export `RETRY_AFTER`

```rust
pub use self::name::RETRY_AFTER;
```

#### Re-export `SEC_WEBSOCKET_ACCEPT`

```rust
pub use self::name::SEC_WEBSOCKET_ACCEPT;
```

#### Re-export `SEC_WEBSOCKET_EXTENSIONS`

```rust
pub use self::name::SEC_WEBSOCKET_EXTENSIONS;
```

#### Re-export `SEC_WEBSOCKET_KEY`

```rust
pub use self::name::SEC_WEBSOCKET_KEY;
```

#### Re-export `SEC_WEBSOCKET_PROTOCOL`

```rust
pub use self::name::SEC_WEBSOCKET_PROTOCOL;
```

#### Re-export `SEC_WEBSOCKET_VERSION`

```rust
pub use self::name::SEC_WEBSOCKET_VERSION;
```

#### Re-export `SERVER`

```rust
pub use self::name::SERVER;
```

#### Re-export `SET_COOKIE`

```rust
pub use self::name::SET_COOKIE;
```

#### Re-export `STRICT_TRANSPORT_SECURITY`

```rust
pub use self::name::STRICT_TRANSPORT_SECURITY;
```

#### Re-export `TE`

```rust
pub use self::name::TE;
```

#### Re-export `TRAILER`

```rust
pub use self::name::TRAILER;
```

#### Re-export `TRANSFER_ENCODING`

```rust
pub use self::name::TRANSFER_ENCODING;
```

#### Re-export `UPGRADE`

```rust
pub use self::name::UPGRADE;
```

#### Re-export `UPGRADE_INSECURE_REQUESTS`

```rust
pub use self::name::UPGRADE_INSECURE_REQUESTS;
```

#### Re-export `USER_AGENT`

```rust
pub use self::name::USER_AGENT;
```

#### Re-export `VARY`

```rust
pub use self::name::VARY;
```

#### Re-export `VIA`

```rust
pub use self::name::VIA;
```

#### Re-export `WARNING`

```rust
pub use self::name::WARNING;
```

#### Re-export `WWW_AUTHENTICATE`

```rust
pub use self::name::WWW_AUTHENTICATE;
```

#### Re-export `X_CONTENT_TYPE_OPTIONS`

```rust
pub use self::name::X_CONTENT_TYPE_OPTIONS;
```

#### Re-export `X_DNS_PREFETCH_CONTROL`

```rust
pub use self::name::X_DNS_PREFETCH_CONTROL;
```

#### Re-export `X_FRAME_OPTIONS`

```rust
pub use self::name::X_FRAME_OPTIONS;
```

#### Re-export `X_XSS_PROTECTION`

```rust
pub use self::name::X_XSS_PROTECTION;
```

## Module `method`

The HTTP request method

This module contains HTTP-method related structs and errors and such. The
main type of this module, `Method`, is also reexported at the root of the
crate as `http::Method` and is intended for import through that location
primarily.

# Examples

```
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

```rust
pub mod method { /* ... */ }
```

### Types

#### Struct `Method`

The Request Method (VERB)

This type also contains constants for a number of common HTTP methods such
as GET, POST, etc.

Currently includes 8 variants representing the 8 methods defined in
[RFC 7230](https://tools.ietf.org/html/rfc7231#section-4.1), plus PATCH,
and an Extension variant for all extensions.

# Examples

```
use http::Method;

assert_eq!(Method::GET, Method::from_bytes(b"GET").unwrap());
assert!(Method::GET.is_idempotent());
assert_eq!(Method::POST.as_str(), "POST");
```

```rust
pub struct Method(pub(in ::method) Inner);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Inner` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> { /* ... */ }
  ```
  Converts a slice of bytes to an HTTP method.

- ```rust
  pub(in ::method) fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> { /* ... */ }
  ```

- ```rust
  pub fn is_safe(self: &Self) -> bool { /* ... */ }
  ```
  Whether a method is considered "safe", meaning the request is

- ```rust
  pub fn is_idempotent(self: &Self) -> bool { /* ... */ }
  ```
  Whether a method is considered "idempotent", meaning the request has

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return a &str representation of the HTTP method

###### Trait Implementations

- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(t: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sealed**
- **Default**
  - ```rust
    fn default() -> Method { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn from(t: &''a Method) -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Method { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Method) -> bool { /* ... */ }
    ```

- **HttpTryFrom**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `InvalidMethod`

A possible error value when converting `Method` from bytes.

```rust
pub struct InvalidMethod {
    pub(in ::method) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::method) fn new() -> InvalidMethod { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: method::InvalidMethod) -> Error { /* ... */ }
    ```

#### Enum `Inner`

```rust
pub(in ::method) enum Inner {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline([u8; 15], u8),
    ExtensionAllocated(Box<[u8]>),
}
```

##### Variants

###### `Options`

###### `Get`

###### `Post`

###### `Put`

###### `Delete`

###### `Head`

###### `Trace`

###### `Connect`

###### `Patch`

###### `ExtensionInline`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 15]` |  |
| 1 | `u8` |  |

###### `ExtensionAllocated`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<[u8]>` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Inner) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Inner { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
### Functions

#### Function `write_checked`

```rust
pub(in ::method) fn write_checked(src: &[u8], dst: &mut [u8]) -> Result<(), InvalidMethod> { /* ... */ }
```

### Constants and Statics

#### Constant `MAX_INLINE`

```rust
pub(in ::method) const MAX_INLINE: usize = 15;
```

#### Constant `METHOD_CHARS`

```rust
pub(in ::method) const METHOD_CHARS: [u8; 256] = _;
```

## Module `request`

HTTP request types.

This module contains structs related to HTTP requests, notably the
`Request` type itself as well as a builder to create requests. Typically
you'll import the `http::Request` type rather than reaching into this
module itself.

# Examples

Creating a `Request` to send

```no_run
use http::{Request, Response};

let mut request = Request::builder();
request.uri("https://www.rust-lang.org/")
       .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
# panic!()
}
```

```rust
pub mod request { /* ... */ }
```

### Types

#### Struct `Request`

Represents an HTTP request.

An HTTP request consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

# Examples

Creating a `Request` to send

```no_run
use http::{Request, Response};

let mut request = Request::builder();
request.uri("https://www.rust-lang.org/")
       .header("User-Agent", "my-awesome-agent/1.0");

if needs_awesome_header() {
    request.header("Awesome", "yes");
}

let response = send(request.body(()).unwrap());

# fn needs_awesome_header() -> bool {
#     true
# }
#
fn send(req: Request<()>) -> Response<()> {
    // ...
# panic!()
}
```

Inspecting a request to see what was sent.

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    if req.uri() != "/awesome-url" {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(())
    }

    let has_awesome_header = req.headers().contains_key("Awesome");
    let body = req.body();

    // ...
# panic!()
}
```

Deserialize a request of bytes via json:

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Request;
use serde::de;

fn deserialize<T>(req: Request<Vec<u8>>) -> serde_json::Result<Request<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
```

Or alternatively, serialize the body of a request to json

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Request;
use serde::ser;

fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Request::from_parts(parts, body))
}
#
# fn main() {}
```

```rust
pub struct Request<T> {
    pub(in ::request) head: Parts,
    pub(in ::request) body: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `head` | `Parts` |  |
| `body` | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Request`

- ```rust
  pub fn get<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a GET method and the given URI.

- ```rust
  pub fn put<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a PUT method and the given URI.

- ```rust
  pub fn post<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a POST method and the given URI.

- ```rust
  pub fn delete<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a DELETE method and the given URI.

- ```rust
  pub fn options<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with an OPTIONS method and the given URI.

- ```rust
  pub fn head<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a HEAD method and the given URI.

- ```rust
  pub fn connect<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a CONNECT method and the given URI.

- ```rust
  pub fn patch<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a PATCH method and the given URI.

- ```rust
  pub fn trace<T>(uri: T) -> Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Creates a new `Builder` initialized with a TRACE method and the given URI.

- ```rust
  pub fn new(body: T) -> Request<T> { /* ... */ }
  ```
  Creates a new blank `Request` with the body

- ```rust
  pub fn from_parts(parts: Parts, body: T) -> Request<T> { /* ... */ }
  ```
  Creates a new `Request` with the given components parts and body.

- ```rust
  pub fn method(self: &Self) -> &Method { /* ... */ }
  ```
  Returns a reference to the associated HTTP method.

- ```rust
  pub fn method_mut(self: &mut Self) -> &mut Method { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP method.

- ```rust
  pub fn uri(self: &Self) -> &Uri { /* ... */ }
  ```
  Returns a reference to the associated URI.

- ```rust
  pub fn uri_mut(self: &mut Self) -> &mut Uri { /* ... */ }
  ```
  Returns a mutable reference to the associated URI.

- ```rust
  pub fn version(self: &Self) -> Version { /* ... */ }
  ```
  Returns the associated version.

- ```rust
  pub fn version_mut(self: &mut Self) -> &mut Version { /* ... */ }
  ```
  Returns a mutable reference to the associated version.

- ```rust
  pub fn headers(self: &Self) -> &HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a reference to the associated header field map.

- ```rust
  pub fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a mutable reference to the associated header field map.

- ```rust
  pub fn extensions(self: &Self) -> &Extensions { /* ... */ }
  ```
  Returns a reference to the associated extensions.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> &mut Extensions { /* ... */ }
  ```
  Returns a mutable reference to the associated extensions.

- ```rust
  pub fn body(self: &Self) -> &T { /* ... */ }
  ```
  Returns a reference to the associated HTTP body.

- ```rust
  pub fn body_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP body.

- ```rust
  pub fn into_body(self: Self) -> T { /* ... */ }
  ```
  Consumes the request, returning just the body.

- ```rust
  pub fn into_parts(self: Self) -> (Parts, T) { /* ... */ }
  ```
  Consumes the request returning the head and body parts.

- ```rust
  pub fn map<F, U>(self: Self, f: F) -> Request<U>
where
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Consumes the request returning a new request with body mapped to the

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Request<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `Parts`

Component parts of an HTTP `Request`

The HTTP request head consists of a method, uri, version, and a set of
header fields.

```rust
pub struct Parts {
    pub method: method::Method,
    pub uri: Uri,
    pub version: version::Version,
    pub headers: header::HeaderMap<header::HeaderValue>,
    pub extensions: Extensions,
    pub(in ::request) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `method` | `method::Method` | The request's method |
| `uri` | `Uri` | The request's URI |
| `version` | `version::Version` | The request's version |
| `headers` | `header::HeaderMap<header::HeaderValue>` | The request's headers |
| `extensions` | `Extensions` | The request's extensions |
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::request) fn new() -> Parts { /* ... */ }
  ```
  Creates a new default instance of `Parts`

###### Trait Implementations

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Builder`

An HTTP request builder

This type can be used to construct an instance or `Request`
through a builder-like pattern.

```rust
pub struct Builder {
    pub(in ::request) head: Option<Parts>,
    pub(in ::request) err: Option<Error>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `head` | `Option<Parts>` |  |
| `err` | `Option<Error>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Builder { /* ... */ }
  ```
  Creates a new default instance of `Builder` to construct a `Request`.

- ```rust
  pub fn method<T>(self: &mut Self, method: T) -> &mut Builder
where
    Method: HttpTryFrom<T> { /* ... */ }
  ```
  Set the HTTP method for this request.

- ```rust
  pub fn method_ref(self: &Self) -> Option<&Method> { /* ... */ }
  ```
  Get the HTTP Method for this request.

- ```rust
  pub fn uri<T>(self: &mut Self, uri: T) -> &mut Builder
where
    Uri: HttpTryFrom<T> { /* ... */ }
  ```
  Set the URI for this request.

- ```rust
  pub fn uri_ref(self: &Self) -> Option<&Uri> { /* ... */ }
  ```
  Get the URI for this request

- ```rust
  pub fn version(self: &mut Self, version: Version) -> &mut Builder { /* ... */ }
  ```
  Set the HTTP version for this request.

- ```rust
  pub fn header<K, V>(self: &mut Self, key: K, value: V) -> &mut Builder
where
    HeaderName: HttpTryFrom<K>,
    HeaderValue: HttpTryFrom<V> { /* ... */ }
  ```
  Appends a header to this request builder.

- ```rust
  pub fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this request builder.

- ```rust
  pub fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this request builder.

- ```rust
  pub fn extension<T>(self: &mut Self, extension: T) -> &mut Builder
where
    T: Any + Send + Sync + ''static { /* ... */ }
  ```
  Adds an extension to this builder

- ```rust
  pub(in ::request) fn take_parts(self: &mut Self) -> Result<Parts> { /* ... */ }
  ```

- ```rust
  pub fn body<T>(self: &mut Self, body: T) -> Result<Request<T>> { /* ... */ }
  ```
  "Consumes" this builder, using the provided `body` to return a

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
### Functions

#### Function `head`

```rust
pub(in ::request) fn head<''a>(head: &''a mut Option<Parts>, err: &Option<Error>) -> Option<&''a mut Parts> { /* ... */ }
```

## Module `response`

HTTP response types.

This module contains structs related to HTTP responses, notably the
`Response` type itself as well as a builder to create responses. Typically
you'll import the `http::Response` type rather than reaching into this
module itself.

# Examples

Creating a `Response` to return

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut response = Response::builder();
    response.header("Foo", "Bar")
            .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        response.header("Another-Header", "Ack");
    }

    response.body(())
}
```

A simple 404 handler

```
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}
```

Or otherwise inspecting the result of a request:

```no_run
use http::{Request, Response};

fn get(url: &str) -> http::Result<Response<()>> {
    // ...
# panic!()
}

let response = get("https://www.rust-lang.org/").unwrap();

if !response.status().is_success() {
    panic!("failed to get a successful response status!");
}

if let Some(date) = response.headers().get("Date") {
    // we've got a `Date` header!
}

let body = response.body();
// ...
```

```rust
pub mod response { /* ... */ }
```

### Types

#### Struct `Response`

Represents an HTTP response

An HTTP response consists of a head and a potentially optional body. The body
component is generic, enabling arbitrary types to represent the HTTP body.
For example, the body could be `Vec<u8>`, a `Stream` of byte chunks, or a
value that has been deserialized.

Typically you'll work with responses on the client side as the result of
sending a `Request` and on the server you'll be generating a `Request` to
send back to the client.

# Examples

Creating a `Response` to return

```
use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut response = Response::builder();
    response.header("Foo", "Bar")
            .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        response.header("Another-Header", "Ack");
    }

    response.body(())
}
```

A simple 404 handler

```
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}
```

Or otherwise inspecting the result of a request:

```no_run
use http::{Request, Response};

fn get(url: &str) -> http::Result<Response<()>> {
    // ...
# panic!()
}

let response = get("https://www.rust-lang.org/").unwrap();

if !response.status().is_success() {
    panic!("failed to get a successful response status!");
}

if let Some(date) = response.headers().get("Date") {
    // we've got a `Date` header!
}

let body = response.body();
// ...
```

Deserialize a response of bytes via json:

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Response;
use serde::de;

fn deserialize<T>(req: Response<Vec<u8>>) -> serde_json::Result<Response<T>>
    where for<'de> T: de::Deserialize<'de>,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::from_slice(&body)?;
    Ok(Response::from_parts(parts, body))
}
#
# fn main() {}
```

Or alternatively, serialize the body of a response to json

```
# extern crate serde;
# extern crate serde_json;
# extern crate http;
use http::Response;
use serde::ser;

fn serialize<T>(req: Response<T>) -> serde_json::Result<Response<Vec<u8>>>
    where T: ser::Serialize,
{
    let (parts, body) = req.into_parts();
    let body = serde_json::to_vec(&body)?;
    Ok(Response::from_parts(parts, body))
}
#
# fn main() {}
```

```rust
pub struct Response<T> {
    pub(in ::response) head: Parts,
    pub(in ::response) body: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `head` | `Parts` |  |
| `body` | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Response`

- ```rust
  pub fn new(body: T) -> Response<T> { /* ... */ }
  ```
  Creates a new blank `Response` with the body

- ```rust
  pub fn from_parts(parts: Parts, body: T) -> Response<T> { /* ... */ }
  ```
  Creates a new `Response` with the given head and body

- ```rust
  pub fn status(self: &Self) -> StatusCode { /* ... */ }
  ```
  Returns the `StatusCode`.

- ```rust
  pub fn status_mut(self: &mut Self) -> &mut StatusCode { /* ... */ }
  ```
  Returns a mutable reference to the associated `StatusCode`.

- ```rust
  pub fn version(self: &Self) -> Version { /* ... */ }
  ```
  Returns a reference to the associated version.

- ```rust
  pub fn version_mut(self: &mut Self) -> &mut Version { /* ... */ }
  ```
  Returns a mutable reference to the associated version.

- ```rust
  pub fn headers(self: &Self) -> &HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a reference to the associated header field map.

- ```rust
  pub fn headers_mut(self: &mut Self) -> &mut HeaderMap<HeaderValue> { /* ... */ }
  ```
  Returns a mutable reference to the associated header field map.

- ```rust
  pub fn extensions(self: &Self) -> &Extensions { /* ... */ }
  ```
  Returns a reference to the associated extensions.

- ```rust
  pub fn extensions_mut(self: &mut Self) -> &mut Extensions { /* ... */ }
  ```
  Returns a mutable reference to the associated extensions.

- ```rust
  pub fn body(self: &Self) -> &T { /* ... */ }
  ```
  Returns a reference to the associated HTTP body.

- ```rust
  pub fn body_mut(self: &mut Self) -> &mut T { /* ... */ }
  ```
  Returns a mutable reference to the associated HTTP body.

- ```rust
  pub fn into_body(self: Self) -> T { /* ... */ }
  ```
  Consumes the response, returning just the body.

- ```rust
  pub fn into_parts(self: Self) -> (Parts, T) { /* ... */ }
  ```
  Consumes the response returning the head and body parts.

- ```rust
  pub fn map<F, U>(self: Self, f: F) -> Response<U>
where
    F: FnOnce(T) -> U { /* ... */ }
  ```
  Consumes the response returning a new response with body mapped to the

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Response<T> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
#### Struct `Parts`

Component parts of an HTTP `Response`

The HTTP response head consists of a status, version, and a set of
header fields.

```rust
pub struct Parts {
    pub status: status::StatusCode,
    pub version: version::Version,
    pub headers: header::HeaderMap<header::HeaderValue>,
    pub extensions: Extensions,
    pub(in ::response) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `status` | `status::StatusCode` | The response's status |
| `version` | `version::Version` | The response's version |
| `headers` | `header::HeaderMap<header::HeaderValue>` | The response's headers |
| `extensions` | `Extensions` | The response's extensions |
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::response) fn new() -> Parts { /* ... */ }
  ```
  Creates a new default instance of `Parts`

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Builder`

An HTTP response builder

This type can be used to construct an instance of `Response` through a
builder-like pattern.

```rust
pub struct Builder {
    pub(in ::response) head: Option<Parts>,
    pub(in ::response) err: Option<Error>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `head` | `Option<Parts>` |  |
| `err` | `Option<Error>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Builder { /* ... */ }
  ```
  Creates a new default instance of `Builder` to construct either a

- ```rust
  pub fn status<T>(self: &mut Self, status: T) -> &mut Builder
where
    StatusCode: HttpTryFrom<T> { /* ... */ }
  ```
  Set the HTTP status for this response.

- ```rust
  pub fn version(self: &mut Self, version: Version) -> &mut Builder { /* ... */ }
  ```
  Set the HTTP version for this response.

- ```rust
  pub fn header<K, V>(self: &mut Self, key: K, value: V) -> &mut Builder
where
    HeaderName: HttpTryFrom<K>,
    HeaderValue: HttpTryFrom<V> { /* ... */ }
  ```
  Appends a header to this response builder.

- ```rust
  pub fn headers_ref(self: &Self) -> Option<&HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this response builder.

- ```rust
  pub fn headers_mut(self: &mut Self) -> Option<&mut HeaderMap<HeaderValue>> { /* ... */ }
  ```
  Get header on this response builder.

- ```rust
  pub fn extension<T>(self: &mut Self, extension: T) -> &mut Builder
where
    T: Any + Send + Sync + ''static { /* ... */ }
  ```
  Adds an extension to this builder

- ```rust
  pub(in ::response) fn take_parts(self: &mut Self) -> Result<Parts> { /* ... */ }
  ```

- ```rust
  pub fn body<T>(self: &mut Self, body: T) -> Result<Response<T>> { /* ... */ }
  ```
  "Consumes" this builder, using the provided `body` to return a

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `head`

```rust
pub(in ::response) fn head<''a>(head: &''a mut Option<Parts>, err: &Option<Error>) -> Option<&''a mut Parts> { /* ... */ }
```

## Module `status`

HTTP status codes

This module contains HTTP-status code related structs an errors. The main
type in this module is `StatusCode` which is not intended to be used through
this module but rather the `http::StatusCode` type.

# Examples

```
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND, 404);
assert!(StatusCode::OK.is_success());
```

```rust
pub mod status { /* ... */ }
```

### Types

#### Struct `StatusCode`

An HTTP status code (`status-code` in RFC 7230 et al.).

This type contains constants for all common status codes.
It allows status codes in the range [100, 599].

IANA maintain the [Hypertext Transfer Protocol (HTTP) Status Code
Registry](http://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml) which is
the source for this enum (with one exception, 418 I'm a teapot, which is
inexplicably not in the register).

# Examples

```
use http::StatusCode;

assert_eq!(StatusCode::from_u16(200).unwrap(), StatusCode::OK);
assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
assert!(StatusCode::OK.is_success());
```

```rust
pub struct StatusCode(pub(in ::status) u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
  ```
  Converts a u16 to a status code.

- ```rust
  pub fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
  ```
  Converts a &[u8] to a status code

- ```rust
  pub fn as_u16(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the `u16` corresponding to this `StatusCode`.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns a &str representation of the `StatusCode`

- ```rust
  pub fn canonical_reason(self: &Self) -> Option<&''static str> { /* ... */ }
  ```
  Get the standardised `reason-phrase` for this status code.

- ```rust
  pub fn is_informational(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 100-199.

- ```rust
  pub fn is_success(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 200-299.

- ```rust
  pub fn is_redirection(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 300-399.

- ```rust
  pub fn is_client_error(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 400-499.

- ```rust
  pub fn is_server_error(self: &Self) -> bool { /* ... */ }
  ```
  Check if status is within 500-599.

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StatusCode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sealed**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &StatusCode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StatusCode { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StatusCode) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &StatusCode) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> StatusCode { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(status: StatusCode) -> u16 { /* ... */ }
    ```

  - ```rust
    fn from(t: &''a StatusCode) -> Self { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Copy**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> { /* ... */ }
    ```

- **UnwindSafe**
- **HttpTryFrom**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Struct `InvalidStatusCode`

A possible error value when converting a `StatusCode` from a `u16` or `&str`

This error indicates that the supplied input was not a valid number, was less
than 100, or was greater than 599.

```rust
pub struct InvalidStatusCode {
    pub(in ::status) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_priv` | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::status) fn new() -> InvalidStatusCode { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: status::InvalidStatusCode) -> Error { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
### Functions

#### Function `canonical_reason`

```rust
pub(in ::status) fn canonical_reason(num: u16) -> Option<&''static str> { /* ... */ }
```

### Constants and Statics

#### Constant `CODES_AS_STR`

```rust
pub(in ::status) const CODES_AS_STR: [&''static str; 500] = _;
```

### Macros

#### Macro `status_codes`

```rust
pub(crate) macro_rules! status_codes {
    /* macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => { ... };
} */
}
```

#### Macro `status_code_strs`

```rust
pub(crate) macro_rules! status_code_strs {
    /* macro_rules! status_code_strs {
    ($($num:expr,)+) => { ... };
} */
}
```

## Module `version`

HTTP version

This module contains a definition of the `Version` type. The `Version`
type is intended to be accessed through the root of the crate
(`http::Version`) rather than this module.

The `Version` type contains constants that represent the various versions
of the HTTP protocol.

# Examples

```
use http::Version;

let http11 = Version::HTTP_11;
let http2 = Version::HTTP_2;
assert!(http11 != http2);

println!("{:?}", http2);
```

```rust
pub mod version { /* ... */ }
```

### Types

#### Struct `Version`

Represents a version of the HTTP spec.

```rust
pub struct Version(pub(in ::version) Http);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Http` |  |

##### Implementations

###### Methods

###### Trait Implementations

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Version) -> bool { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Version { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Version) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Version) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **Default**
  - ```rust
    fn default() -> Version { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Enum `Http`

```rust
pub(in ::version) enum Http {
    Http09,
    Http10,
    Http11,
    H2,
}
```

##### Variants

###### `Http09`

###### `Http10`

###### `Http11`

###### `H2`

##### Implementations

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Http) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Http) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Http { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Http) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

## Module `uri`

URI component of request and response lines

This module primarily contains the `Uri` type which is a component of all
HTTP requests and also reexports this type at the root of the crate. A URI
is not always a "full URL" in the sense of something you'd type into a web
browser, but HTTP requests may only have paths on servers but may have full
schemes and hostnames on clients.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_part().map(|s| s.as_str()), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

```rust
pub mod uri { /* ... */ }
```

### Modules

## Module `authority`

```rust
pub(in ::uri) mod authority { /* ... */ }
```

### Types

#### Struct `Authority`

Represents the authority component of a URI.

```rust
pub struct Authority {
    pub(in ::uri) data: byte_str::ByteStr,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `byte_str::ByteStr` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::uri) fn empty() -> Self { /* ... */ }
  ```

- ```rust
  pub fn from_shared(s: Bytes) -> Result<Self, InvalidUriBytes> { /* ... */ }
  ```
  Attempt to convert an `Authority` from `Bytes`.

- ```rust
  pub fn from_static(src: &''static str) -> Self { /* ... */ }
  ```
  Attempt to convert an `Authority` from a static string.

- ```rust
  pub(in ::uri) fn parse(s: &[u8]) -> Result<usize, InvalidUri> { /* ... */ }
  ```

- ```rust
  pub(in ::uri::authority) fn parse_non_empty(s: &[u8]) -> Result<usize, InvalidUri> { /* ... */ }
  ```

- ```rust
  pub fn host(self: &Self) -> &str { /* ... */ }
  ```
  Get the host of this `Authority`.

- ```rust
  pub fn port_part(self: &Self) -> Option<Port<&str>> { /* ... */ }
  ```
  Get the port part of this `Authority`.

- ```rust
  pub fn port_u16(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Get the port of this `Authority` as a `u16`.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return a str representation of the authority

- ```rust
  pub fn into_bytes(self: Self) -> Bytes { /* ... */ }
  ```
  Converts this `Authority` back to a sequence of bytes

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Authority) -> Bytes { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **HttpTryFrom**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Authority) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Authority) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Authority) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &Authority) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, InvalidUri> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Authority) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Authority) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Authority) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Authority) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Authority { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sealed**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `host`

```rust
pub(in ::uri::authority) fn host(auth: &str) -> &str { /* ... */ }
```

## Module `builder`

```rust
pub(in ::uri) mod builder { /* ... */ }
```

### Types

#### Struct `Builder`

A builder for `Uri`s.

This type can be used to construct an instance of `Uri`
through a builder pattern.

```rust
pub struct Builder {
    pub(in ::uri::builder) parts: Option<Result<super::Parts>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parts` | `Option<Result<super::Parts>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Builder { /* ... */ }
  ```
  Creates a new default instance of `Builder` to construct a `Uri`.

- ```rust
  pub fn scheme<T>(self: &mut Self, scheme: T) -> &mut Self
where
    Scheme: HttpTryFrom<T> { /* ... */ }
  ```
  Set the `Scheme` for this URI.

- ```rust
  pub fn authority<T>(self: &mut Self, auth: T) -> &mut Self
where
    Authority: HttpTryFrom<T> { /* ... */ }
  ```
  Set the `Authority` for this URI.

- ```rust
  pub fn path_and_query<T>(self: &mut Self, p_and_q: T) -> &mut Self
where
    PathAndQuery: HttpTryFrom<T> { /* ... */ }
  ```
  Set the `PathAndQuery` for this URI.

- ```rust
  pub fn build(self: &mut Self) -> Result<Uri> { /* ... */ }
  ```
  Consumes this builder, and tries to construct a valid `Uri` from

- ```rust
  pub(in ::uri::builder) fn map<F>(self: &mut Self, f: F) -> &mut Self
where
    F: FnOnce(&mut Parts) -> Result<()> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Builder { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
## Module `path`

```rust
pub(in ::uri) mod path { /* ... */ }
```

### Types

#### Struct `PathAndQuery`

Represents the path component of a URI

```rust
pub struct PathAndQuery {
    pub(in ::uri) data: byte_str::ByteStr,
    pub(in ::uri) query: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `byte_str::ByteStr` |  |
| `query` | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_shared(src: Bytes) -> Result<Self, InvalidUriBytes> { /* ... */ }
  ```
  Attempt to convert a `PathAndQuery` from `Bytes`.

- ```rust
  pub fn from_static(src: &''static str) -> Self { /* ... */ }
  ```
  Convert a `PathAndQuery` from a static string.

- ```rust
  pub(in ::uri) fn empty() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::uri) fn slash() -> Self { /* ... */ }
  ```

- ```rust
  pub(in ::uri) fn star() -> Self { /* ... */ }
  ```

- ```rust
  pub fn path(self: &Self) -> &str { /* ... */ }
  ```
  Returns the path component

- ```rust
  pub fn query(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the query string component

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns the path and query as a string component.

- ```rust
  pub fn into_bytes(self: Self) -> Bytes { /* ... */ }
  ```
  Converts this `PathAndQuery` back to a sequence of bytes

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PathAndQuery) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathAndQuery) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &&''a str) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathAndQuery) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &String) -> Option<cmp::Ordering> { /* ... */ }
    ```

  - ```rust
    fn partial_cmp(self: &Self, other: &PathAndQuery) -> Option<cmp::Ordering> { /* ... */ }
    ```

- **Sealed**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **HttpTryFrom**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PathAndQuery) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathAndQuery) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathAndQuery) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &String) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &PathAndQuery) -> bool { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: PathAndQuery) -> Bytes { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PathAndQuery { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, InvalidUri> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Constants and Statics

#### Constant `NONE`

```rust
pub(in ::uri::path) const NONE: u16 = ::std::u16::MAX;
```

## Module `port`

```rust
pub(in ::uri) mod port { /* ... */ }
```

### Types

#### Struct `Port`

The port component of a URI.

```rust
pub struct Port<T> {
    pub(in ::uri::port) port: u16,
    pub(in ::uri::port) repr: T,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `port` | `u16` |  |
| `repr` | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_u16(self: &Self) -> u16 { /* ... */ }
  ```
  Returns the port number as a `u16`.

- ```rust
  pub(crate) fn from_str(bytes: T) -> Result<Self, InvalidUri> { /* ... */ }
  ```
  Converts a `str` to a port number.

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns the port number as a `str`.

###### Trait Implementations

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Port<U>) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &u16) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Port<T>) -> bool { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn from(port: Port<T>) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

## Module `scheme`

```rust
pub(in ::uri) mod scheme { /* ... */ }
```

### Types

#### Struct `Scheme`

Represents the scheme component of a URI

```rust
pub struct Scheme {
    pub(in ::uri) inner: Scheme2,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Scheme2` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_shared(s: Bytes) -> Result<Self, InvalidUriBytes> { /* ... */ }
  ```
  Attempt to convert a `Scheme` from `Bytes`

- ```rust
  pub(in ::uri) fn empty() -> Self { /* ... */ }
  ```

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Return a str representation of the scheme

- ```rust
  pub fn into_bytes(self: Self) -> Bytes { /* ... */ }
  ```
  Converts this `Scheme` back to a sequence of bytes

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

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
    fn from(src: Scheme) -> Self { /* ... */ }
    ```

- **Sealed**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Scheme { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **HttpTryFrom**
- **Unpin**
- **Eq**
- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Scheme) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &Scheme) -> bool { /* ... */ }
    ```

- **Sync**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

#### Enum `Scheme2`

```rust
pub(in ::uri) enum Scheme2<T = Box<byte_str::ByteStr>> {
    None,
    Standard(Protocol),
    Other(T),
}
```

##### Variants

###### `None`

###### `Standard`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Protocol` |  |

###### `Other`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::uri) fn is_none(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::uri::scheme) fn parse_exact(s: &[u8]) -> Result<Scheme2<()>, InvalidUri> { /* ... */ }
  ```

- ```rust
  pub(in ::uri) fn parse(s: &[u8]) -> Result<Scheme2<usize>, InvalidUri> { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Scheme2<T> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Protocol) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

#### Enum `Protocol`

```rust
pub(in ::uri) enum Protocol {
    Http,
    Https,
}
```

##### Variants

###### `Http`

###### `Https`

##### Implementations

###### Methods

- ```rust
  pub(in ::uri) fn len(self: &Self) -> usize { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Protocol { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Protocol) -> Self { /* ... */ }
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

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Constants and Statics

#### Constant `MAX_SCHEME_LEN`

```rust
pub(in ::uri::scheme) const MAX_SCHEME_LEN: usize = 64;
```

#### Constant `SCHEME_CHARS`

```rust
pub(in ::uri::scheme) const SCHEME_CHARS: [u8; 256] = _;
```

### Types

#### Struct `Uri`

The URI component of a request.

For HTTP 1, this is included as part of the request line. From Section 5.3,
Request Target:

> Once an inbound connection is obtained, the client sends an HTTP
> request message (Section 3) with a request-target derived from the
> target URI.  There are four distinct formats for the request-target,
> depending on both the method being requested and whether the request
> is to a proxy.
>
> ```notrust
> request-target = origin-form
>                / absolute-form
>                / authority-form
>                / asterisk-form
> ```

The URI is structured as follows:

```notrust
abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1
|-|   |-------------------------------||--------| |-------------------| |-----|
 |                  |                       |               |              |
scheme          authority                 path            query         fragment
```

For HTTP 2.0, the URI is encoded using pseudoheaders.

# Examples

```
use http::Uri;

let uri = "/foo/bar?baz".parse::<Uri>().unwrap();
assert_eq!(uri.path(), "/foo/bar");
assert_eq!(uri.query(), Some("baz"));
assert_eq!(uri.host(), None);

let uri = "https://www.rust-lang.org/install.html".parse::<Uri>().unwrap();
assert_eq!(uri.scheme_part().map(|s| s.as_str()), Some("https"));
assert_eq!(uri.host(), Some("www.rust-lang.org"));
assert_eq!(uri.path(), "/install.html");
```

```rust
pub struct Uri {
    pub(in ::uri) scheme: Scheme,
    pub(in ::uri) authority: Authority,
    pub(in ::uri) path_and_query: PathAndQuery,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scheme` | `Scheme` |  |
| `authority` | `Authority` |  |
| `path_and_query` | `PathAndQuery` |  |

##### Implementations

###### Methods

- ```rust
  pub fn builder() -> Builder { /* ... */ }
  ```
  Creates a new builder-style object to manufacture a `Uri`.

- ```rust
  pub fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> { /* ... */ }
  ```
  Attempt to convert a `Uri` from `Parts`

- ```rust
  pub fn from_shared(s: Bytes) -> Result<Uri, InvalidUriBytes> { /* ... */ }
  ```
  Attempt to convert a `Uri` from `Bytes`

- ```rust
  pub fn from_static(src: &''static str) -> Self { /* ... */ }
  ```
  Convert a `Uri` from a static string.

- ```rust
  pub fn into_parts(self: Self) -> Parts { /* ... */ }
  ```
  Convert a `Uri` into `Parts`.

- ```rust
  pub fn path_and_query(self: &Self) -> Option<&PathAndQuery> { /* ... */ }
  ```
  Returns the path & query components of the Uri

- ```rust
  pub fn path(self: &Self) -> &str { /* ... */ }
  ```
  Get the path of this `Uri`.

- ```rust
  pub fn scheme_part(self: &Self) -> Option<&Scheme> { /* ... */ }
  ```
  Get the scheme of this `Uri`.

- ```rust
  pub fn scheme_str(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the scheme of this `Uri` as a `&str`.

- ```rust
  pub fn authority_part(self: &Self) -> Option<&Authority> { /* ... */ }
  ```
  Get the authority of this `Uri`.

- ```rust
  pub fn host(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the host of this `Uri`.

- ```rust
  pub fn port_part(self: &Self) -> Option<Port<&str>> { /* ... */ }
  ```
  Get the port part of this `Uri`.

- ```rust
  pub fn port_u16(self: &Self) -> Option<u16> { /* ... */ }
  ```
  Get the port of this `Uri` as a `u16`.

- ```rust
  pub fn query(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Get the query string of this `Uri`, starting after the `?`.

- ```rust
  pub(in ::uri) fn has_path(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Uri { /* ... */ }
    ```

- **Sealed**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Uri) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, uri: &Uri) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, other: &&''a str) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, uri: &Uri) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Uri { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Uri, InvalidUri> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<H>(self: &Self, state: &mut H)
where
    H: Hasher { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **HttpTryFrom**
- **Eq**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Uri) -> Self { /* ... */ }
    ```

#### Struct `Parts`

The various parts of a URI.

This struct is used to provide to and retrieve from a URI.

```rust
pub struct Parts {
    pub scheme: Option<Scheme>,
    pub authority: Option<Authority>,
    pub path_and_query: Option<PathAndQuery>,
    pub(in ::uri) _priv: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `scheme` | `Option<Scheme>` | The scheme component of a URI |
| `authority` | `Option<Authority>` | The authority component of a URI |
| `path_and_query` | `Option<PathAndQuery>` | The origin-form component of a URI |
| `_priv` | `()` | Allow extending in the future |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: Uri) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **HttpTryFrom**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Parts { /* ... */ }
    ```

#### Struct `InvalidUri`

An error resulting from a failed attempt to construct a URI.

```rust
pub struct InvalidUri(pub(in ::uri) ErrorKind);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ErrorKind` |  |

##### Implementations

###### Trait Implementations

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
    fn from(src: ErrorKind) -> InvalidUri { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUri) -> Error { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
#### Struct `InvalidUriBytes`

An error resulting from a failed attempt to construct a URI.

```rust
pub struct InvalidUriBytes(pub(in ::uri) InvalidUri);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InvalidUri` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: ErrorKind) -> InvalidUriBytes { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUriBytes) -> Error { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `InvalidUriParts`

An error resulting from a failed attempt to construct a URI.

```rust
pub struct InvalidUriParts(pub(in ::uri) InvalidUri);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InvalidUri` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: ErrorKind) -> InvalidUriParts { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUriParts) -> Error { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Enum `ErrorKind`

```rust
pub(in ::uri) enum ErrorKind {
    InvalidUriChar,
    InvalidScheme,
    InvalidAuthority,
    InvalidPort,
    InvalidFormat,
    SchemeMissing,
    AuthorityMissing,
    PathAndQueryMissing,
    TooLong,
    Empty,
    SchemeTooLong,
}
```

##### Variants

###### `InvalidUriChar`

###### `InvalidScheme`

###### `InvalidAuthority`

###### `InvalidPort`

###### `InvalidFormat`

###### `SchemeMissing`

###### `AuthorityMissing`

###### `PathAndQueryMissing`

###### `TooLong`

###### `Empty`

###### `SchemeTooLong`

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ErrorKind) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: ErrorKind) -> InvalidUri { /* ... */ }
    ```

  - ```rust
    fn from(src: ErrorKind) -> InvalidUriBytes { /* ... */ }
    ```

  - ```rust
    fn from(src: ErrorKind) -> InvalidUriParts { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `parse_full`

```rust
pub(in ::uri) fn parse_full(s: bytes::Bytes) -> Result<Uri, InvalidUriBytes> { /* ... */ }
```

### Constants and Statics

#### Constant `MAX_LEN`

```rust
pub(in ::uri) const MAX_LEN: usize = _;
```

#### Constant `URI_CHARS`

```rust
pub(in ::uri) const URI_CHARS: [u8; 256] = _;
```

### Re-exports

#### Re-export `Authority`

```rust
pub use self::authority::Authority;
```

#### Re-export `Builder`

```rust
pub use self::builder::Builder;
```

#### Re-export `PathAndQuery`

```rust
pub use self::path::PathAndQuery;
```

#### Re-export `Scheme`

```rust
pub use self::scheme::Scheme;
```

#### Re-export `Port`

```rust
pub use self::port::Port;
```

## Module `byte_str`

```rust
pub(crate) mod byte_str { /* ... */ }
```

### Types

#### Struct `ByteStr`

```rust
pub(crate) struct ByteStr {
    pub(in ::byte_str) bytes: bytes::Bytes,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bytes` | `bytes::Bytes` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> ByteStr { /* ... */ }
  ```

- ```rust
  pub fn from_static(val: &''static str) -> ByteStr { /* ... */ }
  ```

- ```rust
  pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr { /* ... */ }
  ```

###### Trait Implementations

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **Receiver**
- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ByteStr) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(src: String) -> ByteStr { /* ... */ }
    ```

  - ```rust
    fn from(src: &''a str) -> ByteStr { /* ... */ }
    ```

  - ```rust
    fn from(src: ByteStr) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ByteStr) -> bool { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ByteStr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &str { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ByteStr { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `convert`

```rust
pub(crate) mod convert { /* ... */ }
```

### Traits

#### Trait `HttpTryFrom`

Private trait for the `http` crate to have generic methods with fallible
conversions.

This trait is similar to the `TryFrom` trait proposed in the standard
library, except this is specialized for the `http` crate and isn't intended
for general consumption.

This trait cannot be implemented types outside of the `http` crate, and is
only intended for use as a generic bound on methods in the `http` crate.

```rust
pub trait HttpTryFrom<T>: Sized + Sealed {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Error`: Associated error with the conversion this implementation represents.

##### Implementations

This trait is implemented for the following types:

- `HeaderMap<T>` with <''a, K, V, T>
- `HeaderName` with <''a>
- `HeaderName` with <''a>
- `HeaderName` with <''a>
- `HeaderName` with <''a>
- `HeaderName`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `HeaderValue` with <''a>
- `HeaderValue` with <''a>
- `HeaderValue` with <''a>
- `HeaderValue` with <''a>
- `HeaderValue`
- `HeaderValue`
- `HeaderValue`
- `Method` with <''a>
- `Method` with <''a>
- `Method` with <''a>
- `StatusCode` with <''a>
- `StatusCode` with <''a>
- `StatusCode` with <''a>
- `StatusCode`
- `Authority`
- `Authority` with <''a>
- `Authority` with <''a>
- `PathAndQuery`
- `PathAndQuery` with <''a>
- `PathAndQuery` with <''a>
- `Scheme`
- `Scheme` with <''a>
- `Scheme` with <''a>
- `Uri` with <''a>
- `Uri` with <''a>
- `Uri`
- `Uri`
- `Uri`
- `Uri` with <''a>
- `uri::Uri`
- `method::Method`
- `status::StatusCode`
- `header::HeaderName`
- `header::HeaderValue`
- `uri::Scheme`
- `uri::Authority`
- `uri::PathAndQuery`
- `header::HeaderMap<T>` with <T>

#### Trait `HttpTryInto`

```rust
pub(crate) trait HttpTryInto<T>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `http_try_into`

### Macros

#### Macro `reflexive`

```rust
pub(crate) macro_rules! reflexive {
    /* macro_rules! reflexive {
    ($($t:ty,)*) => { ... };
} */
}
```

## Module `error`

```rust
pub(crate) mod error { /* ... */ }
```

### Types

#### Struct `Error`

A generic "error" for HTTP connections

This error type is less specific than the error returned from other
functions in this crate, but all other errors can be converted to this
error. Consumers of this crate can typically consume and work with this form
of error for conversions with the `?` operator.

```rust
pub struct Error {
    pub(in ::error) inner: ErrorKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `ErrorKind` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is<T: error::Error + ''static>(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the underlying error has the same type as T.

- ```rust
  pub fn get_ref(self: &Self) -> &dyn error::Error + ''static { /* ... */ }
  ```
  Return a reference to the lower level, inner error.

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: status::InvalidStatusCode) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: method::InvalidMethod) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUri) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUriBytes) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: uri::InvalidUriParts) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: header::InvalidHeaderName) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: header::InvalidHeaderNameBytes) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: header::InvalidHeaderValue) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: header::InvalidHeaderValueBytes) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(never: Never) -> Error { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn cause(self: &Self) -> Option<&dyn error::Error> { /* ... */ }
    ```

#### Type Alias `Result`

A `Result` typedef to use with the `http::Error` type

```rust
pub type Result<T> = result::Result<T, Error>;
```

#### Enum `ErrorKind`

```rust
pub(in ::error) enum ErrorKind {
    StatusCode(status::InvalidStatusCode),
    Method(method::InvalidMethod),
    Uri(uri::InvalidUri),
    UriShared(uri::InvalidUriBytes),
    UriParts(uri::InvalidUriParts),
    HeaderName(header::InvalidHeaderName),
    HeaderNameShared(header::InvalidHeaderNameBytes),
    HeaderValue(header::InvalidHeaderValue),
    HeaderValueShared(header::InvalidHeaderValueBytes),
}
```

##### Variants

###### `StatusCode`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `status::InvalidStatusCode` |  |

###### `Method`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `method::InvalidMethod` |  |

###### `Uri`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `uri::InvalidUri` |  |

###### `UriShared`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `uri::InvalidUriBytes` |  |

###### `UriParts`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `uri::InvalidUriParts` |  |

###### `HeaderName`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `header::InvalidHeaderName` |  |

###### `HeaderNameShared`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `header::InvalidHeaderNameBytes` |  |

###### `HeaderValue`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `header::InvalidHeaderValue` |  |

###### `HeaderValueShared`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `header::InvalidHeaderValueBytes` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Enum `Never`

```rust
pub enum Never {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, _f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, _f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(never: Never) -> Error { /* ... */ }
    ```

- **Error**
  - ```rust
    fn description(self: &Self) -> &str { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `extensions`

```rust
pub(crate) mod extensions { /* ... */ }
```

### Types

#### Type Alias `AnyMap`

**Attributes:**

- `#[allow(warnings)]`

```rust
pub(in ::extensions) type AnyMap = std::collections::HashMap<std::any::TypeId, Box<dyn Any + Send + Sync>, std::hash::BuildHasherDefault<IdHasher>>;
```

#### Struct `IdHasher`

```rust
pub(in ::extensions) struct IdHasher(pub(in ::extensions) u64);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u64` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Hasher**
  - ```rust
    fn write(self: &mut Self, _: &[u8]) { /* ... */ }
    ```

  - ```rust
    fn write_u64(self: &mut Self, id: u64) { /* ... */ }
    ```

  - ```rust
    fn finish(self: &Self) -> u64 { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Default**
  - ```rust
    fn default() -> IdHasher { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
#### Struct `Extensions`

A type map of protocol extensions.

`Extensions` can be used by `Request` and `Response` to store
extra data derived from the underlying protocol.

```rust
pub struct Extensions {
    pub(in ::extensions) map: Option<Box<std::collections::HashMap<std::any::TypeId, Box<dyn Any + Send + Sync>, std::hash::BuildHasherDefault<IdHasher>>>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `map` | `Option<Box<std::collections::HashMap<std::any::TypeId, Box<dyn Any + Send + Sync>, std::hash::BuildHasherDefault<IdHasher>>>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Extensions { /* ... */ }
  ```
  Create an empty `Extensions`.

- ```rust
  pub fn insert<T: Send + Sync + ''static>(self: &mut Self, val: T) -> Option<T> { /* ... */ }
  ```
  Insert a type into this `Extensions`.

- ```rust
  pub fn get<T: Send + Sync + ''static>(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Get a reference to a type previously inserted on this `Extensions`.

- ```rust
  pub fn get_mut<T: Send + Sync + ''static>(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Get a mutable reference to a type previously inserted on this `Extensions`.

- ```rust
  pub fn remove<T: Send + Sync + ''static>(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Remove a type from this `Extensions`.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clear the `Extensions` of all inserted extensions.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> Extensions { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `sealed`

```rust
pub(crate) mod sealed { /* ... */ }
```

### Traits

#### Trait `Sealed`

Private trait to this crate to prevent traits from being implemented in
downstream crates.

```rust
pub trait Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `uri::Uri`
- `method::Method`
- `status::StatusCode`
- `header::HeaderName`
- `header::HeaderValue`
- `uri::Scheme`
- `uri::Authority`
- `uri::PathAndQuery`
- `header::HeaderMap<T>` with <T>

## Functions

### Function `_assert_types`

```rust
pub(crate) fn _assert_types() { /* ... */ }
```

## Re-exports

### Re-export `HttpTryFrom`

```rust
pub use convert::HttpTryFrom;
```

### Re-export `Error`

```rust
pub use error::Error;
```

### Re-export `Result`

```rust
pub use error::Result;
```

### Re-export `Extensions`

```rust
pub use extensions::Extensions;
```

### Re-export `HeaderMap`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use header::HeaderMap;
```

### Re-export `HeaderValue`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use header::HeaderValue;
```

### Re-export `Method`

```rust
pub use method::Method;
```

### Re-export `Request`

```rust
pub use request::Request;
```

### Re-export `Response`

```rust
pub use response::Response;
```

### Re-export `StatusCode`

```rust
pub use status::StatusCode;
```

### Re-export `Uri`

```rust
pub use uri::Uri;
```

### Re-export `Version`

```rust
pub use version::Version;
```

