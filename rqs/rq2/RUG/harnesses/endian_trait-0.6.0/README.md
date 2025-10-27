# Endian Trait

[![Crate][crate_svg]][crate]
[![Docs][docs_svg]][docs]
[![Gitlab CI Status][gitlab_svg]][gitlab]
[![Travis CI Status][travis_svg]][travis]

This crate provides a trait, `Endian`, which requires four methods for
converting primitives with multi-byte representations between big- and little-
endian orders. In addition to declaring the trait, this library implements it on
Rust's primitives (`bool`, `char`, `{i,u}{8,16,32,64}`, `f32`, and `f64`), and
on all slices `&mut [Endian]`.

This crate also provides a custom derive macro available with `#[macro_use]`.

The primary purpose of this library is to aid in the direct binary serialization
of Rust types across machine boundaries. This is not a robust means of moving
data across a network or filesystem, but it can be used as a basis for building
stronger binary serialization procedures. Note that common transmission methods
will also require implementing a conversion to/from byte arrays, which is
currently beyond the scope of this library.

## Usage

**MINIMUM RUST VERSION: 1.20**

Require this crate (`endian_trait`) in your Cargo.toml, and tag it with
`#[macro_use]` for access to the custom derive macro.

```toml
[dependencies]
endian_trait = "0.5"
```

Import them in your crate root:

```rust
#[macro_use]
extern crate endian_trait;
```

and then use the `Endian` trait to flip bytes.

```rust
use endian_trait::Endian;

#[derive(Endian)]
struct Foo {
    bar: i32,
    baz: f64,
}

#[derive(Endian)]
struct Quux {
    f: Foo,
    g: bool,
}

let q = Quux {
    f: Foo {
        bar: 42,
        baz: 6.283185,
    },
    g: true,
}.to_be();

let q2: Quux = q.from_be();
```

### Useful ... Usage

Endian conversions destroy the utility, and in some cases (floats, chars) the
validity, of the data on which they are performed. Once data is transformed
away from local endian, it can no longer be used as anything but a bare sequence
of bytes with no further meaning. Similarly, the transformations from an order
to local endian are only useful to perform on a sequence of bytes that are known
to be in the right shape.

In my projects that use this, I have the following workflow for binary ser/des:

```rust
#[derive(Endian)]
struct Foo {
    //  fields
}
impl From<[u8; N]> for Foo {
    fn from(src: [u8; N]) -> Self {
        //  move fields into a byte array
    }
}
impl Into<[u8; N]> for Foo {
    fn into(self) -> [u8; N] {
        //  pull segments of the array into fields
    }
}

let f: Foo = make_a_foo();
let fbytes: [u8; N] = f.to_be().into();

let raw_foo: [u8; N] = read_from_network();
let build_foo: Foo = Foo::from(raw_foo).from_be();
```

Do keep in mind that once data is converted to a transport endian order, it can
no longer be considered as anything but a collection of bytes. Converting a char
or float will almost always result in a bit pattern that is invalid to be read
as its stated type, and will remain so until converted back to native order on
the other side. The `From` and `Into` impls used for binary ser/des should just
be transmutes and byte shunts, as they will be likely working with data that is
the correct width but of logically invalid form.

You could also move the endian conversions into the `From`/`Into` methods, but I
personally prefer keeping those uncoupled.

There's really no other reason to use this trait, as far as I'm aware.

## Extra Features

You can compile with `--features arrays` to have Endian implemented on arrays
`[T: Endian; N]` where N is in the range 0 ≤ N ≤ 256. That's right; I support
eight times as many arrays as the standard library does.

We really need type level integers.

On nightly ([RFC #1504][0], [issue #35118][1]), you can compile with
`--features e128` to have Endian implemented on `i128` and `u128`.

In your `Cargo.toml`, replace the original dependency on `endian_trait` with:

```toml
[dependencies.endian_trait]
version = 0.5
features = [
    "arrays",
    "e128", # currently only available on nightly, issue #35118, RFC #1504
]
```

[0]: https://github.com/rust-lang/rfcs/pull/1504
[1]: https://github.com/rust-lang/rust/issues/35118
[crate]: https://crates.io/crates/endian_trait
[crate_svg]: https://img.shields.io/crates/v/endian_trait.svg
[docs]: https://docs.rs/endian_trait
[docs_svg]: https://docs.rs/endian_trait/badge.svg
[gitlab]: https://gitlab.com/myrrlyn/endian_trait
[gitlab_svg]: https://gitlab.com/myrrlyn/endian_trait/badges/master/build.svg
[travis]: https://travis-ci.org/myrrlyn/endian_trait
[travis_svg]: https://travis-ci.org/myrrlyn/endian_trait.svg?branch=master
