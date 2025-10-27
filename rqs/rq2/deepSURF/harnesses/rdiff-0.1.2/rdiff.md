# Crate Documentation

**Version:** 0.1.2

**Format Version:** 39

# Module `rdiff`

Finds the difference between sequential versions of files.

Based on the rsync algorithm.
The `BlockHashes` struct will find the differences between versions of the same file.
It does this through the [`diff_and_update()`](struct.BlockHashes.html#method.diff_and_update) method.

# Example

```
use std::io::Cursor;
use rdiff::BlockHashes;

let mut hash = BlockHashes::new(Cursor::new("The initial version"), 8).unwrap();
let diffs = hash.diff_and_update(Cursor::new("The next version")).unwrap();
println!("Diffs: {:?}", diffs);
// Outputs "Diffs: Diff{inserts: [Insert(0, The next vers)], deletes:[Delete(13, 16)]}"
```

This crate also contains methods relating to finding the differences between two strings, in the [string_diff](string_diff/index.html) module.
These methods can be used to refine the course differences found through the rsync method.

## Modules

## Module `window`

```rust
pub(crate) mod window { /* ... */ }
```

## Module `hashing`

```rust
pub(crate) mod hashing { /* ... */ }
```

### Types

#### Struct `RollingHash`

Implements a weak, but easy to calculate hash for a block of bytes

The hash is comprised of two bytes.  The first is the sum of the bytes

```rust
pub(in ::hashing) struct RollingHash {
    pub(in ::hashing) a: u16,
    pub(in ::hashing) b: u16,
    pub(in ::hashing) block_size: u16,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `a` | `u16` |  |
| `b` | `u16` |  |
| `block_size` | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''a, I: Iterator<Item = &''a u8>>(initial_data: I) -> RollingHash { /* ... */ }
  ```
  Creates a new rolling hash over the bytes in `initial_data`.

- ```rust
  pub fn get_hash(self: &Self) -> u32 { /* ... */ }
  ```
  Gets the hash as it currently stands

- ```rust
  pub fn roll_hash(self: &mut Self, new_byte: Option<u8>, old_byte: u8) { /* ... */ }
  ```
  Roll the has forward one byte.  This function will remove `old_byte` from its calculation

- ```rust
  pub fn hash_buffer(buffer: &[u8]) -> u32 { /* ... */ }
  ```
  Calculate the hash of a collection of bytes.

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
## Module `string_diff`

Used for finding the minimal set of operations to transform one string into another.

The primary function of this module is [find diff](fn.find_diff.html).

```rust
pub mod string_diff { /* ... */ }
```

### Types

#### Struct `EditDistance`

Used as the classiscal definition of edit distance.

That is:

* Insert is cost -1
* Delete is cost -1
* Substitution is cost -2 (an insert + a delete)
* Matching is cost 0

```rust
pub struct EditDistance;
```

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **OperationScore**
  - ```rust
    fn insert_score(self: &Self, _: char) -> i32 { /* ... */ }
    ```

  - ```rust
    fn delete_score(self: &Self, _: char) -> i32 { /* ... */ }
    ```

  - ```rust
    fn substitution_score(self: &Self, _: char, _: char) -> i32 { /* ... */ }
    ```

  - ```rust
    fn match_score(self: &Self, _: char) -> i32 { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
### Traits

#### Trait `OperationScore`

Used to calculate the score for each operation that
will be performed.  The score can be static, or it can
vary based on which character is being deleted inserted or substituted.
It is highly recommended to inline the implementation of these characters

```rust
pub trait OperationScore {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `insert_score`: The score for inserting character `c` into the string
- `delete_score`: The score for deleting character `c` from the string
- `substitution_score`: The score for replacing character `old` with character `new`
- `match_score`: The score for when a character is one string matches the character in the other string

##### Implementations

This trait is implemented for the following types:

- `EditDistance`

### Functions

#### Function `find_diff`

Finds the difference on a character by character level between two strings

Uses the Hirschberg algorithm (doi: [10.1145/360825.360861](http://dx.doi.org/10.1145/360825.360861))
which operates in `O(x * y)` time and `O(y)` space.  The algorithm finds the minimal set of operations
that will transform 'old' into 'new'.  The 'weight' of each operation is determined by the `scorer.`
For more details about weighting, see the [OperationScore](trait.OperationScore.html) documentation.

The operations in the returned `Diff `are presented in file order, with offsets assuming the
previous operations have already been performed.  Furthermore, the inserts are assumed to
be performed prior to the deletes.

# Example

```
use rdiff::string_diff::{find_diff, EditDistance};
// Find the difference between meadow and yellowing using the edit distance as the weighting.
let diff = find_diff("meadow", "yellowing", &EditDistance{});
// prints (0, 'y'), (3, 'll') and (9, 'ing')
for insert in diff.inserts() {
    println!("{:?}", insert);
}
// prints (1, 1) and (4, 2)
for delete in diff.deletes() {
    println!("{:?}", delete);
}
assert_eq!("yellowing", diff.apply_to_string("meadow").unwrap());
```

```rust
pub fn find_diff<S: OperationScore>(old: &str, new: &str, scorer: &S) -> super::Diff { /* ... */ }
```

#### Function `hirschberg`

Uses the Hirschberg algorithm to calculate the optimal set of operations to transform 'old' into 'new'.
The only parameters that are input are 'old', 'new' and `scorer`.  `x_rev` and `y_rev` are just
cached so that 'old' and 'new' don't need to be reversed for every recursion of the algorithm.
`diff` is the output of the algorithm and `insert_index` and `delete_index` are simply intermediate state
being passed around.

```rust
pub(in ::string_diff) fn hirschberg<S: OperationScore>(old: &str, new: &str, old_rev: &str, new_rev: &str, scorer: &S, diff: &mut super::Diff, insert_index: &mut usize, delete_index: &mut usize) { /* ... */ }
```

#### Function `nw_score`

Calculate the score based on the Needleman-Wunsch algorithm.  This algorithm
calculates the cost of transforming string 'old' into string 'new' using operation scoring
given by `scorer`.

It operates by iteratively generating the score for progressively longer
substrings of 'old' and 'new'.  The result is a vector of the transformation score
from 'old' to a substring of length `i` of 'new' where `i` is the index of an element in
the resulting vector.

```rust
pub(in ::string_diff) fn nw_score<S: OperationScore>(old: &str, new: &str, scorer: &S) -> Vec<i32> { /* ... */ }
```

### Macros

#### Macro `do_insert`

Handles updating the diff and relevant indexes when inserting a string
Needed because the string must be converted to bytes before it can be used in the diff

```rust
pub(crate) macro_rules! do_insert {
    /* macro_rules! do_insert {
    ($s: expr, $index: expr, $diff: expr) => { ... };
} */
}
```

#### Macro `do_delete`

Handles updating the diff and relevant indexes when deleting a suvstring
Needed because the string must be converted to bytes before it can be used in the diff

```rust
pub(crate) macro_rules! do_delete {
    /* macro_rules! do_delete {
    ($length: expr, $delete_index: expr, $insert_index: expr, $diff: expr) => { ... };
} */
}
```

## Types

### Struct `BlockHashes`

Used for calculating and re-calculating the differences between two versions of the same file

See the [module level documentation](index.html) for examples on how to use this

```rust
pub struct BlockHashes {
    pub(crate) hashes: std::collections::HashMap<u32, Vec<(usize, [u8; 16])>>,
    pub(crate) block_size: usize,
    pub(crate) file_size: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `hashes` | `std::collections::HashMap<u32, Vec<(usize, [u8; 16])>>` |  |
| `block_size` | `usize` |  |
| `file_size` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new<R: Read>(data_source: R, block_size: usize) -> Result<BlockHashes> { /* ... */ }
  ```
  Create a new BlockHash based on the data in data_source.  This method

- ```rust
  pub fn empty(block_size: usize) -> BlockHashes { /* ... */ }
  ```
  Construct a new block hash for a file that was just created

- ```rust
  pub fn diff_and_update<R: Read>(self: &mut Self, new_data: R) -> Result<Diff> { /* ... */ }
  ```
  Compare the data in `new_data` with the hashes computed from either

- ```rust
  pub fn verify_unchanged<R: Read>(self: &Self, data_source: &mut R) -> Result<bool> { /* ... */ }
  ```
  Checks if `data_source` has changed since the last time the hashes were updated.

- ```rust
  pub fn compress_to<W: Write>(self: &Self, writer: &mut W) -> Result<()> { /* ... */ }
  ```
  Compress these Hashes and write to `writer`.  The output can then be expanded

- ```rust
  pub fn expand_from<R: Read>(reader: &mut R) -> Result<BlockHashes> { /* ... */ }
  ```
  Expand these hashes from previously compressed data in `reader`.  The data in reader

- ```rust
  pub(in ::hashing) fn check_match<R: Read>(self: &Self, weak_hasher: &RollingHash, strong_hasher: &mut Md5, window: &Window<R>, last_matching_block_index: &mut i32) -> Option<usize> { /* ... */ }
  ```
  Checks if the current window frame matches any existing block with an index greater than the previously matched block.

- ```rust
  pub(in ::hashing) fn hash_match<R: Read>(self: &Self, weak_hasher: &RollingHash, strong_hasher: &mut Md5, window: &Window<R>) -> Option<usize> { /* ... */ }
  ```
  Checks to see if the hash of the current window frame matches an existing hash.

##### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **Send**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BlockHashes) -> bool { /* ... */ }
    ```

### Struct `Insert`

Represents an operation to insert bytes at a particular position into a file

```rust
pub struct Insert {
    pub(crate) position: usize,
    pub(crate) data: Vec<u8>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `position` | `usize` |  |
| `data` | `Vec<u8>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn get_position(self: &Self) -> usize { /* ... */ }
  ```
  Gets the byte position of this insert operation in its file

- ```rust
  pub fn get_data(self: &Self) -> &Vec<u8> { /* ... */ }
  ```
  Gets the data this insert operation will insert

- ```rust
  pub fn compress_to<W: Write>(self: &Self, writer: &mut W) -> io::Result<()> { /* ... */ }
  ```
  Compress this operation and write to `writer`.  The output can then be expanded

- ```rust
  pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Insert> { /* ... */ }
  ```
  Expand this operation from previously compressed data in `reader`.  The data in reader

##### Trait Implementations

- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Insert) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **StructuralPartialEq**
- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `Delete`

Represents an operation to delete a certain number of bytes at a particular position in a file

```rust
pub struct Delete {
    pub(crate) position: usize,
    pub(crate) len: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `position` | `usize` |  |
| `len` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn get_position(self: &Self) -> usize { /* ... */ }
  ```
  Gets the byte position of this delete operation in its file

- ```rust
  pub fn get_length(self: &Self) -> usize { /* ... */ }
  ```
  Gets the length in bytes of this delete operation

- ```rust
  pub fn compress_to<W: Write>(self: &Self, writer: &mut W) -> io::Result<()> { /* ... */ }
  ```
  Compress this operation and write to `writer`.  The output can then be expanded

- ```rust
  pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Delete> { /* ... */ }
  ```
  Expand this operation from previously compressed data in `reader`.  The data in reader

##### Trait Implementations

- **Sync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Delete) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
### Struct `Diff`

Represents a series of operations that were performed on a file to transform it into a new
version.

The operations are stored in file order, which means that every operation that affects
an earlier part of the file must be stored before an operation that affects a later part.
The diff also assumes that insert operations are performed prior to delete operations.

```rust
pub struct Diff {
    pub(crate) inserts: Vec<Insert>,
    pub(crate) deletes: Vec<Delete>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inserts` | `Vec<Insert>` |  |
| `deletes` | `Vec<Delete>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Diff { /* ... */ }
  ```
  Creates a new `Diff`

- ```rust
  pub(crate) fn add_insert(self: &mut Self, position: usize, data: Vec<u8>) { /* ... */ }
  ```
  Adds an insert operation into this diff.  The operation must occur after

- ```rust
  pub(crate) fn add_delete(self: &mut Self, position: usize, len: usize) { /* ... */ }
  ```
  all previously added insert and delete operations in file order.  If the operation

- ```rust
  pub fn inserts(self: &Self) -> Iter<''_, Insert> { /* ... */ }
  ```
  Gets an iterator over all insert operations

- ```rust
  pub fn deletes(self: &Self) -> Iter<''_, Delete> { /* ... */ }
  ```
  Gets an iterator over all delete operations

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Checks if this set of diffs has any actual content

- ```rust
  pub fn apply_to_string(self: &Self, string: &str) -> Result<String, FromUtf8Error> { /* ... */ }
  ```
  Applies all of the operations in the diff to the given string.

- ```rust
  pub fn apply(self: &Self, file: &mut File) -> io::Result<()> { /* ... */ }
  ```
  Apply the operations in this sequence to a file.  This should not be called until after

- ```rust
  pub fn compress_to<W: Write>(self: &Self, writer: &mut W) -> io::Result<()> { /* ... */ }
  ```
  Compress this diff and write to `writer`.  The output can then be expanded

- ```rust
  pub fn expand_from<R: Read>(reader: &mut R) -> io::Result<Diff> { /* ... */ }
  ```
  Expand this diff from previously compressed data in `reader`.  The data in reader

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Diff) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
### Struct `Window`

A sliding window over a reader.  This monatins an internal buffer read from the file,
which can be read from at any time.

```rust
pub(crate) struct Window<R: Read> {
    pub(crate) front: Vec<u8>,
    pub(crate) back: Vec<u8>,
    pub(crate) block_size: usize,
    pub(crate) offset: usize,
    pub(crate) bytes_read: usize,
    pub(crate) reader: R,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `front` | `Vec<u8>` |  |
| `back` | `Vec<u8>` |  |
| `block_size` | `usize` |  |
| `offset` | `usize` |  |
| `bytes_read` | `usize` |  |
| `reader` | `R` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new(reader: R, block_size: usize) -> Result<Window<R>> { /* ... */ }
  ```

- ```rust
  pub fn advance(self: &mut Self) -> Result<(Option<u8>, Option<u8>)> { /* ... */ }
  ```

- ```rust
  pub(in ::window) fn get_head(self: &Self) -> Option<u8> { /* ... */ }
  ```

- ```rust
  pub(in ::window) fn load_next_block(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn frame<''a>(self: &''a Self) -> (&''a [u8], &''a [u8]) { /* ... */ }
  ```

- ```rust
  pub fn frame_size(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn on_boundry(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn get_bytes_read(self: &Self) -> usize { /* ... */ }
  ```

##### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

