# Crate Documentation

**Version:** 0.1.8

**Format Version:** 39

# Module `algorithmica`

## Modules

## Module `graph`

```rust
pub mod graph { /* ... */ }
```

## Module `math`

```rust
pub mod math { /* ... */ }
```

### Modules

## Module `matrix`

```rust
pub mod matrix { /* ... */ }
```

### Functions

#### Function `multiply`

```rust
pub fn multiply<Matrix: AsRef<[Row]>, Matrix2: AsRef<[Row2]>, Row: AsRef<[f32]>, Row2: AsRef<[f32]>>(mat1: &Matrix, mat2: &Matrix2) -> Vec<Vec<f32>> { /* ... */ }
```

#### Function `add`

```rust
pub fn add<Matrix: AsRef<[Row]>, Row: AsRef<[f32]>>(mat1: &Matrix, mat2: &Matrix) -> Vec<Vec<f32>> { /* ... */ }
```

## Module `search`

```rust
pub mod search { /* ... */ }
```

### Modules

## Module `binary`

```rust
pub mod binary { /* ... */ }
```

### Functions

#### Function `binary_search_util`

```rust
pub(in ::search::binary) fn binary_search_util<T>(list: &[T], element: &T, start: isize, end: isize) -> bool
where
    T: PartialOrd { /* ... */ }
```

#### Function `search`

```rust
pub fn search<T>(list: &[T], element: &T) -> bool
where
    T: PartialOrd { /* ... */ }
```

## Module `sort`

```rust
pub mod sort { /* ... */ }
```

### Modules

## Module `bubble`

```rust
pub mod bubble { /* ... */ }
```

### Functions

#### Function `sort`

```rust
pub fn sort<T>(list: &mut [T])
where
    T: Ord { /* ... */ }
```

#### Function `sort_by`

```rust
pub fn sort_by<T, F>(list: &mut [T], f: F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering { /* ... */ }
```

## Module `heap_sort`

```rust
pub mod heap_sort { /* ... */ }
```

## Module `insertion`

```rust
pub mod insertion { /* ... */ }
```

### Functions

#### Function `sort`

```rust
pub fn sort<T>(list: &mut [T])
where
    T: Ord + Clone { /* ... */ }
```

#### Function `sort_by`

```rust
pub fn sort_by<T, F>(list: &mut [T], f: F)
where
    T: Ord + Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering { /* ... */ }
```

## Module `is_sorted`

```rust
pub mod is_sorted { /* ... */ }
```

### Functions

#### Function `is_sorted`

```rust
pub fn is_sorted<T>(list: &[T]) -> bool
where
    T: Ord { /* ... */ }
```

#### Function `is_sorted_desc`

```rust
pub fn is_sorted_desc<T>(list: &[T]) -> bool
where
    T: Ord { /* ... */ }
```

#### Function `is_sorted_by`

```rust
pub fn is_sorted_by<T, F>(list: &[T], f: F) -> bool
where
    T: Ord,
    F: Fn(&T, &T) -> bool { /* ... */ }
```

## Module `merge_sort`

```rust
pub mod merge_sort { /* ... */ }
```

### Functions

#### Function `get_by_index`

```rust
pub(in ::sort::merge_sort) unsafe fn get_by_index<T>(list: &[T], index: isize) -> *const T { /* ... */ }
```

#### Function `merge`

```rust
pub(in ::sort::merge_sort) fn merge<T: Debug, F>(list: &mut [T], start: usize, mid: usize, end: usize, compare: &F)
where
    F: Fn(&T, &T) -> bool { /* ... */ }
```

#### Function `merge_sort`

```rust
pub(in ::sort::merge_sort) fn merge_sort<T: Debug, F>(list: &mut [T], start: usize, end: usize, f: &F)
where
    F: Fn(&T, &T) -> bool { /* ... */ }
```

#### Function `sort`

```rust
pub fn sort<T>(list: &mut [T])
where
    T: Ord + Debug { /* ... */ }
```

#### Function `sort_by`

```rust
pub fn sort_by<T, F>(list: &mut [T], compare: &F)
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
    T: Debug { /* ... */ }
```

## Module `quick_sort`

```rust
pub mod quick_sort { /* ... */ }
```

### Functions

#### Function `quick_sort`

```rust
pub(in ::sort::quick_sort) fn quick_sort<T>(list: &mut [T], start: usize, end: usize)
where
    T: Ord + Clone { /* ... */ }
```

#### Function `sort`

```rust
pub fn sort<T>(list: &mut [T])
where
    T: Ord + Clone { /* ... */ }
```

## Module `selection`

```rust
pub mod selection { /* ... */ }
```

### Functions

#### Function `sort`

```rust
pub fn sort<T>(list: &mut [T])
where
    T: Ord { /* ... */ }
```

#### Function `sort_by`

```rust
pub fn sort_by<T, F>(list: &mut [T], f: F)
where
    T: Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering { /* ... */ }
```

## Module `subset`

```rust
pub mod subset { /* ... */ }
```

### Functions

#### Function `subset_util`

```rust
pub fn subset_util<T>(arr: &[T], st: usize, end: usize, reserve: &mut Vec<T>, subsets: &mut Vec<Vec<T>>)
where
    T: Clone { /* ... */ }
```

#### Function `find_all_subset`

This method will give all subsets of a set which is cloneable
pub fn find_all_subset<T>(arr: &[T]) -> Vec<Vec<T>> where  T: Clone

# Examples
```rust
use algorithmica::subset::find_all_subset;
let v = vec![1, 2, 3];
assert_eq!(
           find_all_subset(&v),
           vec![
               vec![1],
               vec![1, 2],
               vec![1, 2, 3],
               vec![1, 3],
               vec![2],
               vec![2, 3],
               vec![3]
           ]
       );
```

```rust
pub fn find_all_subset<T>(arr: &[T]) -> Vec<Vec<T>>
where
    T: Clone { /* ... */ }
```

## Module `tree`

```rust
pub mod tree { /* ... */ }
```

### Modules

## Module `bst`

```rust
pub mod bst { /* ... */ }
```

### Types

#### Enum `BST`

```rust
pub enum BST<T: Ord> {
    Leaf {
        value: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>,
    },
    Empty,
}
```

##### Variants

###### `Leaf`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `value` | `T` |  |
| `left` | `Box<BST<T>>` |  |
| `right` | `Box<BST<T>>` |  |

###### `Empty`

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn create(value: T) -> Self { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, new_value: T) { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn find(self: &Self, find_value: T) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `red_black`

```rust
pub mod red_black { /* ... */ }
```

### Types

#### Struct `Node`

```rust
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `value` | `i32` |  |
| `left` | `Option<Box<Node>>` |  |
| `right` | `Option<Box<Node>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn create(value: i32) -> Box<Self> { /* ... */ }
  ```

- ```rust
  pub fn add_new(root: Option<Box<Node>>, value: i32) -> Option<Box<Self>> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
