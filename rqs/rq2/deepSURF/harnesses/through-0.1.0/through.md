# Crate Documentation

**Version:** 0.1.0

**Format Version:** 39

# Module `through`

## Functions

### Function `through`

Mutate a referenced element by transferring ownership through a function.

```rust
pub fn through<T, /* synthetic */ impl FnOnce(T) -> T: FnOnce(T) -> T>(elem: &mut T, func: impl FnOnce(T) -> T) { /* ... */ }
```

### Function `through_and`

Mutate a referenced element by transferring ownership through a function, which also
produces an output datum which is returned from this function.

```rust
pub fn through_and<T, O, /* synthetic */ impl FnOnce(T) -> (T, O): FnOnce(T) -> (T, O)>(elem: &mut T, func: impl FnOnce(T) -> (T, O)) -> O { /* ... */ }
```

