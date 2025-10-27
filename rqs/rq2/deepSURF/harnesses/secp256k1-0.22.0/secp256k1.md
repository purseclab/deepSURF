# Crate Documentation

**Version:** 0.22.0

**Format Version:** 39

# Module `secp256k1`

Rust bindings for Pieter Wuille's secp256k1 library, which is used for
fast and accurate manipulation of ECDSA signatures on the secp256k1
curve. Such signatures are used extensively by the Bitcoin network
and its derivatives.

To minimize dependencies, some functions are feature-gated. To generate
random keys or to re-randomize a context object, compile with the
`rand-std` feature. If you are willing to use the `rand-std` feature, we
have enabled an additional defense-in-depth sidechannel protection for
our context objects, which re-blinds certain operations on secret key
data. To de/serialize objects with serde, compile with "serde".
**Important**: `serde` encoding is **not** the same as consensus
encoding!

Where possible, the bindings use the Rust type system to ensure that
API usage errors are impossible. For example, the library uses context
objects that contain precomputation tables which are created on object
construction. Since this is a slow operation (10+ milliseconds, vs ~50
microseconds for typical crypto operations, on a 2.70 Ghz i7-6820HQ)
the tables are optional, giving a performance boost for users who only
care about signing, only care about verification, or only care about
parsing. In the upstream library, if you attempt to sign a message using
a context that does not support this, it will trigger an assertion
failure and terminate the program. In `rust-secp256k1`, this is caught
at compile-time; in fact, it is impossible to compile code that will
trigger any assertion failures in the upstream library.

```rust
# #[cfg(all(feature = "std", feature="rand-std", feature="bitcoin_hashes"))] {
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::sha256;

let secp = Secp256k1::new();
let mut rng = OsRng::new().expect("OsRng");
let (secret_key, public_key) = secp.generate_keypair(&mut rng);
let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

let sig = secp.sign_ecdsa(&message, &secret_key);
assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
# }
```

If the "global-context" feature is enabled you have access to an alternate API.

```rust
# #[cfg(all(feature="global-context", feature = "std", feature="rand-std", features = "bitcoin_hashes"))] {
use secp256k1::rand::thread_rng;
use secp256k1::{generate_keypair, Message};
use secp256k1::hashes::sha256;

let (secret_key, public_key) = generate_keypair(&mut thread_rng());
let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

let sig = secret_key.sign_ecdsa(&message, &secret_key);
assert!(sig.verify(&message, &public_key).is_ok());
# }
```

The above code requires `rust-secp256k1` to be compiled with the `rand-std` and `bitcoin_hashes`
feature enabled, to get access to [`generate_keypair`](struct.Secp256k1.html#method.generate_keypair)
Alternately, keys and messages can be parsed from slices, like

```rust
# #[cfg(any(feature = "alloc", features = "std"))] {
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};

let secp = Secp256k1::new();
let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
let public_key = PublicKey::from_secret_key(&secp, &secret_key);
// This is unsafe unless the supplied byte slice is the output of a cryptographic hash function.
// See the above example for how to use this library together with `bitcoin_hashes`.
let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

let sig = secp.sign_ecdsa(&message, &secret_key);
assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
# }
```

Users who only want to verify signatures can use a cheaper context, like so:

```rust
# #[cfg(any(feature = "alloc", feature = "std"))] {
use secp256k1::{Secp256k1, Message, ecdsa, PublicKey};

let secp = Secp256k1::verification_only();

let public_key = PublicKey::from_slice(&[
    0x02,
    0xc6, 0x6e, 0x7d, 0x89, 0x66, 0xb5, 0xc5, 0x55,
    0xaf, 0x58, 0x05, 0x98, 0x9d, 0xa9, 0xfb, 0xf8,
    0xdb, 0x95, 0xe1, 0x56, 0x31, 0xce, 0x35, 0x8c,
    0x3a, 0x17, 0x10, 0xc9, 0x62, 0x67, 0x90, 0x63,
]).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");

let message = Message::from_slice(&[
    0xaa, 0xdf, 0x7d, 0xe7, 0x82, 0x03, 0x4f, 0xbe,
    0x3d, 0x3d, 0xb2, 0xcb, 0x13, 0xc0, 0xcd, 0x91,
    0xbf, 0x41, 0xcb, 0x08, 0xfa, 0xc7, 0xbd, 0x61,
    0xd5, 0x44, 0x53, 0xcf, 0x6e, 0x82, 0xb4, 0x50,
]).expect("messages must be 32 bytes and are expected to be hashes");

let sig = ecdsa::Signature::from_compact(&[
    0xdc, 0x4d, 0xc2, 0x64, 0xa9, 0xfe, 0xf1, 0x7a,
    0x3f, 0x25, 0x34, 0x49, 0xcf, 0x8c, 0x39, 0x7a,
    0xb6, 0xf1, 0x6f, 0xb3, 0xd6, 0x3d, 0x86, 0x94,
    0x0b, 0x55, 0x86, 0x82, 0x3d, 0xfd, 0x02, 0xae,
    0x3b, 0x46, 0x1b, 0xb4, 0x33, 0x6b, 0x5e, 0xcb,
    0xae, 0xfd, 0x66, 0x27, 0xaa, 0x92, 0x2e, 0xfc,
    0x04, 0x8f, 0xec, 0x0c, 0x88, 0x1c, 0x10, 0xc4,
    0xc9, 0x42, 0x8f, 0xca, 0x69, 0xc1, 0x32, 0xa2,
]).expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");

# #[cfg(not(fuzzing))]
assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
# }
```

Observe that the same code using, say [`signing_only`](struct.Secp256k1.html#method.signing_only)
to generate a context would simply not compile.

## Crate features/optional dependencies

This crate provides the following opt-in Cargo features:

* `std` - use standard Rust library, enabled by default.
* `alloc` - use the `alloc` standard Rust library to provide heap allocations.
* `rand` - use `rand` library to provide random generator (e.g. to generate keys).
* `rand-std` - use `rand` library with its `std` feature enabled. (Implies `rand`.)
* `recovery` - enable functions that can compute the public key from signature.
* `lowmemory` - optimize the library for low-memory environments.
* `global-context` - enable use of global secp256k1 context (implies `std`).
* `serde` - implements serialization and deserialization for types in this crate using `serde`.
          **Important**: `serde` encoding is **not** the same as consensus encoding!
* `bitcoin_hashes` - enables interaction with the `bitcoin-hashes` crate (e.g. conversions).

## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

```rust
pub(crate) mod macros { /* ... */ }
```

### Macros

#### Macro `impl_pretty_debug`

```rust
pub(crate) macro_rules! impl_pretty_debug {
    /* macro_rules! impl_pretty_debug {
    ($thing:ident) => { ... };
} */
}
```

## Module `secret`

**Attributes:**

- `#[macro_use]`

Helpers for displaying secret values

```rust
pub(crate) mod secret { /* ... */ }
```

### Types

#### Struct `DisplaySecret`

Helper struct for safely printing secrets (like [`SecretKey`] value).
Formats the explicit byte value of the secret kept inside the type as a
little-endian hexadecimal string using the provided formatter.

Secrets should not implement neither [`Debug`] and [`Display`] traits directly,
and instead provide `fn display_secret<'a>(&'a self) -> DisplaySecret<'a>`
function to be used in different display contexts (see "examples" below).

[`Display`]: fmt::Display
[`Debug`]: fmt::Debug

```rust
pub struct DisplaySecret {
    pub(in ::secret) secret: [u8; 32],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `secret` | `[u8; 32]` |  |

##### Implementations

###### Trait Implementations

- **Eq**
- **Copy**
- **Sync**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &DisplaySecret) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &DisplaySecret) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DisplaySecret { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DisplaySecret) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

### Macros

#### Macro `impl_display_secret`

```rust
pub(crate) macro_rules! impl_display_secret {
    /* macro_rules! impl_display_secret {
    ($thing:ident) => { ... };
} */
}
```

## Module `context`

```rust
pub(crate) mod context { /* ... */ }
```

### Modules

## Module `private`

```rust
pub(in ::context) mod private { /* ... */ }
```

### Traits

#### Trait `Sealed`

```rust
pub trait Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `AllPreallocated<''buf>` with <''buf>
- `VerifyOnlyPreallocated<''buf>` with <''buf>
- `SignOnlyPreallocated<''buf>` with <''buf>
- `SignOnly`
- `All`
- `VerifyOnly`

## Module `alloc_only`

**Attributes:**

- `#[cfg(any(feature = "std", feature = "alloc"))]`

```rust
pub(in ::context) mod alloc_only { /* ... */ }
```

### Types

#### Enum `SignOnly`

Represents the set of capabilities needed for signing.

```rust
pub enum SignOnly {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Eq**
- **Signing**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SignOnly) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SignOnly) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SignOnly) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SignOnly { /* ... */ }
    ```

- **Context**
  - ```rust
    unsafe fn deallocate(ptr: *mut u8, size: usize) { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sealed**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `VerifyOnly`

Represents the set of capabilities needed for verification.

```rust
pub enum VerifyOnly {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VerifyOnly) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &VerifyOnly) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Verification**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Context**
  - ```rust
    unsafe fn deallocate(ptr: *mut u8, size: usize) { /* ... */ }
    ```

- **Sealed**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &VerifyOnly) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> VerifyOnly { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

#### Enum `All`

Represents the set of all capabilities.

```rust
pub enum All {
}
```

##### Variants

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Verification**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> All { /* ... */ }
    ```

- **Send**
- **Signing**
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

- **Freeze**
- **RefUnwindSafe**
- **Sealed**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &All) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &All) -> bool { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &All) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Context**
  - ```rust
    unsafe fn deallocate(ptr: *mut u8, size: usize) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Constants and Statics

#### Constant `ALIGN_TO`

```rust
pub(in ::context::alloc_only) const ALIGN_TO: usize = _;
```

### Types

#### Struct `SignOnlyPreallocated`

Represents the set of capabilities needed for signing with a user preallocated memory.

```rust
pub struct SignOnlyPreallocated<''buf> {
    pub(in ::context) phantom: core::marker::PhantomData<&''buf ()>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `phantom` | `core::marker::PhantomData<&''buf ()>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Unpin**
- **Copy**
- **Sealed**
- **Signing**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SignOnlyPreallocated<''buf>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SignOnlyPreallocated<''buf>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **RefUnwindSafe**
- **Context**
  - ```rust
    unsafe fn deallocate(_ptr: *mut u8, _size: usize) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SignOnlyPreallocated<''buf> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SignOnlyPreallocated<''buf>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `VerifyOnlyPreallocated`

Represents the set of capabilities needed for verification with a user preallocated memory.

```rust
pub struct VerifyOnlyPreallocated<''buf> {
    pub(in ::context) phantom: core::marker::PhantomData<&''buf ()>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `phantom` | `core::marker::PhantomData<&''buf ()>` |  |

##### Implementations

###### Trait Implementations

- **Context**
  - ```rust
    unsafe fn deallocate(_ptr: *mut u8, _size: usize) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VerifyOnlyPreallocated<''buf> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &VerifyOnlyPreallocated<''buf>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Verification**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &VerifyOnlyPreallocated<''buf>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sealed**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VerifyOnlyPreallocated<''buf>) -> bool { /* ... */ }
    ```

#### Struct `AllPreallocated`

Represents the set of all capabilities with a user preallocated memory.

```rust
pub struct AllPreallocated<''buf> {
    pub(in ::context) phantom: core::marker::PhantomData<&''buf ()>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `phantom` | `core::marker::PhantomData<&''buf ()>` |  |

##### Implementations

###### Trait Implementations

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &AllPreallocated<''buf>) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AllPreallocated<''buf>) -> bool { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &AllPreallocated<''buf>) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **Verification**
- **Context**
  - ```rust
    unsafe fn deallocate(_ptr: *mut u8, _size: usize) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sealed**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Unpin**
- **Signing**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AllPreallocated<''buf> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
### Traits

#### Trait `Context`

A trait for all kinds of contexts that lets you define the exact flags and a function to
deallocate memory. It isn't possible to implement this for types outside this crate.

```rust
pub unsafe trait Context: private::Sealed {
    /* Associated items */
}
```

> This trait is unsafe to implement.

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `FLAGS`: Flags for the ffi.
- `DESCRIPTION`: A constant description of the context.

###### Required Methods

- `deallocate`: A function to deallocate the memory when the context is dropped.

##### Implementations

This trait is implemented for the following types:

- `SignOnly`
- `VerifyOnly`
- `All`
- `SignOnlyPreallocated<''buf>` with <''buf>
- `VerifyOnlyPreallocated<''buf>` with <''buf>
- `AllPreallocated<''buf>` with <''buf>

#### Trait `Signing`

Marker trait for indicating that an instance of `Secp256k1` can be used for signing.

```rust
pub trait Signing: Context {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `SignOnly`
- `All`
- `SignOnlyPreallocated<''buf>` with <''buf>
- `AllPreallocated<''buf>` with <''buf>

#### Trait `Verification`

Marker trait for indicating that an instance of `Secp256k1` can be used for verification.

```rust
pub trait Verification: Context {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Implementations

This trait is implemented for the following types:

- `VerifyOnly`
- `All`
- `VerifyOnlyPreallocated<''buf>` with <''buf>
- `AllPreallocated<''buf>` with <''buf>

### Re-exports

#### Re-export `self::alloc_only::*`

**Attributes:**

- `#[cfg(any(feature = "std", feature = "alloc"))]`

```rust
pub use self::alloc_only::*;
```

## Module `key`

Public and secret keys.


```rust
pub(crate) mod key { /* ... */ }
```

### Types

#### Struct `SecretKey`

Secret 256-bit key used as `x` in an ECDSA signature.

# Examples

Basic usage:

```
# #[cfg(all(feature = "std", feature =  "rand-std"))] {
use secp256k1::{rand, Secp256k1, SecretKey};

let secp = Secp256k1::new();
let secret_key = SecretKey::new(&mut rand::thread_rng());
# }
```

```rust
pub struct SecretKey(pub(in ::key) [u8; 32]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 32]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn display_secret(self: &Self) -> DisplaySecret { /* ... */ }
  ```
  Formats the explicit byte value of the secret key kept inside the type as a

- ```rust
  pub fn as_ptr(self: &Self) -> *const u8 { /* ... */ }
  ```
  Converts the object to a raw pointer for FFI interfacing

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut u8 { /* ... */ }
  ```
  Converts the object to a mutable raw pointer for FFI interfacing

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of the object as an array

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the object as an array is empty

- ```rust
  pub fn from_slice(data: &[u8]) -> Result<SecretKey, Error> { /* ... */ }
  ```
  Converts a `SECRET_KEY_SIZE`-byte slice to a secret key.

- ```rust
  pub fn from_keypair(keypair: &KeyPair) -> Self { /* ... */ }
  ```
  Creates a new secret key using data from BIP-340 [`KeyPair`].

- ```rust
  pub fn secret_bytes(self: &Self) -> [u8; 32] { /* ... */ }
  ```
  Returns the secret key as a byte value.

- ```rust
  pub fn negate_assign(self: &mut Self) { /* ... */ }
  ```
  Negates one secret key.

- ```rust
  pub fn add_assign(self: &mut Self, other: &[u8]) -> Result<(), Error> { /* ... */ }
  ```
  Adds one secret key to another, modulo the curve order.

- ```rust
  pub fn mul_assign(self: &mut Self, other: &[u8]) -> Result<(), Error> { /* ... */ }
  ```
  Multiplies one secret key by another, modulo the curve order. Will

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SecretKey { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SecretKey) -> bool { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Unpin**
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SecretKey) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 32] { /* ... */ }
    ```
    Gets a reference to the underlying array

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &u8 { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::Range<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeTo<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeFrom<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, _: ::core::ops::RangeFull) -> &[u8] { /* ... */ }
    ```

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(pair: KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(pair: &''a KeyPair) -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: ::core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<SecretKey, Error> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SecretKey) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `PublicKey`

**Attributes:**

- `#[repr(transparent)]`

A Secp256k1 public key, used for verification of signatures.

# Examples

Basic usage:

```
# #[cfg(any(feature =  "alloc", feature = "std"))] {
use secp256k1::{SecretKey, Secp256k1, PublicKey};

let secp = Secp256k1::new();
let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
let public_key = PublicKey::from_secret_key(&secp, &secret_key);
# }
```

```rust
pub struct PublicKey(pub(in ::key) ffi::PublicKey);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ffi::PublicKey` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::PublicKey { /* ... */ }
  ```
  Obtains a raw const pointer suitable for use with FFI functions.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut ffi::PublicKey { /* ... */ }
  ```
  Obtains a raw mutable pointer suitable for use with FFI functions.

- ```rust
  pub fn from_secret_key<C: Signing>(secp: &Secp256k1<C>, sk: &SecretKey) -> PublicKey { /* ... */ }
  ```
  Creates a new public key from a [`SecretKey`].

- ```rust
  pub fn from_slice(data: &[u8]) -> Result<PublicKey, Error> { /* ... */ }
  ```
  Creates a public key directly from a slice.

- ```rust
  pub fn from_keypair(keypair: &KeyPair) -> Self { /* ... */ }
  ```
  Creates a new compressed public key using data from BIP-340 [`KeyPair`].

- ```rust
  pub fn serialize(self: &Self) -> [u8; 33] { /* ... */ }
  ```
  Serializes the key as a byte-encoded pair of values. In compressed form the y-coordinate is

- ```rust
  pub fn serialize_uncompressed(self: &Self) -> [u8; 65] { /* ... */ }
  ```
  Serializes the key as a byte-encoded pair of values, in uncompressed form.

- ```rust
  pub fn negate_assign<C: Verification>(self: &mut Self, secp: &Secp256k1<C>) { /* ... */ }
  ```
  Negates the public key in place.

- ```rust
  pub fn add_exp_assign<C: Verification>(self: &mut Self, secp: &Secp256k1<C>, other: &[u8]) -> Result<(), Error> { /* ... */ }
  ```
  Adds the `other` public key to `self` in place.

- ```rust
  pub fn mul_assign<C: Verification>(self: &mut Self, secp: &Secp256k1<C>, other: &[u8]) -> Result<(), Error> { /* ... */ }
  ```
  Muliplies the public key in place by the scalar `other`.

- ```rust
  pub fn combine(self: &Self, other: &PublicKey) -> Result<PublicKey, Error> { /* ... */ }
  ```
   Adds a second key to this one, returning the sum.

- ```rust
  pub fn combine_keys(keys: &[&PublicKey]) -> Result<PublicKey, Error> { /* ... */ }
  ```
  Adds the keys in the provided slice together, returning the sum.

###### Trait Implementations

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(pk: ffi::PublicKey) -> PublicKey { /* ... */ }
    ```

  - ```rust
    fn from(pair: KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(pair: &''a KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(src: ::key::PublicKey) -> XOnlyPublicKey { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PublicKey { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PublicKey) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

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

- **Copy**
- **RefUnwindSafe**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<PublicKey, Error> { /* ... */ }
    ```

- **UnwindSafe**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &PublicKey) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PublicKey) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
#### Struct `KeyPair`

Opaque data structure that holds a keypair consisting of a secret and a public key.

# Serde support

[`Serialize`] and [`Deserialize`] are not implemented for this type, even with the `serde`
feature active. This is due to security considerations, see the [`serde_keypair`] documentation
for details.

If the `serde` and `global-context` features are active `KeyPair`s can be serialized and
deserialized by annotating them with `#[serde(with = "secp256k1::serde_keypair")]`
inside structs or enums for which [`Serialize`] and [`Deserialize`] are being derived.

# Examples

Basic usage:

```
# #[cfg(all(feature = "std", feature =  "rand-std"))] {
use secp256k1::{rand, KeyPair, Secp256k1};

let secp = Secp256k1::new();
let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
let key_pair = KeyPair::from_secret_key(&secp, secret_key);
# }
```
[`Deserialize`]: serde::Deserialize
[`Serialize`]: serde::Serialize

```rust
pub struct KeyPair(pub(in ::key) ffi::KeyPair);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ffi::KeyPair` |  |

##### Implementations

###### Methods

- ```rust
  pub fn display_secret(self: &Self) -> DisplaySecret { /* ... */ }
  ```
  Formats the explicit byte value of the secret key kept inside the type as a

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::KeyPair { /* ... */ }
  ```
  Obtains a raw const pointer suitable for use with FFI functions.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut ffi::KeyPair { /* ... */ }
  ```
  Obtains a raw mutable pointer suitable for use with FFI functions.

- ```rust
  pub fn from_secret_key<C: Signing>(secp: &Secp256k1<C>, sk: SecretKey) -> KeyPair { /* ... */ }
  ```
  Creates a Schnorr [`KeyPair`] directly from generic Secp256k1 secret key.

- ```rust
  pub fn from_seckey_slice<C: Signing>(secp: &Secp256k1<C>, data: &[u8]) -> Result<KeyPair, Error> { /* ... */ }
  ```
  Creates a Schnorr [`KeyPair`] directly from a secret key slice.

- ```rust
  pub fn from_seckey_str<C: Signing>(secp: &Secp256k1<C>, s: &str) -> Result<KeyPair, Error> { /* ... */ }
  ```
  Creates a Schnorr [`KeyPair`] directly from a secret key string.

- ```rust
  pub fn secret_bytes(self: &Self) -> [u8; 32] { /* ... */ }
  ```
  Returns the secret bytes for this key pair.

- ```rust
  pub fn tweak_add_assign<C: Verification>(self: &mut Self, secp: &Secp256k1<C>, tweak: &[u8]) -> Result<(), Error> { /* ... */ }
  ```
  Tweaks a keypair by adding the given tweak to the secret key and updating the public key

- ```rust
  pub fn public_key(self: &Self) -> XOnlyPublicKey { /* ... */ }
  ```
  Gets the [XOnlyPublicKey] for this [KeyPair].

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **Eq**
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

  - ```rust
    fn from(pair: KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(pair: &''a KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(pair: KeyPair) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(pair: &''a KeyPair) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &KeyPair) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> KeyPair { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &KeyPair) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &KeyPair) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `XOnlyPublicKey`

An x-only public key, used for verification of Schnorr signatures and serialized according to BIP-340.

# Examples

Basic usage:

```
# #[cfg(all(feature = "std", feature =  "rand-std"))] {
use secp256k1::{rand, Secp256k1, KeyPair, XOnlyPublicKey};

let secp = Secp256k1::new();
let key_pair = KeyPair::new(&secp, &mut rand::thread_rng());
let xonly = XOnlyPublicKey::from_keypair(&key_pair);
# }
```

```rust
pub struct XOnlyPublicKey(pub(in ::key) ffi::XOnlyPublicKey);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ffi::XOnlyPublicKey` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::XOnlyPublicKey { /* ... */ }
  ```
  Obtains a raw const pointer suitable for use with FFI functions.

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut ffi::XOnlyPublicKey { /* ... */ }
  ```
  Obtains a raw mutable pointer suitable for use with FFI functions.

- ```rust
  pub fn from_keypair(keypair: &KeyPair) -> XOnlyPublicKey { /* ... */ }
  ```
  Creates a new Schnorr public key from a Schnorr key pair.

- ```rust
  pub fn from_slice(data: &[u8]) -> Result<XOnlyPublicKey, Error> { /* ... */ }
  ```
  Creates a Schnorr public key directly from a slice.

- ```rust
  pub fn serialize(self: &Self) -> [u8; 32] { /* ... */ }
  ```
  Serializes the key as a byte-encoded x coordinate value (32 bytes).

- ```rust
  pub fn tweak_add_assign<V: Verification>(self: &mut Self, secp: &Secp256k1<V>, tweak: &[u8]) -> Result<Parity, Error> { /* ... */ }
  ```
  Tweaks an x-only PublicKey by adding the generator multiplied with the given tweak to it.

- ```rust
  pub fn tweak_add_check<V: Verification>(self: &Self, secp: &Secp256k1<V>, tweaked_key: &Self, tweaked_parity: Parity, tweak: [u8; 32]) -> bool { /* ... */ }
  ```
  Verifies that a tweak produced by [`XOnlyPublicKey::tweak_add_assign`] was computed correctly.

###### Trait Implementations

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
    fn from(pk: ffi::XOnlyPublicKey) -> XOnlyPublicKey { /* ... */ }
    ```

  - ```rust
    fn from(src: ::key::PublicKey) -> XOnlyPublicKey { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &XOnlyPublicKey) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **Eq**
- **UnwindSafe**
- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &XOnlyPublicKey) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> XOnlyPublicKey { /* ... */ }
    ```

- **Freeze**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &XOnlyPublicKey) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<XOnlyPublicKey, Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

#### Enum `Parity`

Represents the parity passed between FFI function calls.

```rust
pub enum Parity {
    Even = 0,
    Odd = 1,
}
```

##### Variants

###### `Even`

Even parity.

Discriminant: `0`

Discriminant value: `0`

###### `Odd`

Odd parity.

Discriminant: `1`

Discriminant value: `1`

##### Implementations

###### Methods

- ```rust
  pub fn to_u8(self: Self) -> u8 { /* ... */ }
  ```
  Converts parity into an integer (byte) value.

- ```rust
  pub fn to_i32(self: Self) -> i32 { /* ... */ }
  ```
  Converts parity into an integer value.

- ```rust
  pub fn from_u8(parity: u8) -> Result<Parity, InvalidParityValue> { /* ... */ }
  ```
  Constructs a [`Parity`] from a byte.

- ```rust
  pub fn from_i32(parity: i32) -> Result<Parity, InvalidParityValue> { /* ... */ }
  ```
  Constructs a [`Parity`] from a signed integer.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Parity) -> <Self as >::Output { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Parity { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Parity) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

  - ```rust
    fn from(parity: Parity) -> i32 { /* ... */ }
    ```

  - ```rust
    fn from(parity: Parity) -> u8 { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Parity) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Parity) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
#### Struct `InvalidParityValue`

Error returned when conversion from an integer to `Parity` fails.

```rust
pub struct InvalidParityValue(pub(in ::key) i32);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvalidParityValue { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
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

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InvalidParityValue) -> bool { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(error: InvalidParityValue) -> Self { /* ... */ }
    ```

- **Eq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &InvalidParityValue) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Error**
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **StructuralPartialEq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &InvalidParityValue) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Constants and Statics

#### Constant `ONE_KEY`

The number 1 encoded as a secret key.

```rust
pub const ONE_KEY: SecretKey = _;
```

## Module `constants`

Constants related to the API and the underlying curve.


```rust
pub mod constants { /* ... */ }
```

### Constants and Statics

#### Constant `MESSAGE_SIZE`

The size (in bytes) of a message.

```rust
pub const MESSAGE_SIZE: usize = 32;
```

#### Constant `SECRET_KEY_SIZE`

The size (in bytes) of a secret key.

```rust
pub const SECRET_KEY_SIZE: usize = 32;
```

#### Constant `PUBLIC_KEY_SIZE`

The size (in bytes) of a serialized public key.

```rust
pub const PUBLIC_KEY_SIZE: usize = 33;
```

#### Constant `UNCOMPRESSED_PUBLIC_KEY_SIZE`

The size (in bytes) of an serialized uncompressed public key.

```rust
pub const UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = 65;
```

#### Constant `MAX_SIGNATURE_SIZE`

The maximum size of a signature.

```rust
pub const MAX_SIGNATURE_SIZE: usize = 72;
```

#### Constant `COMPACT_SIGNATURE_SIZE`

The maximum size of a compact signature.

```rust
pub const COMPACT_SIGNATURE_SIZE: usize = 64;
```

#### Constant `SCHNORR_SIGNATURE_SIZE`

The size of a Schnorr signature.

```rust
pub const SCHNORR_SIGNATURE_SIZE: usize = 64;
```

#### Constant `SCHNORRSIG_SIGNATURE_SIZE`

**Attributes:**

- `#[deprecated(since = "0.22.0", note = "Use SCHNORR_SIGNATURE_SIZE instead.")]`

**⚠️ Deprecated since 0.22.0**: Use SCHNORR_SIGNATURE_SIZE instead.

The size of a Schnorr signature.

```rust
pub const SCHNORRSIG_SIGNATURE_SIZE: usize = SCHNORR_SIGNATURE_SIZE;
```

#### Constant `SCHNORR_PUBLIC_KEY_SIZE`

The size of a Schnorr public key.

```rust
pub const SCHNORR_PUBLIC_KEY_SIZE: usize = 32;
```

#### Constant `SCHNORRSIG_PUBLIC_KEY_SIZE`

**Attributes:**

- `#[deprecated(since = "0.22.0", note = "Use SCHNORR_PUBLIC_KEY_SIZE instead.")]`

**⚠️ Deprecated since 0.22.0**: Use SCHNORR_PUBLIC_KEY_SIZE instead.

The size of a Schnorr public key.

```rust
pub const SCHNORRSIG_PUBLIC_KEY_SIZE: usize = SCHNORR_PUBLIC_KEY_SIZE;
```

#### Constant `KEY_PAIR_SIZE`

The size of a key pair.

```rust
pub const KEY_PAIR_SIZE: usize = 96;
```

#### Constant `FIELD_SIZE`

The Prime for the secp256k1 field element.

```rust
pub const FIELD_SIZE: [u8; 32] = _;
```

#### Constant `CURVE_ORDER`

The order of the secp256k1 curve.

```rust
pub const CURVE_ORDER: [u8; 32] = _;
```

#### Constant `GENERATOR_X`

The X coordinate of the generator.

```rust
pub const GENERATOR_X: [u8; 32] = _;
```

#### Constant `GENERATOR_Y`

The Y coordinate of the generator.

```rust
pub const GENERATOR_Y: [u8; 32] = _;
```

## Module `ecdh`

Support for shared secret computations.


```rust
pub mod ecdh { /* ... */ }
```

### Types

#### Struct `SharedSecret`

Enables two parties to create a shared secret without revealing their own secrets.

# Examples

```
# #[cfg(all(feature = "std", feature = "rand-std"))] {
# use secp256k1::Secp256k1;
# use secp256k1::ecdh::SharedSecret;
# use secp256k1::rand::thread_rng;
let s = Secp256k1::new();
let (sk1, pk1) = s.generate_keypair(&mut thread_rng());
let (sk2, pk2) = s.generate_keypair(&mut thread_rng());
let sec1 = SharedSecret::new(&pk2, &sk1);
let sec2 = SharedSecret::new(&pk1, &sk2);
assert_eq!(sec1, sec2);
# }

```rust
pub struct SharedSecret(pub(in ::ecdh) [u8; 32]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 32]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn display_secret(self: &Self) -> DisplaySecret { /* ... */ }
  ```
  Formats the explicit byte value of the shared secret kept inside the type as a

- ```rust
  pub fn new(point: &PublicKey, scalar: &SecretKey) -> SharedSecret { /* ... */ }
  ```
  Creates a new shared secret from a pubkey and secret key.

- ```rust
  pub fn secret_bytes(self: &Self) -> [u8; 32] { /* ... */ }
  ```
  Returns the shared secret as a byte value.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SharedSecret { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SharedSecret) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SharedSecret) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SharedSecret) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `shared_secret_point`

Creates a shared point from public key and secret key.

**Important: use of a strong cryptographic hash function may be critical to security! Do NOT use
unless you understand cryptographical implications.** If not, use SharedSecret instead.

Can be used like `SharedSecret` but caller is responsible for then hashing the returned buffer.
This allows for the use of a custom hash function since `SharedSecret` uses SHA256.

# Returns

64 bytes representing the (x,y) co-ordinates of a point on the curve (32 bytes each).

# Examples
```
# #[cfg(all(feature = "bitcoin_hashes", feature = "rand-std", feature = "std"))] {
# use secp256k1::{ecdh, Secp256k1, PublicKey, SecretKey};
# use secp256k1::hashes::{Hash, sha512};
# use secp256k1::rand::thread_rng;

let s = Secp256k1::new();
let (sk1, pk1) = s.generate_keypair(&mut thread_rng());
let (sk2, pk2) = s.generate_keypair(&mut thread_rng());

let point1 = ecdh::shared_secret_point(&pk2, &sk1);
let secret1 = sha512::Hash::hash(&point1);
let point2 = ecdh::shared_secret_point(&pk1, &sk2);
let secret2 = sha512::Hash::hash(&point2);
assert_eq!(secret1, secret2)
# }
```

```rust
pub fn shared_secret_point(point: &key::PublicKey, scalar: &key::SecretKey) -> [u8; 64] { /* ... */ }
```

#### Function `c_callback`

```rust
pub(in ::ecdh) unsafe extern "C" fn c_callback(output: *mut secp256k1_sys::types::c_uchar, x: *const secp256k1_sys::types::c_uchar, y: *const secp256k1_sys::types::c_uchar, _data: *mut secp256k1_sys::types::c_void) -> secp256k1_sys::types::c_int { /* ... */ }
```

### Constants and Statics

#### Constant `SHARED_SECRET_SIZE`

```rust
pub(in ::ecdh) const SHARED_SECRET_SIZE: usize = constants::SECRET_KEY_SIZE;
```

## Module `ecdsa`

Structs and functionality related to the ECDSA signature algorithm.

```rust
pub mod ecdsa { /* ... */ }
```

### Types

#### Struct `Signature`

An ECDSA signature

```rust
pub struct Signature(pub(crate) ffi::Signature);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ffi::Signature` |  |

##### Implementations

###### Methods

- ```rust
  pub fn from_der(data: &[u8]) -> Result<Signature, Error> { /* ... */ }
  ```
  Converts a DER-encoded byte slice to a signature

- ```rust
  pub fn from_compact(data: &[u8]) -> Result<Signature, Error> { /* ... */ }
  ```
  Converts a 64-byte compact-encoded byte slice to a signature

- ```rust
  pub fn from_der_lax(data: &[u8]) -> Result<Signature, Error> { /* ... */ }
  ```
  Converts a "lax DER"-encoded byte slice to a signature. This is basically

- ```rust
  pub fn normalize_s(self: &mut Self) { /* ... */ }
  ```
  Normalizes a signature to a "low S" form. In ECDSA, signatures are

- ```rust
  pub fn as_ptr(self: &Self) -> *const ffi::Signature { /* ... */ }
  ```
  Obtains a raw pointer suitable for use with FFI functions

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut ffi::Signature { /* ... */ }
  ```
  Obtains a raw mutable pointer suitable for use with FFI functions

- ```rust
  pub fn serialize_der(self: &Self) -> SerializedSignature { /* ... */ }
  ```
  Serializes the signature in DER format

- ```rust
  pub fn serialize_compact(self: &Self) -> [u8; 64] { /* ... */ }
  ```
  Serializes the signature in compact format

###### Trait Implementations

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Signature) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(sig: ffi::Signature) -> Signature { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Signature { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Signature, Error> { /* ... */ }
    ```

#### Struct `SerializedSignature`

A DER serialized Signature

```rust
pub struct SerializedSignature {
    pub(in ::ecdsa) data: [u8; 72],
    pub(in ::ecdsa) len: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `[u8; 72]` |  |
| `len` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn get_data_mut_ptr(self: &mut Self) -> *mut u8 { /* ... */ }
  ```
  Get a pointer to the underlying data with the specified capacity.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Get the capacity of the underlying data buffer.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Get the len of the used data.

- ```rust
  pub(crate) fn set_len(self: &mut Self, len: usize) { /* ... */ }
  ```
  Set the length of the object.

- ```rust
  pub fn to_signature(self: &Self) -> Result<Signature, Error> { /* ... */ }
  ```
  Convert the serialized signature into the Signature struct.

- ```rust
  pub fn from_signature(sig: &Signature) -> SerializedSignature { /* ... */ }
  ```
  Create a SerializedSignature from a Signature.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Check if the space is zero.

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SerializedSignature { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SerializedSignature) -> bool { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &[u8] { /* ... */ }
    ```

- **Unpin**
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
    fn clone(self: &Self) -> SerializedSignature { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Receiver**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
### Functions

#### Function `compact_sig_has_zero_first_bit`

```rust
pub(crate) fn compact_sig_has_zero_first_bit(sig: &ffi::Signature) -> bool { /* ... */ }
```

#### Function `der_length_check`

```rust
pub(crate) fn der_length_check(sig: &ffi::Signature, max_len: usize) -> bool { /* ... */ }
```

## Module `schnorr`

# schnorrsig
Support for Schnorr signatures.


```rust
pub mod schnorr { /* ... */ }
```

### Types

#### Struct `Signature`

Represents a Schnorr signature.

```rust
pub struct Signature(pub(in ::schnorr) [u8; 64]);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 64]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_ptr(self: &Self) -> *const u8 { /* ... */ }
  ```
  Converts the object to a raw pointer for FFI interfacing

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut u8 { /* ... */ }
  ```
  Converts the object to a mutable raw pointer for FFI interfacing

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of the object as an array

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the object as an array is empty

- ```rust
  pub fn from_slice(data: &[u8]) -> Result<Signature, Error> { /* ... */ }
  ```
  Creates a Signature directly from a slice

###### Trait Implementations

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Signature { /* ... */ }
    ```

- **Unpin**
- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &u8 { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::Range<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeTo<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeFrom<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, _: ::core::ops::RangeFull) -> &[u8] { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **Hash**
  - ```rust
    fn hash<H: ::core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Signature) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 64] { /* ... */ }
    ```
    Gets a reference to the underlying array

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Signature, Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Signature) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Signature) -> bool { /* ... */ }
    ```

- **UnwindSafe**
## Module `schnorrsig`

**Attributes:**

- `#[deprecated(since = "0.21.0", note = "Use schnorr instead.")]`

**⚠️ Deprecated since 0.21.0**: Use schnorr instead.

Schnorr Signature related methods.

```rust
pub mod schnorrsig { /* ... */ }
```

### Types

#### Type Alias `PublicKey`

**Attributes:**

- `#[deprecated(since = "0.21.0", note = "Use crate::XOnlyPublicKey instead.")]`

**⚠️ Deprecated since 0.21.0**: Use crate::XOnlyPublicKey instead.

backwards compatible re-export of xonly key

```rust
pub type PublicKey = super::XOnlyPublicKey;
```

#### Type Alias `KeyPair`

**Attributes:**

- `#[deprecated(since = "0.21.0", note = "Use crate::KeyPair instead.")]`

**⚠️ Deprecated since 0.21.0**: Use crate::KeyPair instead.

backwards compatible re-export of keypair

```rust
pub type KeyPair = super::KeyPair;
```

#### Type Alias `Signature`

**Attributes:**

- `#[deprecated(since = "0.21.0", note = "Use schnorr::Signature instead.")]`

**⚠️ Deprecated since 0.21.0**: Use schnorr::Signature instead.

backwards compatible re-export of schnorr signatures

```rust
pub type Signature = super::schnorr::Signature;
```

## Types

### Type Alias `Signature`

**Attributes:**

- `#[deprecated(since = "0.21.0", note = "Use ecdsa::Signature instead.")]`

**⚠️ Deprecated since 0.21.0**: Use ecdsa::Signature instead.

backwards compatible re-export of ecdsa signatures

```rust
pub type Signature = ecdsa::Signature;
```

### Struct `Message`

A (hashed) message input to an ECDSA signature.

```rust
pub struct Message(pub(crate) [u8; 32]);
```

#### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `[u8; 32]` |  |

#### Implementations

##### Methods

- ```rust
  pub fn as_ptr(self: &Self) -> *const u8 { /* ... */ }
  ```
  Converts the object to a raw pointer for FFI interfacing

- ```rust
  pub fn as_mut_ptr(self: &mut Self) -> *mut u8 { /* ... */ }
  ```
  Converts the object to a mutable raw pointer for FFI interfacing

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the length of the object as an array

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the object as an array is empty

- ```rust
  pub fn from_slice(data: &[u8]) -> Result<Message, Error> { /* ... */ }
  ```
  **If you just want to sign an arbitrary message use `Message::from_hashed_data` instead.**

##### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut ::core::fmt::Formatter<''_>) -> ::core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Message { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Message) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &[u8; 32] { /* ... */ }
    ```
    Gets a reference to the underlying array

- **Send**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Message) -> ::core::cmp::Ordering { /* ... */ }
    ```

- **Sync**
- **Eq**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: T) -> Message { /* ... */ }
    ```
    Converts a 32-byte hash directly to a message without error paths.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: ::core::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **CPtr**
  - ```rust
    fn as_c_ptr(self: &Self) -> *const <Self as >::Target { /* ... */ }
    ```

  - ```rust
    fn as_mut_c_ptr(self: &mut Self) -> *mut <Self as >::Target { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Message) -> Option<::core::cmp::Ordering> { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &u8 { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::Range<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeTo<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, index: ::core::ops::RangeFrom<usize>) -> &[u8] { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, _: ::core::ops::RangeFull) -> &[u8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

### Enum `Error`

An ECDSA error

```rust
pub enum Error {
    IncorrectSignature,
    InvalidMessage,
    InvalidPublicKey,
    InvalidSignature,
    InvalidSecretKey,
    InvalidRecoveryId,
    InvalidTweak,
    NotEnoughMemory,
    InvalidPublicKeySum,
    InvalidParityValue(key::InvalidParityValue),
}
```

#### Variants

##### `IncorrectSignature`

Signature failed verification

##### `InvalidMessage`

Badly sized message ("messages" are actually fixed-sized digests; see the `MESSAGE_SIZE`
constant).

##### `InvalidPublicKey`

Bad public key.

##### `InvalidSignature`

Bad signature.

##### `InvalidSecretKey`

Bad secret key.

##### `InvalidRecoveryId`

Bad recovery id.

##### `InvalidTweak`

Invalid tweak for `add_*_assign` or `mul_*_assign`.

##### `NotEnoughMemory`

Didn't pass enough memory to context creation with preallocated memory.

##### `InvalidPublicKeySum`

Bad set of public keys.

##### `InvalidParityValue`

The only valid parity values are 0 or 1.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `key::InvalidParityValue` |  |

#### Implementations

##### Methods

- ```rust
  pub(crate) fn as_str(self: &Self) -> &str { /* ... */ }
  ```

##### Trait Implementations

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **Error**
  - ```rust
    fn cause(self: &Self) -> Option<&dyn std::error::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Error) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> Result<(), fmt::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(error: InvalidParityValue) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Error { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Error) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Struct `Secp256k1`

The secp256k1 engine, used to execute all signature operations.

```rust
pub struct Secp256k1<C: Context> {
    pub(crate) ctx: *mut ffi::Context,
    pub(crate) phantom: core::marker::PhantomData<C>,
    pub(crate) size: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ctx` | `*mut ffi::Context` |  |
| `phantom` | `core::marker::PhantomData<C>` |  |
| `size` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn gen_new() -> Secp256k1<C> { /* ... */ }
  ```
  Lets you create a context in a generic manner (sign/verify/all).

- ```rust
  pub fn new() -> Secp256k1<All> { /* ... */ }
  ```
  Creates a new Secp256k1 context with all capabilities.

- ```rust
  pub fn signing_only() -> Secp256k1<SignOnly> { /* ... */ }
  ```
  Creates a new Secp256k1 context that can only be used for signing.

- ```rust
  pub fn verification_only() -> Secp256k1<VerifyOnly> { /* ... */ }
  ```
  Creates a new Secp256k1 context that can only be used for verification.

- ```rust
  pub fn preallocated_gen_new(buf: &''buf mut [AlignedType]) -> Result<Secp256k1<C>, Error> { /* ... */ }
  ```
  Lets you create a context with preallocated buffer in a generic manner(sign/verify/all)

- ```rust
  pub fn preallocated_new(buf: &''buf mut [AlignedType]) -> Result<Secp256k1<AllPreallocated<''buf>>, Error> { /* ... */ }
  ```
  Creates a new Secp256k1 context with all capabilities

- ```rust
  pub fn preallocate_size() -> usize { /* ... */ }
  ```
  Uses the ffi `secp256k1_context_preallocated_size` to check the memory size needed for a context.

- ```rust
  pub unsafe fn from_raw_all(raw_ctx: *mut ffi::Context) -> ManuallyDrop<Secp256k1<AllPreallocated<''buf>>> { /* ... */ }
  ```
  Create a context from a raw context.

- ```rust
  pub fn preallocated_signing_only(buf: &''buf mut [AlignedType]) -> Result<Secp256k1<SignOnlyPreallocated<''buf>>, Error> { /* ... */ }
  ```
  Creates a new Secp256k1 context that can only be used for signing.

- ```rust
  pub fn preallocate_signing_size() -> usize { /* ... */ }
  ```
  Uses the ffi `secp256k1_context_preallocated_size` to check the memory size needed for the context.

- ```rust
  pub unsafe fn from_raw_signining_only(raw_ctx: *mut ffi::Context) -> ManuallyDrop<Secp256k1<SignOnlyPreallocated<''buf>>> { /* ... */ }
  ```
  Create a context from a raw context.

- ```rust
  pub fn preallocated_verification_only(buf: &''buf mut [AlignedType]) -> Result<Secp256k1<VerifyOnlyPreallocated<''buf>>, Error> { /* ... */ }
  ```
  Creates a new Secp256k1 context that can only be used for verification

- ```rust
  pub fn preallocate_verification_size() -> usize { /* ... */ }
  ```
  Uses the ffi `secp256k1_context_preallocated_size` to check the memory size needed for the context.

- ```rust
  pub unsafe fn from_raw_verification_only(raw_ctx: *mut ffi::Context) -> ManuallyDrop<Secp256k1<VerifyOnlyPreallocated<''buf>>> { /* ... */ }
  ```
  Create a context from a raw context.

- ```rust
  pub fn sign(self: &Self, msg: &Message, sk: &SecretKey) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk` and RFC6979 nonce

- ```rust
  pub fn sign_ecdsa(self: &Self, msg: &Message, sk: &SecretKey) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk` and RFC6979 nonce

- ```rust
  pub(in ::ecdsa) fn sign_grind_with_check</* synthetic */ impl Fn(&ffi::Signature) -> bool: Fn(&ffi::Signature) -> bool>(self: &Self, msg: &Message, sk: &SecretKey, check: impl Fn(&ffi::Signature) -> bool) -> Signature { /* ... */ }
  ```

- ```rust
  pub fn sign_grind_r(self: &Self, msg: &Message, sk: &SecretKey, bytes_to_grind: usize) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk`, RFC6979 nonce

- ```rust
  pub fn sign_ecdsa_grind_r(self: &Self, msg: &Message, sk: &SecretKey, bytes_to_grind: usize) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk`, RFC6979 nonce

- ```rust
  pub fn sign_low_r(self: &Self, msg: &Message, sk: &SecretKey) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk`, RFC6979 nonce

- ```rust
  pub fn sign_ecdsa_low_r(self: &Self, msg: &Message, sk: &SecretKey) -> Signature { /* ... */ }
  ```
  Constructs a signature for `msg` using the secret key `sk`, RFC6979 nonce

- ```rust
  pub fn verify(self: &Self, msg: &Message, sig: &Signature, pk: &PublicKey) -> Result<(), Error> { /* ... */ }
  ```
  Checks that `sig` is a valid ECDSA signature for `msg` using the public

- ```rust
  pub fn verify_ecdsa(self: &Self, msg: &Message, sig: &Signature, pk: &PublicKey) -> Result<(), Error> { /* ... */ }
  ```
  Checks that `sig` is a valid ECDSA signature for `msg` using the public

- ```rust
  pub(in ::schnorr) fn sign_schnorr_helper(self: &Self, msg: &Message, keypair: &KeyPair, nonce_data: *const ffi::types::c_uchar) -> Signature { /* ... */ }
  ```

- ```rust
  pub fn schnorrsig_sign_no_aux_rand(self: &Self, msg: &Message, keypair: &KeyPair) -> Signature { /* ... */ }
  ```
  Create a schnorr signature without using any auxiliary random data.

- ```rust
  pub fn sign_schnorr_no_aux_rand(self: &Self, msg: &Message, keypair: &KeyPair) -> Signature { /* ... */ }
  ```
  Create a schnorr signature without using any auxiliary random data.

- ```rust
  pub fn schnorrsig_sign_with_aux_rand(self: &Self, msg: &Message, keypair: &KeyPair, aux_rand: &[u8; 32]) -> Signature { /* ... */ }
  ```
  Create a Schnorr signature using the given auxiliary random data.

- ```rust
  pub fn sign_schnorr_with_aux_rand(self: &Self, msg: &Message, keypair: &KeyPair, aux_rand: &[u8; 32]) -> Signature { /* ... */ }
  ```
  Create a Schnorr signature using the given auxiliary random data.

- ```rust
  pub fn schnorrsig_verify(self: &Self, sig: &Signature, msg: &Message, pubkey: &XOnlyPublicKey) -> Result<(), Error> { /* ... */ }
  ```
  Verify a Schnorr signature.

- ```rust
  pub fn verify_schnorr(self: &Self, sig: &Signature, msg: &Message, pubkey: &XOnlyPublicKey) -> Result<(), Error> { /* ... */ }
  ```
  Verify a Schnorr signature.

- ```rust
  pub fn ctx(self: &Self) -> &*mut ffi::Context { /* ... */ }
  ```
  Getter for the raw pointer to the underlying secp256k1 context. This

- ```rust
  pub fn preallocate_size_gen() -> usize { /* ... */ }
  ```
  Returns the required memory for a preallocated context buffer in a generic manner(sign/verify/all).

- ```rust
  pub fn seeded_randomize(self: &mut Self, seed: &[u8; 32]) { /* ... */ }
  ```
  (Re)randomizes the Secp256k1 context for extra sidechannel resistance given 32 bytes of

##### Trait Implementations

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Secp256k1<C> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
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

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, _other: &Secp256k1<C>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
## Traits

### Trait `ThirtyTwoByteHash`

Trait describing something that promises to be a 32-byte random number; in particular,
it has negligible probability of being zero or overflowing the group order. Such objects
may be converted to `Message`s without any error paths.

```rust
pub trait ThirtyTwoByteHash {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `into_32`: Converts the object into a 32-byte array

## Functions

### Function `from_hex`

Utility function used to parse hex into a target u8 buffer. Returns
the number of bytes converted or an error if it encounters an invalid
character or unexpected end of string.

```rust
pub(crate) fn from_hex(hex: &str, target: &mut [u8]) -> Result<usize, ()> { /* ... */ }
```

### Function `to_hex`

**Attributes:**

- `#[inline]`

Utility function used to encode hex into a target u8 buffer. Returns
a reference to the target buffer as an str. Returns an error if the target
buffer isn't big enough.

```rust
pub(crate) fn to_hex<''a>(src: &[u8], target: &''a mut [u8]) -> Result<&''a str, ()> { /* ... */ }
```

## Re-exports

### Re-export `secp256k1_sys`

```rust
pub use secp256k1_sys as ffi;
```

### Re-export `key::*`

```rust
pub use key::*;
```

### Re-export `context::*`

```rust
pub use context::*;
```

## Other Items

### Extern Crate `secp256k1_sys`

**Attributes:**

- `#[macro_use]`

```rust
pub extern crate secp256k1_sys;
```

