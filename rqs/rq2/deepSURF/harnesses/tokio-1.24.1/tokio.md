# Crate Documentation

**Version:** 1.24.1

**Format Version:** 39

# Module `tokio`

A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous
applications with the Rust programming language. At a high level, it
provides a few major components:

* Tools for [working with asynchronous tasks][tasks], including
  [synchronization primitives and channels][sync] and [timeouts, sleeps, and
  intervals][time].
* APIs for [performing asynchronous I/O][io], including [TCP and UDP][net] sockets,
  [filesystem][fs] operations, and [process] and [signal] management.
* A [runtime] for executing asynchronous code, including a task scheduler,
  an I/O driver backed by the operating system's event queue (epoll, kqueue,
  IOCP, etc...), and a high performance timer.

Guide level documentation is found on the [website].

[tasks]: #working-with-tasks
[sync]: crate::sync
[time]: crate::time
[io]: #asynchronous-io
[net]: crate::net
[fs]: crate::fs
[process]: crate::process
[signal]: crate::signal
[fs]: crate::fs
[runtime]: crate::runtime
[website]: https://tokio.rs/tokio/tutorial

# A Tour of Tokio

Tokio consists of a number of modules that provide a range of functionality
essential for implementing asynchronous applications in Rust. In this
section, we will take a brief tour of Tokio, summarizing the major APIs and
their uses.

The easiest way to get started is to enable all features. Do this by
enabling the `full` feature flag:

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring applications

Tokio is great for writing applications and most users in this case shouldn't
worry too much about what features they should pick. If you're unsure, we suggest
going with `full` to ensure that you don't run into any road blocks while you're
building your application.

#### Example

This example shows the quickest way to get started with Tokio.

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring libraries

As a library author your goal should be to provide the lightest weight crate
that is based on Tokio. To achieve this you should ensure that you only enable
the features you need. This allows users to pick up your crate without having
to enable unnecessary features.

#### Example

This example shows how you may want to import features for a library that just
needs to `tokio::spawn` and use a `TcpStream`.

```toml
tokio = { version = "1", features = ["rt", "net"] }
```

## Working With Tasks

Asynchronous programs in Rust are based around lightweight, non-blocking
units of execution called [_tasks_][tasks]. The [`tokio::task`] module provides
important tools for working with tasks:

* The [`spawn`] function and [`JoinHandle`] type, for scheduling a new task
  on the Tokio runtime and awaiting the output of a spawned task, respectively,
* Functions for [running blocking operations][blocking] in an asynchronous
  task context.

The [`tokio::task`] module is present only when the "rt" feature flag
is enabled.

[tasks]: task/index.html#what-are-tasks
[`tokio::task`]: crate::task
[`spawn`]: crate::task::spawn()
[`JoinHandle`]: crate::task::JoinHandle
[blocking]: task/index.html#blocking-and-yielding

The [`tokio::sync`] module contains synchronization primitives to use when
needing to communicate or share data. These include:

* channels ([`oneshot`], [`mpsc`], [`watch`], and [`broadcast`]), for sending values
  between tasks,
* a non-blocking [`Mutex`], for controlling access to a shared, mutable
  value,
* an asynchronous [`Barrier`] type, for multiple tasks to synchronize before
  beginning a computation.

The `tokio::sync` module is present only when the "sync" feature flag is
enabled.

[`tokio::sync`]: crate::sync
[`Mutex`]: crate::sync::Mutex
[`Barrier`]: crate::sync::Barrier
[`oneshot`]: crate::sync::oneshot
[`mpsc`]: crate::sync::mpsc
[`watch`]: crate::sync::watch
[`broadcast`]: crate::sync::broadcast

The [`tokio::time`] module provides utilities for tracking time and
scheduling work. This includes functions for setting [timeouts][timeout] for
tasks, [sleeping][sleep] work to run in the future, or [repeating an operation at an
interval][interval].

In order to use `tokio::time`, the "time" feature flag must be enabled.

[`tokio::time`]: crate::time
[sleep]: crate::time::sleep()
[interval]: crate::time::interval()
[timeout]: crate::time::timeout()

Finally, Tokio provides a _runtime_ for executing asynchronous tasks. Most
applications can use the [`#[tokio::main]`][main] macro to run their code on the
Tokio runtime. However, this macro provides only basic configuration options. As
an alternative, the [`tokio::runtime`] module provides more powerful APIs for configuring
and managing runtimes. You should use that module if the `#[tokio::main]` macro doesn't
provide the functionality you need.

Using the runtime requires the "rt" or "rt-multi-thread" feature flags, to
enable the current-thread [single-threaded scheduler][rt] and the [multi-thread
scheduler][rt-multi-thread], respectively. See the [`runtime` module
documentation][rt-features] for details. In addition, the "macros" feature
flag enables the `#[tokio::main]` and `#[tokio::test]` attributes.

[main]: attr.main.html
[`tokio::runtime`]: crate::runtime
[`Builder`]: crate::runtime::Builder
[`Runtime`]: crate::runtime::Runtime
[rt]: runtime/index.html#current-thread-scheduler
[rt-multi-thread]: runtime/index.html#multi-thread-scheduler
[rt-features]: runtime/index.html#runtime-scheduler

## CPU-bound tasks and blocking code

Tokio is able to concurrently run many tasks on a few threads by repeatedly
swapping the currently running task on each thread. However, this kind of
swapping can only happen at `.await` points, so code that spends a long time
without reaching an `.await` will prevent other tasks from running. To
combat this, Tokio provides two kinds of threads: Core threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default
spawn one for each CPU core. You can use the environment variable `TOKIO_WORKER_THREADS`
to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code
that would otherwise block other tasks from running and are kept alive when
not used for a certain amount of time which can be configured with [`thread_keep_alive`].
Since it is not possible for Tokio to swap out blocking tasks, like it
can do with asynchronous code, the upper limit on the number of blocking
threads is very large. These limits can be configured on the [`Builder`].

To spawn a blocking task, you should use the [`spawn_blocking`] function.

[`Builder`]: crate::runtime::Builder
[`spawn_blocking`]: crate::task::spawn_blocking()
[`thread_keep_alive`]: crate::runtime::Builder::thread_keep_alive()

```
#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
```

If your code is CPU-bound and you wish to limit the number of threads used
to run it, you should use a separate thread pool dedicated to CPU bound tasks.
For example, you could consider using the [rayon] library for CPU-bound
tasks. It is also possible to create an extra Tokio runtime dedicated to
CPU-bound tasks, but if you do this, you should be careful that the extra
runtime runs _only_ CPU-bound tasks, as IO-bound tasks on that runtime
will behave poorly.

Hint: If using rayon, you can use a [`oneshot`] channel to send the result back
to Tokio when the rayon task finishes.

[rayon]: https://docs.rs/rayon
[`oneshot`]: crate::sync::oneshot

## Asynchronous IO

As well as scheduling and running tasks, Tokio provides everything you need
to perform input and output asynchronously.

The [`tokio::io`] module provides Tokio's asynchronous core I/O primitives,
the [`AsyncRead`], [`AsyncWrite`], and [`AsyncBufRead`] traits. In addition,
when the "io-util" feature flag is enabled, it also provides combinators and
functions for working with these traits, forming as an asynchronous
counterpart to [`std::io`].

Tokio also includes APIs for performing various kinds of I/O and interacting
with the operating system asynchronously. These include:

* [`tokio::net`], which contains non-blocking versions of [TCP], [UDP], and
  [Unix Domain Sockets][UDS] (enabled by the "net" feature flag),
* [`tokio::fs`], similar to [`std::fs`] but for performing filesystem I/O
  asynchronously (enabled by the "fs" feature flag),
* [`tokio::signal`], for asynchronously handling Unix and Windows OS signals
  (enabled by the "signal" feature flag),
* [`tokio::process`], for spawning and managing child processes (enabled by
  the "process" feature flag).

[`tokio::io`]: crate::io
[`AsyncRead`]: crate::io::AsyncRead
[`AsyncWrite`]: crate::io::AsyncWrite
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`std::io`]: std::io
[`tokio::net`]: crate::net
[TCP]: crate::net::tcp
[UDP]: crate::net::UdpSocket
[UDS]: crate::net::unix
[`tokio::fs`]: crate::fs
[`std::fs`]: std::fs
[`tokio::signal`]: crate::signal
[`tokio::process`]: crate::process

# Examples

A simple TCP echo server:

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

## Feature flags

Tokio uses a set of [feature flags] to reduce the amount of compiled code. It
is possible to just enable certain features over others. By default, Tokio
does not enable any features but allows one to enable a subset for their use
case. Below is a list of the available feature flags. You may also notice
above each function, struct and trait there is listed one or more feature flags
that are required for that item to be used. If you are new to Tokio it is
recommended that you use the `full` feature flag which will enable all public APIs.
Beware though that this will pull in many extra dependencies that you may not
need.

- `full`: Enables all features listed below except `test-util` and `tracing`.
- `rt`: Enables `tokio::spawn`, the current-thread scheduler,
        and non-scheduler utilities.
- `rt-multi-thread`: Enables the heavier, multi-threaded, work-stealing scheduler.
- `io-util`: Enables the IO based `Ext` traits.
- `io-std`: Enable `Stdout`, `Stdin` and `Stderr` types.
- `net`: Enables `tokio::net` types such as `TcpStream`, `UnixStream` and
         `UdpSocket`, as well as (on Unix-like systems) `AsyncFd` and (on
         FreeBSD) `PollAio`.
- `time`: Enables `tokio::time` types and allows the schedulers to enable
          the built in timer.
- `process`: Enables `tokio::process` types.
- `macros`: Enables `#[tokio::main]` and `#[tokio::test]` macros.
- `sync`: Enables all `tokio::sync` types.
- `signal`: Enables all `tokio::signal` types.
- `fs`: Enables `tokio::fs` types.
- `test-util`: Enables testing based infrastructure for the Tokio runtime.
- `parking_lot`: As a potential optimization, use the _parking_lot_ crate's
                 synchronization primitives internally. Also, this
                 dependency is necessary to construct some of our primitives
                 in a const context. MSRV may increase according to the
                 _parking_lot_ release in use.

_Note: `AsyncRead` and `AsyncWrite` traits do not require any features and are
always available._

### Unstable features

Some feature flags are only available when specifying the `tokio_unstable` flag:

- `tracing`: Enables tracing events.

Likewise, some parts of the API are only available with the same flag:

- [`task::Builder`]
- Some methods on [`task::JoinSet`]
- [`runtime::RuntimeMetrics`]
- [`runtime::Builder::unhandled_panic`]
- [`task::Id`]

This flag enables **unstable** features. The public API of these features
may break in 1.x releases. To enable these features, the `--cfg
tokio_unstable` argument must be passed to `rustc` when compiling. This
serves to explicitly opt-in to features which may break semver conventions,
since Cargo [does not yet directly support such opt-ins][unstable features].

You can specify it in your project's `.cargo/config.toml` file:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

Alternatively, you can specify it with an environment variable:

```sh
## Many *nix shells:
export RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

```powershell
## Windows PowerShell:
$Env:RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

[unstable features]: https://internals.rust-lang.org/t/feature-request-unstable-opt-in-non-transitive-crate-features/16193#why-not-a-crate-feature-2
[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

## WASM support

Tokio has some limited support for the WASM platform. Without the
`tokio_unstable` flag, the following features are supported:

 * `sync`
 * `macros`
 * `io-util`
 * `rt`
 * `time`

Enabling any other feature (including `full`) will cause a compilation
failure.

The `time` module will only work on WASM platforms that have support for
timers (e.g. wasm32-wasi). The timing functions will panic if used on a WASM
platform that does not support timers.

Note also that if the runtime becomes indefinitely idle, it will panic
immediately instead of blocking forever. On platforms that don't support
time, this means that the runtime can never be idle in any way.

### Unstable WASM support

Tokio also has unstable support for some additional WASM features. This
requires the use of the `tokio_unstable` flag.

Using this flag enables the use of `tokio::net` on the wasm32-wasi target.
However, not all methods are available on the networking types as WASI
currently does not support the creation of new sockets from within WASM.
Because of this, sockets must currently be created via the `FromRawFd`
trait.

## Modules

## Module `future`

**Attributes:**

- `#![allow(unreachable_pub)]`

Asynchronous values.

```rust
pub(crate) mod future { /* ... */ }
```

### Modules

## Module `poll_fn`

**Attributes:**

- `#![allow(dead_code)]`

Definition of the `PollFn` adapter combinator.

```rust
pub(in ::future) mod poll_fn { /* ... */ }
```

### Types

#### Struct `PollFn`

Future for the [`poll_fn`] function.

```rust
pub struct PollFn<F> {
    pub(in ::future::poll_fn) f: F,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `f` | `F` |  |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<T> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
### Functions

#### Function `poll_fn`

Creates a new future wrapping around a function returning [`Poll`].

```rust
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut std::task::Context<''_>) -> std::task::Poll<T> { /* ... */ }
```

### Re-exports

#### Re-export `poll_fn`

```rust
pub use poll_fn::poll_fn;
```

## Module `io`

**Attributes:**

- `#![allow(dead_code, unused_imports)]`

Traits, helpers, and type definitions for asynchronous I/O functionality.

This module is the asynchronous version of `std::io`. Primarily, it
defines two traits, [`AsyncRead`] and [`AsyncWrite`], which are asynchronous
versions of the [`Read`] and [`Write`] traits in the standard library.

# AsyncRead and AsyncWrite

Like the standard library's [`Read`] and [`Write`] traits, [`AsyncRead`] and
[`AsyncWrite`] provide the most general interface for reading and writing
input and output. Unlike the standard library's traits, however, they are
_asynchronous_ &mdash; meaning that reading from or writing to a `tokio::io`
type will _yield_ to the Tokio scheduler when IO is not ready, rather than
blocking. This allows other tasks to run while waiting on IO.

Another difference is that `AsyncRead` and `AsyncWrite` only contain
core methods needed to provide asynchronous reading and writing
functionality. Instead, utility methods are defined in the [`AsyncReadExt`]
and [`AsyncWriteExt`] extension traits. These traits are automatically
implemented for all values that implement `AsyncRead` and `AsyncWrite`
respectively.

End users will rarely interact directly with `AsyncRead` and
`AsyncWrite`. Instead, they will use the async functions defined in the
extension traits. Library authors are expected to implement `AsyncRead`
and `AsyncWrite` in order to provide types that behave like byte streams.

Even with these differences, Tokio's `AsyncRead` and `AsyncWrite` traits
can be used in almost exactly the same manner as the standard library's
`Read` and `Write`. Most types in the standard library that implement `Read`
and `Write` have asynchronous equivalents in `tokio` that implement
`AsyncRead` and `AsyncWrite`, such as [`File`] and [`TcpStream`].

For example, the standard library documentation introduces `Read` by
[demonstrating][std_example] reading some bytes from a [`std::fs::File`]. We
can do the same with [`tokio::fs::File`][`File`]:

```no_run
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
```

[`File`]: crate::fs::File
[`TcpStream`]: crate::net::TcpStream
[`std::fs::File`]: std::fs::File
[std_example]: std::io#read-and-write

## Buffered Readers and Writers

Byte-based interfaces are unwieldy and can be inefficient, as we'd need to be
making near-constant calls to the operating system. To help with this,
`std::io` comes with [support for _buffered_ readers and writers][stdbuf],
and therefore, `tokio::io` does as well.

Tokio provides an async version of the [`std::io::BufRead`] trait,
[`AsyncBufRead`]; and async [`BufReader`] and [`BufWriter`] structs, which
wrap readers and writers. These wrappers use a buffer, reducing the number
of calls and providing nicer methods for accessing exactly what you want.

For example, [`BufReader`] works with the [`AsyncBufRead`] trait to add
extra methods to any async reader:

```no_run
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::open("foo.txt").await?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer).await?;

    println!("{}", buffer);
    Ok(())
}
```

[`BufWriter`] doesn't add any new ways of writing; it just buffers every call
to [`write`](crate::io::AsyncWriteExt::write). However, you **must** flush
[`BufWriter`] to ensure that any buffered data is written.

```no_run
use tokio::io::{self, BufWriter, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::create("foo.txt").await?;
    {
        let mut writer = BufWriter::new(f);

        // Write a byte to the buffer.
        writer.write(&[42u8]).await?;

        // Flush the buffer before it goes out of scope.
        writer.flush().await?;

    } // Unless flushed or shut down, the contents of the buffer is discarded on drop.

    Ok(())
}
```

[stdbuf]: std::io#bufreader-and-bufwriter
[`std::io::BufRead`]: std::io::BufRead
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`BufReader`]: crate::io::BufReader
[`BufWriter`]: crate::io::BufWriter

## Implementing AsyncRead and AsyncWrite

Because they are traits, we can implement [`AsyncRead`] and [`AsyncWrite`] for
our own types, as well. Note that these traits must only be implemented for
non-blocking I/O types that integrate with the futures type system. In
other words, these types must never block the thread, and instead the
current task is notified when the I/O resource is ready.

## Conversion to and from Sink/Stream

It is often convenient to encapsulate the reading and writing of
bytes and instead work with a [`Sink`] or [`Stream`] of some data
type that is encoded as bytes and/or decoded from bytes. Tokio
provides some utility traits in the [tokio-util] crate that
abstract the asynchronous buffering that is required and allows
you to write [`Encoder`] and [`Decoder`] functions working with a
buffer of bytes, and then use that ["codec"] to transform anything
that implements [`AsyncRead`] and [`AsyncWrite`] into a `Sink`/`Stream` of
your structured data.

[tokio-util]: https://docs.rs/tokio-util/0.6/tokio_util/codec/index.html

# Standard input and output

Tokio provides asynchronous APIs to standard [input], [output], and [error].
These APIs are very similar to the ones provided by `std`, but they also
implement [`AsyncRead`] and [`AsyncWrite`].

Note that the standard input / output APIs  **must** be used from the
context of the Tokio runtime, as they require Tokio-specific features to
function. Calling these functions outside of a Tokio runtime will panic.

[input]: fn@stdin
[output]: fn@stdout
[error]: fn@stderr

# `std` re-exports

Additionally, [`Error`], [`ErrorKind`], [`Result`], and [`SeekFrom`] are
re-exported from `std::io` for ease of use.

[`AsyncRead`]: trait@AsyncRead
[`AsyncWrite`]: trait@AsyncWrite
[`AsyncReadExt`]: trait@AsyncReadExt
[`AsyncWriteExt`]: trait@AsyncWriteExt
["codec"]: https://docs.rs/tokio-util/0.6/tokio_util/codec/index.html
[`Encoder`]: https://docs.rs/tokio-util/0.6/tokio_util/codec/trait.Encoder.html
[`Decoder`]: https://docs.rs/tokio-util/0.6/tokio_util/codec/trait.Decoder.html
[`Error`]: struct@Error
[`ErrorKind`]: enum@ErrorKind
[`Result`]: type@Result
[`Read`]: std::io::Read
[`SeekFrom`]: enum@SeekFrom
[`Sink`]: https://docs.rs/futures/0.3/futures/sink/trait.Sink.html
[`Stream`]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html
[`Write`]: std::io::Write

```rust
pub mod io { /* ... */ }
```

### Modules

## Module `async_buf_read`

```rust
pub(in ::io) mod async_buf_read { /* ... */ }
```

### Traits

#### Trait `AsyncBufRead`

Reads bytes asynchronously.

This trait is analogous to [`std::io::BufRead`], but integrates with
the asynchronous task system. In particular, the [`poll_fill_buf`] method,
unlike [`BufRead::fill_buf`], will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Utilities for working with `AsyncBufRead` values are provided by
[`AsyncBufReadExt`].

[`std::io::BufRead`]: std::io::BufRead
[`poll_fill_buf`]: AsyncBufRead::poll_fill_buf
[`BufRead::fill_buf`]: std::io::BufRead::fill_buf
[`AsyncBufReadExt`]: crate::io::AsyncBufReadExt

```rust
pub trait AsyncBufRead: AsyncRead {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `poll_fill_buf`: Attempts to return the contents of the internal buffer, filling it with more data
- `consume`: Tells this buffer that `amt` bytes have been consumed from the buffer,

##### Implementations

This trait is implemented for the following types:

- `Box<T>` with <T: ?Sized + AsyncBufRead + Unpin>
- `&mut T` with <T: ?Sized + AsyncBufRead + Unpin>
- `std::pin::Pin<P>` with <P>
- `&[u8]`
- `io::Cursor<T>` with <T: AsRef<[u8]> + Unpin>

### Macros

#### Macro `deref_async_buf_read`

```rust
pub(crate) macro_rules! deref_async_buf_read {
    /* macro_rules! deref_async_buf_read {
    () => { ... };
} */
}
```

## Module `async_read`

```rust
pub(in ::io) mod async_read { /* ... */ }
```

### Traits

#### Trait `AsyncRead`

Reads bytes from a source.

This trait is analogous to the [`std::io::Read`] trait, but integrates with
the asynchronous task system. In particular, the [`poll_read`] method,
unlike [`Read::read`], will automatically queue the current task for wakeup
and return if data is not yet available, rather than blocking the calling
thread.

Specifically, this means that the `poll_read` function will return one of
the following:

* `Poll::Ready(Ok(()))` means that data was immediately read and placed into
  the output buffer. The amount of data read can be determined by the
  increase in the length of the slice returned by `ReadBuf::filled`. If the
  difference is 0, EOF has been reached.

* `Poll::Pending` means that no data was read into the buffer
  provided. The I/O object is not currently readable but may become readable
  in the future. Most importantly, **the current future's task is scheduled
  to get unparked when the object is readable**. This means that like
  `Future::poll` you'll receive a notification when the I/O object is
  readable again.

* `Poll::Ready(Err(e))` for other errors are standard I/O errors coming from the
  underlying object.

This trait importantly means that the `read` method only works in the
context of a future's task. The object may panic if used outside of a task.

Utilities for working with `AsyncRead` values are provided by
[`AsyncReadExt`].

[`poll_read`]: AsyncRead::poll_read
[`std::io::Read`]: std::io::Read
[`Read::read`]: std::io::Read::read
[`AsyncReadExt`]: crate::io::AsyncReadExt

```rust
pub trait AsyncRead {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `poll_read`: Attempts to read from the `AsyncRead` into `buf`.

##### Implementations

This trait is implemented for the following types:

- `Box<T>` with <T: ?Sized + AsyncRead + Unpin>
- `&mut T` with <T: ?Sized + AsyncRead + Unpin>
- `std::pin::Pin<P>` with <P>
- `&[u8]`
- `io::Cursor<T>` with <T: AsRef<[u8]> + Unpin>

### Macros

#### Macro `deref_async_read`

```rust
pub(crate) macro_rules! deref_async_read {
    /* macro_rules! deref_async_read {
    () => { ... };
} */
}
```

## Module `async_seek`

```rust
pub(in ::io) mod async_seek { /* ... */ }
```

### Traits

#### Trait `AsyncSeek`

Seek bytes asynchronously.

This trait is analogous to the [`std::io::Seek`] trait, but integrates
with the asynchronous task system. In particular, the `start_seek`
method, unlike [`Seek::seek`], will not block the calling thread.

Utilities for working with `AsyncSeek` values are provided by
[`AsyncSeekExt`].

[`std::io::Seek`]: std::io::Seek
[`Seek::seek`]: std::io::Seek::seek()
[`AsyncSeekExt`]: crate::io::AsyncSeekExt

```rust
pub trait AsyncSeek {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `start_seek`: Attempts to seek to an offset, in bytes, in a stream.
- `poll_complete`: Waits for a seek operation to complete.

##### Implementations

This trait is implemented for the following types:

- `Box<T>` with <T: ?Sized + AsyncSeek + Unpin>
- `&mut T` with <T: ?Sized + AsyncSeek + Unpin>
- `std::pin::Pin<P>` with <P>
- `io::Cursor<T>` with <T: AsRef<[u8]> + Unpin>

### Macros

#### Macro `deref_async_seek`

```rust
pub(crate) macro_rules! deref_async_seek {
    /* macro_rules! deref_async_seek {
    () => { ... };
} */
}
```

## Module `async_write`

```rust
pub(in ::io) mod async_write { /* ... */ }
```

### Traits

#### Trait `AsyncWrite`

Writes bytes asynchronously.

The trait inherits from [`std::io::Write`] and indicates that an I/O object is
**nonblocking**. All non-blocking I/O objects must return an error when
bytes cannot be written instead of blocking the current thread.

Specifically, this means that the [`poll_write`] function will return one of
the following:

* `Poll::Ready(Ok(n))` means that `n` bytes of data was immediately
  written.

* `Poll::Pending` means that no data was written from the buffer
  provided. The I/O object is not currently writable but may become writable
  in the future. Most importantly, **the current future's task is scheduled
  to get unparked when the object is writable**. This means that like
  `Future::poll` you'll receive a notification when the I/O object is
  writable again.

* `Poll::Ready(Err(e))` for other errors are standard I/O errors coming from the
  underlying object.

This trait importantly means that the [`write`][stdwrite] method only works in
the context of a future's task. The object may panic if used outside of a task.

Note that this trait also represents that the  [`Write::flush`][stdflush] method
works very similarly to the `write` method, notably that `Ok(())` means that the
writer has successfully been flushed, a "would block" error means that the
current task is ready to receive a notification when flushing can make more
progress, and otherwise normal errors can happen as well.

Utilities for working with `AsyncWrite` values are provided by
[`AsyncWriteExt`].

[`std::io::Write`]: std::io::Write
[`poll_write`]: AsyncWrite::poll_write()
[stdwrite]: std::io::Write::write()
[stdflush]: std::io::Write::flush()
[`AsyncWriteExt`]: crate::io::AsyncWriteExt

```rust
pub trait AsyncWrite {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `poll_write`: Attempt to write bytes from `buf` into the object.
- `poll_flush`: Attempts to flush the object, ensuring that any buffered data reach
- `poll_shutdown`: Initiates or attempts to shut down this writer, returning success when

##### Provided Methods

- ```rust
  fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<''_>, bufs: &[IoSlice<''_>]) -> Poll<Result<usize, io::Error>> { /* ... */ }
  ```
  Like [`poll_write`], except that it writes from a slice of buffers.

- ```rust
  fn is_write_vectored(self: &Self) -> bool { /* ... */ }
  ```
  Determines if this writer has an efficient [`poll_write_vectored`]

##### Implementations

This trait is implemented for the following types:

- `Box<T>` with <T: ?Sized + AsyncWrite + Unpin>
- `&mut T` with <T: ?Sized + AsyncWrite + Unpin>
- `std::pin::Pin<P>` with <P>
- `Vec<u8>`
- `io::Cursor<&mut [u8]>`
- `io::Cursor<&mut Vec<u8>>`
- `io::Cursor<Vec<u8>>`
- `io::Cursor<Box<[u8]>>`

### Macros

#### Macro `deref_async_write`

```rust
pub(crate) macro_rules! deref_async_write {
    /* macro_rules! deref_async_write {
    () => { ... };
} */
}
```

## Module `read_buf`

```rust
pub(in ::io) mod read_buf { /* ... */ }
```

### Types

#### Struct `ReadBuf`

A wrapper around a byte buffer that is incrementally filled and initialized.

This type is a sort of "double cursor". It tracks three regions in the
buffer: a region at the beginning of the buffer that has been logically
filled with data, a region that has been initialized at some point but not
yet logically filled, and a region at the end that may be uninitialized.
The filled region is guaranteed to be a subset of the initialized region.

In summary, the contents of the buffer can be visualized as:

```not_rust
[             capacity              ]
[ filled |         unfilled         ]
[    initialized    | uninitialized ]
```

It is undefined behavior to de-initialize any bytes from the uninitialized
region, since it is merely unknown whether this region is uninitialized or
not, and if part of it turns out to be initialized, it must stay initialized.

```rust
pub struct ReadBuf<''a> {
    pub(in ::io::read_buf) buf: &''a mut [std::mem::MaybeUninit<u8>],
    pub(in ::io::read_buf) filled: usize,
    pub(in ::io::read_buf) initialized: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a mut [std::mem::MaybeUninit<u8>]` |  |
| `filled` | `usize` |  |
| `initialized` | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(buf: &''a mut [u8]) -> ReadBuf<''a> { /* ... */ }
  ```
  Creates a new `ReadBuf` from a fully initialized buffer.

- ```rust
  pub fn uninit(buf: &''a mut [MaybeUninit<u8>]) -> ReadBuf<''a> { /* ... */ }
  ```
  Creates a new `ReadBuf` from a fully uninitialized buffer.

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total capacity of the buffer.

- ```rust
  pub fn filled(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns a shared reference to the filled portion of the buffer.

- ```rust
  pub fn filled_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Returns a mutable reference to the filled portion of the buffer.

- ```rust
  pub fn take(self: &mut Self, n: usize) -> ReadBuf<''_> { /* ... */ }
  ```
  Returns a new `ReadBuf` comprised of the unfilled section up to `n`.

- ```rust
  pub fn initialized(self: &Self) -> &[u8] { /* ... */ }
  ```
  Returns a shared reference to the initialized portion of the buffer.

- ```rust
  pub fn initialized_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Returns a mutable reference to the initialized portion of the buffer.

- ```rust
  pub unsafe fn inner_mut(self: &mut Self) -> &mut [MaybeUninit<u8>] { /* ... */ }
  ```
  Returns a mutable reference to the entire buffer, without ensuring that it has been fully

- ```rust
  pub unsafe fn unfilled_mut(self: &mut Self) -> &mut [MaybeUninit<u8>] { /* ... */ }
  ```
  Returns a mutable reference to the unfilled part of the buffer without ensuring that it has been fully

- ```rust
  pub fn initialize_unfilled(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Returns a mutable reference to the unfilled part of the buffer, ensuring it is fully initialized.

- ```rust
  pub fn initialize_unfilled_to(self: &mut Self, n: usize) -> &mut [u8] { /* ... */ }
  ```
  Returns a mutable reference to the first `n` bytes of the unfilled part of the buffer, ensuring it is

- ```rust
  pub fn remaining(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of bytes at the end of the slice that have not yet been filled.

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the buffer, resetting the filled region to empty.

- ```rust
  pub fn advance(self: &mut Self, n: usize) { /* ... */ }
  ```
  Advances the size of the filled region of the buffer.

- ```rust
  pub fn set_filled(self: &mut Self, n: usize) { /* ... */ }
  ```
  Sets the size of the filled region of the buffer.

- ```rust
  pub unsafe fn assume_init(self: &mut Self, n: usize) { /* ... */ }
  ```
  Asserts that the first `n` unfilled bytes of the buffer are initialized.

- ```rust
  pub fn put_slice(self: &mut Self, buf: &[u8]) { /* ... */ }
  ```
  Appends data to the buffer, advancing the written position and possibly also the initialized position.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
### Functions

#### Function `slice_to_uninit_mut`

```rust
pub(in ::io::read_buf) unsafe fn slice_to_uninit_mut(slice: &mut [u8]) -> &mut [std::mem::MaybeUninit<u8>] { /* ... */ }
```

#### Function `slice_assume_init`

```rust
pub(in ::io::read_buf) unsafe fn slice_assume_init(slice: &[std::mem::MaybeUninit<u8>]) -> &[u8] { /* ... */ }
```

#### Function `slice_assume_init_mut`

```rust
pub(in ::io::read_buf) unsafe fn slice_assume_init_mut(slice: &mut [std::mem::MaybeUninit<u8>]) -> &mut [u8] { /* ... */ }
```

### Re-exports

#### Re-export `AsyncBufRead`

```rust
pub use self::async_buf_read::AsyncBufRead;
```

#### Re-export `AsyncRead`

```rust
pub use self::async_read::AsyncRead;
```

#### Re-export `AsyncSeek`

```rust
pub use self::async_seek::AsyncSeek;
```

#### Re-export `AsyncWrite`

```rust
pub use self::async_write::AsyncWrite;
```

#### Re-export `ReadBuf`

```rust
pub use self::read_buf::ReadBuf;
```

#### Re-export `Error`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Error;
```

#### Re-export `ErrorKind`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::ErrorKind;
```

#### Re-export `Result`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::Result;
```

#### Re-export `SeekFrom`

**Attributes:**

- `#[doc(no_inline)]`

```rust
pub use std::io::SeekFrom;
```

## Module `net`

**Attributes:**

- `#![cfg(not(loom))]`

TCP/UDP/Unix bindings for `tokio`.

This module contains the TCP/UDP/Unix networking types, similar to the standard
library, which can be used to implement networking protocols.

# Organization

* [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
* [`UdpSocket`] provides functionality for communication over UDP
* [`UnixListener`] and [`UnixStream`] provide functionality for communication over a
Unix Domain Stream Socket **(available on Unix only)**
* [`UnixDatagram`] provides functionality for communication
over Unix Domain Datagram Socket **(available on Unix only)**

[`TcpListener`]: TcpListener
[`TcpStream`]: TcpStream
[`UdpSocket`]: UdpSocket
[`UnixListener`]: UnixListener
[`UnixStream`]: UnixStream
[`UnixDatagram`]: UnixDatagram

```rust
pub mod net { /* ... */ }
```

### Modules

## Module `addr`

```rust
pub(in ::net) mod addr { /* ... */ }
```

### Modules

## Module `sealed`

The contents of this trait are intended to remain private and __not__
part of the `ToSocketAddrs` public API. The details will change over
time.

```rust
pub(crate) mod sealed { /* ... */ }
```

### Types

#### Struct `Internal`

**Attributes:**

- `#[allow(missing_debug_implementations)]`

```rust
pub struct Internal;
```

##### Implementations

###### Trait Implementations

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Unpin**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Types

#### Type Alias `ReadyFuture`

```rust
pub(in ::net::addr) type ReadyFuture<T> = future::Ready<io::Result<T>>;
```

### Traits

#### Trait `ToSocketAddrs`

Converts or resolves without blocking to one or more `SocketAddr` values.

# DNS

Implementations of `ToSocketAddrs` for string types require a DNS lookup.

# Calling

Currently, this trait is only used as an argument to Tokio functions that
need to reference a target socket address. To perform a `SocketAddr`
conversion directly, use [`lookup_host()`](super::lookup_host()).

This trait is sealed and is intended to be opaque. The details of the trait
will change. Stabilization is pending enhancements to the Rust language.

```rust
pub trait ToSocketAddrs: sealed::ToSocketAddrsPriv {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `&T` with <T: ToSocketAddrs + ?Sized>
- `std::net::SocketAddr`
- `std::net::SocketAddrV4`
- `std::net::SocketAddrV6`
- `(std::net::IpAddr, u16)`
- `(std::net::Ipv4Addr, u16)`
- `(std::net::Ipv6Addr, u16)`
- `&[std::net::SocketAddr]`

### Re-exports

#### Re-export `ToSocketAddrs`

```rust
pub use addr::ToSocketAddrs;
```

## Module `loom`

**Attributes:**

- `#![allow(unused)]`

This module abstracts over `loom` and `std::sync` depending on whether we
are running tests or not.

```rust
pub(crate) mod loom { /* ... */ }
```

### Modules

## Module `std`

**Attributes:**

- `#[cfg(not(all(test, loom)))]`
- `#![allow(unused_imports, dead_code)]`

```rust
pub(in ::loom) mod std { /* ... */ }
```

### Modules

## Module `atomic_u16`

```rust
pub(in ::loom::std) mod atomic_u16 { /* ... */ }
```

### Types

#### Struct `AtomicU16`

`AtomicU16` providing an additional `unsync_load` function.

```rust
pub(crate) struct AtomicU16 {
    pub(in ::loom::std::atomic_u16) inner: std::cell::UnsafeCell<std::sync::atomic::AtomicU16>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::cell::UnsafeCell<std::sync::atomic::AtomicU16>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) const fn new(val: u16) -> AtomicU16 { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn unsync_load(self: &Self) -> u16 { /* ... */ }
  ```
  Performs an unsynchronized load.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Receiver**
- **Sync**
- **UnwindSafe**
## Module `atomic_u32`

```rust
pub(in ::loom::std) mod atomic_u32 { /* ... */ }
```

### Types

#### Struct `AtomicU32`

`AtomicU32` providing an additional `unsync_load` function.

```rust
pub(crate) struct AtomicU32 {
    pub(in ::loom::std::atomic_u32) inner: std::cell::UnsafeCell<std::sync::atomic::AtomicU32>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::cell::UnsafeCell<std::sync::atomic::AtomicU32>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) const fn new(val: u32) -> AtomicU32 { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn unsync_load(self: &Self) -> u32 { /* ... */ }
  ```
  Performs an unsynchronized load.

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Receiver**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `atomic_u64`

Implementation of an atomic u64 cell. On 64 bit platforms, this is a
re-export of `AtomicU64`. On 32 bit platforms, this is implemented using a
`Mutex`.

```rust
pub(in ::loom::std) mod atomic_u64 { /* ... */ }
```

### Modules

## Module `imp`

**Attributes:**

- `#[cfg(all(target_has_atomic = "64", not(tokio_no_atomic_u64)))]`
- `#[path = "atomic_u64_native.rs"]`

```rust
pub(in ::loom::std::atomic_u64) mod imp { /* ... */ }
```

### Types

#### Type Alias `StaticAtomicU64`

Alias `AtomicU64` to `StaticAtomicU64`

```rust
pub(crate) type StaticAtomicU64 = std::sync::atomic::AtomicU64;
```

## Module `atomic_usize`

```rust
pub(in ::loom::std) mod atomic_usize { /* ... */ }
```

### Types

#### Struct `AtomicUsize`

`AtomicUsize` providing an additional `unsync_load` function.

```rust
pub(crate) struct AtomicUsize {
    pub(in ::loom::std::atomic_usize) inner: std::cell::UnsafeCell<std::sync::atomic::AtomicUsize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::cell::UnsafeCell<std::sync::atomic::AtomicUsize>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) const fn new(val: usize) -> AtomicUsize { /* ... */ }
  ```

- ```rust
  pub(crate) unsafe fn unsync_load(self: &Self) -> usize { /* ... */ }
  ```
  Performs an unsynchronized load.

- ```rust
  pub(crate) fn with_mut<R, /* synthetic */ impl FnOnce(&mut usize) -> R: FnOnce(&mut usize) -> R>(self: &mut Self, f: impl FnOnce(&mut usize) -> R) -> R { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Receiver**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **DerefMut**
  - ```rust
    fn deref_mut(self: &mut Self) -> &mut <Self as >::Target { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
## Module `mutex`

```rust
pub(in ::loom::std) mod mutex { /* ... */ }
```

### Types

#### Struct `Mutex`

Adapter for `std::Mutex` that removes the poisoning aspects
from its api.

```rust
pub(crate) struct Mutex<T: ?Sized>(pub(in ::loom::std::mutex) sync::Mutex<T>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `sync::Mutex<T>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new(t: T) -> Mutex<T> { /* ... */ }
  ```

- ```rust
  pub(crate) const fn const_new(t: T) -> Mutex<T> { /* ... */ }
  ```

- ```rust
  pub(crate) fn lock(self: &Self) -> MutexGuard<''_, T> { /* ... */ }
  ```

- ```rust
  pub(crate) fn try_lock(self: &Self) -> Option<MutexGuard<''_, T>> { /* ... */ }
  ```

###### Trait Implementations

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **Sync**
- **Freeze**
- **Unpin**
## Module `unsafe_cell`

```rust
pub(in ::loom::std) mod unsafe_cell { /* ... */ }
```

### Types

#### Struct `UnsafeCell`

```rust
pub(crate) struct UnsafeCell<T>(pub(in ::loom::std::unsafe_cell) std::cell::UnsafeCell<T>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::cell::UnsafeCell<T>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) const fn new(data: T) -> UnsafeCell<T> { /* ... */ }
  ```

- ```rust
  pub(crate) fn with<R, /* synthetic */ impl FnOnce(*const T) -> R: FnOnce(*const T) -> R>(self: &Self, f: impl FnOnce(*const T) -> R) -> R { /* ... */ }
  ```

- ```rust
  pub(crate) fn with_mut<R, /* synthetic */ impl FnOnce(*mut T) -> R: FnOnce(*mut T) -> R>(self: &Self, f: impl FnOnce(*mut T) -> R) -> R { /* ... */ }
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

- **Unpin**
- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `cell`

```rust
pub(crate) mod cell { /* ... */ }
```

## Module `hint`

```rust
pub(crate) mod hint { /* ... */ }
```

## Module `rand`

```rust
pub(crate) mod rand { /* ... */ }
```

### Functions

#### Function `seed`

```rust
pub(crate) fn seed() -> u64 { /* ... */ }
```

### Constants and Statics

#### Static `COUNTER`

```rust
pub(in ::loom::std::rand) static COUNTER: std::sync::atomic::AtomicU32 = _;
```

## Module `sync`

```rust
pub(crate) mod sync { /* ... */ }
```

### Modules

## Module `atomic`

```rust
pub(crate) mod atomic { /* ... */ }
```

## Module `sys`

```rust
pub(crate) mod sys { /* ... */ }
```

### Functions

#### Function `num_cpus`

**Attributes:**

- `#[cfg(not(feature = "rt-multi-thread"))]`

```rust
pub(crate) fn num_cpus() -> usize { /* ... */ }
```

## Module `thread`

```rust
pub(crate) mod thread { /* ... */ }
```

### Functions

#### Function `yield_now`

**Attributes:**

- `#[inline]`

```rust
pub(crate) fn yield_now() { /* ... */ }
```

## Module `runtime`

**Attributes:**

- `#[cfg(not(feature = "rt"))]`

The Tokio runtime.

Unlike other Rust programs, asynchronous applications require runtime
support. In particular, the following runtime services are necessary:

* An **I/O event loop**, called the driver, which drives I/O resources and
  dispatches I/O events to tasks that depend on them.
* A **scheduler** to execute [tasks] that use these I/O resources.
* A **timer** for scheduling work to run after a set period of time.

Tokio's [`Runtime`] bundles all of these services as a single type, allowing
them to be started, shut down, and configured together. However, often it is
not required to configure a [`Runtime`] manually, and a user may just use the
[`tokio::main`] attribute macro, which creates a [`Runtime`] under the hood.

# Usage

When no fine tuning is required, the [`tokio::main`] attribute macro can be
used.

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

From within the context of the runtime, additional tasks are spawned using
the [`tokio::spawn`] function. Futures spawned using this function will be
executed on the same thread pool used by the [`Runtime`].

A [`Runtime`] instance can also be used directly.

```no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt  = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
```

## Runtime Configurations

Tokio provides multiple task scheduling strategies, suitable for different
applications. The [runtime builder] or `#[tokio::main]` attribute may be
used to select which scheduler to use.

#### Multi-Thread Scheduler

The multi-thread scheduler executes futures on a _thread pool_, using a
work-stealing strategy. By default, it will start a worker thread for each
CPU core available on the system. This tends to be the ideal configuration
for most applications. The multi-thread scheduler requires the `rt-multi-thread`
feature flag, and is selected by default:
```
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let threaded_rt = runtime::Runtime::new()?;
# Ok(()) }
```

Most applications should use the multi-thread scheduler, except in some
niche use-cases, such as when running only a single thread is required.

#### Current-Thread Scheduler

The current-thread scheduler provides a _single-threaded_ future executor.
All tasks will be created and executed on the current thread. This requires
the `rt` feature flag.
```
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let rt = runtime::Builder::new_current_thread()
    .build()?;
# Ok(()) }
```

#### Resource drivers

When configuring a runtime by hand, no resource drivers are enabled by
default. In this case, attempting to use networking types or time types will
fail. In order to enable these types, the resource drivers must be enabled.
This is done with [`Builder::enable_io`] and [`Builder::enable_time`]. As a
shorthand, [`Builder::enable_all`] enables both resource drivers.

## Lifetime of spawned threads

The runtime may spawn threads depending on its configuration and usage. The
multi-thread scheduler spawns threads to schedule tasks and for `spawn_blocking`
calls.

While the `Runtime` is active, threads may shutdown after periods of being
idle. Once `Runtime` is dropped, all runtime threads are forcibly shutdown.
Any tasks that have not yet completed will be dropped.

[tasks]: crate::task
[`Runtime`]: Runtime
[`tokio::spawn`]: crate::spawn
[`tokio::main`]: ../attr.main.html
[runtime builder]: crate::runtime::Builder
[`Runtime::new`]: crate::runtime::Runtime::new
[`Builder::threaded_scheduler`]: crate::runtime::Builder::threaded_scheduler
[`Builder::enable_io`]: crate::runtime::Builder::enable_io
[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

```rust
pub(crate) mod runtime { /* ... */ }
```

### Modules

## Module `context`

```rust
pub(crate) mod context { /* ... */ }
```

### Types

#### Struct `Context`

```rust
pub(in ::runtime::context) struct Context {
    pub(in ::runtime::context) budget: std::cell::Cell<coop::Budget>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `budget` | `std::cell::Cell<coop::Budget>` | Tracks the amount of "work" a task may still do before yielding back to<br>the sheduler |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `budget`

```rust
pub(in ::runtime) fn budget<R, /* synthetic */ impl FnOnce(&Cell<coop::Budget>) -> R: FnOnce(&std::cell::Cell<coop::Budget>) -> R>(f: impl FnOnce(&std::cell::Cell<coop::Budget>) -> R) -> Result<R, std::thread::AccessError> { /* ... */ }
```

### Constants and Statics

#### Constant `CONTEXT`

```rust
pub(in ::runtime::context) const CONTEXT: $crate::thread::LocalKey<Context> = _;
```

## Module `coop`

**Attributes:**

- `#![allow(dead_code)]`

Yield points for improved cooperative scheduling.

Documentation for this can be found in the [`tokio::task`] module.

[`tokio::task`]: crate::task.

```rust
pub(crate) mod coop { /* ... */ }
```

### Types

#### Struct `Budget`

Opaque type tracking the amount of "work" a task may still do before
yielding back to the scheduler.

```rust
pub(crate) struct Budget(pub(in ::runtime::coop) Option<u8>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<u8>` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::runtime::coop) const fn initial() -> Budget { /* ... */ }
  ```
  Budget assigned to a task on each poll.

- ```rust
  pub(in ::runtime) const fn unconstrained() -> Budget { /* ... */ }
  ```
  Returns an unconstrained budget. Operations will not be limited.

- ```rust
  pub(in ::runtime::coop) fn has_remaining(self: Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Copy**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Budget { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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
### Functions

#### Function `budget`

**Attributes:**

- `#[inline(always)]`

Runs the given closure with a cooperative task budget. When the function
returns, the budget is reset to the value prior to calling the function.

```rust
pub(crate) fn budget<R, /* synthetic */ impl FnOnce() -> R: FnOnce() -> R>(f: impl FnOnce() -> R) -> R { /* ... */ }
```

#### Function `with_unconstrained`

**Attributes:**

- `#[inline(always)]`

Runs the given closure with an unconstrained task budget. When the function returns, the budget
is reset to the value prior to calling the function.

```rust
pub(crate) fn with_unconstrained<R, /* synthetic */ impl FnOnce() -> R: FnOnce() -> R>(f: impl FnOnce() -> R) -> R { /* ... */ }
```

#### Function `with_budget`

**Attributes:**

- `#[inline(always)]`

```rust
pub(in ::runtime::coop) fn with_budget<R, /* synthetic */ impl FnOnce() -> R: FnOnce() -> R>(budget: Budget, f: impl FnOnce() -> R) -> R { /* ... */ }
```

#### Function `has_budget_remaining`

**Attributes:**

- `#[inline(always)]`

```rust
pub(crate) fn has_budget_remaining() -> bool { /* ... */ }
```

## Module `park`

**Attributes:**

- `#![allow(dead_code)]`

```rust
pub(crate) mod park { /* ... */ }
```

### Types

#### Struct `ParkThread`

```rust
pub(crate) struct ParkThread {
    pub(in ::runtime::park) inner: std::sync::Arc<Inner>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::sync::Arc<Inner>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub(crate) fn unpark(self: &Self) -> UnparkThread { /* ... */ }
  ```

- ```rust
  pub(crate) fn park(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(crate) fn park_timeout(self: &mut Self, duration: Duration) { /* ... */ }
  ```

- ```rust
  pub(crate) fn shutdown(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `UnparkThread`

Unblocks a thread that was blocked by `ParkThread`.

```rust
pub(crate) struct UnparkThread {
    pub(in ::runtime::park) inner: std::sync::Arc<Inner>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `std::sync::Arc<Inner>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn unpark(self: &Self) { /* ... */ }
  ```

- ```rust
  pub(crate) fn into_waker(self: Self) -> Waker { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnparkThread { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Inner`

```rust
pub(in ::runtime::park) struct Inner {
    pub(in ::runtime::park) state: crate::loom::std::atomic_usize::AtomicUsize,
    pub(in ::runtime::park) mutex: crate::loom::std::mutex::Mutex<()>,
    pub(in ::runtime::park) condvar: std::sync::Condvar,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `state` | `crate::loom::std::atomic_usize::AtomicUsize` |  |
| `mutex` | `crate::loom::std::mutex::Mutex<()>` |  |
| `condvar` | `std::sync::Condvar` |  |

##### Implementations

###### Methods

- ```rust
  pub(in ::runtime::park) fn park(self: &Self) { /* ... */ }
  ```
  Parks the current thread for at most `dur`.

- ```rust
  pub(in ::runtime::park) fn park_timeout(self: &Self, dur: Duration) { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) fn unpark(self: &Self) { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) fn shutdown(self: &Self) { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) fn into_raw(this: Arc<Inner>) -> *const () { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) unsafe fn from_raw(ptr: *const ()) -> Arc<Inner> { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `CachedParkThread`

Blocks the current thread using a condition variable.

```rust
pub(crate) struct CachedParkThread {
    pub(in ::runtime::park) _anchor: std::marker::PhantomData<std::rc::Rc<()>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `_anchor` | `std::marker::PhantomData<std::rc::Rc<()>>` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new() -> CachedParkThread { /* ... */ }
  ```
  Creates a new `ParkThread` handle for the current thread.

- ```rust
  pub(crate) fn waker(self: &Self) -> Result<Waker, AccessError> { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) fn unpark(self: &Self) -> Result<UnparkThread, AccessError> { /* ... */ }
  ```

- ```rust
  pub(crate) fn park(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub(crate) fn park_timeout(self: &mut Self, duration: Duration) { /* ... */ }
  ```

- ```rust
  pub(in ::runtime::park) fn with_current<F, R>(self: &Self, f: F) -> Result<R, AccessError>
where
    F: FnOnce(&ParkThread) -> R { /* ... */ }
  ```
  Gets a reference to the `ParkThread` handle for this thread.

- ```rust
  pub(crate) fn block_on<F: Future>(self: &mut Self, f: F) -> Result<<F as >::Output, AccessError> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `unparker_to_raw_waker`

```rust
pub(in ::runtime::park) unsafe fn unparker_to_raw_waker(unparker: std::sync::Arc<Inner>) -> std::task::RawWaker { /* ... */ }
```

#### Function `clone`

```rust
pub(in ::runtime::park) unsafe fn clone(raw: *const ()) -> std::task::RawWaker { /* ... */ }
```

#### Function `drop_waker`

```rust
pub(in ::runtime::park) unsafe fn drop_waker(raw: *const ()) { /* ... */ }
```

#### Function `wake`

```rust
pub(in ::runtime::park) unsafe fn wake(raw: *const ()) { /* ... */ }
```

#### Function `wake_by_ref`

```rust
pub(in ::runtime::park) unsafe fn wake_by_ref(raw: *const ()) { /* ... */ }
```

### Constants and Statics

#### Constant `EMPTY`

```rust
pub(in ::runtime::park) const EMPTY: usize = 0;
```

#### Constant `PARKED`

```rust
pub(in ::runtime::park) const PARKED: usize = 1;
```

#### Constant `NOTIFIED`

```rust
pub(in ::runtime::park) const NOTIFIED: usize = 2;
```

#### Constant `CURRENT_PARKER`

```rust
pub(in ::runtime::park) const CURRENT_PARKER: $crate::thread::LocalKey<ParkThread> = _;
```

## Module `driver`

**Attributes:**

- `#![allow(dead_code)]`

Abstracts out the entire chain of runtime sub-drivers into common types.

```rust
pub(in ::runtime) mod driver { /* ... */ }
```

### Types

#### Struct `Driver`

```rust
pub(crate) struct Driver {
    pub(in ::runtime::driver) inner: IoStack,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `IoStack` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn new(cfg: Cfg) -> io::Result<(Self, Handle)> { /* ... */ }
  ```

- ```rust
  pub(crate) fn park(self: &mut Self, handle: &Handle) { /* ... */ }
  ```

- ```rust
  pub(crate) fn park_timeout(self: &mut Self, handle: &Handle, duration: Duration) { /* ... */ }
  ```

- ```rust
  pub(crate) fn shutdown(self: &mut Self, handle: &Handle) { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Send**
#### Struct `Handle`

```rust
pub(crate) struct Handle {
    pub(crate) io: crate::runtime::park::UnparkThread,
    pub(crate) signal: (),
    pub(crate) time: (),
    pub(crate) clock: (),
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `io` | `crate::runtime::park::UnparkThread` | IO driver handle |
| `signal` | `()` | Signal driver handle |
| `time` | `()` | Time driver handle |
| `clock` | `()` | Source of `Instant::now()` |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn unpark(self: &Self) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Cfg`

```rust
pub(crate) struct Cfg {
    pub(crate) enable_io: bool,
    pub(crate) enable_time: bool,
    pub(crate) enable_pause_time: bool,
    pub(crate) start_paused: bool,
    pub(crate) nevents: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enable_io` | `bool` |  |
| `enable_time` | `bool` |  |
| `enable_pause_time` | `bool` |  |
| `start_paused` | `bool` |  |
| `nevents` | `usize` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Sync**
- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Type Alias `IoHandle`

**Attributes:**

- `#[cfg(not(any(feature = "net", all(unix, feature = "process"),
all(unix, feature = "signal"),)))]`

```rust
pub(crate) type IoHandle = crate::runtime::park::UnparkThread;
```

#### Struct `IoStack`

**Attributes:**

- `#[cfg(not(any(feature = "net", all(unix, feature = "process"),
all(unix, feature = "signal"),)))]`

```rust
pub(crate) struct IoStack(pub(in ::runtime::driver) crate::runtime::park::ParkThread);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::runtime::park::ParkThread` |  |

##### Implementations

###### Methods

- ```rust
  pub(crate) fn park(self: &mut Self, _handle: &Handle) { /* ... */ }
  ```

- ```rust
  pub(crate) fn park_timeout(self: &mut Self, _handle: &Handle, duration: Duration) { /* ... */ }
  ```

- ```rust
  pub(crate) fn shutdown(self: &mut Self, _handle: &Handle) { /* ... */ }
  ```

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Type Alias `SignalHandle`

**Attributes:**

- `#[cfg(any(loom, not(unix),
not(any(feature = "signal", all(unix, feature = "process")))))]`

```rust
pub(crate) type SignalHandle = ();
```

#### Type Alias `TimeDriver`

**Attributes:**

- `#[cfg(not(feature = "time"))]`

```rust
pub(in ::runtime::driver) type TimeDriver = IoStack;
```

#### Type Alias `Clock`

**Attributes:**

- `#[cfg(not(feature = "time"))]`

```rust
pub(crate) type Clock = ();
```

#### Type Alias `TimeHandle`

**Attributes:**

- `#[cfg(not(feature = "time"))]`

```rust
pub(crate) type TimeHandle = ();
```

### Functions

#### Function `create_io_stack`

**Attributes:**

- `#[cfg(not(any(feature = "net", all(unix, feature = "process"),
all(unix, feature = "signal"),)))]`

```rust
pub(in ::runtime::driver) fn create_io_stack(_enabled: bool, _nevents: usize) -> io::Result<(IoStack, crate::runtime::park::UnparkThread, ())> { /* ... */ }
```

#### Function `create_clock`

**Attributes:**

- `#[cfg(not(feature = "time"))]`

```rust
pub(in ::runtime::driver) fn create_clock(_enable_pausing: bool, _start_paused: bool) { /* ... */ }
```

#### Function `create_time_driver`

**Attributes:**

- `#[cfg(not(feature = "time"))]`

```rust
pub(in ::runtime::driver) fn create_time_driver(_enable: bool, io_stack: IoStack, _clock: ()) -> (IoStack, ()) { /* ... */ }
```

## Module `scheduler`

```rust
pub(crate) mod scheduler { /* ... */ }
```

### Types

#### Enum `Handle`

```rust
pub(crate) enum Handle {
    Disabled,
}
```

##### Variants

###### `Disabled`

##### Implementations

###### Methods

- ```rust
  pub(crate) fn driver(self: &Self) -> &driver::Handle { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Handle { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `sync`

**Attributes:**

- `#[cfg(not(feature = "sync"))]`

Synchronization primitives for use in asynchronous contexts.

Tokio programs tend to be organized as a set of [tasks] where each task
operates independently and may be executed on separate physical threads. The
synchronization primitives provided in this module permit these independent
tasks to communicate together.

[tasks]: crate::task

# Message passing

The most common form of synchronization in a Tokio program is message
passing. Two tasks operate independently and send messages to each other to
synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using channels. A channel supports sending a
message from one producer task to one or more consumer tasks. There are a
few flavors of channels provided by Tokio. Each channel flavor supports
different message passing patterns. When a channel supports multiple
producers, many separate tasks may **send** messages. When a channel
supports multiple consumers, many different separate tasks may **receive**
messages.

Tokio provides many different channel flavors as different message passing
patterns are best handled with different implementations.

## `oneshot` channel

The [`oneshot` channel][oneshot] supports sending a **single** value from a
single producer to a single consumer. This channel is usually used to send
the result of a computation to a waiter.

**Example:** using a [`oneshot` channel][oneshot] to receive the result of a
computation.

```
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = rx.await.unwrap();
}
```

Note, if the task produces a computation result as its final
action before terminating, the [`JoinHandle`] can be used to
receive that value instead of allocating resources for the
`oneshot` channel. Awaiting on [`JoinHandle`] returns `Result`. If
the task panics, the `Joinhandle` yields `Err` with the panic
cause.

**Example:**

```
async fn some_computation() -> String {
    "the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let join_handle = tokio::spawn(async move {
        some_computation().await
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = join_handle.await.unwrap();
}
```

[oneshot]: oneshot
[`JoinHandle`]: crate::task::JoinHandle

## `mpsc` channel

The [`mpsc` channel][mpsc] supports sending **many** values from **many**
producers to a single consumer. This channel is often used to send work to a
task or to receive the result of many computations.

This is also the channel you should use if you want to send many messages
from a single producer to a single consumer. There is no dedicated spsc
channel.

**Example:** using an mpsc to incrementally stream the results of a series
of computations.

```
use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            let res = some_computation(i).await;
            tx.send(res).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!("got = {}", res);
    }
}
```

The argument to `mpsc::channel` is the channel capacity. This is the maximum
number of values that can be stored in the channel pending receipt at any
given time. Properly setting this value is key in implementing robust
programs as the channel capacity plays a critical part in handling back
pressure.

A common concurrency pattern for resource management is to spawn a task
dedicated to managing that resource and using message passing between other
tasks to interact with the resource. The resource may be anything that may
not be concurrently used. Some examples include a socket and program state.
For example, if multiple tasks need to send data over a single socket, spawn
a task to manage the socket and use a channel to synchronize.

**Example:** sending data from many tasks over a single socket using message
passing.

```no_run
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let (tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the
        // original handle.
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.
    drop(tx);

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}
```

The [`mpsc`][mpsc] and [`oneshot`][oneshot] channels can be combined to
provide a request / response type synchronization pattern with a shared
resource. A task is spawned to synchronize a resource and waits on commands
received on a [`mpsc`][mpsc] channel. Each command includes a
[`oneshot`][oneshot] `Sender` on which the result of the command is sent.

**Example:** use a task to synchronize a `u64` counter. Each task sends an
"fetch and increment" command. The counter value **before** the increment is
sent over the provided `oneshot` channel.

```
use tokio::sync::{oneshot, mpsc};
use Command::Increment;

enum Command {
    Increment,
    // Other commands can be added here
}

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

    // Spawn a task to manage the counter
    tokio::spawn(async move {
        let mut counter: u64 = 0;

        while let Some((cmd, response)) = cmd_rx.recv().await {
            match cmd {
                Increment => {
                    let prev = counter;
                    counter += 1;
                    response.send(prev).unwrap();
                }
            }
        }
    });

    let mut join_handles = vec![];

    // Spawn tasks that will send the increment command.
    for _ in 0..10 {
        let cmd_tx = cmd_tx.clone();

        join_handles.push(tokio::spawn(async move {
            let (resp_tx, resp_rx) = oneshot::channel();

            cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
            let res = resp_rx.await.unwrap();

            println!("previous value = {}", res);
        }));
    }

    // Wait for all tasks to complete
    for join_handle in join_handles.drain(..) {
        join_handle.await.unwrap();
    }
}
```

[mpsc]: mpsc

## `broadcast` channel

The [`broadcast` channel] supports sending **many** values from
**many** producers to **many** consumers. Each consumer will receive
**each** value. This channel can be used to implement "fan out" style
patterns common with pub / sub or "chat" systems.

This channel tends to be used less often than `oneshot` and `mpsc` but still
has its use cases.

This is also the channel you should use if you want to broadcast values from
a single producer to many consumers. There is no dedicated spmc broadcast
channel.

Basic usage

```
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();
}
```

[`broadcast` channel]: crate::sync::broadcast

## `watch` channel

The [`watch` channel] supports sending **many** values from a **single**
producer to **many** consumers. However, only the **most recent** value is
stored in the channel. Consumers are notified when a new value is sent, but
there is no guarantee that consumers will see **all** values.

The [`watch` channel] is similar to a [`broadcast` channel] with capacity 1.

Use cases for the [`watch` channel] include broadcasting configuration
changes or signalling program state changes, such as transitioning to
shutdown.

**Example:** use a [`watch` channel] to notify tasks of configuration
changes. In this example, a configuration file is checked periodically. When
the file changes, the configuration changes are signalled to consumers.

```
use tokio::sync::watch;
use tokio::time::{self, Duration, Instant};

use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
# Ok(Config { timeout: Duration::from_secs(1) })
    }
}

async fn my_async_operation() {
    // Do something here
}

#[tokio::main]
async fn main() {
    // Load initial configuration value
    let mut config = Config::load_from_file().await.unwrap();

    // Create the watch channel, initialized with the loaded configuration
    let (tx, rx) = watch::channel(config.clone());

    // Spawn a task to monitor the file.
    tokio::spawn(async move {
        loop {
            // Wait 10 seconds between checks
            time::sleep(Duration::from_secs(10)).await;

            // Load the configuration file
            let new_config = Config::load_from_file().await.unwrap();

            // If the configuration changed, send the new config value
            // on the watch channel.
            if new_config != config {
                tx.send(new_config.clone()).unwrap();
                config = new_config;
            }
        }
    });

    let mut handles = vec![];

    // Spawn tasks that runs the async operation for at most `timeout`. If
    // the timeout elapses, restart the operation.
    //
    // The task simultaneously watches the `Config` for changes. When the
    // timeout duration changes, the timeout is updated without restarting
    // the in-flight operation.
    for _ in 0..5 {
        // Clone a config watch handle for use in this task
        let mut rx = rx.clone();

        let handle = tokio::spawn(async move {
            // Start the initial operation and pin the future to the stack.
            // Pinning to the stack is required to resume the operation
            // across multiple calls to `select!`
            let op = my_async_operation();
            tokio::pin!(op);

            // Get the initial config value
            let mut conf = rx.borrow().clone();

            let mut op_start = Instant::now();
            let sleep = time::sleep_until(op_start + conf.timeout);
            tokio::pin!(sleep);

            loop {
                tokio::select! {
                    _ = &mut sleep => {
                        // The operation elapsed. Restart it
                        op.set(my_async_operation());

                        // Track the new start time
                        op_start = Instant::now();

                        // Restart the timeout
                        sleep.set(time::sleep_until(op_start + conf.timeout));
                    }
                    _ = rx.changed() => {
                        conf = rx.borrow().clone();

                        // The configuration has been updated. Update the
                        // `sleep` using the new `timeout` value.
                        sleep.as_mut().reset(op_start + conf.timeout);
                    }
                    _ = &mut op => {
                        // The operation completed!
                        return
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles.drain(..) {
        handle.await.unwrap();
    }
}
```

[`watch` channel]: mod@crate::sync::watch
[`broadcast` channel]: mod@crate::sync::broadcast

# State synchronization

The remaining synchronization primitives focus on synchronizing state.
These are asynchronous equivalents to versions provided by `std`. They
operate in a similar way as their `std` counterparts but will wait
asynchronously instead of blocking the thread.

* [`Barrier`](Barrier) Ensures multiple tasks will wait for each other to
  reach a point in the program, before continuing execution all together.

* [`Mutex`](Mutex) Mutual Exclusion mechanism, which ensures that at most
  one thread at a time is able to access some data.

* [`Notify`](Notify) Basic task notification. `Notify` supports notifying a
  receiving task without sending data. In this case, the task wakes up and
  resumes processing.

* [`RwLock`](RwLock) Provides a mutual exclusion mechanism which allows
  multiple readers at the same time, while allowing only one writer at a
  time. In some cases, this can be more efficient than a mutex.

* [`Semaphore`](Semaphore) Limits the amount of concurrency. A semaphore
  holds a number of permits, which tasks may request in order to enter a
  critical section. Semaphores are useful for implementing limiting or
  bounding of any kind.

```rust
pub(crate) mod sync { /* ... */ }
```

## Module `task`

Asynchronous green-threads.

## What are Tasks?

A _task_ is a light weight, non-blocking unit of execution. A task is similar
to an OS thread, but rather than being managed by the OS scheduler, they are
managed by the [Tokio runtime][rt]. Another name for this general pattern is
[green threads]. If you are familiar with [Go's goroutines], [Kotlin's
coroutines], or [Erlang's processes], you can think of Tokio's tasks as
something similar.

Key points about tasks include:

* Tasks are **light weight**. Because tasks are scheduled by the Tokio
  runtime rather than the operating system, creating new tasks or switching
  between tasks does not require a context switch and has fairly low
  overhead. Creating, running, and destroying large numbers of tasks is
  quite cheap, especially compared to OS threads.

* Tasks are scheduled **cooperatively**. Most operating systems implement
  _preemptive multitasking_. This is a scheduling technique where the
  operating system allows each thread to run for a period of time, and then
  _preempts_ it, temporarily pausing that thread and switching to another.
  Tasks, on the other hand, implement _cooperative multitasking_. In
  cooperative multitasking, a task is allowed to run until it _yields_,
  indicating to the Tokio runtime's scheduler that it cannot currently
  continue executing. When a task yields, the Tokio runtime switches to
  executing the next task.

* Tasks are **non-blocking**. Typically, when an OS thread performs I/O or
  must synchronize with another thread, it _blocks_, allowing the OS to
  schedule another thread. When a task cannot continue executing, it must
  yield instead, allowing the Tokio runtime to schedule another task. Tasks
  should generally not perform system calls or other operations that could
  block a thread, as this would prevent other tasks running on the same
  thread from executing as well. Instead, this module provides APIs for
  running blocking operations in an asynchronous context.

[rt]: crate::runtime
[green threads]: https://en.wikipedia.org/wiki/Green_threads
[Go's goroutines]: https://tour.golang.org/concurrency/1
[Kotlin's coroutines]: https://kotlinlang.org/docs/reference/coroutines-overview.html
[Erlang's processes]: http://erlang.org/doc/getting_started/conc_prog.html#processes

## Working with Tasks

This module provides the following APIs for working with tasks:

### Spawning

Perhaps the most important function in this module is [`task::spawn`]. This
function can be thought of as an async equivalent to the standard library's
[`thread::spawn`][`std::thread::spawn`]. It takes an `async` block or other
[future], and creates a new task to run that work concurrently:

```
use tokio::task;

# async fn doc() {
task::spawn(async {
    // perform some work here...
});
# }
```

Like [`std::thread::spawn`], `task::spawn` returns a [`JoinHandle`] struct.
A `JoinHandle` is itself a future which may be used to await the output of
the spawned task. For example:

```
use tokio::task;

# #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
let join = task::spawn(async {
    // ...
    "hello world!"
});

// ...

// Await the result of the spawned task.
let result = join.await?;
assert_eq!(result, "hello world!");
# Ok(())
# }
```

Again, like `std::thread`'s [`JoinHandle` type][thread_join], if the spawned
task panics, awaiting its `JoinHandle` will return a [`JoinError`]. For
example:

```
use tokio::task;

# #[tokio::main] async fn main() {
let join = task::spawn(async {
    panic!("something bad happened!")
});

// The returned result indicates that the task failed.
assert!(join.await.is_err());
# }
```

`spawn`, `JoinHandle`, and `JoinError` are present when the "rt"
feature flag is enabled.

[`task::spawn`]: crate::task::spawn()
[future]: std::future::Future
[`std::thread::spawn`]: std::thread::spawn
[`JoinHandle`]: crate::task::JoinHandle
[thread_join]: std::thread::JoinHandle
[`JoinError`]: crate::task::JoinError

### Blocking and Yielding

As we discussed above, code running in asynchronous tasks should not perform
operations that can block. A blocking operation performed in a task running
on a thread that is also running other tasks would block the entire thread,
preventing other tasks from running.

Instead, Tokio provides two APIs for running blocking operations in an
asynchronous context: [`task::spawn_blocking`] and [`task::block_in_place`].

Be aware that if you call a non-async method from async code, that non-async
method is still inside the asynchronous context, so you should also avoid
blocking operations there. This includes destructors of objects destroyed in
async code.

#### spawn_blocking

The `task::spawn_blocking` function is similar to the `task::spawn` function
discussed in the previous section, but rather than spawning an
_non-blocking_ future on the Tokio runtime, it instead spawns a
_blocking_ function on a dedicated thread pool for blocking tasks. For
example:

```
use tokio::task;

# async fn docs() {
task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
});
# }
```

Just like `task::spawn`, `task::spawn_blocking` returns a `JoinHandle`
which we can use to await the result of the blocking operation:

```rust
# use tokio::task;
# async fn docs() -> Result<(), Box<dyn std::error::Error>>{
let join = task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

let result = join.await?;
assert_eq!(result, "blocking completed");
# Ok(())
# }
```

#### block_in_place

When using the [multi-threaded runtime][rt-multi-thread], the [`task::block_in_place`]
function is also available. Like `task::spawn_blocking`, this function
allows running a blocking operation from an asynchronous context. Unlike
`spawn_blocking`, however, `block_in_place` works by transitioning the
_current_ worker thread to a blocking thread, moving other tasks running on
that thread to another worker thread. This can improve performance by avoiding
context switches.

For example:

```
use tokio::task;

# async fn docs() {
let result = task::block_in_place(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

assert_eq!(result, "blocking completed");
# }
```

#### yield_now

In addition, this module provides a [`task::yield_now`] async function
that is analogous to the standard library's [`thread::yield_now`]. Calling
and `await`ing this function will cause the current task to yield to the
Tokio runtime's scheduler, allowing other tasks to be
scheduled. Eventually, the yielding task will be polled again, allowing it
to execute. For example:

```rust
use tokio::task;

# #[tokio::main] async fn main() {
async {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");
}
# .await;
# }
```

### Cooperative scheduling

A single call to [`poll`] on a top-level task may potentially do a lot of
work before it returns `Poll::Pending`. If a task runs for a long period of
time without yielding back to the executor, it can starve other tasks
waiting on that executor to execute them, or drive underlying resources.
Since Rust does not have a runtime, it is difficult to forcibly preempt a
long-running task. Instead, this module provides an opt-in mechanism for
futures to collaborate with the executor to avoid starvation.

Consider a future like this one:

```
# use tokio_stream::{Stream, StreamExt};
async fn drop_all<I: Stream + Unpin>(mut input: I) {
    while let Some(_) = input.next().await {}
}
```

It may look harmless, but consider what happens under heavy load if the
input stream is _always_ ready. If we spawn `drop_all`, the task will never
yield, and will starve other tasks and resources on the same executor.

To account for this, Tokio has explicit yield points in a number of library
functions, which force tasks to return to the executor periodically.


#### unconstrained

If necessary, [`task::unconstrained`] lets you opt a future out of of Tokio's cooperative
scheduling. When a future is wrapped with `unconstrained`, it will never be forced to yield to
Tokio. For example:

```
# #[tokio::main]
# async fn main() {
use tokio::{task, sync::mpsc};

let fut = async {
    let (tx, mut rx) = mpsc::unbounded_channel();

    for i in 0..1000 {
        let _ = tx.send(());
        // This will always be ready. If coop was in effect, this code would be forced to yield
        // periodically. However, if left unconstrained, then this code will never yield.
        rx.recv().await;
    }
};

task::unconstrained(fut).await;
# }
```

[`task::spawn_blocking`]: crate::task::spawn_blocking
[`task::block_in_place`]: crate::task::block_in_place
[rt-multi-thread]: ../runtime/index.html#threaded-scheduler
[`task::yield_now`]: crate::task::yield_now()
[`thread::yield_now`]: std::thread::yield_now
[`task::unconstrained`]: crate::task::unconstrained()
[`poll`]: method@std::future::Future::poll

```rust
pub mod task { /* ... */ }
```

## Module `util`

```rust
pub(crate) mod util { /* ... */ }
```

### Modules

## Module `trace`

```rust
pub(crate) mod trace { /* ... */ }
```

## Module `error`

**Attributes:**

- `#![allow(dead_code)]`

```rust
pub(crate) mod error { /* ... */ }
```

### Constants and Statics

#### Constant `CONTEXT_MISSING_ERROR`

Error string explaining that the Tokio context hasn't been instantiated.

```rust
pub(crate) const CONTEXT_MISSING_ERROR: &str = "there is no reactor running, must be called from the context of a Tokio 1.x runtime";
```

#### Constant `RUNTIME_SHUTTING_DOWN_ERROR`

Error string explaining that the Tokio context is shutting down and cannot drive timers.

```rust
pub(crate) const RUNTIME_SHUTTING_DOWN_ERROR: &str = "A Tokio 1.x context was found, but it is being shutdown.";
```

#### Constant `THREAD_LOCAL_DESTROYED_ERROR`

Error string explaining that the Tokio context is not available because the
thread-local storing it has been destroyed. This usually only happens during
destructors of other thread-locals.

```rust
pub(crate) const THREAD_LOCAL_DESTROYED_ERROR: &str = "The Tokio context thread-local variable has been destroyed.";
```

## Module `stream`

Due to the `Stream` trait's inclusion in `std` landing later than Tokio's 1.0
release, most of the Tokio stream utilities have been moved into the [`tokio-stream`]
crate.

# Why was `Stream` not included in Tokio 1.0?

Originally, we had planned to ship Tokio 1.0 with a stable `Stream` type
but unfortunately the [RFC] had not been merged in time for `Stream` to
reach `std` on a stable compiler in time for the 1.0 release of Tokio. For
this reason, the team has decided to move all `Stream` based utilities to
the [`tokio-stream`] crate. While this is not ideal, once `Stream` has made
it into the standard library and the MSRV period has passed, we will implement
stream for our different types.

While this may seem unfortunate, not all is lost as you can get much of the
`Stream` support with `async/await` and `while let` loops. It is also possible
to create a `impl Stream` from `async fn` using the [`async-stream`] crate.

[`tokio-stream`]: https://docs.rs/tokio-stream
[`async-stream`]: https://docs.rs/async-stream
[RFC]: https://github.com/rust-lang/rfcs/pull/2996

# Example

Convert a [`sync::mpsc::Receiver`] to an `impl Stream`.

```rust,no_run
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<usize>(16);

let stream = async_stream::stream! {
    while let Some(item) = rx.recv().await {
        yield item;
    }
};
```

```rust
pub mod stream { /* ... */ }
```

## Macros

### Macro `pin`

**Attributes:**

- `#[macro_export]`

Pins a value on the stack.

Calls to `async fn` return anonymous [`Future`] values that are `!Unpin`.
These values must be pinned before they can be polled. Calling `.await` will
handle this, but consumes the future. If it is required to call `.await` on
a `&mut _` reference, the caller is responsible for pinning the future.

Pinning may be done by allocating with [`Box::pin`] or by using the stack
with the `pin!` macro.

The following will **fail to compile**:

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = my_async_fn();
    (&mut future).await;
}
```

To make this work requires pinning:

```
use tokio::pin;

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let future = my_async_fn();
    pin!(future);

    (&mut future).await;
}
```

Pinning is useful when using `select!` and stream operators that require `T:
Stream + Unpin`.

[`Future`]: trait@std::future::Future
[`Box::pin`]: std::boxed::Box::pin

# Usage

The `pin!` macro takes **identifiers** as arguments. It does **not** work
with expressions.

The following does not compile as an expression is passed to `pin!`.

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = pin!(my_async_fn());
    (&mut future).await;
}
```

# Examples

Using with select:

```
use tokio::{pin, select};
use tokio_stream::{self as stream, StreamExt};

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3, 4]);

    let future = my_async_fn();
    pin!(future);

    loop {
        select! {
            _ = &mut future => {
                // Stop looping `future` will be polled after completion
                break;
            }
            Some(val) = stream.next() => {
                println!("got value = {}", val);
            }
        }
    }
}
```

Because assigning to a variable followed by pinning is common, there is also
a variant of the macro that supports doing both in one go.

```
use tokio::{pin, select};

async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    pin! {
        let future1 = my_async_fn();
        let future2 = my_async_fn();
    }

    select! {
        _ = &mut future1 => {}
        _ = &mut future2 => {}
    }
}
```

```rust
pub macro_rules! pin {
    /* macro_rules! pin {
    ($($x:ident),*) => { ... };
    ($(
            let $x:ident = $init:expr;
    )*) => { ... };
} */
}
```

