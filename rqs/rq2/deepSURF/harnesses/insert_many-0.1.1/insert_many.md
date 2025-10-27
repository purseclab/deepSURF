# Crate Documentation

**Version:** 0.1.1

**Format Version:** 39

# Module `insert_many`

Insert many optimization.
Like `Vec::insert`, but inserts a series of items at an index rather than a single one.
This can lead to significant speedup where multiple items need to be inserted.

## Traits

### Trait `InsertMany`

Generalized trait for inserting many items at once.

```rust
pub trait InsertMany<T> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

#### Required Items

##### Required Methods

- `insert_many`: Insert all the items in the given iterable at `index`, shifting all elements after it to the right.

#### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>
- `smallvec::SmallVec<A>` with <A: smallvec::Array>

## Macros

### Macro `impl_veclike`

```rust
pub(crate) macro_rules! impl_veclike {
    /* macro_rules! impl_veclike {
    ($s: ident, $index: ident, $iterable: ident) => { ... };
} */
}
```

