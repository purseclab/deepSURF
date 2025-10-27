![Build Status](https://github.com/antonmarsden/toodee/workflows/Test/badge.svg)
[![Current Version](https://img.shields.io/crates/v/toodee.svg)](https://crates.io/crates/toodee)
[![Documentation](https://docs.rs/toodee/badge.svg)](https://docs.rs/toodee)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/toodee.svg)](#license)

# toodee

`TooDee` is a lightweight and high performance two-dimensional wrapper around a growable `Vec`.

`TooDeeView` and `TooDeeViewMut` allow you create two-dimensional wrappers around a **slice**.

## Core features

- Raw access to the underlying vector's slice via `data()` and `data_mut()`.
- Creation of performant two-dimensional subsets using `view()` and `view_mut()`.
- `get_unchecked(Coordinate)` and `get_unchecked_row(usize)` for faster (unsafe) access to cells or rows.
- Most operations are implemented for both `TooDee` and `TooDeeViewMut` structs - see below for how this pattern can be extended.
- Get/set specific cells using indexing, e.g., `let my_row = toodee[row]; my_row[col] = val;`.
- Index with a `Coordinate` if you prefer, e.g., `toodee[(col, row)] = val`.
- Index by row index (i.e., row major) to access row slices, e.g., `&toodee[row]`.
- Iteration, any which way - `rows()`, `rows_mut()`, `col()`, `col_mut()`, `cells()`, `cells_mut()`.
- `#[no_std]` compliant.
- Can create a new `TooDeeView`  from a `&[T]`, or a `TooDeeViewMut`  from  a `&mut [T]`.
- `insert_col()`, `remove_col()`, `insert_row()`, and `remove_row()` implementations with good performance.

## Additional Algorithms

### CopyOps (`copy` feature, included by default)

Various operations that copy data within the same 2D array, or copy data from one array to another. Many of these
operations are named like their slice counterparts, e.g., `copy_from_slice()` or `copy_from_toodee()`.

### TranslateOps (`translate` feature, included by default)

The `TranslateOps` trait provides common translation algorithms, including:
- `translate_with_wrap()`, a way to shift data around vertically and horizontally.
- `flip_rows()`, i.e., a mirror translation of data about the center row.
- `flip_cols()`, i.e., a mirror translation of data about the center column.

### SortOps (`sort` feature, included by default)

The `SortOps` trait provides efficient implementations of:
- `sort_by_row()` operations, with stable and unstable variants.
- `sort_by_col()` operations, with stable and unstable variants.

## Build Your Own 2D Algorithms

Traits such as `SortOps` contain additional algorithms. These traits are defined by extending
the `TooDeeOpsMut` trait, which has been implemented for `TooDee` and `TooDeeViewMut`. I recommend
taking the same approach because the algorithms you implement will then work on both structs. This may not seem
useful at first glance, but a great use case would be sorting a spreadsheet by column. If each column
had a header row, you'd want to exclude that header row from sorting. You can achieve this by creating
a `TooDeeViewMut` and sorting the view.

The implementation of your new trait could look something like:

```
pub trait FooOps<T> : TooDeeOpsMut<T> {

    fn foo(&mut self) -> Bar {
        ...
        return bar;
    }
}
```

The above code would provide a default `foo()` implementation that could be overridden if required. Then it's
simply a matter of stating that both `TooDee` and `TooDeeOpsMut` implement `FooOps`:

```
impl<T> FooOps<T> for TooDeeViewMut<'_, T> {}

impl<T> FooOps<T> for TooDee<T> {}
```

Once the implementations are available, just call the methods, e.g.,

```
let bar = my_toodee.foo();
let bar_view = my_toodee_mut_view.foo();
```

Happy coding :)

## TODO

- Pathfinding algorithms?
- Image/bitmap algorithms?!

## Motivation

Similar libraries do exist, but they lacked either performance, flexibility, or functionality. 

Here's a small feature comparison chart:

<table>
  <tr><th></th><th>Storage order</th><th>Structs supported</th><th>Growable?</th><th>Mutable views?</th><th>Raw data access?</th><th>Iterate over row slices?</th><th>Notes</th></tr>
  <tr><td><code>toodee::TooDee</code></td><td>Row-major</td><td>Anything (<code>Sized</code>)</td><td>Yes</td><td>Yes</td><td>Yes</td><td>Yes</td><td></td></tr>
  <tr><td><code>image::ImageBuffer</code></td><td>Row-major</td><td><code>image::Pixel</code></td><td>No</td><td>No</td><td>Yes</td><td>No</td><td>Good for image processing - see the <code>imageproc</code> crate.</tr>
  <tr><td><code>image::SubImage</code></td><td>Row-major</td><td><code>image::Pixel</code></td><td>No</td><td>Yes</td><td>No</td><td>No</td><td></td></tr>
  <tr><td><code>grid::Grid</code></td><td>Row-major</td><td><code>Clone</code></td><td>Yes</td><td>No</td><td>Yes</td><td>No</td><td>Similar to <code>TooDee</code>, but not as functionally rich.</td></tr>
  <tr><td><code>array2d::Array2D</code></td><td>Row-major</td><td><code>Clone</code></td><td>No</td><td>No</td><td>No</td><td>No</td><td></td></tr>
  <tr><td><code>imgref::Img</code></td><td>Row-major</td><td>Anything (<code>Sized</code>)</td><td>No</td><td>Yes</td><td>Yes</td><td>Yes</td><td></td></tr>
  <tr><td><code>nalgebra::Matrix</code></td><td>Column-major</td><td><code>Scalar</code></td><td>Yes</td><td>Yes</td><td>Yes</td><td>No</td><td>Use this for vector/matrix math.</td></tr>
</table>

## Goals
 
- High performance and good flexibility, with the constraint of using a 1-D vector.
- Suitable for use in image processing, but not restricted to this problem domain.
- Provide solid implementations of non-trivial 2D operations.

## Non-goals
 
- GPU integration

## Limitations

- Views are not nested for the time being, The only impact is that the `bounds()` of a view
  are always relative to the underlying `TooDee` array.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
