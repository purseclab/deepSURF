# Crate Documentation

**Version:** 0.26.1

**Format Version:** 39

# Module `rusqlite`

Rusqlite is an ergonomic wrapper for using SQLite from Rust. It attempts to
expose an interface similar to [rust-postgres](https://github.com/sfackler/rust-postgres).

```rust
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
```

## Modules

## Module `error`

```rust
pub(crate) mod error { /* ... */ }
```

### Types

#### Enum `Error`

**Attributes:**

- `#[allow(clippy::enum_variant_names)]`
- `#[non_exhaustive]`

Enum listing possible errors from rusqlite.

```rust
pub enum Error {
    SqliteFailure(ffi::Error, Option<String>),
    SqliteSingleThreadedMode,
    FromSqlConversionFailure(usize, crate::types::Type, Box<dyn error::Error + Send + Sync + ''static>),
    IntegralValueOutOfRange(usize, i64),
    Utf8Error(str::Utf8Error),
    NulError(::std::ffi::NulError),
    InvalidParameterName(String),
    InvalidPath(std::path::PathBuf),
    ExecuteReturnedResults,
    QueryReturnedNoRows,
    InvalidColumnIndex(usize),
    InvalidColumnName(String),
    InvalidColumnType(usize, String, crate::types::Type),
    StatementChangedRows(usize),
    ToSqlConversionFailure(Box<dyn error::Error + Send + Sync + ''static>),
    InvalidQuery,
    MultipleStatement,
    InvalidParameterCount(usize, usize),
}
```

##### Variants

###### `SqliteFailure`

An error from an underlying SQLite call.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ffi::Error` |  |
| 1 | `Option<String>` |  |

###### `SqliteSingleThreadedMode`

Error reported when attempting to open a connection when SQLite was
configured to allow single-threaded use only.

###### `FromSqlConversionFailure`

Error when the value of a particular column is requested, but it cannot
be converted to the requested Rust type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `crate::types::Type` |  |
| 2 | `Box<dyn error::Error + Send + Sync + ''static>` |  |

###### `IntegralValueOutOfRange`

Error when SQLite gives us an integral value outside the range of the
requested type (e.g., trying to get the value 1000 into a `u8`).
The associated `usize` is the column index,
and the associated `i64` is the value returned by SQLite.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `i64` |  |

###### `Utf8Error`

Error converting a string to UTF-8.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `str::Utf8Error` |  |

###### `NulError`

Error converting a string to a C-compatible string because it contained
an embedded nul.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `::std::ffi::NulError` |  |

###### `InvalidParameterName`

Error when using SQL named parameters and passing a parameter name not
present in the SQL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `InvalidPath`

Error converting a file path to a string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::path::PathBuf` |  |

###### `ExecuteReturnedResults`

Error returned when an [`execute`](crate::Connection::execute) call
returns rows.

###### `QueryReturnedNoRows`

Error when a query that was expected to return at least one row (e.g.,
for [`query_row`](crate::Connection::query_row)) did not return any.

###### `InvalidColumnIndex`

Error when the value of a particular column is requested, but the index
is out of range for the statement.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `InvalidColumnName`

Error when the value of a named column is requested, but no column
matches the name for the statement.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `InvalidColumnType`

Error when the value of a particular column is requested, but the type
of the result in that column cannot be converted to the requested
Rust type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `String` |  |
| 2 | `crate::types::Type` |  |

###### `StatementChangedRows`

Error when a query that was expected to insert one row did not insert
any or insert many.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `ToSqlConversionFailure`

Error available for the implementors of the
[`ToSql`](crate::types::ToSql) trait.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn error::Error + Send + Sync + ''static>` |  |

###### `InvalidQuery`

Error when the SQL is not a `SELECT`, is not read-only.

###### `MultipleStatement`

Error when the SQL contains multiple statements.

###### `InvalidParameterCount`

Error when the number of bound parameters does not match the number of
parameters in the query. The first `usize` is how many parameters were
given, the 2nd is how many were expected.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `usize` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn error::Error + ''static> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Error) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(err: str::Utf8Error) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: ::std::ffi::NulError) -> Error { /* ... */ }
    ```

  - ```rust
    fn from(err: FromSqlError) -> Error { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

### Functions

#### Function `error_from_sqlite_code`

**Attributes:**

- `#[cold]`

```rust
pub fn error_from_sqlite_code(code: std::os::raw::c_int, message: Option<String>) -> Error { /* ... */ }
```

#### Function `error_from_handle`

**Attributes:**

- `#[cold]`

```rust
pub unsafe fn error_from_handle(db: *mut ffi::sqlite3, code: std::os::raw::c_int) -> Error { /* ... */ }
```

#### Function `check`

```rust
pub fn check(code: std::os::raw::c_int) -> crate::Result<()> { /* ... */ }
```

### Constants and Statics

#### Constant `UNKNOWN_COLUMN`

```rust
pub(in ::error) const UNKNOWN_COLUMN: usize = std::usize::MAX;
```

## Module `busy`

```rust
pub(crate) mod busy { /* ... */ }
```

## Module `cache`

Prepared statements cache for faster execution.

```rust
pub(crate) mod cache { /* ... */ }
```

### Types

#### Struct `StatementCache`

Prepared statements LRU cache.

```rust
pub struct StatementCache(pub(in ::cache) std::cell::RefCell<hashlink::LruCache<std::sync::Arc<str>, crate::raw_statement::RawStatement>>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::cell::RefCell<hashlink::LruCache<std::sync::Arc<str>, crate::raw_statement::RawStatement>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn with_capacity(capacity: usize) -> StatementCache { /* ... */ }
  ```
  Create a statement cache.

- ```rust
  pub(in ::cache) fn set_capacity(self: &Self, capacity: usize) { /* ... */ }
  ```

- ```rust
  pub(in ::cache) fn get<''conn>(self: &''conn Self, conn: &''conn Connection, sql: &str) -> Result<CachedStatement<''conn>> { /* ... */ }
  ```

- ```rust
  pub(in ::cache) fn cache_stmt(self: &Self, stmt: RawStatement) { /* ... */ }
  ```

- ```rust
  pub(in ::cache) fn flush(self: &Self) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
#### Struct `CachedStatement`

Cacheable statement.

Statement will return automatically to the cache by default.
If you want the statement to be discarded, call
[`discard()`](CachedStatement::discard) on it.

```rust
pub struct CachedStatement<''conn> {
    pub(in ::cache) stmt: Option<crate::Statement<''conn>>,
    pub(in ::cache) cache: &''conn StatementCache,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `stmt` | `Option<crate::Statement<''conn>>` |  |
| `cache` | `&''conn StatementCache` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::cache) fn new<''conn>(stmt: Statement<''conn>, cache: &''conn StatementCache) -> CachedStatement<''conn> { /* ... */ }
  ```

- ```rust
  pub fn discard(self: Self) { /* ... */ }
  ```
  Discard the statement, preventing it from being returned to its

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Statement<''conn> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut Statement<''conn> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Sync**
## Module `column`

```rust
pub(crate) mod column { /* ... */ }
```

### Types

#### Struct `Column`

Information about a column of a SQLite query.

```rust
pub struct Column<''stmt> {
    pub(in ::column) name: &''stmt str,
    pub(in ::column) decl_type: Option<&''stmt str>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `&''stmt str` |  |
| `decl_type` | `Option<&''stmt str>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Returns the name of the column.

- ```rust
  pub fn decl_type(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the type of the column (`None` for expression).

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
## Module `config`

Configure database connections

```rust
pub mod config { /* ... */ }
```

### Types

#### Enum `DbConfig`

**Attributes:**

- `#[repr(i32)]`
- `#[allow(non_snake_case, non_camel_case_types)]`
- `#[non_exhaustive]`
- `#[allow(clippy::upper_case_acronyms)]`

Database Connection Configuration Options
See [Database Connection Configuration Options](https://sqlite.org/c3ref/c_dbconfig_enable_fkey.html) for details.

```rust
pub enum DbConfig {
    SQLITE_DBCONFIG_ENABLE_FKEY = 1002,
    SQLITE_DBCONFIG_ENABLE_TRIGGER = 1003,
    SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER = 1004,
    SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE = 1006,
    SQLITE_DBCONFIG_ENABLE_QPSG = 1007,
    SQLITE_DBCONFIG_TRIGGER_EQP = 1008,
    SQLITE_DBCONFIG_DEFENSIVE = 1010,
}
```

##### Variants

###### `SQLITE_DBCONFIG_ENABLE_FKEY`

Enable or disable the enforcement of foreign key constraints.

Discriminant: `1002`

Discriminant value: `1002`

###### `SQLITE_DBCONFIG_ENABLE_TRIGGER`

Enable or disable triggers.

Discriminant: `1003`

Discriminant value: `1003`

###### `SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER`

Enable or disable the fts3_tokenizer() function which is part of the
FTS3 full-text search engine extension.

Discriminant: `1004`

Discriminant value: `1004`

###### `SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE`

In WAL mode, enable or disable the checkpoint operation before closing
the connection.

Discriminant: `1006`

Discriminant value: `1006`

###### `SQLITE_DBCONFIG_ENABLE_QPSG`

Activates or deactivates the query planner stability guarantee (QPSG).

Discriminant: `1007`

Discriminant value: `1007`

###### `SQLITE_DBCONFIG_TRIGGER_EQP`

Includes or excludes output for any operations performed by trigger
programs from the output of EXPLAIN QUERY PLAN commands.

Discriminant: `1008`

Discriminant value: `1008`

###### `SQLITE_DBCONFIG_DEFENSIVE`

Activates or deactivates the "defensive" flag for a database connection.

Discriminant: `1010`

Discriminant value: `1010`

##### Implementations

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `inner_connection`

```rust
pub(crate) mod inner_connection { /* ... */ }
```

### Types

#### Struct `InnerConnection`

```rust
pub struct InnerConnection {
    pub db: *mut ffi::sqlite3,
    pub(in ::inner_connection) interrupt_lock: std::sync::Arc<std::sync::Mutex<*mut ffi::sqlite3>>,
    pub(in ::inner_connection) owned: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `db` | `*mut ffi::sqlite3` |  |
| `interrupt_lock` | `std::sync::Arc<std::sync::Mutex<*mut ffi::sqlite3>>` |  |
| `owned` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::busy) fn busy_timeout(self: &mut Self, timeout: c_int) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub unsafe fn new(db: *mut ffi::sqlite3, owned: bool) -> InnerConnection { /* ... */ }
  ```

- ```rust
  pub fn open_with_flags(c_path: &CStr, flags: OpenFlags, vfs: Option<&CStr>) -> Result<InnerConnection> { /* ... */ }
  ```

- ```rust
  pub fn db(self: &Self) -> *mut ffi::sqlite3 { /* ... */ }
  ```

- ```rust
  pub fn decode_result(self: &Self, code: c_int) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub(in ::inner_connection) unsafe fn decode_result_raw(db: *mut ffi::sqlite3, code: c_int) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn close(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn get_interrupt_handle(self: &Self) -> InterruptHandle { /* ... */ }
  ```

- ```rust
  pub fn last_insert_rowid(self: &Self) -> i64 { /* ... */ }
  ```

- ```rust
  pub fn prepare<''a>(self: &mut Self, conn: &''a Connection, sql: &str) -> Result<Statement<''a>> { /* ... */ }
  ```

- ```rust
  pub fn changes(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_autocommit(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(in ::inner_connection) fn remove_hooks(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `ensure_valid_sqlite_version`

**Attributes:**

- `#[cfg(not(feature = "bundled"))]`

```rust
pub(in ::inner_connection) fn ensure_valid_sqlite_version() { /* ... */ }
```

#### Function `ensure_safe_sqlite_threading_mode`

**Attributes:**

- `#[cfg(not(any(target_arch = "wasm32")))]`

```rust
pub(in ::inner_connection) fn ensure_safe_sqlite_threading_mode() -> super::Result<()> { /* ... */ }
```

### Constants and Statics

#### Static `SQLITE_VERSION_CHECK`

**Attributes:**

- `#[cfg(not(feature = "bundled"))]`

```rust
pub(in ::inner_connection) static SQLITE_VERSION_CHECK: std::sync::Once = _;
```

#### Static `BYPASS_VERSION_CHECK`

**Attributes:**

- `#[cfg(not(feature = "bundled"))]`

```rust
pub static BYPASS_VERSION_CHECK: std::sync::atomic::AtomicBool = _;
```

#### Static `SQLITE_INIT`

**Attributes:**

- `#[cfg(not(any(target_arch = "wasm32")))]`

```rust
pub(in ::inner_connection) static SQLITE_INIT: std::sync::Once = _;
```

#### Static `BYPASS_SQLITE_INIT`

```rust
pub static BYPASS_SQLITE_INIT: std::sync::atomic::AtomicBool = _;
```

## Module `params`

```rust
pub(crate) mod params { /* ... */ }
```

### Modules

## Module `sealed`

```rust
pub(in ::params) mod sealed { /* ... */ }
```

### Traits

#### Trait `Sealed`

This trait exists just to ensure that the only impls of `trait Params`
that are allowed are ones in this crate.

```rust
pub trait Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `[&dyn ToSql + Send + Sync; 0]`
- `&[&dyn ToSql]`
- `&[(&str, &dyn ToSql)]`
- `&[&T; 1]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 1]` with <T: ToSql + ?Sized>
- `[T; 1]` with <T: ToSql>
- `&[&T; 2]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 2]` with <T: ToSql + ?Sized>
- `[T; 2]` with <T: ToSql>
- `&[&T; 3]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 3]` with <T: ToSql + ?Sized>
- `[T; 3]` with <T: ToSql>
- `&[&T; 4]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 4]` with <T: ToSql + ?Sized>
- `[T; 4]` with <T: ToSql>
- `&[&T; 5]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 5]` with <T: ToSql + ?Sized>
- `[T; 5]` with <T: ToSql>
- `&[&T; 6]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 6]` with <T: ToSql + ?Sized>
- `[T; 6]` with <T: ToSql>
- `&[&T; 7]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 7]` with <T: ToSql + ?Sized>
- `[T; 7]` with <T: ToSql>
- `&[&T; 8]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 8]` with <T: ToSql + ?Sized>
- `[T; 8]` with <T: ToSql>
- `&[&T; 9]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 9]` with <T: ToSql + ?Sized>
- `[T; 9]` with <T: ToSql>
- `&[&T; 10]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 10]` with <T: ToSql + ?Sized>
- `[T; 10]` with <T: ToSql>
- `&[&T; 11]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 11]` with <T: ToSql + ?Sized>
- `[T; 11]` with <T: ToSql>
- `&[&T; 12]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 12]` with <T: ToSql + ?Sized>
- `[T; 12]` with <T: ToSql>
- `&[&T; 13]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 13]` with <T: ToSql + ?Sized>
- `[T; 13]` with <T: ToSql>
- `&[&T; 14]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 14]` with <T: ToSql + ?Sized>
- `[T; 14]` with <T: ToSql>
- `&[&T; 15]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 15]` with <T: ToSql + ?Sized>
- `[T; 15]` with <T: ToSql>
- `&[&T; 16]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 16]` with <T: ToSql + ?Sized>
- `[T; 16]` with <T: ToSql>
- `&[&T; 17]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 17]` with <T: ToSql + ?Sized>
- `[T; 17]` with <T: ToSql>
- `&[&T; 18]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 18]` with <T: ToSql + ?Sized>
- `[T; 18]` with <T: ToSql>
- `&[&T; 19]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 19]` with <T: ToSql + ?Sized>
- `[T; 19]` with <T: ToSql>
- `&[&T; 20]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 20]` with <T: ToSql + ?Sized>
- `[T; 20]` with <T: ToSql>
- `&[&T; 21]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 21]` with <T: ToSql + ?Sized>
- `[T; 21]` with <T: ToSql>
- `&[&T; 22]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 22]` with <T: ToSql + ?Sized>
- `[T; 22]` with <T: ToSql>
- `&[&T; 23]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 23]` with <T: ToSql + ?Sized>
- `[T; 23]` with <T: ToSql>
- `&[&T; 24]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 24]` with <T: ToSql + ?Sized>
- `[T; 24]` with <T: ToSql>
- `&[&T; 25]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 25]` with <T: ToSql + ?Sized>
- `[T; 25]` with <T: ToSql>
- `&[&T; 26]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 26]` with <T: ToSql + ?Sized>
- `[T; 26]` with <T: ToSql>
- `&[&T; 27]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 27]` with <T: ToSql + ?Sized>
- `[T; 27]` with <T: ToSql>
- `&[&T; 29]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 29]` with <T: ToSql + ?Sized>
- `[T; 29]` with <T: ToSql>
- `&[&T; 30]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 30]` with <T: ToSql + ?Sized>
- `[T; 30]` with <T: ToSql>
- `&[&T; 31]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 31]` with <T: ToSql + ?Sized>
- `[T; 31]` with <T: ToSql>
- `&[&T; 32]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 32]` with <T: ToSql + ?Sized>
- `[T; 32]` with <T: ToSql>
- `ParamsFromIter<I>` with <I>

### Types

#### Struct `ParamsFromIter`

Adapter type which allows any iterator over [`ToSql`] values to implement
[`Params`].

This struct is created by the [`params_from_iter`] function.

This can be useful if you have something like an `&[String]` (of unknown
length), and you want to use them with an API that wants something
implementing `Params`. This way, you can avoid having to allocate storage
for something like a `&[&dyn ToSql]`.

This essentially is only ever actually needed when dynamically generating
SQL — static SQL (by definition) has the number of parameters known
statically. As dynamically generating SQL is itself pretty advanced, this
API is itself for advanced use cases (See "Realistic use case" in the
examples).

# Example

## Basic usage

```rust,no_run
use rusqlite::{Connection, Result, params_from_iter};
use std::collections::BTreeSet;

fn query(conn: &Connection, ids: &BTreeSet<String>) -> Result<()> {
    assert_eq!(ids.len(), 3, "Unrealistic sample code");

    let mut stmt = conn.prepare("SELECT * FROM users WHERE id IN (?, ?, ?)")?;
    let _rows = stmt.query(params_from_iter(ids.iter()))?;

    // use _rows...
    Ok(())
}
```

## Realistic use case

Here's how you'd use `ParamsFromIter` to call [`Statement::exists`] with a
dynamic number of parameters.

```rust,no_run
use rusqlite::{Connection, Result};

pub fn any_active_users(conn: &Connection, usernames: &[String]) -> Result<bool> {
    if usernames.is_empty() {
        return Ok(false);
    }

    // Note: `repeat_vars` never returns anything attacker-controlled, so
    // it's fine to use it in a dynamically-built SQL string.
    let vars = repeat_vars(usernames.len());

    let sql = format!(
        // In practice this would probably be better as an `EXISTS` query.
        "SELECT 1 FROM user WHERE is_active AND name IN ({}) LIMIT 1",
        vars,
    );
    let mut stmt = conn.prepare(&sql)?;
    stmt.exists(rusqlite::params_from_iter(usernames))
}

// Helper function to return a comma-separated sequence of `?`.
// - `repeat_vars(0) => panic!(...)`
// - `repeat_vars(1) => "?"`
// - `repeat_vars(2) => "?,?"`
// - `repeat_vars(3) => "?,?,?"`
// - ...
fn repeat_vars(count: usize) -> String {
    assert_ne!(count, 0);
    let mut s = "?,".repeat(count);
    // Remove trailing comma
    s.pop();
    s
}
```

That is fairly complex, and even so would need even more work to be fully
production-ready:

- production code should ensure `usernames` isn't so large that it will
  surpass [`conn.limit(Limit::SQLITE_LIMIT_VARIABLE_NUMBER)`][limits]),
  chunking if too large. (Note that the limits api requires rusqlite to have
  the "limits" feature).

- `repeat_vars` can be implemented in a way that avoids needing to allocate
  a String.

- Etc...

[limits]: crate::Connection::limit

This complexity reflects the fact that `ParamsFromIter` is mainly intended
for advanced use cases — most of the time you should know how many
parameters you have statically (and if you don't, you're either doing
something tricky, or should take a moment to think about the design).

```rust
pub struct ParamsFromIter<I>(pub(in ::params) I);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `I` |  |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sealed**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Params**
- **RefUnwindSafe**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParamsFromIter<I> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
### Traits

#### Trait `Params`

Trait used for [sets of parameter][params] passed into SQL
statements/queries.

[params]: https://www.sqlite.org/c3ref/bind_blob.html

Note: Currently, this trait can only be implemented inside this crate.
Additionally, it's methods (which are `doc(hidden)`) should currently not be
considered part of the stable API, although it's possible they will
stabilize in the future.

# Passing parameters to SQLite

Many functions in this library let you pass parameters to SQLite. Doing this
lets you avoid any risk of SQL injection, and is simpler than escaping
things manually. Aside from deprecated functions and a few helpers, this is
indicated by the function taking a generic argument that implements `Params`
(this trait).

## Positional parameters

For cases where you want to pass a list of parameters where the number of
parameters is known at compile time, this can be done in one of the
following ways:

- Using the [`rusqlite::params!`](crate::params!) macro, e.g.
  `thing.query(rusqlite::params![1, "foo", bar])`. This is mostly useful for
  heterogeneous lists of parameters, or lists where the number of parameters
  exceeds 32.

- For small heterogeneous lists of parameters, they can either be passed as:

    - an array, as in `thing.query([1i32, 2, 3, 4])` or `thing.query(["foo",
      "bar", "baz"])`.

    - a reference to an array of references, as in `thing.query(&["foo",
      "bar", "baz"])` or `thing.query(&[&1i32, &2, &3])`.

        (Note: in this case we don't implement this for slices for coherence
        reasons, so it really is only for the "reference to array" types —
        hence why the number of parameters must be <= 32 or you need to
        reach for `rusqlite::params!`)

    Unfortunately, in the current design it's not possible to allow this for
    references to arrays of non-references (e.g. `&[1i32, 2, 3]`). Code like
    this should instead either use `params!`, an array literal, a `&[&dyn
    ToSql]` or if none of those work, [`ParamsFromIter`].

- As a slice of `ToSql` trait object references, e.g. `&[&dyn ToSql]`. This
  is mostly useful for passing parameter lists around as arguments without
  having every function take a generic `P: Params`.

### Example (positional)

```rust,no_run
# use rusqlite::{Connection, Result, params};
fn update_rows(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO test (a, b) VALUES (?, ?)")?;

    // Using `rusqlite::params!`:
    stmt.execute(params![1i32, "blah"])?;

    // array literal — non-references
    stmt.execute([2i32, 3i32])?;

    // array literal — references
    stmt.execute(["foo", "bar"])?;

    // Slice literal, references:
    stmt.execute(&[&2i32, &3i32])?;

    // Note: The types behind the references don't have to be `Sized`
    stmt.execute(&["foo", "bar"])?;

    // However, this doesn't work (see above):
    // stmt.execute(&[1i32, 2i32])?;
    Ok(())
}
```

## Named parameters

SQLite lets you name parameters using a number of conventions (":foo",
"@foo", "$foo"). You can pass named parameters in to SQLite using rusqlite
in a few ways:

- Using the [`rusqlite::named_params!`](crate::named_params!) macro, as in
  `stmt.execute(named_params!{ ":name": "foo", ":age": 99 })`. Similar to
  the `params` macro, this is most useful for heterogeneous lists of
  parameters, or lists where the number of parameters exceeds 32.

- As a slice of `&[(&str, &dyn ToSql)]`. This is what essentially all of
  these boil down to in the end, conceptually at least. In theory you can
  pass this as `stmt.

- As array references, similar to the positional params. This looks like
  `thing.query(&[(":foo", &1i32), (":bar", &2i32)])` or
  `thing.query(&[(":foo", "abc"), (":bar", "def")])`.

Note: Unbound named parameters will be left to the value they previously
were bound with, falling back to `NULL` for parameters which have never been
bound.

### Example (named)

```rust,no_run
# use rusqlite::{Connection, Result, named_params};
fn insert(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("INSERT INTO test (key, value) VALUES (:key, :value)")?;
    // Using `rusqlite::params!`:
    stmt.execute(named_params!{ ":key": "one", ":val": 2 })?;
    // Alternatively:
    stmt.execute(&[(":key", "three"), (":val", "four")])?;
    // Or:
    stmt.execute(&[(":key", &100), (":val", &200)])?;
    Ok(())
}
```

## No parameters

You can just use an empty array literal for no params. The
`rusqlite::NO_PARAMS` constant which was so common in previous versions of
this library is no longer needed (and is now deprecated).

### Example (no parameters)

```rust,no_run
# use rusqlite::{Connection, Result, params};
fn delete_all_users(conn: &Connection) -> Result<()> {
    // Just use an empty array (e.g. `[]`) for no params.
    conn.execute("DELETE FROM users", [])?;
    Ok(())
}
```

## Dynamic parameter list

If you have a number of parameters which is unknown at compile time (for
example, building a dynamic query at runtime), you have two choices:

- Use a `&[&dyn ToSql]`, which is nice if you have one otherwise might be
  annoying.
- Use the [`ParamsFromIter`] type. This essentially lets you wrap an
  iterator some `T: ToSql` with something that implements `Params`.

A lot of the considerations here are similar either way, so you should see
the [`ParamsFromIter`] documentation for more info / examples.

```rust
pub trait Params: Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `[&dyn ToSql + Send + Sync; 0]`
- `&[&dyn ToSql]`
- `&[(&str, &dyn ToSql)]`
- `&[&T; 1]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 1]` with <T: ToSql + ?Sized>
- `[T; 1]` with <T: ToSql>
- `&[&T; 2]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 2]` with <T: ToSql + ?Sized>
- `[T; 2]` with <T: ToSql>
- `&[&T; 3]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 3]` with <T: ToSql + ?Sized>
- `[T; 3]` with <T: ToSql>
- `&[&T; 4]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 4]` with <T: ToSql + ?Sized>
- `[T; 4]` with <T: ToSql>
- `&[&T; 5]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 5]` with <T: ToSql + ?Sized>
- `[T; 5]` with <T: ToSql>
- `&[&T; 6]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 6]` with <T: ToSql + ?Sized>
- `[T; 6]` with <T: ToSql>
- `&[&T; 7]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 7]` with <T: ToSql + ?Sized>
- `[T; 7]` with <T: ToSql>
- `&[&T; 8]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 8]` with <T: ToSql + ?Sized>
- `[T; 8]` with <T: ToSql>
- `&[&T; 9]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 9]` with <T: ToSql + ?Sized>
- `[T; 9]` with <T: ToSql>
- `&[&T; 10]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 10]` with <T: ToSql + ?Sized>
- `[T; 10]` with <T: ToSql>
- `&[&T; 11]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 11]` with <T: ToSql + ?Sized>
- `[T; 11]` with <T: ToSql>
- `&[&T; 12]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 12]` with <T: ToSql + ?Sized>
- `[T; 12]` with <T: ToSql>
- `&[&T; 13]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 13]` with <T: ToSql + ?Sized>
- `[T; 13]` with <T: ToSql>
- `&[&T; 14]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 14]` with <T: ToSql + ?Sized>
- `[T; 14]` with <T: ToSql>
- `&[&T; 15]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 15]` with <T: ToSql + ?Sized>
- `[T; 15]` with <T: ToSql>
- `&[&T; 16]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 16]` with <T: ToSql + ?Sized>
- `[T; 16]` with <T: ToSql>
- `&[&T; 17]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 17]` with <T: ToSql + ?Sized>
- `[T; 17]` with <T: ToSql>
- `&[&T; 18]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 18]` with <T: ToSql + ?Sized>
- `[T; 18]` with <T: ToSql>
- `&[&T; 19]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 19]` with <T: ToSql + ?Sized>
- `[T; 19]` with <T: ToSql>
- `&[&T; 20]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 20]` with <T: ToSql + ?Sized>
- `[T; 20]` with <T: ToSql>
- `&[&T; 21]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 21]` with <T: ToSql + ?Sized>
- `[T; 21]` with <T: ToSql>
- `&[&T; 22]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 22]` with <T: ToSql + ?Sized>
- `[T; 22]` with <T: ToSql>
- `&[&T; 23]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 23]` with <T: ToSql + ?Sized>
- `[T; 23]` with <T: ToSql>
- `&[&T; 24]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 24]` with <T: ToSql + ?Sized>
- `[T; 24]` with <T: ToSql>
- `&[&T; 25]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 25]` with <T: ToSql + ?Sized>
- `[T; 25]` with <T: ToSql>
- `&[&T; 26]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 26]` with <T: ToSql + ?Sized>
- `[T; 26]` with <T: ToSql>
- `&[&T; 27]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 27]` with <T: ToSql + ?Sized>
- `[T; 27]` with <T: ToSql>
- `&[&T; 29]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 29]` with <T: ToSql + ?Sized>
- `[T; 29]` with <T: ToSql>
- `&[&T; 30]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 30]` with <T: ToSql + ?Sized>
- `[T; 30]` with <T: ToSql>
- `&[&T; 31]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 31]` with <T: ToSql + ?Sized>
- `[T; 31]` with <T: ToSql>
- `&[&T; 32]` with <T: ToSql + ?Sized>
- `&[(&str, &T); 32]` with <T: ToSql + ?Sized>
- `[T; 32]` with <T: ToSql>
- `ParamsFromIter<I>` with <I>

### Functions

#### Function `params_from_iter`

**Attributes:**

- `#[inline]`

Constructor function for a [`ParamsFromIter`]. See its documentation for
more.

```rust
pub fn params_from_iter<I>(iter: I) -> ParamsFromIter<I>
where
    I: IntoIterator,
    <I as >::Item: ToSql { /* ... */ }
```

### Macros

#### Macro `impl_for_array_ref`

```rust
pub(crate) macro_rules! impl_for_array_ref {
    /* macro_rules! impl_for_array_ref {
    ($($N:literal)+) => { ... };
} */
}
```

## Module `pragma`

Pragma helpers

```rust
pub(crate) mod pragma { /* ... */ }
```

### Types

#### Struct `Sql`

```rust
pub struct Sql {
    pub(in ::pragma) buf: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Sql { /* ... */ }
  ```

- ```rust
  pub fn push_pragma(self: &mut Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn push_keyword(self: &mut Self, keyword: &str) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn push_schema_name(self: &mut Self, schema_name: DatabaseName<''_>) { /* ... */ }
  ```

- ```rust
  pub fn push_identifier(self: &mut Self, s: &str) { /* ... */ }
  ```

- ```rust
  pub fn push_value(self: &mut Self, value: &dyn ToSql) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn push_string_literal(self: &mut Self, s: &str) { /* ... */ }
  ```

- ```rust
  pub fn push_int(self: &mut Self, i: i64) { /* ... */ }
  ```

- ```rust
  pub fn push_real(self: &mut Self, f: f64) { /* ... */ }
  ```

- ```rust
  pub fn push_space(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn push_dot(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn push_equal_sign(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn open_brace(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn close_brace(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub(in ::pragma) fn wrap_and_escape(self: &mut Self, s: &str, quote: char) { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Receiver**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &str { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
### Functions

#### Function `is_identifier`

```rust
pub(in ::pragma) fn is_identifier(s: &str) -> bool { /* ... */ }
```

#### Function `is_identifier_start`

```rust
pub(in ::pragma) fn is_identifier_start(c: char) -> bool { /* ... */ }
```

#### Function `is_identifier_continue`

```rust
pub(in ::pragma) fn is_identifier_continue(c: char) -> bool { /* ... */ }
```

## Module `raw_statement`

```rust
pub(crate) mod raw_statement { /* ... */ }
```

### Types

#### Struct `RawStatement`

```rust
pub struct RawStatement {
    pub(in ::raw_statement) ptr: *mut ffi::sqlite3_stmt,
    pub(in ::raw_statement) tail: usize,
    pub(in ::raw_statement) cache: param_cache::ParamIndexCache,
    pub(in ::raw_statement) statement_cache_key: Option<std::sync::Arc<str>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ptr` | `*mut ffi::sqlite3_stmt` |  |
| `tail` | `usize` |  |
| `cache` | `param_cache::ParamIndexCache` |  |
| `statement_cache_key` | `Option<std::sync::Arc<str>>` |  |

##### Implementations

###### Methods

- ```rust
  pub unsafe fn new(stmt: *mut ffi::sqlite3_stmt, tail: usize) -> RawStatement { /* ... */ }
  ```

- ```rust
  pub fn is_null(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub(crate) fn set_statement_cache_key</* synthetic */ impl Into<Arc<str>>: Into<Arc<str>>>(self: &mut Self, p: impl Into<Arc<str>>) { /* ... */ }
  ```

- ```rust
  pub(crate) fn statement_cache_key(self: &Self) -> Option<Arc<str>> { /* ... */ }
  ```

- ```rust
  pub unsafe fn ptr(self: &Self) -> *mut ffi::sqlite3_stmt { /* ... */ }
  ```

- ```rust
  pub fn column_count(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn column_type(self: &Self, idx: usize) -> c_int { /* ... */ }
  ```

- ```rust
  pub fn column_name(self: &Self, idx: usize) -> Option<&CStr> { /* ... */ }
  ```

- ```rust
  pub fn step(self: &Self) -> c_int { /* ... */ }
  ```

- ```rust
  pub fn reset(self: &Self) -> c_int { /* ... */ }
  ```

- ```rust
  pub fn bind_parameter_count(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn bind_parameter_index(self: &Self, name: &str) -> Option<usize> { /* ... */ }
  ```

- ```rust
  pub fn bind_parameter_name(self: &Self, index: i32) -> Option<&CStr> { /* ... */ }
  ```

- ```rust
  pub fn clear_bindings(self: &Self) -> c_int { /* ... */ }
  ```

- ```rust
  pub fn sql(self: &Self) -> Option<&CStr> { /* ... */ }
  ```

- ```rust
  pub fn finalize(self: Self) -> c_int { /* ... */ }
  ```

- ```rust
  pub(in ::raw_statement) fn finalize_(self: &mut Self) -> c_int { /* ... */ }
  ```

- ```rust
  pub fn get_status(self: &Self, status: StatementStatus, reset: bool) -> i32 { /* ... */ }
  ```

- ```rust
  pub fn tail(self: &Self) -> usize { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
## Module `row`

```rust
pub(crate) mod row { /* ... */ }
```

### Modules

## Module `sealed`

```rust
pub(in ::row) mod sealed { /* ... */ }
```

### Traits

#### Trait `Sealed`

This trait exists just to ensure that the only impls of `trait Params`
that are allowed are ones in this crate.

```rust
pub trait Sealed {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `usize`
- `&str`

### Types

#### Struct `Rows`

**Attributes:**

- `#[must_use = "Rows is lazy and will do nothing unless consumed"]`

An handle for the resulting rows of a query.

```rust
pub struct Rows<''stmt> {
    pub(crate) stmt: Option<&''stmt super::Statement<''stmt>>,
    pub(in ::row) row: Option<Row<''stmt>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `stmt` | `Option<&''stmt super::Statement<''stmt>>` |  |
| `row` | `Option<Row<''stmt>>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::row) fn reset(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn next(self: &mut Self) -> Result<Option<&Row<''stmt>>> { /* ... */ }
  ```
  Attempt to get the next row from the query. Returns `Ok(Some(Row))` if

- ```rust
  pub fn map<F, B>(self: Self, f: F) -> Map<''stmt, F>
where
    F: FnMut(&Row<''_>) -> Result<B> { /* ... */ }
  ```
  Map over this `Rows`, converting it to a [`Map`], which

- ```rust
  pub fn mapped<F, B>(self: Self, f: F) -> MappedRows<''stmt, F>
where
    F: FnMut(&Row<''_>) -> Result<B> { /* ... */ }
  ```
  Map over this `Rows`, converting it to a [`MappedRows`], which

- ```rust
  pub fn and_then<F, T, E>(self: Self, f: F) -> AndThenRows<''stmt, F>
where
    F: FnMut(&Row<''_>) -> Result<T, E> { /* ... */ }
  ```
  Map over this `Rows` with a fallible function, converting it to a

- ```rust
  pub fn as_ref(self: &Self) -> Option<&Statement<''stmt>> { /* ... */ }
  ```
  Give access to the underlying statement

- ```rust
  pub(crate) fn new(stmt: &''stmt Statement<''stmt>) -> Rows<''stmt> { /* ... */ }
  ```

- ```rust
  pub(crate) fn get_expected_row(self: &mut Self) -> Result<&Row<''stmt>> { /* ... */ }
  ```

###### Trait Implementations

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FallibleStreamingIterator**
  - ```rust
    fn advance(self: &mut Self) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn get(self: &Self) -> Option<&Row<''stmt>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
#### Struct `Map`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

`F` is used to transform the _streaming_ iterator into a _fallible_
iterator.

```rust
pub struct Map<''stmt, F> {
    pub(in ::row) rows: Rows<''stmt>,
    pub(in ::row) f: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rows` | `Rows<''stmt>` |  |
| `f` | `F` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoFallibleIterator**
  - ```rust
    fn into_fallible_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **FallibleIterator**
  - ```rust
    fn next(self: &mut Self) -> Result<Option<B>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

#### Struct `MappedRows`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

An iterator over the mapped resulting rows of a query.

`F` is used to transform the _streaming_ iterator into a _standard_
iterator.

```rust
pub struct MappedRows<''stmt, F> {
    pub(in ::row) rows: Rows<''stmt>,
    pub(in ::row) map: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rows` | `Rows<''stmt>` |  |
| `map` | `F` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<T>> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `AndThenRows`

**Attributes:**

- `#[must_use = "iterators are lazy and do nothing unless consumed"]`

An iterator over the mapped resulting rows of a query, with an Error type
unifying with Error.

```rust
pub struct AndThenRows<''stmt, F> {
    pub(in ::row) rows: Rows<''stmt>,
    pub(in ::row) map: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rows` | `Rows<''stmt>` |  |
| `map` | `F` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<<Self as >::Item> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Row`

A single result row of a query.

```rust
pub struct Row<''stmt> {
    pub(crate) stmt: &''stmt super::Statement<''stmt>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `stmt` | `&''stmt super::Statement<''stmt>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get_unwrap<I: RowIndex, T: FromSql>(self: &Self, idx: I) -> T { /* ... */ }
  ```
  Get the value of a particular column of the result row.

- ```rust
  pub fn get<I: RowIndex, T: FromSql>(self: &Self, idx: I) -> Result<T> { /* ... */ }
  ```
  Get the value of a particular column of the result row.

- ```rust
  pub fn get_ref<I: RowIndex>(self: &Self, idx: I) -> Result<ValueRef<''_>> { /* ... */ }
  ```
  Get the value of a particular column of the result row as a `ValueRef`,

- ```rust
  pub fn get_ref_unwrap<I: RowIndex>(self: &Self, idx: I) -> ValueRef<''_> { /* ... */ }
  ```
  Get the value of a particular column of the result row as a `ValueRef`,

- ```rust
  pub fn get_raw_checked<I: RowIndex>(self: &Self, idx: I) -> Result<ValueRef<''_>> { /* ... */ }
  ```
  Renamed to [`get_ref`](Row::get_ref).

- ```rust
  pub fn get_raw<I: RowIndex>(self: &Self, idx: I) -> ValueRef<''_> { /* ... */ }
  ```
  Renamed to [`get_ref_unwrap`](Row::get_ref_unwrap).

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **Freeze**
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

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(row: &''a Row<''a>) -> Result<Self> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Statement<''stmt> { /* ... */ }
    ```

### Traits

#### Trait `RowIndex`

A trait implemented by types that can index into columns of a row.

It is only implemented for `usize` and `&str`.

```rust
pub trait RowIndex: sealed::Sealed {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `idx`: Returns the index of the appropriate column, or `None` if no such

##### Implementations

This trait is implemented for the following types:

- `usize`
- `&str`

### Macros

#### Macro `tuple_try_from_row`

```rust
pub(crate) macro_rules! tuple_try_from_row {
    /* macro_rules! tuple_try_from_row {
    ($($field:ident),*) => { ... };
} */
}
```

#### Macro `tuples_try_from_row`

```rust
pub(crate) macro_rules! tuples_try_from_row {
    /* macro_rules! tuples_try_from_row {
    () => { ... };
    ($first:ident $(, $remaining:ident)*) => { ... };
} */
}
```

## Module `statement`

```rust
pub(crate) mod statement { /* ... */ }
```

### Types

#### Struct `Statement`

A prepared statement.

```rust
pub struct Statement<''conn> {
    pub(in ::statement) conn: &''conn super::Connection,
    pub(crate) stmt: crate::raw_statement::RawStatement,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `conn` | `&''conn super::Connection` |  |
| `stmt` | `crate::raw_statement::RawStatement` |  |

##### Implementations

###### Methods

- ```rust
  pub fn column_names(self: &Self) -> Vec<&str> { /* ... */ }
  ```
  Get all the column names in the result set of the prepared statement.

- ```rust
  pub fn column_count(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of columns in the result set returned by the prepared

- ```rust
  pub(crate) fn column_name_unwrap(self: &Self, col: usize) -> &str { /* ... */ }
  ```
  Check that column name reference lifetime is limited:

- ```rust
  pub fn column_name(self: &Self, col: usize) -> Result<&str> { /* ... */ }
  ```
  Returns the name assigned to a particular column in the result set

- ```rust
  pub fn column_index(self: &Self, name: &str) -> Result<usize> { /* ... */ }
  ```
  Returns the column index in the result set for a given column name.

- ```rust
  pub fn execute<P: Params>(self: &mut Self, params: P) -> Result<usize> { /* ... */ }
  ```
  Execute the prepared statement.

- ```rust
  pub fn execute_named(self: &mut Self, params: &[(&str, &dyn ToSql)]) -> Result<usize> { /* ... */ }
  ```
  Execute the prepared statement with named parameter(s).

- ```rust
  pub fn insert<P: Params>(self: &mut Self, params: P) -> Result<i64> { /* ... */ }
  ```
  Execute an INSERT and return the ROWID.

- ```rust
  pub fn query<P: Params>(self: &mut Self, params: P) -> Result<Rows<''_>> { /* ... */ }
  ```
  Execute the prepared statement, returning a handle to the resulting

- ```rust
  pub fn query_named(self: &mut Self, params: &[(&str, &dyn ToSql)]) -> Result<Rows<''_>> { /* ... */ }
  ```
  Execute the prepared statement with named parameter(s), returning a

- ```rust
  pub fn query_map<T, P, F>(self: &mut Self, params: P, f: F) -> Result<MappedRows<''_, F>>
where
    P: Params,
    F: FnMut(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Executes the prepared statement and maps a function over the resulting

- ```rust
  pub fn query_map_named<T, F>(self: &mut Self, params: &[(&str, &dyn ToSql)], f: F) -> Result<MappedRows<''_, F>>
where
    F: FnMut(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Execute the prepared statement with named parameter(s), returning an

- ```rust
  pub fn query_and_then<T, E, P, F>(self: &mut Self, params: P, f: F) -> Result<AndThenRows<''_, F>>
where
    P: Params,
    E: convert::From<Error>,
    F: FnMut(&Row<''_>) -> Result<T, E> { /* ... */ }
  ```
  Executes the prepared statement and maps a function over the resulting

- ```rust
  pub fn query_and_then_named<T, E, F>(self: &mut Self, params: &[(&str, &dyn ToSql)], f: F) -> Result<AndThenRows<''_, F>>
where
    E: convert::From<Error>,
    F: FnMut(&Row<''_>) -> Result<T, E> { /* ... */ }
  ```
  Execute the prepared statement with named parameter(s), returning an

- ```rust
  pub fn exists<P: Params>(self: &mut Self, params: P) -> Result<bool> { /* ... */ }
  ```
  Return `true` if a query in the SQL statement it executes returns one

- ```rust
  pub fn query_row<T, P, F>(self: &mut Self, params: P, f: F) -> Result<T>
where
    P: Params,
    F: FnOnce(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Convenience method to execute a query that is expected to return a

- ```rust
  pub fn query_row_named<T, F>(self: &mut Self, params: &[(&str, &dyn ToSql)], f: F) -> Result<T>
where
    F: FnOnce(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Convenience method to execute a query with named parameter(s) that is

- ```rust
  pub fn finalize(self: Self) -> Result<()> { /* ... */ }
  ```
  Consumes the statement.

- ```rust
  pub fn parameter_index(self: &Self, name: &str) -> Result<Option<usize>> { /* ... */ }
  ```
  Return the (one-based) index of an SQL parameter given its name.

- ```rust
  pub fn parameter_name(self: &Self, index: usize) -> Option<&str> { /* ... */ }
  ```
  Return the SQL parameter name given its (one-based) index (the inverse

- ```rust
  pub(crate) fn bind_parameters<P>(self: &mut Self, params: P) -> Result<()>
where
    P: IntoIterator,
    <P as >::Item: ToSql { /* ... */ }
  ```

- ```rust
  pub(crate) fn bind_parameters_named<T: ?Sized + ToSql>(self: &mut Self, params: &[(&str, &T)]) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn parameter_count(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of parameters that can be bound to this statement.

- ```rust
  pub fn raw_bind_parameter<T: ToSql>(self: &mut Self, one_based_col_index: usize, param: T) -> Result<()> { /* ... */ }
  ```
  Low level API to directly bind a parameter to a given index.

- ```rust
  pub fn raw_execute(self: &mut Self) -> Result<usize> { /* ... */ }
  ```
  Low level API to execute a statement given that all parameters were

- ```rust
  pub fn raw_query(self: &mut Self) -> Rows<''_> { /* ... */ }
  ```
  Low level API to get `Rows` for this query given that all parameters

- ```rust
  pub(in ::statement) fn bind_parameter<P: ?Sized + ToSql>(self: &Self, param: &P, col: usize) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub(in ::statement) fn execute_with_bound_parameters(self: &mut Self) -> Result<usize> { /* ... */ }
  ```

- ```rust
  pub(in ::statement) fn finalize_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub(in ::statement) fn check_update(self: &Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn get_status(self: &Self, status: StatementStatus) -> i32 { /* ... */ }
  ```
  Get the value for one of the status counters for this statement.

- ```rust
  pub fn reset_status(self: &Self, status: StatementStatus) -> i32 { /* ... */ }
  ```
  Reset the value of one of the status counters for this statement,

- ```rust
  pub(crate) fn check_no_tail(self: &Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn into_raw(self: Self) -> RawStatement { /* ... */ }
  ```
  Safety: This is unsafe, because using `sqlite3_stmt` after the

- ```rust
  pub(crate) fn new(conn: &Connection, stmt: RawStatement) -> Statement<''_> { /* ... */ }
  ```

- ```rust
  pub(crate) fn value_ref(self: &Self, col: usize) -> ValueRef<''_> { /* ... */ }
  ```

- ```rust
  pub(crate) fn step(self: &Self) -> Result<bool> { /* ... */ }
  ```

- ```rust
  pub(crate) fn reset(self: &Self) -> c_int { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **RefUnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &Statement<''stmt> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `StatementStatus`

**Attributes:**

- `#[repr(i32)]`
- `#[non_exhaustive]`

Prepared statement status counters.

See `https://www.sqlite.org/c3ref/c_stmtstatus_counter.html`
for explanations of each.

Note that depending on your version of SQLite, all of these
may not be available.

```rust
pub enum StatementStatus {
    FullscanStep = 1,
    Sort = 2,
    AutoIndex = 3,
    VmStep = 4,
    RePrepare = 5,
    Run = 6,
    MemUsed = 99,
}
```

##### Variants

###### `FullscanStep`

Equivalent to SQLITE_STMTSTATUS_FULLSCAN_STEP

Discriminant: `1`

Discriminant value: `1`

###### `Sort`

Equivalent to SQLITE_STMTSTATUS_SORT

Discriminant: `2`

Discriminant value: `2`

###### `AutoIndex`

Equivalent to SQLITE_STMTSTATUS_AUTOINDEX

Discriminant: `3`

Discriminant value: `3`

###### `VmStep`

Equivalent to SQLITE_STMTSTATUS_VM_STEP

Discriminant: `4`

Discriminant value: `4`

###### `RePrepare`

Equivalent to SQLITE_STMTSTATUS_REPREPARE

Discriminant: `5`

Discriminant value: `5`

###### `Run`

Equivalent to SQLITE_STMTSTATUS_RUN

Discriminant: `6`

Discriminant value: `6`

###### `MemUsed`

Equivalent to SQLITE_STMTSTATUS_MEMUSED

Discriminant: `99`

Discriminant value: `99`

##### Implementations

###### Trait Implementations

- **Freeze**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Unpin**
- **Eq**
- **Copy**
- **Sync**
- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StatementStatus) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StatementStatus { /* ... */ }
    ```

## Module `transaction`

```rust
pub(crate) mod transaction { /* ... */ }
```

### Types

#### Enum `TransactionBehavior`

**Attributes:**

- `#[non_exhaustive]`

Options for transaction behavior. See [BEGIN
TRANSACTION](http://www.sqlite.org/lang_transaction.html) for details.

```rust
pub enum TransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
}
```

##### Variants

###### `Deferred`

DEFERRED means that the transaction does not actually start until the
database is first accessed.

###### `Immediate`

IMMEDIATE cause the database connection to start a new write
immediately, without waiting for a writes statement.

###### `Exclusive`

EXCLUSIVE prevents other database connections from reading the database
while the transaction is underway.

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TransactionBehavior { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
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
- **Freeze**
- **Copy**
#### Enum `DropBehavior`

**Attributes:**

- `#[non_exhaustive]`

Options for how a Transaction or Savepoint should behave when it is dropped.

```rust
pub enum DropBehavior {
    Rollback,
    Commit,
    Ignore,
    Panic,
}
```

##### Variants

###### `Rollback`

Roll back the changes. This is the default.

###### `Commit`

Commit the changes.

###### `Ignore`

Do not commit or roll back changes - this will leave the transaction or
savepoint open, so should be used with care.

###### `Panic`

Panic. Used to enforce intentional behavior during development.

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DropBehavior) -> bool { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DropBehavior { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Copy**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `Transaction`

Represents a transaction on a database connection.

## Note

Transactions will roll back by default. Use `commit` method to explicitly
commit the transaction, or use `set_drop_behavior` to change what happens
when the transaction is dropped.

## Example

```rust,no_run
# use rusqlite::{Connection, Result};
# fn do_queries_part_1(_conn: &Connection) -> Result<()> { Ok(()) }
# fn do_queries_part_2(_conn: &Connection) -> Result<()> { Ok(()) }
fn perform_queries(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    do_queries_part_1(&tx)?; // tx causes rollback if this fails
    do_queries_part_2(&tx)?; // tx causes rollback if this fails

    tx.commit()
}
```

```rust
pub struct Transaction<''conn> {
    pub(in ::transaction) conn: &''conn crate::Connection,
    pub(in ::transaction) drop_behavior: DropBehavior,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `conn` | `&''conn crate::Connection` |  |
| `drop_behavior` | `DropBehavior` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(conn: &mut Connection, behavior: TransactionBehavior) -> Result<Transaction<''_>> { /* ... */ }
  ```
  Begin a new transaction. Cannot be nested; see `savepoint` for nested

- ```rust
  pub fn new_unchecked(conn: &Connection, behavior: TransactionBehavior) -> Result<Transaction<''_>> { /* ... */ }
  ```
  Begin a new transaction, failing if a transaction is open.

- ```rust
  pub fn savepoint(self: &mut Self) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Starts a new [savepoint](http://www.sqlite.org/lang_savepoint.html), allowing nested

- ```rust
  pub fn savepoint_with_name<T: Into<String>>(self: &mut Self, name: T) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Create a new savepoint with a custom savepoint name. See `savepoint()`.

- ```rust
  pub fn drop_behavior(self: &Self) -> DropBehavior { /* ... */ }
  ```
  Get the current setting for what happens to the transaction when it is

- ```rust
  pub fn set_drop_behavior(self: &mut Self, drop_behavior: DropBehavior) { /* ... */ }
  ```
  Configure the transaction to perform the specified action when it is

- ```rust
  pub fn commit(self: Self) -> Result<()> { /* ... */ }
  ```
  A convenience method which consumes and commits a transaction.

- ```rust
  pub(in ::transaction) fn commit_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn rollback(self: Self) -> Result<()> { /* ... */ }
  ```
  A convenience method which consumes and rolls back a transaction.

- ```rust
  pub(in ::transaction) fn rollback_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn finish(self: Self) -> Result<()> { /* ... */ }
  ```
  Consumes the transaction, committing or rolling back according to the

- ```rust
  pub(in ::transaction) fn finish_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Connection { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

#### Struct `Savepoint`

Represents a savepoint on a database connection.

## Note

Savepoints will roll back by default. Use `commit` method to explicitly
commit the savepoint, or use `set_drop_behavior` to change what happens
when the savepoint is dropped.

## Example

```rust,no_run
# use rusqlite::{Connection, Result};
# fn do_queries_part_1(_conn: &Connection) -> Result<()> { Ok(()) }
# fn do_queries_part_2(_conn: &Connection) -> Result<()> { Ok(()) }
fn perform_queries(conn: &mut Connection) -> Result<()> {
    let sp = conn.savepoint()?;

    do_queries_part_1(&sp)?; // sp causes rollback if this fails
    do_queries_part_2(&sp)?; // sp causes rollback if this fails

    sp.commit()
}
```

```rust
pub struct Savepoint<''conn> {
    pub(in ::transaction) conn: &''conn crate::Connection,
    pub(in ::transaction) name: String,
    pub(in ::transaction) depth: u32,
    pub(in ::transaction) drop_behavior: DropBehavior,
    pub(in ::transaction) committed: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `conn` | `&''conn crate::Connection` |  |
| `name` | `String` |  |
| `depth` | `u32` |  |
| `drop_behavior` | `DropBehavior` |  |
| `committed` | `bool` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::transaction) fn with_depth_and_name<T: Into<String>>(conn: &Connection, depth: u32, name: T) -> Result<Savepoint<''_>> { /* ... */ }
  ```

- ```rust
  pub(in ::transaction) fn with_depth(conn: &Connection, depth: u32) -> Result<Savepoint<''_>> { /* ... */ }
  ```

- ```rust
  pub fn new(conn: &mut Connection) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a new savepoint. Can be nested.

- ```rust
  pub fn with_name<T: Into<String>>(conn: &mut Connection, name: T) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a new savepoint with a user-provided savepoint name.

- ```rust
  pub fn savepoint(self: &mut Self) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a nested savepoint.

- ```rust
  pub fn savepoint_with_name<T: Into<String>>(self: &mut Self, name: T) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a nested savepoint with a user-provided savepoint name.

- ```rust
  pub fn drop_behavior(self: &Self) -> DropBehavior { /* ... */ }
  ```
  Get the current setting for what happens to the savepoint when it is

- ```rust
  pub fn set_drop_behavior(self: &mut Self, drop_behavior: DropBehavior) { /* ... */ }
  ```
  Configure the savepoint to perform the specified action when it is

- ```rust
  pub fn commit(self: Self) -> Result<()> { /* ... */ }
  ```
  A convenience method which consumes and commits a savepoint.

- ```rust
  pub(in ::transaction) fn commit_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn rollback(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  A convenience method which rolls back a savepoint.

- ```rust
  pub fn finish(self: Self) -> Result<()> { /* ... */ }
  ```
  Consumes the savepoint, committing or rolling back according to the

- ```rust
  pub(in ::transaction) fn finish_(self: &mut Self) -> Result<()> { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &Connection { /* ... */ }
    ```

- **Receiver**
- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `types`

Traits dealing with SQLite data types.

SQLite uses a [dynamic type system](https://www.sqlite.org/datatype3.html). Implementations of
the [`ToSql`] and [`FromSql`] traits are provided for the basic types that
SQLite provides methods for:

* Strings (`String` and `&str`)
* Blobs (`Vec<u8>` and `&[u8]`)
* Numbers

The number situation is a little complicated due to the fact that all
numbers in SQLite are stored as `INTEGER` (`i64`) or `REAL` (`f64`).

[`ToSql`] and [`FromSql`] are implemented for all primitive number types.
[`FromSql`] has different behaviour depending on the SQL and Rust types, and
the value.

* `INTEGER` to integer: returns an
  [`Error::IntegralValueOutOfRange`](crate::Error::IntegralValueOutOfRange)
  error if the value does not fit in the Rust type.
* `REAL` to integer: always returns an
  [`Error::InvalidColumnType`](crate::Error::InvalidColumnType) error.
* `INTEGER` to float: casts using `as` operator. Never fails.
* `REAL` to float: casts using `as` operator. Never fails.

[`ToSql`] always succeeds except when storing a `u64` or `usize` value that
cannot fit in an `INTEGER` (`i64`). Also note that SQLite ignores column
types, so if you store an `i64` in a column with type `REAL` it will be
stored as an `INTEGER`, not a `REAL`.

If the `time` feature is enabled, implementations are
provided for `time::OffsetDateTime` that use the RFC 3339 date/time format,
`"%Y-%m-%dT%H:%M:%S.%fZ"`, to store time values as strings.  These values
can be parsed by SQLite's builtin
[datetime](https://www.sqlite.org/lang_datefunc.html) functions.  If you
want different storage for datetimes, you can use a newtype.
[`ToSql`] and [`FromSql`] are also implemented for `Option<T>` where `T`
implements [`ToSql`] or [`FromSql`] for the cases where you want to know if
a value was NULL (which gets translated to `None`).

```rust
pub mod types { /* ... */ }
```

### Modules

## Module `from_sql`

```rust
pub(in ::types) mod from_sql { /* ... */ }
```

### Types

#### Enum `FromSqlError`

**Attributes:**

- `#[non_exhaustive]`

Enum listing possible errors from [`FromSql`] trait.

```rust
pub enum FromSqlError {
    InvalidType,
    OutOfRange(i64),
    Other(Box<dyn Error + Send + Sync + ''static>),
}
```

##### Variants

###### `InvalidType`

Error when an SQLite value is requested, but the type of the result
cannot be converted to the requested Rust type.

###### `OutOfRange`

Error when the i64 value returned by SQLite cannot be stored into the
requested type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Other`

An error case available for implementors of the [`FromSql`] trait.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<dyn Error + Send + Sync + ''static>` |  |

##### Implementations

###### Trait Implementations

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

  - ```rust
    fn from(err: FromSqlError) -> Error { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FromSqlError) -> bool { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn Error + ''static> { /* ... */ }
    ```

- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

#### Type Alias `FromSqlResult`

Result type for implementors of the [`FromSql`] trait.

```rust
pub type FromSqlResult<T> = Result<T, FromSqlError>;
```

### Traits

#### Trait `FromSql`

A trait for types that can be created from a SQLite value.

```rust
pub trait FromSql: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `column_result`: Converts SQLite value into Rust value.

##### Implementations

This trait is implemented for the following types:

- `i8`
- `i16`
- `i32`
- `isize`
- `u8`
- `u16`
- `u32`
- `u64`
- `usize`
- `i64`
- `f32`
- `f64`
- `bool`
- `String`
- `Box<str>`
- `std::rc::Rc<str>`
- `std::sync::Arc<str>`
- `Vec<u8>`
- `Option<T>` with <T: FromSql>
- `super::Value`

### Macros

#### Macro `from_sql_integral`

```rust
pub(crate) macro_rules! from_sql_integral {
    /* macro_rules! from_sql_integral {
    ($t:ident) => { ... };
} */
}
```

## Module `to_sql`

```rust
pub(in ::types) mod to_sql { /* ... */ }
```

### Types

#### Enum `ToSqlOutput`

**Attributes:**

- `#[non_exhaustive]`

`ToSqlOutput` represents the possible output types for implementers of the
[`ToSql`] trait.

```rust
pub enum ToSqlOutput<''a> {
    Borrowed(super::ValueRef<''a>),
    Owned(super::Value),
}
```

##### Variants

###### `Borrowed`

A borrowed SQLite-representable value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `super::ValueRef<''a>` |  |

###### `Owned`

An owned SQLite-representable value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `super::Value` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ToSqlOutput<''a>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToSql**
  - ```rust
    fn to_sql(self: &Self) -> Result<ToSqlOutput<''_>> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ToSqlOutput<''a> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: &''a T) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: Null) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: bool) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: i8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: isize) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: u8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: Vec<u8>) -> Self { /* ... */ }
    ```

### Traits

#### Trait `ToSql`

A trait for types that can be converted into SQLite values. Returns
[`Error::ToSqlConversionFailure`] if the conversion fails.

```rust
pub trait ToSql {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_sql`: Converts Rust value to SQLite value

##### Implementations

This trait is implemented for the following types:

- `ToSqlOutput<''_>`
- `std::borrow::Cow<''_, T>` with <T: ToSql + ToOwned + ?Sized>
- `Box<T>` with <T: ToSql + ?Sized>
- `std::rc::Rc<T>` with <T: ToSql + ?Sized>
- `std::sync::Arc<T>` with <T: ToSql + ?Sized>
- `super::Null`
- `bool`
- `i8`
- `i16`
- `i32`
- `i64`
- `isize`
- `u8`
- `u16`
- `u32`
- `f32`
- `f64`
- `u64`
- `usize`
- `&T` with <T>
- `String`
- `str`
- `Vec<u8>`
- `[u8]`
- `super::Value`
- `Option<T>` with <T: ToSql>

### Macros

#### Macro `from_value`

```rust
pub(crate) macro_rules! from_value {
    /* macro_rules! from_value {
    ($t:ty) => { ... };
} */
}
```

#### Macro `to_sql_self`

```rust
pub(crate) macro_rules! to_sql_self {
    /* macro_rules! to_sql_self {
    ($t:ty) => { ... };
} */
}
```

#### Macro `to_sql_self_fallible`

```rust
pub(crate) macro_rules! to_sql_self_fallible {
    /* macro_rules! to_sql_self_fallible {
    ($t:ty) => { ... };
} */
}
```

## Module `value`

```rust
pub(in ::types) mod value { /* ... */ }
```

### Types

#### Enum `Value`

Owning [dynamic type value](http://sqlite.org/datatype3.html). Value's type is typically
dictated by SQLite (not by the caller).

See [`ValueRef`](crate::types::ValueRef) for a non-owning dynamic type
value.

```rust
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}
```

##### Variants

###### `Null`

The value is a `NULL` value.

###### `Integer`

The value is a signed integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Real`

The value is a floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

###### `Text`

The value is a text string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Blob`

The value is a blob of data

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<u8>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn data_type(self: &Self) -> Type { /* ... */ }
  ```
  Returns SQLite fundamental datatype.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ToSql**
  - ```rust
    fn to_sql(self: &Self) -> Result<ToSqlOutput<''_>> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **FromSql**
  - ```rust
    fn column_result(value: ValueRef<''_>) -> FromSqlResult<Self> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Value) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Value { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_: Null) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: bool) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: isize) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: i8) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: i16) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: i32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: u8) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: u16) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: u32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(i: i64) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(f: f32) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(f: f64) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(s: String) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(v: Vec<u8>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(v: Option<T>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(borrowed: ValueRef<''_>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a Value) -> ValueRef<''a> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Macros

#### Macro `from_i64`

```rust
pub(crate) macro_rules! from_i64 {
    /* macro_rules! from_i64 {
    ($t:ty) => { ... };
} */
}
```

## Module `value_ref`

```rust
pub(in ::types) mod value_ref { /* ... */ }
```

### Types

#### Enum `ValueRef`

A non-owning [dynamic type value](http://sqlite.org/datatype3.html). Typically the
memory backing this value is owned by SQLite.

See [`Value`](Value) for an owning dynamic type value.

```rust
pub enum ValueRef<''a> {
    Null,
    Integer(i64),
    Real(f64),
    Text(&''a [u8]),
    Blob(&''a [u8]),
}
```

##### Variants

###### `Null`

The value is a `NULL` value.

###### `Integer`

The value is a signed integer.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i64` |  |

###### `Real`

The value is a floating point number.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `f64` |  |

###### `Text`

The value is a text string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a [u8]` |  |

###### `Blob`

The value is a blob of data

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a [u8]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn data_type(self: &Self) -> Type { /* ... */ }
  ```
  Returns SQLite fundamental datatype.

- ```rust
  pub fn as_i64(self: &Self) -> FromSqlResult<i64> { /* ... */ }
  ```
  If `self` is case `Integer`, returns the integral value. Otherwise,

- ```rust
  pub fn as_i64_or_null(self: &Self) -> FromSqlResult<Option<i64>> { /* ... */ }
  ```
  If `self` is case `Null` returns None.

- ```rust
  pub fn as_f64(self: &Self) -> FromSqlResult<f64> { /* ... */ }
  ```
  If `self` is case `Real`, returns the floating point value. Otherwise,

- ```rust
  pub fn as_f64_or_null(self: &Self) -> FromSqlResult<Option<f64>> { /* ... */ }
  ```
  If `self` is case `Null` returns None.

- ```rust
  pub fn as_str(self: &Self) -> FromSqlResult<&''a str> { /* ... */ }
  ```
  If `self` is case `Text`, returns the string value. Otherwise, returns

- ```rust
  pub fn as_str_or_null(self: &Self) -> FromSqlResult<Option<&''a str>> { /* ... */ }
  ```
  If `self` is case `Null` returns None.

- ```rust
  pub fn as_blob(self: &Self) -> FromSqlResult<&''a [u8]> { /* ... */ }
  ```
  If `self` is case `Blob`, returns the byte slice. Otherwise, returns

- ```rust
  pub fn as_blob_or_null(self: &Self) -> FromSqlResult<Option<&''a [u8]>> { /* ... */ }
  ```
  If `self` is case `Null` returns None.

- ```rust
  pub fn as_bytes(self: &Self) -> FromSqlResult<&''a [u8]> { /* ... */ }
  ```
  Returns the byte slice that makes up this ValueRef if it's either

- ```rust
  pub fn as_bytes_or_null(self: &Self) -> FromSqlResult<Option<&''a [u8]>> { /* ... */ }
  ```
  If `self` is case `Null` returns None.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ValueRef<''a> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(borrowed: ValueRef<''_>) -> Value { /* ... */ }
    ```

  - ```rust
    fn from(s: &str) -> ValueRef<''_> { /* ... */ }
    ```

  - ```rust
    fn from(s: &[u8]) -> ValueRef<''_> { /* ... */ }
    ```

  - ```rust
    fn from(value: &''a Value) -> ValueRef<''a> { /* ... */ }
    ```

  - ```rust
    fn from(s: Option<T>) -> ValueRef<''a> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ValueRef<''a>) -> bool { /* ... */ }
    ```

- **Unpin**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Types

#### Struct `Null`

Empty struct that can be used to fill in a query parameter as `NULL`.

## Example

```rust,no_run
# use rusqlite::{Connection, Result};
# use rusqlite::types::{Null};

fn insert_null(conn: &Connection) -> Result<usize> {
    conn.execute("INSERT INTO people (name) VALUES (?)", [Null])
}
```

```rust
pub struct Null;
```

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Null { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToSql**
  - ```rust
    fn to_sql(self: &Self) -> Result<ToSqlOutput<''_>> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(t: Null) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(_: Null) -> Value { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **UnwindSafe**
- **Sync**
- **Send**
#### Enum `Type`

SQLite data types.
See [Fundamental Datatypes](https://sqlite.org/c3ref/c_blob.html).

```rust
pub enum Type {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}
```

##### Variants

###### `Null`

NULL

###### `Integer`

64-bit signed integer

###### `Real`

64-bit IEEE floating point number

###### `Text`

String

###### `Blob`

BLOB

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Type { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Type) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Re-exports

#### Re-export `FromSql`

```rust
pub use self::from_sql::FromSql;
```

#### Re-export `FromSqlError`

```rust
pub use self::from_sql::FromSqlError;
```

#### Re-export `FromSqlResult`

```rust
pub use self::from_sql::FromSqlResult;
```

#### Re-export `ToSql`

```rust
pub use self::to_sql::ToSql;
```

#### Re-export `ToSqlOutput`

```rust
pub use self::to_sql::ToSqlOutput;
```

#### Re-export `Value`

```rust
pub use self::value::Value;
```

#### Re-export `ValueRef`

```rust
pub use self::value_ref::ValueRef;
```

## Module `unlock_notify`

[Unlock Notification](http://sqlite.org/unlock_notify.html)

```rust
pub(crate) mod unlock_notify { /* ... */ }
```

### Functions

#### Function `is_locked`

**Attributes:**

- `#[cfg(not(feature = "unlock_notify"))]`

```rust
pub unsafe fn is_locked(_db: *mut ffi::sqlite3, _rc: std::os::raw::c_int) -> bool { /* ... */ }
```

#### Function `wait_for_unlock_notify`

**Attributes:**

- `#[cfg(not(feature = "unlock_notify"))]`

```rust
pub unsafe fn wait_for_unlock_notify(_db: *mut ffi::sqlite3) -> std::os::raw::c_int { /* ... */ }
```

## Module `version`

```rust
pub(crate) mod version { /* ... */ }
```

### Functions

#### Function `version_number`

**Attributes:**

- `#[inline]`

Returns the SQLite version as an integer; e.g., `3016002` for version
3.16.2.

See [`sqlite3_libversion_number()`](https://www.sqlite.org/c3ref/libversion.html).

```rust
pub fn version_number() -> i32 { /* ... */ }
```

#### Function `version`

**Attributes:**

- `#[inline]`

Returns the SQLite version as a string; e.g., `"3.16.2"` for version 3.16.2.

See [`sqlite3_libversion()`](https://www.sqlite.org/c3ref/libversion.html).

```rust
pub fn version() -> &''static str { /* ... */ }
```

## Module `util`

```rust
pub(crate) mod util { /* ... */ }
```

### Modules

## Module `param_cache`

```rust
pub(crate) mod param_cache { /* ... */ }
```

### Types

#### Struct `ParamIndexCache`

Maps parameter names to parameter indices.

```rust
pub(crate) struct ParamIndexCache(pub(in ::util::param_cache) std::cell::RefCell<std::collections::BTreeMap<small_cstr::SmallCString, usize>>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::cell::RefCell<std::collections::BTreeMap<small_cstr::SmallCString, usize>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn get_or_insert_with<F>(self: &Self, s: &str, func: F) -> Option<usize>
where
    F: FnOnce(&std::ffi::CStr) -> Option<usize> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ParamIndexCache { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParamIndexCache { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
## Module `small_cstr`

```rust
pub(in ::util) mod small_cstr { /* ... */ }
```

### Types

#### Struct `SmallCString`

Similar to std::ffi::CString, but avoids heap allocating if the string is
small enough. Also guarantees it's input is UTF-8 -- used for cases where we
need to pass a NUL-terminated string to SQLite, and we have a `&str`.

```rust
pub(crate) struct SmallCString(pub(in ::util::small_cstr) smallvec::SmallVec<[u8; 16]>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `smallvec::SmallVec<[u8; 16]>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(s: &str) -> Result<Self, NulError> { /* ... */ }
  ```

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn as_bytes_without_nul(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the bytes not including the NUL terminator. E.g. the bytes which

- ```rust
  pub fn as_bytes_with_nul(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the bytes behind this str *including* the NUL terminator. This

- ```rust
  pub(in ::util::small_cstr) fn debug_checks(self: &Self) { /* ... */ }
  ```

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn as_cstr(self: &Self) -> &CStr { /* ... */ }
  ```

- ```rust
  pub(in ::util::small_cstr) fn fabricate_nul_error(b: &str) -> NulError { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SmallCString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, s: &SmallCString) -> bool { /* ... */ }
    ```

  - ```rust
    fn eq(self: &Self, s: &str) -> bool { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &CStr { /* ... */ }
    ```

- **Receiver**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SmallCString { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SmallCString) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SmallCString) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

  - ```rust
    fn borrow(self: &Self) -> &str { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
## Types

### Type Alias `Result`

A typedef of the result returned by many methods.

```rust
pub type Result<T, E = Error> = result::Result<T, E>;
```

### Enum `DatabaseName`

Name for a database within a SQLite connection.

```rust
pub enum DatabaseName<''a> {
    Main,
    Temp,
    Attached(&''a str),
}
```

#### Variants

##### `Main`

The main database.

##### `Temp`

The temporary database (e.g., any "CREATE TEMPORARY TABLE" tables).

##### `Attached`

A database that has been attached via "ATTACH DATABASE ...".

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a str` |  |

#### Implementations

##### Trait Implementations

- **Copy**
- **UnwindSafe**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DatabaseName<''a> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Struct `Connection`

A connection to a SQLite database.

```rust
pub struct Connection {
    pub(crate) db: std::cell::RefCell<crate::inner_connection::InnerConnection>,
    pub(crate) cache: crate::cache::StatementCache,
    pub(crate) path: Option<std::path::PathBuf>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `db` | `std::cell::RefCell<crate::inner_connection::InnerConnection>` |  |
| `cache` | `crate::cache::StatementCache` |  |
| `path` | `Option<std::path::PathBuf>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn busy_timeout(self: &Self, timeout: Duration) -> Result<()> { /* ... */ }
  ```
  Set a busy handler that sleeps for a specified amount of time when a

- ```rust
  pub fn busy_handler(self: &Self, callback: Option<fn(i32) -> bool>) -> Result<()> { /* ... */ }
  ```
  Register a callback to handle `SQLITE_BUSY` errors.

- ```rust
  pub fn prepare_cached(self: &Self, sql: &str) -> Result<CachedStatement<''_>> { /* ... */ }
  ```
  Prepare a SQL statement for execution, returning a previously prepared

- ```rust
  pub fn set_prepared_statement_cache_capacity(self: &Self, capacity: usize) { /* ... */ }
  ```
  Set the maximum number of cached prepared statements this connection

- ```rust
  pub fn flush_prepared_statement_cache(self: &Self) { /* ... */ }
  ```
  Remove/finalize all prepared statements currently in the cache.

- ```rust
  pub fn db_config(self: &Self, config: DbConfig) -> Result<bool> { /* ... */ }
  ```
  Returns the current value of a `config`.

- ```rust
  pub fn set_db_config(self: &Self, config: DbConfig, new_val: bool) -> Result<bool> { /* ... */ }
  ```
  Make configuration changes to a database connection

- ```rust
  pub fn pragma_query_value<T, F>(self: &Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str, f: F) -> Result<T>
where
    F: FnOnce(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Query the current value of `pragma_name`.

- ```rust
  pub fn pragma_query<F>(self: &Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str, f: F) -> Result<()>
where
    F: FnMut(&Row<''_>) -> Result<()> { /* ... */ }
  ```
  Query the current rows/values of `pragma_name`.

- ```rust
  pub fn pragma<F, V>(self: &Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str, pragma_value: V, f: F) -> Result<()>
where
    F: FnMut(&Row<''_>) -> Result<()>,
    V: ToSql { /* ... */ }
  ```
  Query the current value(s) of `pragma_name` associated to

- ```rust
  pub fn pragma_update<V>(self: &Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str, pragma_value: V) -> Result<()>
where
    V: ToSql { /* ... */ }
  ```
  Set a new value to `pragma_name`.

- ```rust
  pub fn pragma_update_and_check<F, T, V>(self: &Self, schema_name: Option<DatabaseName<''_>>, pragma_name: &str, pragma_value: V, f: F) -> Result<T>
where
    F: FnOnce(&Row<''_>) -> Result<T>,
    V: ToSql { /* ... */ }
  ```
  Set a new value to `pragma_name` and return the updated value.

- ```rust
  pub fn transaction(self: &mut Self) -> Result<Transaction<''_>> { /* ... */ }
  ```
  Begin a new transaction with the default behavior (DEFERRED).

- ```rust
  pub fn transaction_with_behavior(self: &mut Self, behavior: TransactionBehavior) -> Result<Transaction<''_>> { /* ... */ }
  ```
  Begin a new transaction with a specified behavior.

- ```rust
  pub fn unchecked_transaction(self: &Self) -> Result<Transaction<''_>> { /* ... */ }
  ```
  Begin a new transaction with the default behavior (DEFERRED).

- ```rust
  pub fn savepoint(self: &mut Self) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a new savepoint with the default behavior (DEFERRED).

- ```rust
  pub fn savepoint_with_name<T: Into<String>>(self: &mut Self, name: T) -> Result<Savepoint<''_>> { /* ... */ }
  ```
  Begin a new savepoint with a specified name.

- ```rust
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to a SQLite database.

- ```rust
  pub fn open_in_memory() -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to an in-memory SQLite database.

- ```rust
  pub fn open_with_flags<P: AsRef<Path>>(path: P, flags: OpenFlags) -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to a SQLite database.

- ```rust
  pub fn open_with_flags_and_vfs<P: AsRef<Path>>(path: P, flags: OpenFlags, vfs: &str) -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to a SQLite database using the specific flags and

- ```rust
  pub fn open_in_memory_with_flags(flags: OpenFlags) -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to an in-memory SQLite database.

- ```rust
  pub fn open_in_memory_with_flags_and_vfs(flags: OpenFlags, vfs: &str) -> Result<Connection> { /* ... */ }
  ```
  Open a new connection to an in-memory SQLite database using the specific

- ```rust
  pub fn execute_batch(self: &Self, sql: &str) -> Result<()> { /* ... */ }
  ```
  Convenience method to run multiple SQL statements (that cannot take any

- ```rust
  pub fn execute<P: Params>(self: &Self, sql: &str, params: P) -> Result<usize> { /* ... */ }
  ```
  Convenience method to prepare and execute a single SQL statement.

- ```rust
  pub fn path(self: &Self) -> Option<&Path> { /* ... */ }
  ```
  Returns the path to the database file, if one exists and is known.

- ```rust
  pub fn execute_named(self: &Self, sql: &str, params: &[(&str, &dyn ToSql)]) -> Result<usize> { /* ... */ }
  ```
  Convenience method to prepare and execute a single SQL statement with

- ```rust
  pub fn last_insert_rowid(self: &Self) -> i64 { /* ... */ }
  ```
  Get the SQLite rowid of the most recent successful INSERT.

- ```rust
  pub fn query_row<T, P, F>(self: &Self, sql: &str, params: P, f: F) -> Result<T>
where
    P: Params,
    F: FnOnce(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Convenience method to execute a query that is expected to return a

- ```rust
  pub fn query_row_named<T, F>(self: &Self, sql: &str, params: &[(&str, &dyn ToSql)], f: F) -> Result<T>
where
    F: FnOnce(&Row<''_>) -> Result<T> { /* ... */ }
  ```
  Convenience method to execute a query with named parameter(s) that is

- ```rust
  pub fn query_row_and_then<T, E, P, F>(self: &Self, sql: &str, params: P, f: F) -> Result<T, E>
where
    P: Params,
    F: FnOnce(&Row<''_>) -> Result<T, E>,
    E: convert::From<Error> { /* ... */ }
  ```
  Convenience method to execute a query that is expected to return a

- ```rust
  pub fn prepare(self: &Self, sql: &str) -> Result<Statement<''_>> { /* ... */ }
  ```
  Prepare a SQL statement for execution.

- ```rust
  pub fn close(self: Self) -> Result<(), (Connection, Error)> { /* ... */ }
  ```
  Close the SQLite connection.

- ```rust
  pub unsafe fn handle(self: &Self) -> *mut ffi::sqlite3 { /* ... */ }
  ```
  Get access to the underlying SQLite database connection handle.

- ```rust
  pub unsafe fn from_handle(db: *mut ffi::sqlite3) -> Result<Connection> { /* ... */ }
  ```
  Create a `Connection` from a raw handle.

- ```rust
  pub fn get_interrupt_handle(self: &Self) -> InterruptHandle { /* ... */ }
  ```
  Get access to a handle that can be used to interrupt long running

- ```rust
  pub(crate) fn decode_result(self: &Self, code: c_int) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub(crate) fn changes(self: &Self) -> usize { /* ... */ }
  ```
  Return the number of rows modified, inserted or deleted by the most

- ```rust
  pub fn is_autocommit(self: &Self) -> bool { /* ... */ }
  ```
  Test for auto-commit mode.

##### Trait Implementations

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
### Struct `Batch`

Batch iterator
```rust
use rusqlite::{Batch, Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let sql = r"
    CREATE TABLE tbl1 (col);
    CREATE TABLE tbl2 (col);
    ";
    let mut batch = Batch::new(&conn, sql);
    while let Some(mut stmt) = batch.next()? {
        stmt.execute([])?;
    }
    Ok(())
}
```

```rust
pub struct Batch<''conn, ''sql> {
    pub(crate) conn: &''conn Connection,
    pub(crate) sql: &''sql str,
    pub(crate) tail: usize,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `conn` | `&''conn Connection` |  |
| `sql` | `&''sql str` |  |
| `tail` | `usize` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new(conn: &''conn Connection, sql: &''sql str) -> Batch<''conn, ''sql> { /* ... */ }
  ```
  Constructor

- ```rust
  pub fn next(self: &mut Self) -> Result<Option<Statement<''conn>>> { /* ... */ }
  ```
  Iterates on each batch statements.

##### Trait Implementations

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Result<Statement<''conn>>> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
- **Unpin**
### Struct `OpenFlags`

**Attributes:**

- `#[repr(C)]`

Flags for opening SQLite database connections.
See [sqlite3_open_v2](http://www.sqlite.org/c3ref/open.html) for details.

```rust
pub struct OpenFlags {
    pub(crate) bits: ::std::os::raw::c_int,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bits` | `::std::os::raw::c_int` |  |

#### Implementations

##### Methods

- ```rust
  pub const fn empty() -> Self { /* ... */ }
  ```
  Returns an empty set of flags.

- ```rust
  pub const fn all() -> Self { /* ... */ }
  ```
  Returns the set containing all flags.

- ```rust
  pub const fn bits(self: &Self) -> ::std::os::raw::c_int { /* ... */ }
  ```
  Returns the raw value of the flags currently stored.

- ```rust
  pub const fn from_bits(bits: ::std::os::raw::c_int) -> $crate::_core::option::Option<Self> { /* ... */ }
  ```
  Convert from underlying bit representation, unless that

- ```rust
  pub const fn from_bits_truncate(bits: ::std::os::raw::c_int) -> Self { /* ... */ }
  ```
  Convert from underlying bit representation, dropping any bits

- ```rust
  pub const unsafe fn from_bits_unchecked(bits: ::std::os::raw::c_int) -> Self { /* ... */ }
  ```
  Convert from underlying bit representation, preserving all

- ```rust
  pub const fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if no flags are currently stored.

- ```rust
  pub const fn is_all(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if all flags are currently set.

- ```rust
  pub const fn intersects(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Returns `true` if there are flags common to both `self` and `other`.

- ```rust
  pub const fn contains(self: &Self, other: Self) -> bool { /* ... */ }
  ```
  Returns `true` if all of the flags in `other` are contained within `self`.

- ```rust
  pub fn insert(self: &mut Self, other: Self) { /* ... */ }
  ```
  Inserts the specified flags in-place.

- ```rust
  pub fn remove(self: &mut Self, other: Self) { /* ... */ }
  ```
  Removes the specified flags in-place.

- ```rust
  pub fn toggle(self: &mut Self, other: Self) { /* ... */ }
  ```
  Toggles the specified flags in-place.

- ```rust
  pub fn set(self: &mut Self, other: Self, value: bool) { /* ... */ }
  ```
  Inserts or removes the specified flags depending on the passed value.

- ```rust
  pub const fn intersection(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the intersection between the flags in `self` and

- ```rust
  pub const fn union(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the union of between the flags in `self` and `other`.

- ```rust
  pub const fn difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the difference between the flags in `self` and `other`.

- ```rust
  pub const fn symmetric_difference(self: Self, other: Self) -> Self { /* ... */ }
  ```
  Returns the [symmetric difference][sym-diff] between the flags

- ```rust
  pub const fn complement(self: Self) -> Self { /* ... */ }
  ```
  Returns the complement of this set of flags.

##### Trait Implementations

- **Sub**
  - ```rust
    fn sub(self: Self, other: Self) -> Self { /* ... */ }
    ```
    Returns the set difference of the two sets of flags.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &OpenFlags) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::_core::fmt::Formatter<''_>) -> $crate::_core::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **CallHasher**
  - ```rust
    fn get_hash<H, B>(value: &H, build_hasher: &B) -> u64
where
    H: Hash + ?Sized,
    B: BuildHasher { /* ... */ }
    ```

- **StructuralPartialEq**
- **BitOr**
  - ```rust
    fn bitor(self: Self, other: OpenFlags) -> Self { /* ... */ }
    ```
    Returns the union of the two sets of flags.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OpenFlags { /* ... */ }
    ```

- **Sync**
- **LowerHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::_core::fmt::Formatter<''_>) -> $crate::_core::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OpenFlags) -> bool { /* ... */ }
    ```

- **Copy**
- **Eq**
- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Freeze**
- **Binary**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::_core::fmt::Formatter<''_>) -> $crate::_core::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitOrAssign**
  - ```rust
    fn bitor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    Adds the set of flags.

- **BitXorAssign**
  - ```rust
    fn bitxor_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    Toggles the set of flags.

- **Octal**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::_core::fmt::Formatter<''_>) -> $crate::_core::fmt::Result { /* ... */ }
    ```

- **UpperHex**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::_core::fmt::Formatter<''_>) -> $crate::_core::fmt::Result { /* ... */ }
    ```

- **SubAssign**
  - ```rust
    fn sub_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    Disables all flags enabled in the set.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &OpenFlags) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, other: Self) -> Self { /* ... */ }
    ```
    Returns the left flags, but with all the right flags toggled.

- **BitAnd**
  - ```rust
    fn bitand(self: Self, other: Self) -> Self { /* ... */ }
    ```
    Returns the intersection between the two sets of flags.

- **Extend**
  - ```rust
    fn extend<T: $crate::_core::iter::IntoIterator<Item = Self>>(self: &mut Self, iterator: T) { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: $crate::_core::iter::IntoIterator<Item = Self>>(iterator: T) -> Self { /* ... */ }
    ```

- **BitAndAssign**
  - ```rust
    fn bitand_assign(self: &mut Self, other: Self) { /* ... */ }
    ```
    Disables all flags disabled in the set.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> OpenFlags { /* ... */ }
    ```

- **Not**
  - ```rust
    fn not(self: Self) -> Self { /* ... */ }
    ```
    Returns the complement of this set of flags.

### Struct `InterruptHandle`

Allows interrupting a long-running computation.

```rust
pub struct InterruptHandle {
    pub(crate) db_lock: std::sync::Arc<std::sync::Mutex<*mut ffi::sqlite3>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `db_lock` | `std::sync::Arc<std::sync::Mutex<*mut ffi::sqlite3>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn interrupt(self: &Self) { /* ... */ }
  ```
  Interrupt the query currently executing on another thread. This will

##### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Sync**
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

- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Traits

### Trait `OptionalExtension`

See the [method documentation](#tymethod.optional).

```rust
pub trait OptionalExtension<T> {
    /* Associated items */
}
```

#### Required Items

##### Required Methods

- `optional`: Converts a `Result<T>` into a `Result<Option<T>>`.

#### Implementations

This trait is implemented for the following types:

- `Result<T>` with <T>

## Functions

### Function `errmsg_to_string`

```rust
pub(crate) unsafe fn errmsg_to_string(errmsg: *const std::os::raw::c_char) -> String { /* ... */ }
```

### Function `str_to_cstring`

```rust
pub(crate) fn str_to_cstring(s: &str) -> Result<small_cstr::SmallCString> { /* ... */ }
```

### Function `str_for_sqlite`

Returns `Ok((string ptr, len as c_int, SQLITE_STATIC | SQLITE_TRANSIENT))`
normally.
Returns error if the string is too large for sqlite.
The `sqlite3_destructor_type` item is always `SQLITE_TRANSIENT` unless
the string was empty (in which case it's `SQLITE_STATIC`, and the ptr is
static).

```rust
pub(crate) fn str_for_sqlite(s: &[u8]) -> Result<(*const std::os::raw::c_char, std::os::raw::c_int, ffi::sqlite3_destructor_type)> { /* ... */ }
```

### Function `len_as_c_int`

```rust
pub(crate) fn len_as_c_int(len: usize) -> Result<std::os::raw::c_int> { /* ... */ }
```

### Function `path_to_cstring`

**Attributes:**

- `#[cfg(unix)]`

```rust
pub(crate) fn path_to_cstring(p: &std::path::Path) -> Result<std::ffi::CString> { /* ... */ }
```

### Function `bypass_sqlite_initialization`

rusqlite's check for a safe SQLite threading mode requires SQLite 3.7.0 or
later. If you are running against a SQLite older than that, rusqlite
attempts to ensure safety by performing configuration and initialization of
SQLite itself the first time you
attempt to open a connection. By default, rusqlite panics if that
initialization fails, since that could mean SQLite has been initialized in
single-thread mode.

If you are encountering that panic _and_ can ensure that SQLite has been
initialized in either multi-thread or serialized mode, call this function
prior to attempting to open a connection and rusqlite's initialization
process will by skipped.

# Safety

This function is unsafe because if you call it and SQLite has actually been
configured to run in single-thread mode,
you may encounter memory errors or data corruption or any number of terrible
things that should not be possible when you're using Rust.

```rust
pub unsafe fn bypass_sqlite_initialization() { /* ... */ }
```

### Function `bypass_sqlite_version_check`

rusqlite performs a one-time check that the runtime SQLite version is at
least as new as the version of SQLite found when rusqlite was built.
Bypassing this check may be dangerous; e.g., if you use features of SQLite
that are not present in the runtime version.

# Safety

If you are sure the runtime version is compatible with the
build-time version for your usage, you can bypass the version check by
calling this function before your first connection attempt.

```rust
pub unsafe fn bypass_sqlite_version_check() { /* ... */ }
```

### Function `db_filename`

**Attributes:**

- `#[cfg(not(feature = "modern_sqlite"))]`

```rust
pub(crate) unsafe fn db_filename(_: *mut ffi::sqlite3) -> Option<std::path::PathBuf> { /* ... */ }
```

## Constants and Statics

### Constant `STATEMENT_CACHE_DEFAULT_CAPACITY`

```rust
pub(crate) const STATEMENT_CACHE_DEFAULT_CAPACITY: usize = 16;
```

### Constant `NO_PARAMS`

**Attributes:**

- `#[deprecated =
"Use an empty array instead; `stmt.execute(NO_PARAMS)` => `stmt.execute([])`"]`

**⚠️ Deprecated**: Use an empty array instead; `stmt.execute(NO_PARAMS)` => `stmt.execute([])`

To be used when your statement has no [parameter][sqlite-varparam].

[sqlite-varparam]: https://sqlite.org/lang_expr.html#varparam

This is deprecated in favor of using an empty array literal.

```rust
pub const NO_PARAMS: &[&dyn ToSql] = _;
```

### Constant `MAIN_DB`

Shorthand for [`DatabaseName::Main`].

```rust
pub const MAIN_DB: DatabaseName<''static> = DatabaseName::Main;
```

### Constant `TEMP_DB`

Shorthand for [`DatabaseName::Temp`].

```rust
pub const TEMP_DB: DatabaseName<''static> = DatabaseName::Temp;
```

## Macros

### Macro `params`

**Attributes:**

- `#[macro_export]`

A macro making it more convenient to pass heterogeneous or long lists of
parameters as a `&[&dyn ToSql]`.

# Example

```rust,no_run
# use rusqlite::{Result, Connection, params};

struct Person {
    name: String,
    age_in_years: u8,
    data: Option<Vec<u8>>,
}

fn add_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute("INSERT INTO person (name, age_in_years, data)
                  VALUES (?1, ?2, ?3)",
                 params![person.name, person.age_in_years, person.data])?;
    Ok(())
}
```

```rust
pub macro_rules! params {
    /* macro_rules! params {
    () => { ... };
    ($($param:expr),+ $(,)?) => { ... };
} */
}
```

### Macro `named_params`

**Attributes:**

- `#[macro_export]`

A macro making it more convenient to pass lists of named parameters
as a `&[(&str, &dyn ToSql)]`.

# Example

```rust,no_run
# use rusqlite::{Result, Connection, named_params};

struct Person {
    name: String,
    age_in_years: u8,
    data: Option<Vec<u8>>,
}

fn add_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person (name, age_in_years, data)
         VALUES (:name, :age, :data)",
        named_params!{
            ":name": person.name,
            ":age": person.age_in_years,
            ":data": person.data,
        }
    )?;
    Ok(())
}
```

```rust
pub macro_rules! named_params {
    /* macro_rules! named_params {
    () => { ... };
    ($($param_name:literal: $param_val:expr),+ $(,)?) => { ... };
} */
}
```

## Re-exports

### Re-export `libsqlite3_sys`

```rust
pub use libsqlite3_sys as ffi;
```

### Re-export `CachedStatement`

```rust
pub use crate::cache::CachedStatement;
```

### Re-export `Column`

```rust
pub use crate::column::Column;
```

### Re-export `Error`

```rust
pub use crate::error::Error;
```

### Re-export `ErrorCode`

```rust
pub use crate::ffi::ErrorCode;
```

### Re-export `params_from_iter`

```rust
pub use crate::params::params_from_iter;
```

### Re-export `Params`

```rust
pub use crate::params::Params;
```

### Re-export `ParamsFromIter`

```rust
pub use crate::params::ParamsFromIter;
```

### Re-export `AndThenRows`

```rust
pub use crate::row::AndThenRows;
```

### Re-export `Map`

```rust
pub use crate::row::Map;
```

### Re-export `MappedRows`

```rust
pub use crate::row::MappedRows;
```

### Re-export `Row`

```rust
pub use crate::row::Row;
```

### Re-export `RowIndex`

```rust
pub use crate::row::RowIndex;
```

### Re-export `Rows`

```rust
pub use crate::row::Rows;
```

### Re-export `Statement`

```rust
pub use crate::statement::Statement;
```

### Re-export `StatementStatus`

```rust
pub use crate::statement::StatementStatus;
```

### Re-export `DropBehavior`

```rust
pub use crate::transaction::DropBehavior;
```

### Re-export `Savepoint`

```rust
pub use crate::transaction::Savepoint;
```

### Re-export `Transaction`

```rust
pub use crate::transaction::Transaction;
```

### Re-export `TransactionBehavior`

```rust
pub use crate::transaction::TransactionBehavior;
```

### Re-export `ToSql`

```rust
pub use crate::types::ToSql;
```

### Re-export `crate::version::*`

```rust
pub use crate::version::*;
```

