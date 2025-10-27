# Crate Documentation

**Version:** 0.6.0

**Format Version:** 39

# Module `endian_trait`

 Endian conversion trait

This trait declares methods that perform endian conversions on data types. For
the primitives, which are essentially atomic in structure, this conversion is
simple: flip all their bytes around. This conversion is also defined as inherent
methods on the integral primitives, so `Endian::from_be(n: i32)` is equivalent
to `::std::i32::from_be(n: i32)`
!

## Modules

## Module `slices`

 Implement `Endian` on mutable slices.
!

```rust
pub(crate) mod slices { /* ... */ }
```

## Traits

### Trait `Endian`

Convert a type from one endian order to another.

The standard implementation of this trait is simply to call the methods on
the component members of a data type which are themselves `Endian`, until the
call stack bottoms out at one of Rust's primitives.

```rust
pub trait Endian {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `to_be`: Converts from host endian to big-endian order.
- `to_le`: Converts from host endian to little-endian order.
- `from_be`: Converts from big-endian order to host endian.
- `from_le`: Converts from little-endian order to host endian.

#### Implementations

This trait is implemented for the following types:

- `bool`
- `char`
- `i8`
- `u8`
- `i16`
- `u16`
- `i32`
- `u32`
- `i64`
- `u64`
- `i128`
- `u128`
- `f32`
- `f64`
- `&''a mut [T]` with <''a, T: Endian>

## Macros

### Macro `implendian`

Implementing Endian on the integer primitives just means delegating to their
inherent methods. As there are many integer primitives, this macro kills the
needless code duplication.

```rust
pub(crate) macro_rules! implendian {
    /* macro_rules! implendian {
    ( $( $t:tt ),* ) => { ... };
} */
}
```

### Macro `implendian_f`

Implement Endian on the floats by flipping their byte repr.

The to_ conversions use bare transmute, as the result may wind up looking
invalid on the host architecture when used in floating-point contexts. The
from_ conversions use Rust's from_bits functions, as the final result must
be a valid local floating-point number.

The to/from _bits() APIs on f32/64 were stabilized in Rust 1.20, and thus
this code cannot be run on Rust version below that.

```rust
pub(crate) macro_rules! implendian_f {
    /* macro_rules! implendian_f {
    ( $( $t:tt ),* ) => { ... };
} */
}
```

## Re-exports

### Re-export `endian_trait_derive::*`

```rust
pub use endian_trait_derive::*;
```

