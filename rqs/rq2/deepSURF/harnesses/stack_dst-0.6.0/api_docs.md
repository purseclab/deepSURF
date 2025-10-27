# Crate Documentation

**Version:** 0.6.0

**Format Version:** 45

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

