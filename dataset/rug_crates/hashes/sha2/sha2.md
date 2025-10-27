# Crate Documentation

**Version:** 0.10.7

**Format Version:** 39

# Module `sha2`

An implementation of the [SHA-2][1] cryptographic hash algorithms.

There are 6 standard algorithms specified in the SHA-2 standard: [`Sha224`],
[`Sha256`], [`Sha512_224`], [`Sha512_256`], [`Sha384`], and [`Sha512`].

Algorithmically, there are only 2 core algorithms: SHA-256 and SHA-512.
All other algorithms are just applications of these with different initial
hash values, and truncated to different digest bit lengths. The first two
algorithms in the list are based on SHA-256, while the last four are based
on SHA-512.

# Usage

```rust
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

// create a Sha256 object
let mut hasher = Sha256::new();

// write input message
hasher.update(b"hello world");

// read hash digest and consume hasher
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
")[..]);

// same for Sha512
let mut hasher = Sha512::new();
hasher.update(b"hello world");
let result = hasher.finalize();

assert_eq!(result[..], hex!("
    309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
    989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
")[..]);
```

Also see [RustCrypto/hashes][2] readme.

[1]: https://en.wikipedia.org/wiki/SHA-2
[2]: https://github.com/RustCrypto/hashes

## Modules

## Module `consts`

**Attributes:**

- `#[rustfmt::skip]`
- `#![allow(dead_code, clippy::unreadable_literal)]`

```rust
pub(crate) mod consts { /* ... */ }
```

### Types

#### Type Alias `State256`

```rust
pub type State256 = [u32; 8];
```

#### Type Alias `State512`

```rust
pub type State512 = [u64; 8];
```

### Constants and Statics

#### Constant `STATE_LEN`

```rust
pub const STATE_LEN: usize = 8;
```

#### Constant `BLOCK_LEN`

```rust
pub const BLOCK_LEN: usize = 16;
```

#### Constant `K32`

Constants necessary for SHA-256 family of digests.

```rust
pub const K32: [u32; 64] = _;
```

#### Constant `K32X4`

Constants necessary for SHA-256 family of digests.

```rust
pub const K32X4: [[u32; 4]; 16] = _;
```

#### Constant `K64`

Constants necessary for SHA-512 family of digests.

```rust
pub const K64: [u64; 80] = _;
```

#### Constant `K64X2`

Constants necessary for SHA-512 family of digests.

```rust
pub const K64X2: [[u64; 2]; 40] = _;
```

#### Constant `H256_224`

```rust
pub const H256_224: [u32; 8] = _;
```

#### Constant `H256_256`

```rust
pub const H256_256: [u32; 8] = _;
```

#### Constant `H512_224`

```rust
pub const H512_224: [u64; 8] = _;
```

#### Constant `H512_256`

```rust
pub const H512_256: [u64; 8] = _;
```

#### Constant `H512_384`

```rust
pub const H512_384: [u64; 8] = _;
```

#### Constant `H512_512`

```rust
pub const H512_512: [u64; 8] = _;
```

## Module `core_api`

```rust
pub(crate) mod core_api { /* ... */ }
```

### Types

#### Struct `Sha256VarCore`

Core block-level SHA-256 hasher with variable output size.

Supports initialization only for 28 and 32 byte output sizes,
i.e. 224 and 256 bits respectively.

```rust
pub struct Sha256VarCore {
    pub(in ::core_api) state: [u32; 8],
    pub(in ::core_api) block_len: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `state` | `[u32; 8]` |  |
| `block_len` | `u64` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **HashMarker**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **BufferKindUser**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **AlgorithmName**
  - ```rust
    fn write_alg_name(f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **UpdateCore**
  - ```rust
    fn update_blocks(self: &mut Self, blocks: &[Block<Self>]) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VariableOutputCore**
  - ```rust
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> { /* ... */ }
    ```

  - ```rust
    fn finalize_variable_core(self: &mut Self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BlockSizeUser**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha256VarCore { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **OutputSizeUser**
#### Struct `Sha512VarCore`

Core block-level SHA-512 hasher with variable output size.

Supports initialization only for 28, 32, 48, and 64 byte output sizes,
i.e. 224, 256, 384, and 512 bits respectively.

```rust
pub struct Sha512VarCore {
    pub(in ::core_api) state: [u64; 8],
    pub(in ::core_api) block_len: u128,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `state` | `[u64; 8]` |  |
| `block_len` | `u128` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sha512VarCore { /* ... */ }
    ```

- **HashMarker**
- **BlockSizeUser**
- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **BufferKindUser**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UpdateCore**
  - ```rust
    fn update_blocks(self: &mut Self, blocks: &[Block<Self>]) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VariableOutputCore**
  - ```rust
    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> { /* ... */ }
    ```

  - ```rust
    fn finalize_variable_core(self: &mut Self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) { /* ... */ }
    ```

- **AlgorithmName**
  - ```rust
    fn write_alg_name(f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **OutputSizeUser**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
## Module `sha256`

```rust
pub(crate) mod sha256 { /* ... */ }
```

### Modules

## Module `soft`

**Attributes:**

- `#[cfg(not(feature = "asm"))]`
- `#![allow(clippy::many_single_char_names)]`

```rust
pub(in ::sha256) mod soft { /* ... */ }
```

### Functions

#### Function `shl`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha256::soft) fn shl(v: [u32; 4], o: u32) -> [u32; 4] { /* ... */ }
```

#### Function `shr`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha256::soft) fn shr(v: [u32; 4], o: u32) -> [u32; 4] { /* ... */ }
```

#### Function `or`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha256::soft) fn or(a: [u32; 4], b: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `xor`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha256::soft) fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `add`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha256::soft) fn add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256load`

```rust
pub(in ::sha256::soft) fn sha256load(v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256swap`

```rust
pub(in ::sha256::soft) fn sha256swap(v0: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256msg1`

```rust
pub(in ::sha256::soft) fn sha256msg1(v0: [u32; 4], v1: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256msg2`

```rust
pub(in ::sha256::soft) fn sha256msg2(v4: [u32; 4], v3: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256_digest_round_x2`

```rust
pub(in ::sha256::soft) fn sha256_digest_round_x2(cdgh: [u32; 4], abef: [u32; 4], wk: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `schedule`

```rust
pub(in ::sha256::soft) fn schedule(v0: [u32; 4], v1: [u32; 4], v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] { /* ... */ }
```

#### Function `sha256_digest_block_u32`

Process a block with the SHA-256 algorithm.

```rust
pub(in ::sha256::soft) fn sha256_digest_block_u32(state: &mut [u32; 8], block: &[u32; 16]) { /* ... */ }
```

#### Function `compress`

```rust
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) { /* ... */ }
```

### Macros

#### Macro `rounds4`

```rust
pub(crate) macro_rules! rounds4 {
    /* macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => { ... };
} */
}
```

#### Macro `schedule_rounds4`

```rust
pub(crate) macro_rules! schedule_rounds4 {
    /* macro_rules! schedule_rounds4 {
    (
        $abef:ident, $cdgh:ident,
        $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i: expr
    ) => { ... };
} */
}
```

## Module `x86`

**Attributes:**

- `#![allow(clippy::many_single_char_names)]`

SHA-256 `x86`/`x86_64` backend

```rust
pub(in ::sha256) mod x86 { /* ... */ }
```

### Modules

## Module `shani_cpuid`

```rust
pub(in ::sha256::x86) mod shani_cpuid { /* ... */ }
```

### Types

#### Struct `InitToken`

Initialization token

```rust
pub struct InitToken(pub(in ::sha256::x86::shani_cpuid) ());
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get(self: &Self) -> bool { /* ... */ }
  ```
  Get initialized value

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InitToken { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
### Functions

#### Function `init_get`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
stored value and initialization token.

```rust
pub fn init_get() -> (InitToken, bool) { /* ... */ }
```

#### Function `init`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
initialization token.

```rust
pub fn init() -> InitToken { /* ... */ }
```

#### Function `get`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
stored value.

```rust
pub fn get() -> bool { /* ... */ }
```

### Constants and Statics

#### Constant `UNINIT`

```rust
pub(in ::sha256::x86::shani_cpuid) const UNINIT: u8 = _;
```

#### Static `STORAGE`

```rust
pub(in ::sha256::x86::shani_cpuid) static STORAGE: AtomicU8 = _;
```

### Functions

#### Function `schedule`

```rust
pub(in ::sha256::x86) unsafe fn schedule(v0: __m128i, v1: __m128i, v2: __m128i, v3: __m128i) -> __m128i { /* ... */ }
```

#### Function `digest_blocks`

**Attributes:**

- `#[allow(clippy::cast_ptr_alignment)]`
- `#[target_feature(enable = "sha,sse2,ssse3,sse4.1")]`

```rust
pub(in ::sha256::x86) unsafe fn digest_blocks(state: &mut [u32; 8], blocks: &[[u8; 64]]) { /* ... */ }
```

#### Function `compress`

```rust
pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) { /* ... */ }
```

### Macros

#### Macro `rounds4`

```rust
pub(crate) macro_rules! rounds4 {
    /* macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => { ... };
} */
}
```

#### Macro `schedule_rounds4`

```rust
pub(crate) macro_rules! schedule_rounds4 {
    /* macro_rules! schedule_rounds4 {
    (
        $abef:ident, $cdgh:ident,
        $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i: expr
    ) => { ... };
} */
}
```

### Functions

#### Function `compress256`

Raw SHA-256 compression function.

This is a low-level "hazmat" API which provides direct access to the core
functionality of SHA-256.

```rust
pub fn compress256(state: &mut [u32; 8], blocks: &[digest::generic_array::GenericArray<u8, digest::typenum::U64>]) { /* ... */ }
```

## Module `sha512`

```rust
pub(crate) mod sha512 { /* ... */ }
```

### Modules

## Module `soft`

**Attributes:**

- `#[cfg(not(feature = "asm"))]`
- `#![allow(clippy::many_single_char_names)]`

```rust
pub(in ::sha512) mod soft { /* ... */ }
```

### Functions

#### Function `add`

```rust
pub(in ::sha512::soft) fn add(a: [u64; 2], b: [u64; 2]) -> [u64; 2] { /* ... */ }
```

#### Function `sha512load`

Not an intrinsic, but works like an unaligned load.

```rust
pub(in ::sha512::soft) fn sha512load(v0: [u64; 2], v1: [u64; 2]) -> [u64; 2] { /* ... */ }
```

#### Function `sha512_schedule_x2`

Performs 2 rounds of the SHA-512 message schedule update.

```rust
pub fn sha512_schedule_x2(v0: [u64; 2], v1: [u64; 2], v4to5: [u64; 2], v7: [u64; 2]) -> [u64; 2] { /* ... */ }
```

#### Function `sha512_digest_round`

Performs one round of the SHA-512 message block digest.

```rust
pub fn sha512_digest_round(ae: [u64; 2], bf: [u64; 2], cg: [u64; 2], dh: [u64; 2], wk0: u64) -> [u64; 2] { /* ... */ }
```

#### Function `sha512_digest_block_u64`

Process a block with the SHA-512 algorithm.

```rust
pub fn sha512_digest_block_u64(state: &mut [u64; 8], block: &[u64; 16]) { /* ... */ }
```

#### Function `compress`

```rust
pub fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) { /* ... */ }
```

## Module `x86`

**Attributes:**

- `#![allow(clippy::many_single_char_names)]`

SHA-512 `x86`/`x86_64` backend

```rust
pub(in ::sha512) mod x86 { /* ... */ }
```

### Modules

## Module `avx2_cpuid`

```rust
pub(in ::sha512::x86) mod avx2_cpuid { /* ... */ }
```

### Types

#### Struct `InitToken`

Initialization token

```rust
pub struct InitToken(pub(in ::sha512::x86::avx2_cpuid) ());
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `()` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get(self: &Self) -> bool { /* ... */ }
  ```
  Get initialized value

###### Trait Implementations

- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InitToken { /* ... */ }
    ```

- **RefUnwindSafe**
- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
### Functions

#### Function `init_get`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
stored value and initialization token.

```rust
pub fn init_get() -> (InitToken, bool) { /* ... */ }
```

#### Function `init`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
initialization token.

```rust
pub fn init() -> InitToken { /* ... */ }
```

#### Function `get`

**Attributes:**

- `#[inline]`

Initialize underlying storage if needed and get
stored value.

```rust
pub fn get() -> bool { /* ... */ }
```

### Constants and Statics

#### Constant `UNINIT`

```rust
pub(in ::sha512::x86::avx2_cpuid) const UNINIT: u8 = _;
```

#### Static `STORAGE`

```rust
pub(in ::sha512::x86::avx2_cpuid) static STORAGE: AtomicU8 = _;
```

### Types

#### Type Alias `State`

```rust
pub(in ::sha512::x86) type State = [u64; 8];
```

#### Type Alias `MsgSchedule`

```rust
pub(in ::sha512::x86) type MsgSchedule = [__m128i; 8];
```

#### Type Alias `RoundStates`

```rust
pub(in ::sha512::x86) type RoundStates = [__m128i; 40];
```

### Functions

#### Function `compress`

```rust
pub fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) { /* ... */ }
```

#### Function `sha512_compress_x86_64_avx2`

**Attributes:**

- `#[target_feature(enable = "avx2")]`

```rust
pub(in ::sha512::x86) unsafe fn sha512_compress_x86_64_avx2(state: &mut [u64; 8], blocks: &[[u8; 128]]) { /* ... */ }
```

#### Function `sha512_compress_x86_64_avx`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) unsafe fn sha512_compress_x86_64_avx(state: &mut [u64; 8], block: &[u8; 128]) { /* ... */ }
```

#### Function `load_data_avx`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) unsafe fn load_data_avx(x: &mut [__m128i; 8], ms: &mut [__m128i; 8], data: *const __m128i) { /* ... */ }
```

#### Function `load_data_avx2`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) unsafe fn load_data_avx2(x: &mut [__m256i; 8], ms: &mut [__m128i; 8], t2: &mut [__m128i; 40], data: *const __m128i) { /* ... */ }
```

#### Function `rounds_0_63_avx`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) unsafe fn rounds_0_63_avx(current_state: &mut [u64; 8], x: &mut [__m128i; 8], ms: &mut [__m128i; 8]) { /* ... */ }
```

#### Function `rounds_0_63_avx2`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) unsafe fn rounds_0_63_avx2(current_state: &mut [u64; 8], x: &mut [__m256i; 8], ms: &mut [__m128i; 8], t2: &mut [__m128i; 40]) { /* ... */ }
```

#### Function `rounds_64_79`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn rounds_64_79(current_state: &mut [u64; 8], ms: &[__m128i; 8]) { /* ... */ }
```

#### Function `process_second_block`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn process_second_block(current_state: &mut [u64; 8], t2: &[__m128i; 40]) { /* ... */ }
```

#### Function `sha_round`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn sha_round(s: &mut [u64; 8], x: u64) { /* ... */ }
```

#### Function `accumulate_state`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn accumulate_state(dst: &mut [u64; 8], src: &[u64; 8]) { /* ... */ }
```

#### Function `sha512_update_x_avx`

```rust
pub(in ::sha512::x86) unsafe fn sha512_update_x_avx(x: &mut [__m128i; 8], k64: __m128i) -> __m128i { /* ... */ }
```

#### Function `sha512_update_x_avx2`

```rust
pub(in ::sha512::x86) unsafe fn sha512_update_x_avx2(x: &mut [__m256i; 8], k64: __m256i) -> __m256i { /* ... */ }
```

#### Function `cast_ms`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn cast_ms(ms: &[__m128i; 8]) -> &[u64; 16] { /* ... */ }
```

#### Function `cast_rs`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::sha512::x86) fn cast_rs(rs: &[__m128i; 40]) -> &[u64; 80] { /* ... */ }
```

### Constants and Statics

#### Constant `SHA512_BLOCK_BYTE_LEN`

```rust
pub(in ::sha512::x86) const SHA512_BLOCK_BYTE_LEN: usize = 128;
```

#### Constant `SHA512_ROUNDS_NUM`

```rust
pub(in ::sha512::x86) const SHA512_ROUNDS_NUM: usize = 80;
```

#### Constant `SHA512_HASH_BYTE_LEN`

```rust
pub(in ::sha512::x86) const SHA512_HASH_BYTE_LEN: usize = 64;
```

#### Constant `SHA512_HASH_WORDS_NUM`

```rust
pub(in ::sha512::x86) const SHA512_HASH_WORDS_NUM: usize = _;
```

#### Constant `SHA512_BLOCK_WORDS_NUM`

```rust
pub(in ::sha512::x86) const SHA512_BLOCK_WORDS_NUM: usize = _;
```

### Macros

#### Macro `fn_sha512_update_x`

```rust
pub(crate) macro_rules! fn_sha512_update_x {
    /* macro_rules! fn_sha512_update_x {
    ($name:ident, $ty:ident, {
        ADD64 = $ADD64:ident,
        ALIGNR8 = $ALIGNR8:ident,
        SRL64 = $SRL64:ident,
        SLL64 = $SLL64:ident,
        XOR = $XOR:ident,
    }) => { ... };
} */
}
```

### Functions

#### Function `compress512`

Raw SHA-512 compression function.

This is a low-level "hazmat" API which provides direct access to the core
functionality of SHA-512.

```rust
pub fn compress512(state: &mut [u64; 8], blocks: &[digest::generic_array::GenericArray<u8, digest::typenum::U128>]) { /* ... */ }
```

## Types

### Type Alias `Sha224`

SHA-224 hasher.

```rust
pub type Sha224 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha256VarCore, digest::consts::U28, OidSha224>>;
```

### Type Alias `Sha256`

SHA-256 hasher.

```rust
pub type Sha256 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha256VarCore, digest::consts::U32, OidSha256>>;
```

### Type Alias `Sha512_224`

SHA-512/224 hasher.

```rust
pub type Sha512_224 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha512VarCore, digest::consts::U28, OidSha512_224>>;
```

### Type Alias `Sha512_256`

SHA-512/256 hasher.

```rust
pub type Sha512_256 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha512VarCore, digest::consts::U32, OidSha512_256>>;
```

### Type Alias `Sha384`

SHA-384 hasher.

```rust
pub type Sha384 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha512VarCore, digest::consts::U48, OidSha384>>;
```

### Type Alias `Sha512`

SHA-512 hasher.

```rust
pub type Sha512 = digest::core_api::CoreWrapper<digest::core_api::CtVariableCoreWrapper<Sha512VarCore, digest::consts::U64, OidSha512>>;
```

## Re-exports

### Re-export `digest`

```rust
pub use digest;
```

### Re-export `Digest`

```rust
pub use digest::Digest;
```

### Re-export `Sha256VarCore`

```rust
pub use core_api::Sha256VarCore;
```

### Re-export `Sha512VarCore`

```rust
pub use core_api::Sha512VarCore;
```

