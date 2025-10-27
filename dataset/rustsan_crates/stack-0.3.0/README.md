# stack

[![travis-badge][]][travis] [![release-badge][]][cargo] [![docs-badge][]][docs] [![license-badge][]][license]

Implements dynamically sized types over stack allocated arrays.

- The `Vector` trait exposes a generic trait for `Vec`-like operations.
- `ArrayVec` implements a `Vector` interface over a fixed-size array.
- `SmallVec` abstracts over a stack allocated `ArrayVec`, and falls back to a
   heap `Vec` upon overflow.
- `SmallDST` holds a DST such as a closure or other types in a `Vector`.


[travis-badge]: https://img.shields.io/travis/arcnmx/stack-rs/master.svg?style=flat-square
[travis]: https://travis-ci.org/arcnmx/stack-rs
[release-badge]: https://img.shields.io/crates/v/stack.svg?style=flat-square
[cargo]: https://crates.io/crates/stack
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://arcnmx.github.io/stack-rs/stack/
[license-badge]: https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square
[license]: https://github.com/arcnmx/stack-rs/blob/master/COPYING
