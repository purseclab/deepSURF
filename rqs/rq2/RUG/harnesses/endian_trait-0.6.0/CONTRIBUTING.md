# Contributing

If you find something you want added to this, you can let me know on GitHub,
Twitter, reddit, maybe the Rust IRC, under this name, or you can file an issue
or open a PR.

## Design Questions

I'm open to discussion on how more complex types should be handled, and on what
if any constraints should be placed on `Endian`. I am not entirely sure it makes
sense to allow `Endian` on types that aren't `Copy`, or at the very least
`!Drop`, since this trait is only useful for copying plain data into some
serialization mechanism.

I would like to resolve whether to specify `Endian: !Drop` or leave it unbounded
before making a 1.0 release.
