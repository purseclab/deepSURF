# Changelog

## 0.6.0

### Changed

Increased the minimum Rust version to 1.26.0 and removed the feature gate for
128-bit integers, which are now available by default.

Also overhaul the Justfile scripts to ease development.

## 0.5.0

### Changed

The implementation of `Endian` on `&mut [Endian]` is now provided by default,
and is not kept behind a feature gate. The feature gate `arrays` is still
required to access the implementations on the `[Endian; N]` implementations.

## 0.4.0

### Added

- re-export the `#[derive(Endian)]` macro from `endian_trait_derive`
- CI and tests
- This changelog
- `Endian` can now derive on discriminant-only (C-like) enums with integer
    representation. It cannot derive on `#[repr(C)]` enums or data-carrying
    enums.

### Changed

- pinned Rust version to 1.20.0

### Removed

- implementations of `Endian` for `isize` and `usize`.

## 0.3.0

### Added

- implement `Endian` on `i128`, `u128`, and `[T; N]` for arrays of zero through
    256

## 0.2.0

### Added

- implement `Endian` on `bool`, `char`, `f32`, `f64`

### Changed

- use stable Rust instead of nightly

## 0.1.1

### Added

- crates.io metadata in `Cargo.toml`

## 0.1.0

### Added

- `Endian` trait, implemented on the integer primitives
