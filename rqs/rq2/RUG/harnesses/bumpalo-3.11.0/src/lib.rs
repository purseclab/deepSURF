#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(
    feature = "allocator_api",
    feature(allocator_api, nonnull_slice_from_raw_parts)
)]
#[doc(hidden)]
pub extern crate alloc as core_alloc;
#[cfg(feature = "boxed")]
pub mod boxed;
#[cfg(feature = "collections")]
pub mod collections;
mod alloc;
use core::cell::Cell;
use core::fmt::Display;
use core::iter;
use core::marker::PhantomData;
use core::mem;
use core::ptr::{self, NonNull};
use core::slice;
use core::str;
use core_alloc::alloc::{alloc, dealloc, Layout};
#[cfg(feature = "allocator_api")]
use core_alloc::alloc::{AllocError, Allocator};
pub use alloc::AllocErr;
/// An error returned from [`Bump::try_alloc_try_with`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AllocOrInitError<E> {
    /// Indicates that the initial allocation failed.
    Alloc(AllocErr),
    /// Indicates that the initializer failed with the contained error after
    /// allocation.
    ///
    /// It is possible but not guaranteed that the allocated memory has been
    /// released back to the allocator at this point.
    Init(E),
}
impl<E> From<AllocErr> for AllocOrInitError<E> {
    fn from(e: AllocErr) -> Self {
        Self::Alloc(e)
    }
}
impl<E: Display> Display for AllocOrInitError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AllocOrInitError::Alloc(err) => err.fmt(f),
            AllocOrInitError::Init(err) => write!(f, "initialization failed: {}", err),
        }
    }
}
/// An arena to bump allocate into.
///
/// ## No `Drop`s
///
/// Objects that are bump-allocated will never have their [`Drop`] implementation
/// called &mdash; unless you do it manually yourself. This makes it relatively
/// easy to leak memory or other resources.
///
/// If you have a type which internally manages
///
/// * an allocation from the global heap (e.g. [`Vec<T>`]),
/// * open file descriptors (e.g. [`std::fs::File`]), or
/// * any other resource that must be cleaned up (e.g. an `mmap`)
///
/// and relies on its `Drop` implementation to clean up the internal resource,
/// then if you allocate that type with a `Bump`, you need to find a new way to
/// clean up after it yourself.
///
/// Potential solutions are:
///
/// * Using [`bumpalo::boxed::Box::new_in`] instead of [`Bump::alloc`], that
///   will drop wrapped values similarly to [`std::boxed::Box`]. Note that this
///   requires enabling the `"boxed"` Cargo feature for this crate. **This is
///   often the easiest solution.**
///
/// * Calling [`drop_in_place`][drop_in_place] or using
///   [`std::mem::ManuallyDrop`][manuallydrop] to manually drop these types.
///
/// * Using [`bumpalo::collections::Vec`] instead of [`std::vec::Vec`].
///
/// * Avoiding allocating these problematic types within a `Bump`.
///
/// Note that not calling `Drop` is memory safe! Destructors are never
/// guaranteed to run in Rust, you can't rely on them for enforcing memory
/// safety.
///
/// [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
/// [`Vec<T>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`std::fs::File`]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [drop_in_place]: https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html
/// [manuallydrop]: https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html
/// [`bumpalo::collections::Vec`]: collections/vec/struct.Vec.html
/// [`std::vec::Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`bumpalo::boxed::Box::new_in`]: boxed/struct.Box.html#method.new_in
/// [`std::boxed::Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
///
/// ## Example
///
/// ```
/// use bumpalo::Bump;
///
/// // Create a new bump arena.
/// let bump = Bump::new();
///
/// // Allocate values into the arena.
/// let forty_two = bump.alloc(42);
/// assert_eq!(*forty_two, 42);
///
/// // Mutable references are returned from allocation.
/// let mut s = bump.alloc("bumpalo");
/// *s = "the bump allocator; and also is a buffalo";
/// ```
///
/// ## Allocation Methods Come in Many Flavors
///
/// There are various allocation methods on `Bump`, the simplest being
/// [`alloc`][Bump::alloc]. The others exist to satisfy some combination of
/// fallible allocation and initialization. The allocation methods are
/// summarized in the following table:
///
/// <table>
///   <thead>
///     <tr>
///       <th></th>
///       <th>Infallible Allocation</th>
///       <th>Fallible Allocation</th>
///     </tr>
///   </thead>
///     <tr>
///       <th>By Value</th>
///       <td><a href="#method.alloc"><code>alloc</code></a></td>
///       <td><a href="#method.try_alloc"><code>try_alloc</code></a></td>
///     </tr>
///     <tr>
///       <th>Infallible Initializer Function</th>
///       <td><a href="#method.alloc_with"><code>alloc_with</code></a></td>
///       <td><a href="#method.try_alloc_with"><code>try_alloc_with</code></a></td>
///     </tr>
///     <tr>
///       <th>Fallible Initializer Function</th>
///       <td><a href="#method.alloc_try_with"><code>alloc_try_with</code></a></td>
///       <td><a href="#method.try_alloc_try_with"><code>try_alloc_try_with</code></a></td>
///     </tr>
///   <tbody>
///   </tbody>
/// </table>
///
/// ### Fallible Allocation: The `try_alloc_` Method Prefix
///
/// These allocation methods let you recover from out-of-memory (OOM)
/// scenarioes, rather than raising a panic on OOM.
///
/// ```
/// use bumpalo::Bump;
///
/// let bump = Bump::new();
///
/// match bump.try_alloc(MyStruct {
///     // ...
/// }) {
///     Ok(my_struct) => {
///         // Allocation succeeded.
///     }
///     Err(e) => {
///         // Out of memory.
///     }
/// }
///
/// struct MyStruct {
///     // ...
/// }
/// ```
///
/// ### Initializer Functions: The `_with` Method Suffix
///
/// Calling one of the generic `…alloc(x)` methods is essentially equivalent to
/// the matching [`…alloc_with(|| x)`](?search=alloc_with). However if you use
/// `…alloc_with`, then the closure will not be invoked until after allocating
/// space for storing `x` on the heap.
///
/// This can be useful in certain edge-cases related to compiler optimizations.
/// When evaluating for example `bump.alloc(x)`, semantically `x` is first put
/// on the stack and then moved onto the heap. In some cases, the compiler is
/// able to optimize this into constructing `x` directly on the heap, however
/// in many cases it does not.
///
/// The `…alloc_with` functions try to help the compiler be smarter. In most
/// cases doing for example `bump.try_alloc_with(|| x)` on release mode will be
/// enough to help the compiler realize that this optimization is valid and
/// to construct `x` directly onto the heap.
///
/// #### Warning
///
/// These functions critically depend on compiler optimizations to achieve their
/// desired effect. This means that it is not an effective tool when compiling
/// without optimizations on.
///
/// Even when optimizations are on, these functions do not **guarantee** that
/// the value is constructed on the heap. To the best of our knowledge no such
/// guarantee can be made in stable Rust as of 1.54.
///
/// ### Fallible Initialization: The `_try_with` Method Suffix
///
/// The generic [`…alloc_try_with(|| x)`](?search=_try_with) methods behave
/// like the purely `_with` suffixed methods explained above. However, they
/// allow for fallible initialization by accepting a closure that returns a
/// [`Result`] and will attempt to undo the initial allocation if this closure
/// returns [`Err`].
///
/// #### Warning
///
/// If the inner closure returns [`Ok`], space for the entire [`Result`] remains
/// allocated inside `self`. This can be a problem especially if the [`Err`]
/// variant is larger, but even otherwise there may be overhead for the
/// [`Result`]'s discriminant.
///
/// <p><details><summary>Undoing the allocation in the <code>Err</code> case
/// always fails if <code>f</code> successfully made any additional allocations
/// in <code>self</code>.</summary>
///
/// For example, the following will always leak also space for the [`Result`]
/// into this `Bump`, even though the inner reference isn't kept and the [`Err`]
/// payload is returned semantically by value:
///
/// ```rust
/// let bump = bumpalo::Bump::new();
///
/// let r: Result<&mut [u8; 1000], ()> = bump.alloc_try_with(|| {
///     let _ = bump.alloc(0_u8);
///     Err(())
/// });
///
/// assert!(r.is_err());
/// ```
///
///</details></p>
///
/// Since [`Err`] payloads are first placed on the heap and then moved to the
/// stack, `bump.…alloc_try_with(|| x)?` is likely to execute more slowly than
/// the matching `bump.…alloc(x?)` in case of initialization failure. If this
/// happens frequently, using the plain un-suffixed method may perform better.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
/// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
///
/// ### `Bump` Allocation Limits
///
/// `bumpalo` supports setting a limit on the maximum bytes of memory that can
/// be allocated for use in a particular `Bump` arena. This limit can be set and removed with
/// [`set_allocation_limit`][Bump::set_allocation_limit].
/// The allocation limit is only enforced when allocating new backing chunks for
/// a `Bump`. Updating the allocation limit will not affect existing allocations
/// or any future allocations within the `Bump`'s current chunk.
///
/// #### Example
///
/// ```
/// let bump = bumpalo::Bump::new();
///
/// assert_eq!(bump.allocation_limit(), None);
/// bump.set_allocation_limit(Some(0));
///
/// assert!(bump.try_alloc(5).is_err());
///
/// bump.set_allocation_limit(Some(6));
///
/// assert_eq!(bump.allocation_limit(), Some(6));
///
/// bump.set_allocation_limit(None);
///
/// assert_eq!(bump.allocation_limit(), None);
/// ```
///
/// #### Warning
///
/// Because of backwards compatibility, allocations that fail
/// due to allocation limits will not present differently than
/// errors due to resource exhaustion.
#[derive(Debug)]
pub struct Bump {
    current_chunk_footer: Cell<NonNull<ChunkFooter>>,
    allocation_limit: Cell<Option<usize>>,
}
#[repr(C)]
#[derive(Debug)]
struct ChunkFooter {
    data: NonNull<u8>,
    layout: Layout,
    prev: Cell<NonNull<ChunkFooter>>,
    ptr: Cell<NonNull<u8>>,
    allocated_bytes: usize,
}
/// A wrapper type for the canonical, statically allocated empty chunk.
///
/// For the canonical empty chunk to be `static`, its type must be `Sync`, which
/// is the purpose of this wrapper type. This is safe because the empty chunk is
/// immutable and never actually modified.
#[repr(transparent)]
struct EmptyChunkFooter(ChunkFooter);
unsafe impl Sync for EmptyChunkFooter {}
static EMPTY_CHUNK: EmptyChunkFooter = EmptyChunkFooter(ChunkFooter {
    layout: Layout::new::<ChunkFooter>(),
    data: unsafe {
        NonNull::new_unchecked(&EMPTY_CHUNK as *const EmptyChunkFooter as *mut u8)
    },
    ptr: Cell::new(unsafe {
        NonNull::new_unchecked(&EMPTY_CHUNK as *const EmptyChunkFooter as *mut u8)
    }),
    prev: Cell::new(unsafe {
        NonNull::new_unchecked(
            &EMPTY_CHUNK as *const EmptyChunkFooter as *mut ChunkFooter,
        )
    }),
    allocated_bytes: 0,
});
impl EmptyChunkFooter {
    fn get(&'static self) -> NonNull<ChunkFooter> {
        unsafe {
            NonNull::new_unchecked(&self.0 as *const ChunkFooter as *mut ChunkFooter)
        }
    }
}
impl ChunkFooter {
    fn as_raw_parts(&self) -> (*const u8, usize) {
        let data = self.data.as_ptr() as *const u8;
        let ptr = self.ptr.get().as_ptr() as *const u8;
        debug_assert!(data <= ptr);
        debug_assert!(ptr <= self as * const ChunkFooter as * const u8);
        let len = unsafe {
            (self as *const ChunkFooter as *const u8).offset_from(ptr) as usize
        };
        (ptr, len)
    }
    /// Is this chunk the last empty chunk?
    fn is_empty(&self) -> bool {
        ptr::eq(self, EMPTY_CHUNK.get().as_ptr())
    }
}
impl Default for Bump {
    fn default() -> Bump {
        Bump::new()
    }
}
impl Drop for Bump {
    fn drop(&mut self) {
        unsafe {
            dealloc_chunk_list(self.current_chunk_footer.get());
        }
    }
}
#[inline]
unsafe fn dealloc_chunk_list(mut footer: NonNull<ChunkFooter>) {
    while !footer.as_ref().is_empty() {
        let f = footer;
        footer = f.as_ref().prev.get();
        dealloc(f.as_ref().data.as_ptr(), f.as_ref().layout);
    }
}
unsafe impl Send for Bump {}
#[inline]
pub(crate) fn round_up_to(n: usize, divisor: usize) -> Option<usize> {
    debug_assert!(divisor > 0);
    debug_assert!(divisor.is_power_of_two());
    Some(n.checked_add(divisor - 1)? & !(divisor - 1))
}
#[inline]
pub(crate) fn round_down_to(n: usize, divisor: usize) -> usize {
    debug_assert!(divisor > 0);
    debug_assert!(divisor.is_power_of_two());
    n & !(divisor - 1)
}
const PAGE_STRATEGY_CUTOFF: usize = 0x1000;
const SUPPORTED_ITER_ALIGNMENT: usize = 16;
const CHUNK_ALIGN: usize = SUPPORTED_ITER_ALIGNMENT;
const FOOTER_SIZE: usize = mem::size_of::<ChunkFooter>();
const _FOOTER_ALIGN_ASSERTION: bool = mem::align_of::<ChunkFooter>() <= CHUNK_ALIGN;
const _: [(); _FOOTER_ALIGN_ASSERTION as usize] = [()];
const MALLOC_OVERHEAD: usize = 16;
const OVERHEAD: usize = (MALLOC_OVERHEAD + FOOTER_SIZE + (CHUNK_ALIGN - 1))
    & !(CHUNK_ALIGN - 1);
const FIRST_ALLOCATION_GOAL: usize = 1 << 9;
const DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER: usize = FIRST_ALLOCATION_GOAL - OVERHEAD;
/// The memory size and alignment details for a potential new chunk
/// allocation.
#[derive(Debug, Clone, Copy)]
struct NewChunkMemoryDetails {
    new_size_without_footer: usize,
    align: usize,
    size: usize,
}
/// Wrapper around `Layout::from_size_align` that adds debug assertions.
#[inline]
unsafe fn layout_from_size_align(size: usize, align: usize) -> Layout {
    if cfg!(debug_assertions) {
        Layout::from_size_align(size, align).unwrap()
    } else {
        Layout::from_size_align_unchecked(size, align)
    }
}
#[inline(never)]
fn allocation_size_overflow<T>() -> T {
    panic!("requested allocation size overflowed")
}
fn abs_diff(a: usize, b: usize) -> usize {
    usize::max(a, b) - usize::min(a, b)
}
impl Bump {
    /// Construct a new arena to bump allocate into.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// # let _ = bump;
    /// ```
    pub fn new() -> Bump {
        Self::with_capacity(0)
    }
    /// Attempt to construct a new arena to bump allocate into.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::try_new();
    /// # let _ = bump.unwrap();
    /// ```
    pub fn try_new() -> Result<Bump, AllocErr> {
        Bump::try_with_capacity(0)
    }
    /// Construct a new arena with the specified byte capacity to bump allocate into.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::with_capacity(100);
    /// # let _ = bump;
    /// ```
    pub fn with_capacity(capacity: usize) -> Bump {
        Bump::try_with_capacity(capacity).unwrap_or_else(|_| oom())
    }
    /// Attempt to construct a new arena with the specified byte capacity to bump allocate into.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::try_with_capacity(100);
    /// # let _ = bump.unwrap();
    /// ```
    pub fn try_with_capacity(capacity: usize) -> Result<Self, AllocErr> {
        if capacity == 0 {
            return Ok(Bump {
                current_chunk_footer: Cell::new(EMPTY_CHUNK.get()),
                allocation_limit: Cell::new(None),
            });
        }
        let layout = unsafe { layout_from_size_align(capacity, 1) };
        let chunk_footer = unsafe {
            Self::new_chunk(
                    Bump::new_chunk_memory_details(None, layout).ok_or(AllocErr)?,
                    layout,
                    EMPTY_CHUNK.get(),
                )
                .ok_or(AllocErr)?
        };
        Ok(Bump {
            current_chunk_footer: Cell::new(chunk_footer),
            allocation_limit: Cell::new(None),
        })
    }
    /// The allocation limit for this arena in bytes.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::with_capacity(0);
    ///
    /// assert_eq!(bump.allocation_limit(), None);
    ///
    /// bump.set_allocation_limit(Some(6));
    ///
    /// assert_eq!(bump.allocation_limit(), Some(6));
    ///
    /// bump.set_allocation_limit(None);
    ///
    /// assert_eq!(bump.allocation_limit(), None);
    /// ```
    pub fn allocation_limit(&self) -> Option<usize> {
        self.allocation_limit.get()
    }
    /// Set the allocation limit in bytes for this arena.
    ///
    /// The allocation limit is only enforced when allocating new backing chunks for
    /// a `Bump`. Updating the allocation limit will not affect existing allocations
    /// or any future allocations within the `Bump`'s current chunk.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::with_capacity(0);
    ///
    /// bump.set_allocation_limit(Some(0));
    ///
    /// assert!(bump.try_alloc(5).is_err());
    /// ```
    pub fn set_allocation_limit(&self, limit: Option<usize>) {
        self.allocation_limit.set(limit)
    }
    /// How much headroom an arena has before it hits its allocation
    /// limit.
    fn allocation_limit_remaining(&self) -> Option<usize> {
        self.allocation_limit
            .get()
            .and_then(|allocation_limit| {
                let allocated_bytes = self.allocated_bytes();
                if allocated_bytes > allocation_limit {
                    None
                } else {
                    Some(abs_diff(allocation_limit, allocated_bytes))
                }
            })
    }
    /// Whether a request to allocate a new chunk with a given size for a given
    /// requested layout will fit under the allocation limit set on a `Bump`.
    fn chunk_fits_under_limit(
        allocation_limit_remaining: Option<usize>,
        new_chunk_memory_details: NewChunkMemoryDetails,
    ) -> bool {
        allocation_limit_remaining
            .map(|allocation_limit_left| {
                allocation_limit_left >= new_chunk_memory_details.new_size_without_footer
            })
            .unwrap_or(true)
    }
    /// Determine the memory details including final size, alignment and
    /// final size without footer for a new chunk that would be allocated
    /// to fulfill an allocation request.
    fn new_chunk_memory_details(
        new_size_without_footer: Option<usize>,
        requested_layout: Layout,
    ) -> Option<NewChunkMemoryDetails> {
        let mut new_size_without_footer = new_size_without_footer
            .unwrap_or(DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
        let mut align = CHUNK_ALIGN;
        align = align.max(requested_layout.align());
        let requested_size = round_up_to(requested_layout.size(), align)
            .unwrap_or_else(allocation_size_overflow);
        new_size_without_footer = new_size_without_footer.max(requested_size);
        if new_size_without_footer < PAGE_STRATEGY_CUTOFF {
            new_size_without_footer = (new_size_without_footer + OVERHEAD)
                .next_power_of_two() - OVERHEAD;
        } else {
            new_size_without_footer = round_up_to(
                new_size_without_footer + OVERHEAD,
                0x1000,
            )? - OVERHEAD;
        }
        debug_assert_eq!(align % CHUNK_ALIGN, 0);
        debug_assert_eq!(new_size_without_footer % CHUNK_ALIGN, 0);
        let size = new_size_without_footer
            .checked_add(FOOTER_SIZE)
            .unwrap_or_else(allocation_size_overflow);
        Some(NewChunkMemoryDetails {
            new_size_without_footer,
            size,
            align,
        })
    }
    /// Allocate a new chunk and return its initialized footer.
    ///
    /// If given, `layouts` is a tuple of the current chunk size and the
    /// layout of the allocation request that triggered us to fall back to
    /// allocating a new chunk of memory.
    unsafe fn new_chunk(
        new_chunk_memory_details: NewChunkMemoryDetails,
        requested_layout: Layout,
        prev: NonNull<ChunkFooter>,
    ) -> Option<NonNull<ChunkFooter>> {
        let NewChunkMemoryDetails { new_size_without_footer, align, size } = new_chunk_memory_details;
        let layout = layout_from_size_align(size, align);
        debug_assert!(size >= requested_layout.size());
        let data = alloc(layout);
        let data = NonNull::new(data)?;
        let footer_ptr = data.as_ptr().add(new_size_without_footer);
        debug_assert_eq!((data.as_ptr() as usize) % align, 0);
        debug_assert_eq!(footer_ptr as usize % CHUNK_ALIGN, 0);
        let footer_ptr = footer_ptr as *mut ChunkFooter;
        let ptr = Cell::new(NonNull::new_unchecked(footer_ptr as *mut u8));
        let allocated_bytes = prev.as_ref().allocated_bytes + new_size_without_footer;
        ptr::write(
            footer_ptr,
            ChunkFooter {
                data,
                layout,
                prev: Cell::new(prev),
                ptr,
                allocated_bytes,
            },
        );
        Some(NonNull::new_unchecked(footer_ptr))
    }
    /// Reset this bump allocator.
    ///
    /// Performs mass deallocation on everything allocated in this arena by
    /// resetting the pointer into the underlying chunk of memory to the start
    /// of the chunk. Does not run any `Drop` implementations on deallocated
    /// objects; see [the top-level documentation](struct.Bump.html) for details.
    ///
    /// If this arena has allocated multiple chunks to bump allocate into, then
    /// the excess chunks are returned to the global allocator.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut bump = bumpalo::Bump::new();
    ///
    /// // Allocate a bunch of things.
    /// {
    ///     for i in 0..100 {
    ///         bump.alloc(i);
    ///     }
    /// }
    ///
    /// // Reset the arena.
    /// bump.reset();
    ///
    /// // Allocate some new things in the space previously occupied by the
    /// // original things.
    /// for j in 200..400 {
    ///     bump.alloc(j);
    /// }
    ///```
    pub fn reset(&mut self) {
        unsafe {
            if self.current_chunk_footer.get().as_ref().is_empty() {
                return;
            }
            let mut cur_chunk = self.current_chunk_footer.get();
            let prev_chunk = cur_chunk.as_ref().prev.replace(EMPTY_CHUNK.get());
            dealloc_chunk_list(prev_chunk);
            cur_chunk.as_ref().ptr.set(cur_chunk.cast());
            cur_chunk.as_mut().allocated_bytes = cur_chunk.as_ref().layout.size();
            debug_assert!(
                self.current_chunk_footer.get().as_ref().prev.get().as_ref().is_empty(),
                "We should only have a single chunk"
            );
            debug_assert_eq!(
                self.current_chunk_footer.get().as_ref().ptr.get(), self
                .current_chunk_footer.get().cast(),
                "Our chunk's bump finger should be reset to the start of its allocation"
            );
        }
    }
    /// Allocate an object in this `Bump` and return an exclusive reference to
    /// it.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for `T` fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc("hello");
    /// assert_eq!(*x, "hello");
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc<T>(&self, val: T) -> &mut T {
        self.alloc_with(|| val)
    }
    /// Try to allocate an object in this `Bump` and return an exclusive
    /// reference to it.
    ///
    /// ## Errors
    ///
    /// Errors if reserving space for `T` fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.try_alloc("hello");
    /// assert_eq!(x, Ok(&mut "hello"));
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn try_alloc<T>(&self, val: T) -> Result<&mut T, AllocErr> {
        self.try_alloc_with(|| val)
    }
    /// Pre-allocate space for an object in this `Bump`, initializes it using
    /// the closure, then returns an exclusive reference to it.
    ///
    /// See [The `_with` Method Suffix](#initializer-functions-the-_with-method-suffix) for a
    /// discussion on the differences between the `_with` suffixed methods and
    /// those methods without it, their performance characteristics, and when
    /// you might or might not choose a `_with` suffixed method.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for `T` fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_with(|| "hello");
    /// assert_eq!(*x, "hello");
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_with<F, T>(&self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        #[inline(always)]
        unsafe fn inner_writer<T, F>(ptr: *mut T, f: F)
        where
            F: FnOnce() -> T,
        {
            ptr::write(ptr, f())
        }
        let layout = Layout::new::<T>();
        unsafe {
            let p = self.alloc_layout(layout);
            let p = p.as_ptr() as *mut T;
            inner_writer(p, f);
            &mut *p
        }
    }
    /// Tries to pre-allocate space for an object in this `Bump`, initializes
    /// it using the closure, then returns an exclusive reference to it.
    ///
    /// See [The `_with` Method Suffix](#initializer-functions-the-_with-method-suffix) for a
    /// discussion on the differences between the `_with` suffixed methods and
    /// those methods without it, their performance characteristics, and when
    /// you might or might not choose a `_with` suffixed method.
    ///
    /// ## Errors
    ///
    /// Errors if reserving space for `T` fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.try_alloc_with(|| "hello");
    /// assert_eq!(x, Ok(&mut "hello"));
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn try_alloc_with<F, T>(&self, f: F) -> Result<&mut T, AllocErr>
    where
        F: FnOnce() -> T,
    {
        #[inline(always)]
        unsafe fn inner_writer<T, F>(ptr: *mut T, f: F)
        where
            F: FnOnce() -> T,
        {
            ptr::write(ptr, f())
        }
        let layout = Layout::new::<T>();
        let p = self.try_alloc_layout(layout)?;
        let p = p.as_ptr() as *mut T;
        unsafe {
            inner_writer(p, f);
            Ok(&mut *p)
        }
    }
    /// Pre-allocates space for a [`Result`] in this `Bump`, initializes it using
    /// the closure, then returns an exclusive reference to its `T` if [`Ok`].
    ///
    /// Iff the allocation fails, the closure is not run.
    ///
    /// Iff [`Err`], an allocator rewind is *attempted* and the `E` instance is
    /// moved out of the allocator to be consumed or dropped as normal.
    ///
    /// See [The `_with` Method Suffix](#initializer-functions-the-_with-method-suffix) for a
    /// discussion on the differences between the `_with` suffixed methods and
    /// those methods without it, their performance characteristics, and when
    /// you might or might not choose a `_with` suffixed method.
    ///
    /// For caveats specific to fallible initialization, see
    /// [The `_try_with` Method Suffix](#fallible-initialization-the-_try_with-method-suffix).
    ///
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    ///
    /// ## Errors
    ///
    /// Iff the allocation succeeds but `f` fails, that error is forwarded by value.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for `Result<T, E>` fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_try_with(|| Ok("hello"))?;
    /// assert_eq!(*x, "hello");
    /// # Result::<_, ()>::Ok(())
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_try_with<F, T, E>(&self, f: F) -> Result<&mut T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let rewind_footer = self.current_chunk_footer.get();
        let rewind_ptr = unsafe { rewind_footer.as_ref() }.ptr.get();
        let mut inner_result_ptr = NonNull::from(self.alloc_with(f));
        match unsafe { inner_result_ptr.as_mut() } {
            Ok(t) => Ok(unsafe { &mut *(t as *mut _) }),
            Err(e) => {
                unsafe {
                    if self.is_last_allocation(inner_result_ptr.cast()) {
                        let current_footer_p = self.current_chunk_footer.get();
                        let current_ptr = &current_footer_p.as_ref().ptr;
                        if current_footer_p == rewind_footer {
                            current_ptr.set(rewind_ptr);
                        } else {
                            current_ptr.set(current_footer_p.as_ref().data);
                        }
                    }
                    Err(ptr::read(e as *const _))
                }
            }
        }
    }
    /// Tries to pre-allocates space for a [`Result`] in this `Bump`,
    /// initializes it using the closure, then returns an exclusive reference
    /// to its `T` if all [`Ok`].
    ///
    /// Iff the allocation fails, the closure is not run.
    ///
    /// Iff the closure returns [`Err`], an allocator rewind is *attempted* and
    /// the `E` instance is moved out of the allocator to be consumed or dropped
    /// as normal.
    ///
    /// See [The `_with` Method Suffix](#initializer-functions-the-_with-method-suffix) for a
    /// discussion on the differences between the `_with` suffixed methods and
    /// those methods without it, their performance characteristics, and when
    /// you might or might not choose a `_with` suffixed method.
    ///
    /// For caveats specific to fallible initialization, see
    /// [The `_try_with` Method Suffix](#fallible-initialization-the-_try_with-method-suffix).
    ///
    /// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
    /// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
    /// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
    ///
    /// ## Errors
    ///
    /// Errors with the [`Alloc`](`AllocOrInitError::Alloc`) variant iff
    /// reserving space for `Result<T, E>` fails.
    ///
    /// Iff the allocation succeeds but `f` fails, that error is forwarded by
    /// value inside the [`Init`](`AllocOrInitError::Init`) variant.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.try_alloc_try_with(|| Ok("hello"))?;
    /// assert_eq!(*x, "hello");
    /// # Result::<_, bumpalo::AllocOrInitError<()>>::Ok(())
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn try_alloc_try_with<F, T, E>(
        &self,
        f: F,
    ) -> Result<&mut T, AllocOrInitError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let rewind_footer = self.current_chunk_footer.get();
        let rewind_ptr = unsafe { rewind_footer.as_ref() }.ptr.get();
        let mut inner_result_ptr = NonNull::from(self.try_alloc_with(f)?);
        match unsafe { inner_result_ptr.as_mut() } {
            Ok(t) => Ok(unsafe { &mut *(t as *mut _) }),
            Err(e) => {
                unsafe {
                    if self.is_last_allocation(inner_result_ptr.cast()) {
                        let current_footer_p = self.current_chunk_footer.get();
                        let current_ptr = &current_footer_p.as_ref().ptr;
                        if current_footer_p == rewind_footer {
                            current_ptr.set(rewind_ptr);
                        } else {
                            current_ptr.set(current_footer_p.as_ref().data);
                        }
                    }
                    Err(AllocOrInitError::Init(ptr::read(e as *const _)))
                }
            }
        }
    }
    /// `Copy` a slice into this `Bump` and return an exclusive reference to
    /// the copy.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_copy(&[1, 2, 3]);
    /// assert_eq!(x, &[1, 2, 3]);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_copy<T>(&self, src: &[T]) -> &mut [T]
    where
        T: Copy,
    {
        let layout = Layout::for_value(src);
        let dst = self.alloc_layout(layout).cast::<T>();
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), dst.as_ptr(), src.len());
            slice::from_raw_parts_mut(dst.as_ptr(), src.len())
        }
    }
    /// `Clone` a slice into this `Bump` and return an exclusive reference to
    /// the clone. Prefer [`alloc_slice_copy`](#method.alloc_slice_copy) if `T` is `Copy`.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// #[derive(Clone, Debug, Eq, PartialEq)]
    /// struct Sheep {
    ///     name: String,
    /// }
    ///
    /// let originals = [
    ///     Sheep { name: "Alice".into() },
    ///     Sheep { name: "Bob".into() },
    ///     Sheep { name: "Cathy".into() },
    /// ];
    ///
    /// let bump = bumpalo::Bump::new();
    /// let clones = bump.alloc_slice_clone(&originals);
    /// assert_eq!(originals, clones);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_clone<T>(&self, src: &[T]) -> &mut [T]
    where
        T: Clone,
    {
        let layout = Layout::for_value(src);
        let dst = self.alloc_layout(layout).cast::<T>();
        unsafe {
            for (i, val) in src.iter().cloned().enumerate() {
                ptr::write(dst.as_ptr().add(i), val);
            }
            slice::from_raw_parts_mut(dst.as_ptr(), src.len())
        }
    }
    /// `Copy` a string slice into this `Bump` and return an exclusive reference to it.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the string fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let hello = bump.alloc_str("hello world");
    /// assert_eq!("hello world", hello);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_str(&self, src: &str) -> &mut str {
        let buffer = self.alloc_slice_copy(src.as_bytes());
        unsafe { str::from_utf8_unchecked_mut(buffer) }
    }
    /// Allocates a new slice of size `len` into this `Bump` and returns an
    /// exclusive reference to the copy.
    ///
    /// The elements of the slice are initialized using the supplied closure.
    /// The closure argument is the position in the slice.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_with(5, |i| 5 * (i + 1));
    /// assert_eq!(x, &[5, 10, 15, 20, 25]);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_fill_with<T, F>(&self, len: usize, mut f: F) -> &mut [T]
    where
        F: FnMut(usize) -> T,
    {
        let layout = Layout::array::<T>(len).unwrap_or_else(|_| oom());
        let dst = self.alloc_layout(layout).cast::<T>();
        unsafe {
            for i in 0..len {
                ptr::write(dst.as_ptr().add(i), f(i));
            }
            let result = slice::from_raw_parts_mut(dst.as_ptr(), len);
            debug_assert_eq!(Layout::for_value(result), layout);
            result
        }
    }
    /// Allocates a new slice of size `len` into this `Bump` and returns an
    /// exclusive reference to the copy.
    ///
    /// All elements of the slice are initialized to `value`.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_copy(5, 42);
    /// assert_eq!(x, &[42, 42, 42, 42, 42]);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_fill_copy<T: Copy>(&self, len: usize, value: T) -> &mut [T] {
        self.alloc_slice_fill_with(len, |_| value)
    }
    /// Allocates a new slice of size `len` slice into this `Bump` and return an
    /// exclusive reference to the copy.
    ///
    /// All elements of the slice are initialized to `value.clone()`.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let s: String = "Hello Bump!".to_string();
    /// let x: &[String] = bump.alloc_slice_fill_clone(2, &s);
    /// assert_eq!(x.len(), 2);
    /// assert_eq!(&x[0], &s);
    /// assert_eq!(&x[1], &s);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_fill_clone<T: Clone>(&self, len: usize, value: &T) -> &mut [T] {
        self.alloc_slice_fill_with(len, |_| value.clone())
    }
    /// Allocates a new slice of size `len` slice into this `Bump` and return an
    /// exclusive reference to the copy.
    ///
    /// The elements are initialized using the supplied iterator.
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails, or if the supplied
    /// iterator returns fewer elements than it promised.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x: &[i32] = bump.alloc_slice_fill_iter([2, 3, 5].iter().cloned().map(|i| i * i));
    /// assert_eq!(x, [4, 9, 25]);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_fill_iter<T, I>(&self, iter: I) -> &mut [T]
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut iter = iter.into_iter();
        self.alloc_slice_fill_with(
            iter.len(),
            |_| { iter.next().expect("Iterator supplied too few elements") },
        )
    }
    /// Allocates a new slice of size `len` slice into this `Bump` and return an
    /// exclusive reference to the copy.
    ///
    /// All elements of the slice are initialized to [`T::default()`].
    ///
    /// [`T::default()`]: https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default
    ///
    /// ## Panics
    ///
    /// Panics if reserving space for the slice fails.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let x = bump.alloc_slice_fill_default::<u32>(5);
    /// assert_eq!(x, &[0, 0, 0, 0, 0]);
    /// ```
    #[inline(always)]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc_slice_fill_default<T: Default>(&self, len: usize) -> &mut [T] {
        self.alloc_slice_fill_with(len, |_| T::default())
    }
    /// Allocate space for an object with the given `Layout`.
    ///
    /// The returned pointer points at uninitialized memory, and should be
    /// initialized with
    /// [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).
    ///
    /// # Panics
    ///
    /// Panics if reserving space matching `layout` fails.
    #[inline(always)]
    pub fn alloc_layout(&self, layout: Layout) -> NonNull<u8> {
        self.try_alloc_layout(layout).unwrap_or_else(|_| oom())
    }
    /// Attempts to allocate space for an object with the given `Layout` or else returns
    /// an `Err`.
    ///
    /// The returned pointer points at uninitialized memory, and should be
    /// initialized with
    /// [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).
    ///
    /// # Errors
    ///
    /// Errors if reserving space matching `layout` fails.
    #[inline(always)]
    pub fn try_alloc_layout(&self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        if let Some(p) = self.try_alloc_layout_fast(layout) {
            Ok(p)
        } else {
            self.alloc_layout_slow(layout).ok_or(AllocErr)
        }
    }
    #[inline(always)]
    fn try_alloc_layout_fast(&self, layout: Layout) -> Option<NonNull<u8>> {
        unsafe {
            let footer = self.current_chunk_footer.get();
            let footer = footer.as_ref();
            let ptr = footer.ptr.get().as_ptr();
            let start = footer.data.as_ptr();
            debug_assert!(start <= ptr);
            debug_assert!(ptr as * const u8 <= footer as * const _ as * const u8);
            if (ptr as usize) < layout.size() {
                return None;
            }
            let ptr = ptr.wrapping_sub(layout.size());
            let rem = ptr as usize % layout.align();
            let aligned_ptr = ptr.wrapping_sub(rem);
            if aligned_ptr >= start {
                let aligned_ptr = NonNull::new_unchecked(aligned_ptr as *mut u8);
                footer.ptr.set(aligned_ptr);
                Some(aligned_ptr)
            } else {
                None
            }
        }
    }
    /// Gets the remaining capacity in the current chunk (in bytes).
    ///
    /// ## Example
    ///
    /// ```
    /// use bumpalo::Bump;
    ///
    /// let bump = Bump::with_capacity(100);
    ///
    /// let capacity = bump.chunk_capacity();
    /// assert!(capacity >= 100);
    /// ```
    pub fn chunk_capacity(&self) -> usize {
        let current_footer = self.current_chunk_footer.get();
        let current_footer = unsafe { current_footer.as_ref() };
        current_footer as *const _ as usize - current_footer.data.as_ptr() as usize
    }
    /// Slow path allocation for when we need to allocate a new chunk from the
    /// parent bump set because there isn't enough room in our current chunk.
    #[inline(never)]
    fn alloc_layout_slow(&self, layout: Layout) -> Option<NonNull<u8>> {
        unsafe {
            let size = layout.size();
            let allocation_limit_remaining = self.allocation_limit_remaining();
            let current_footer = self.current_chunk_footer.get();
            let current_layout = current_footer.as_ref().layout;
            let min_new_chunk_size = layout
                .size()
                .max(DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER);
            let mut base_size = (current_layout.size() - FOOTER_SIZE)
                .checked_mul(2)?
                .max(min_new_chunk_size);
            let chunk_memory_details = iter::from_fn(|| {
                let bypass_min_chunk_size_for_small_limits = match self
                    .allocation_limit()
                {
                    Some(
                        limit,
                    ) if layout.size() < limit && base_size >= layout.size()
                        && limit < DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER
                        && self.allocated_bytes() == 0 => true,
                    _ => false,
                };
                if base_size >= min_new_chunk_size
                    || bypass_min_chunk_size_for_small_limits
                {
                    let size = base_size;
                    base_size = base_size / 2;
                    Bump::new_chunk_memory_details(Some(size), layout)
                } else {
                    None
                }
            });
            let new_footer = chunk_memory_details
                .filter_map(|chunk_memory_details| {
                    if Bump::chunk_fits_under_limit(
                        allocation_limit_remaining,
                        chunk_memory_details,
                    ) {
                        Bump::new_chunk(chunk_memory_details, layout, current_footer)
                    } else {
                        None
                    }
                })
                .next()?;
            debug_assert_eq!(
                new_footer.as_ref().data.as_ptr() as usize % layout.align(), 0
            );
            self.current_chunk_footer.set(new_footer);
            let new_footer = new_footer.as_ref();
            let mut ptr = new_footer.ptr.get().as_ptr().sub(size);
            ptr = ptr.sub(ptr as usize % layout.align());
            debug_assert!(
                ptr as * const _ <= new_footer, "{:p} <= {:p}", ptr, new_footer
            );
            let ptr = NonNull::new_unchecked(ptr as *mut u8);
            new_footer.ptr.set(ptr);
            Some(ptr)
        }
    }
    /// Returns an iterator over each chunk of allocated memory that
    /// this arena has bump allocated into.
    ///
    /// The chunks are returned ordered by allocation time, with the most
    /// recently allocated chunk being returned first, and the least recently
    /// allocated chunk being returned last.
    ///
    /// The values inside each chunk are also ordered by allocation time, with
    /// the most recent allocation being earlier in the slice, and the least
    /// recent allocation being towards the end of the slice.
    ///
    /// ## Safety
    ///
    /// Because this method takes `&mut self`, we know that the bump arena
    /// reference is unique and therefore there aren't any active references to
    /// any of the objects we've allocated in it either. This potential aliasing
    /// of exclusive references is one common footgun for unsafe code that we
    /// don't need to worry about here.
    ///
    /// However, there could be regions of uninitialized memory used as padding
    /// between allocations, which is why this iterator has items of type
    /// `[MaybeUninit<u8>]`, instead of simply `[u8]`.
    ///
    /// The only way to guarantee that there is no padding between allocations
    /// or within allocated objects is if all of these properties hold:
    ///
    /// 1. Every object allocated in this arena has the same alignment,
    ///    and that alignment is at most 16.
    /// 2. Every object's size is a multiple of its alignment.
    /// 3. None of the objects allocated in this arena contain any internal
    ///    padding.
    ///
    /// If you want to use this `iter_allocated_chunks` method, it is *your*
    /// responsibility to ensure that these properties hold before calling
    /// `MaybeUninit::assume_init` or otherwise reading the returned values.
    ///
    /// Finally, you must also ensure that any values allocated into the bump
    /// arena have not had their `Drop` implementations called on them,
    /// e.g. after dropping a [`bumpalo::boxed::Box<T>`][crate::boxed::Box].
    ///
    /// ## Example
    ///
    /// ```
    /// let mut bump = bumpalo::Bump::new();
    ///
    /// // Allocate a bunch of `i32`s in this bump arena, potentially causing
    /// // additional memory chunks to be reserved.
    /// for i in 0..10000 {
    ///     bump.alloc(i);
    /// }
    ///
    /// // Iterate over each chunk we've bump allocated into. This is safe
    /// // because we have only allocated `i32`s in this arena, which fulfills
    /// // the above requirements.
    /// for ch in bump.iter_allocated_chunks() {
    ///     println!("Used a chunk that is {} bytes long", ch.len());
    ///     println!("The first byte is {:?}", unsafe {
    ///         ch[0].assume_init()
    ///     });
    /// }
    ///
    /// // Within a chunk, allocations are ordered from most recent to least
    /// // recent. If we allocated 'a', then 'b', then 'c', when we iterate
    /// // through the chunk's data, we get them in the order 'c', then 'b',
    /// // then 'a'.
    ///
    /// bump.reset();
    /// bump.alloc(b'a');
    /// bump.alloc(b'b');
    /// bump.alloc(b'c');
    ///
    /// assert_eq!(bump.iter_allocated_chunks().count(), 1);
    /// let chunk = bump.iter_allocated_chunks().nth(0).unwrap();
    /// assert_eq!(chunk.len(), 3);
    ///
    /// // Safe because we've only allocated `u8`s in this arena, which
    /// // fulfills the above requirements.
    /// unsafe {
    ///     assert_eq!(chunk[0].assume_init(), b'c');
    ///     assert_eq!(chunk[1].assume_init(), b'b');
    ///     assert_eq!(chunk[2].assume_init(), b'a');
    /// }
    /// ```
    pub fn iter_allocated_chunks(&mut self) -> ChunkIter<'_> {
        let raw = unsafe { self.iter_allocated_chunks_raw() };
        ChunkIter {
            raw,
            bump: PhantomData,
        }
    }
    /// Returns an iterator over raw pointers to chunks of allocated memory that
    /// this arena has bump allocated into.
    ///
    /// This is an unsafe version of [`iter_allocated_chunks()`](Bump::iter_allocated_chunks),
    /// with the caller responsible for safe usage of the returned pointers as
    /// well as ensuring that the iterator is not invalidated by new
    /// allocations.
    ///
    /// ## Safety
    ///
    /// Allocations from this arena must not be performed while the returned
    /// iterator is alive. If reading the chunk data (or casting to a reference)
    /// the caller must ensure that there exist no mutable references to
    /// previously allocated data.
    ///
    /// In addition, all of the caveats when reading the chunk data from
    /// [`iter_allocated_chunks()`](Bump::iter_allocated_chunks) still apply.
    pub unsafe fn iter_allocated_chunks_raw(&self) -> ChunkRawIter<'_> {
        ChunkRawIter {
            footer: self.current_chunk_footer.get(),
            bump: PhantomData,
        }
    }
    /// Calculates the number of bytes currently allocated across all chunks in
    /// this bump arena.
    ///
    /// If you allocate types of different alignments or types with
    /// larger-than-typical alignment in the same arena, some padding
    /// bytes might get allocated in the bump arena. Note that those padding
    /// bytes will add to this method's resulting sum, so you cannot rely
    /// on it only counting the sum of the sizes of the things
    /// you've allocated in the arena.
    ///
    /// ## Example
    ///
    /// ```
    /// let bump = bumpalo::Bump::new();
    /// let _x = bump.alloc_slice_fill_default::<u32>(5);
    /// let bytes = bump.allocated_bytes();
    /// assert!(bytes >= core::mem::size_of::<u32>() * 5);
    /// ```
    pub fn allocated_bytes(&self) -> usize {
        let footer = self.current_chunk_footer.get();
        unsafe { footer.as_ref().allocated_bytes }
    }
    #[inline]
    unsafe fn is_last_allocation(&self, ptr: NonNull<u8>) -> bool {
        let footer = self.current_chunk_footer.get();
        let footer = footer.as_ref();
        footer.ptr.get() == ptr
    }
    #[inline]
    unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
        if self.is_last_allocation(ptr) {
            let ptr = NonNull::new_unchecked(ptr.as_ptr().add(layout.size()));
            self.current_chunk_footer.get().as_ref().ptr.set(ptr);
        }
    }
    #[inline]
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, AllocErr> {
        let old_size = old_layout.size();
        let new_size = new_layout.size();
        let align_is_compatible = old_layout.align() >= new_layout.align();
        if !align_is_compatible {
            return Err(AllocErr);
        }
        let delta = round_down_to(old_size - new_size, new_layout.align());
        if self.is_last_allocation(ptr) && delta >= old_size / 2 {
            let footer = self.current_chunk_footer.get();
            let footer = footer.as_ref();
            let new_ptr = NonNull::new_unchecked(footer.ptr.get().as_ptr().add(delta));
            footer.ptr.set(new_ptr);
            ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), new_size);
            return Ok(new_ptr);
        } else {
            return Ok(ptr);
        }
    }
    #[inline]
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<u8>, AllocErr> {
        let old_size = old_layout.size();
        let new_size = new_layout.size();
        let align_is_compatible = old_layout.align() >= new_layout.align();
        if align_is_compatible && self.is_last_allocation(ptr) {
            let delta = new_size - old_size;
            if let Some(p)
                = self
                    .try_alloc_layout_fast(
                        layout_from_size_align(delta, old_layout.align()),
                    )
            {
                ptr::copy(ptr.as_ptr(), p.as_ptr(), old_size);
                return Ok(p);
            }
        }
        let new_ptr = self.try_alloc_layout(new_layout)?;
        ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), old_size);
        Ok(new_ptr)
    }
}
/// An iterator over each chunk of allocated memory that
/// an arena has bump allocated into.
///
/// The chunks are returned ordered by allocation time, with the most recently
/// allocated chunk being returned first.
///
/// The values inside each chunk are also ordered by allocation time, with the most
/// recent allocation being earlier in the slice.
///
/// This struct is created by the [`iter_allocated_chunks`] method on
/// [`Bump`]. See that function for a safety description regarding reading from the returned items.
///
/// [`Bump`]: struct.Bump.html
/// [`iter_allocated_chunks`]: struct.Bump.html#method.iter_allocated_chunks
#[derive(Debug)]
pub struct ChunkIter<'a> {
    raw: ChunkRawIter<'a>,
    bump: PhantomData<&'a mut Bump>,
}
impl<'a> Iterator for ChunkIter<'a> {
    type Item = &'a [mem::MaybeUninit<u8>];
    fn next(&mut self) -> Option<&'a [mem::MaybeUninit<u8>]> {
        unsafe {
            let (ptr, len) = self.raw.next()?;
            let slice = slice::from_raw_parts(ptr as *const mem::MaybeUninit<u8>, len);
            Some(slice)
        }
    }
}
impl<'a> iter::FusedIterator for ChunkIter<'a> {}
/// An iterator over raw pointers to chunks of allocated memory that this
/// arena has bump allocated into.
///
/// See [`ChunkIter`] for details regarding the returned chunks.
///
/// This struct is created by the [`iter_allocated_chunks_raw`] method on
/// [`Bump`]. See that function for a safety description regarding reading from
/// the returned items.
///
/// [`Bump`]: struct.Bump.html
/// [`iter_allocated_chunks_raw`]: struct.Bump.html#method.iter_allocated_chunks_raw
#[derive(Debug)]
pub struct ChunkRawIter<'a> {
    footer: NonNull<ChunkFooter>,
    bump: PhantomData<&'a Bump>,
}
impl Iterator for ChunkRawIter<'_> {
    type Item = (*mut u8, usize);
    fn next(&mut self) -> Option<(*mut u8, usize)> {
        unsafe {
            let foot = self.footer.as_ref();
            if foot.is_empty() {
                return None;
            }
            let (ptr, len) = foot.as_raw_parts();
            self.footer = foot.prev.get();
            Some((ptr as *mut u8, len))
        }
    }
}
impl iter::FusedIterator for ChunkRawIter<'_> {}
#[inline(never)]
#[cold]
fn oom() -> ! {
    panic!("out of memory")
}
unsafe impl<'a> alloc::Alloc for &'a Bump {
    #[inline(always)]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        self.try_alloc_layout(layout)
    }
    #[inline]
    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        Bump::dealloc(self, ptr, layout)
    }
    #[inline]
    unsafe fn realloc(
        &mut self,
        ptr: NonNull<u8>,
        layout: Layout,
        new_size: usize,
    ) -> Result<NonNull<u8>, AllocErr> {
        let old_size = layout.size();
        if old_size == 0 {
            return self.try_alloc_layout(layout);
        }
        let new_layout = layout_from_size_align(new_size, layout.align());
        if new_size <= old_size {
            self.shrink(ptr, layout, new_layout)
        } else {
            self.grow(ptr, layout, new_layout)
        }
    }
}
#[cfg(feature = "allocator_api")]
unsafe impl<'a> Allocator for &'a Bump {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.try_alloc_layout(layout)
            .map(|p| NonNull::slice_from_raw_parts(p, layout.size()))
            .map_err(|_| AllocError)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        Bump::dealloc(self, ptr, layout)
    }
    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Bump::shrink(self, ptr, old_layout, new_layout)
            .map(|p| NonNull::slice_from_raw_parts(p, new_layout.size()))
            .map_err(|_| AllocError)
    }
    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        Bump::grow(self, ptr, old_layout, new_layout)
            .map(|p| NonNull::slice_from_raw_parts(p, new_layout.size()))
            .map_err(|_| AllocError)
    }
    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let mut ptr = self.grow(ptr, old_layout, new_layout)?;
        ptr.as_mut()[old_layout.size()..].fill(0);
        Ok(ptr)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chunk_footer_is_five_words() {
        assert_eq!(mem::size_of::< ChunkFooter > (), mem::size_of::< usize > () * 6);
    }
    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_realloc() {
        use crate::alloc::Alloc;
        unsafe {
            const CAPACITY: usize = 1024 - OVERHEAD;
            let mut b = Bump::with_capacity(CAPACITY);
            let layout = Layout::from_size_align(100, 1).unwrap();
            let p = b.alloc_layout(layout);
            let q = (&b).realloc(p, layout, 51).unwrap();
            assert_eq!(p, q);
            b.reset();
            let layout = Layout::from_size_align(100, 1).unwrap();
            let p = b.alloc_layout(layout);
            let q = (&b).realloc(p, layout, 50).unwrap();
            assert!(p != q);
            b.reset();
            let layout = Layout::from_size_align(10, 1).unwrap();
            let p = b.alloc_layout(layout);
            let q = (&b).realloc(p, layout, 11).unwrap();
            assert_eq!(q.as_ptr() as usize, p.as_ptr() as usize - 1);
            b.reset();
            let layout = Layout::from_size_align(1, 1).unwrap();
            let p = b.alloc_layout(layout);
            let q = (&b).realloc(p, layout, CAPACITY + 1).unwrap();
            assert!(q.as_ptr() as usize != p.as_ptr() as usize - CAPACITY);
            b = Bump::with_capacity(CAPACITY);
            let layout = Layout::from_size_align(1, 1).unwrap();
            let p = b.alloc_layout(layout);
            let _ = b.alloc_layout(layout);
            let q = (&b).realloc(p, layout, 2).unwrap();
            assert!(q.as_ptr() as usize != p.as_ptr() as usize - 1);
            b.reset();
        }
    }
    #[test]
    fn invalid_read() {
        use alloc::Alloc;
        let mut b = &Bump::new();
        unsafe {
            let l1 = Layout::from_size_align(12000, 4).unwrap();
            let p1 = Alloc::alloc(&mut b, l1).unwrap();
            let l2 = Layout::from_size_align(1000, 4).unwrap();
            Alloc::alloc(&mut b, l2).unwrap();
            let p1 = b.realloc(p1, l1, 24000).unwrap();
            let l3 = Layout::from_size_align(24000, 4).unwrap();
            b.realloc(p1, l3, 48000).unwrap();
        }
    }
}
#[cfg(test)]
mod tests_rug_21 {
    use super::*;
    #[test]
    fn test_round_up_to() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(round_up_to(p0, p1), Some(16));
             }
});    }
}
#[cfg(test)]
mod tests_rug_22 {
    use super::*;
    #[test]
    fn test_round_down_to() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        debug_assert_eq!(round_down_to(p0, p1), 16);
             }
});    }
}
#[cfg(test)]
mod tests_rug_24 {
    use super::*;
    use crate::allocation_size_overflow;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_24_rrrruuuugggg_test_rug = 0;
        allocation_size_overflow::<usize>();
        let _rug_ed_tests_rug_24_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_25 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: usize = rug_fuzz_0;
        let mut p1: usize = rug_fuzz_1;
        crate::abs_diff(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_28 {
    use super::oom;
    #[test]
    fn test_oom() {
        let _rug_st_tests_rug_28_rrrruuuugggg_test_oom = 0;
        oom();
        let _rug_ed_tests_rug_28_rrrruuuugggg_test_oom = 0;
    }
}
#[cfg(test)]
mod tests_rug_33 {
    use super::*;
    use crate::Bump;
    use core::default::Default;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_33_rrrruuuugggg_test_rug = 0;
        let _bump_default: Bump = <Bump as Default>::default();
        let _rug_ed_tests_rug_33_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_35 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_35_rrrruuuugggg_test_rug = 0;
        let bump: Bump = Bump::new();
        let _rug_ed_tests_rug_35_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_36 {
    use super::*;
    use crate::{Bump, AllocErr};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_36_rrrruuuugggg_test_rug = 0;
        let result = Bump::try_new();
        debug_assert!(result.is_ok());
        let _rug_ed_tests_rug_36_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_37 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_with_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        Bump::with_capacity(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_38 {
    use super::*;
    use crate::{Bump, layout_from_size_align, AllocErr, EMPTY_CHUNK};
    #[test]
    fn test_try_with_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let p0: usize = rug_fuzz_0;
        let _ = Bump::try_with_capacity(p0);
             }
});    }
}
#[cfg(test)]
mod tests_rug_39 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_allocation_limit() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bump = Bump::with_capacity(rug_fuzz_0);
        debug_assert_eq!(bump.allocation_limit(), None);
        bump.set_allocation_limit(Some(rug_fuzz_1));
        debug_assert_eq!(bump.allocation_limit(), Some(6));
        bump.set_allocation_limit(None);
        debug_assert_eq!(bump.allocation_limit(), None);
             }
});    }
}
#[cfg(test)]
mod tests_rug_40 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1 = Some(rug_fuzz_0);
        p0.set_allocation_limit(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_41 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_allocation_limit_remaining() {
        let _rug_st_tests_rug_41_rrrruuuugggg_test_allocation_limit_remaining = 0;
        let mut p0 = Bump::default();
        p0.allocation_limit_remaining();
        let _rug_ed_tests_rug_41_rrrruuuugggg_test_allocation_limit_remaining = 0;
    }
}
#[cfg(test)]
mod tests_rug_42 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: Option<usize> = Some(rug_fuzz_0);
        let mut p1: NewChunkMemoryDetails = NewChunkMemoryDetails {
            new_size_without_footer: rug_fuzz_1,
            align: rug_fuzz_2,
            size: rug_fuzz_3,
        };
        debug_assert_eq!(Bump::chunk_fits_under_limit(p0, p1), true);
             }
});    }
}
#[cfg(test)]
mod tests_rug_43 {
    use super::*;
    use crate::Bump;
    use crate::allocation_size_overflow;
    use crate::round_up_to;
    use crate::NewChunkMemoryDetails;
    use crate::DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER;
    use crate::CHUNK_ALIGN;
    use crate::OVERHEAD;
    use crate::PAGE_STRATEGY_CUTOFF;
    use crate::FOOTER_SIZE;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3)) = <(usize, usize, usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0: core::option::Option<usize> = core::option::Option::Some(rug_fuzz_0);
        let mut p1: core::alloc::Layout = core::alloc::Layout::from_size_align(
                rug_fuzz_1,
                rug_fuzz_2,
            )
            .expect(rug_fuzz_3);
        Bump::new_chunk_memory_details(p0, p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_44 {
    use super::*;
    use crate::{Bump, ChunkFooter, NewChunkMemoryDetails, layout_from_size_align};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2, mut rug_fuzz_3, mut rug_fuzz_4)) = <(usize, usize, usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let new_chunk_memory_details = NewChunkMemoryDetails {
            new_size_without_footer: rug_fuzz_0,
            align: rug_fuzz_1,
            size: rug_fuzz_2,
        };
        let requested_layout = core::alloc::Layout::from_size_align(
                rug_fuzz_3,
                rug_fuzz_4,
            )
            .unwrap();
        let prev = core::ptr::NonNull::<ChunkFooter>::dangling();
        unsafe {
            Bump::new_chunk(new_chunk_memory_details, requested_layout, prev);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_45 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_reset() {
        let _rug_st_tests_rug_45_rrrruuuugggg_test_reset = 0;
        let mut bump = Bump::new();
        Bump::reset(&mut bump);
        let _rug_ed_tests_rug_45_rrrruuuugggg_test_reset = 0;
    }
}
#[cfg(test)]
mod tests_rug_46 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1: &str = rug_fuzz_0;
        p0.alloc(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_47 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1: &str = rug_fuzz_0;
        let result = p0.try_alloc(p1);
        debug_assert_eq!(result, Ok(& mut "hello"));
             }
});    }
}
#[cfg(test)]
mod tests_rug_48 {
    use super::*;
    use crate::{Bump, Layout};
    use core::ptr;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bump = Bump::new();
        let mut p0 = bump;
        let f_impl = || rug_fuzz_0;
        p0.alloc_with(f_impl);
             }
});    }
}
#[cfg(test)]
mod tests_rug_49 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_try_alloc_with() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bump = Bump::new();
        let mut p0 = bump;
        let mut p1 = || rug_fuzz_0;
        let result = p0.try_alloc_with(p1);
        debug_assert_eq!(result, Ok(& mut "hello"));
             }
});    }
}
#[cfg(test)]
mod tests_rug_50 {
    use super::*;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1 = || Ok(rug_fuzz_0);
        Bump::alloc_try_with::<_, _, ()>(&p0, p1).unwrap();
             }
});    }
}
#[cfg(test)]
mod tests_rug_51 {
    use super::*;
    use crate::{Bump, AllocOrInitError};
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_51_rrrruuuugggg_test_rug = 0;
        let rug_fuzz_0 = "hello";
        let mut p0 = Bump::default();
        let p1 = || -> Result<&'static str, &'static str> { Ok(rug_fuzz_0) };
        <Bump>::try_alloc_try_with(&p0, p1).unwrap();
        let _rug_ed_tests_rug_51_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_52 {
    use super::*;
    use crate::Bump;
    use core::alloc::Layout;
    use core::ptr;
    use core::slice;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::default();
        let mut p1: &[i32] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        let result = p0.alloc_slice_copy(p1);
        debug_assert_eq!(result, & [1, 2, 3]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_53 {
    use super::*;
    use crate::{Bump, Layout, ptr, slice};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(i32, i32, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1: &[i32] = &[rug_fuzz_0, rug_fuzz_1, rug_fuzz_2];
        p0.alloc_slice_clone(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_54 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_alloc_str() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(&str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bump = Bump::new();
        let src = rug_fuzz_0;
        let result = bump.alloc_str(src);
        debug_assert_eq!(src, result);
             }
});    }
}
#[cfg(test)]
mod tests_rug_55 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1, mut rug_fuzz_2)) = <(usize, usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let bump = Bump::new();
        let len = rug_fuzz_0;
        let f = |i| rug_fuzz_1 * (i + rug_fuzz_2);
        bump.alloc_slice_fill_with(len, f);
             }
});    }
}
#[cfg(test)]
mod tests_rug_56 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_alloc_slice_fill_copy() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, i32) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bump = Bump::new();
        let mut len = rug_fuzz_0;
        let mut value = rug_fuzz_1;
        let result = bump.alloc_slice_fill_copy(len, value);
        debug_assert_eq!(result, & [42, 42, 42, 42, 42]);
             }
});    }
}
#[cfg(test)]
mod tests_rug_57 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, &str) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::new();
        let mut p1: usize = rug_fuzz_0;
        let mut p2: &str = rug_fuzz_1;
        p0.alloc_slice_fill_clone(p1, &p2);
             }
});    }
}
#[cfg(test)]
mod tests_rug_58 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let mut p0 = Bump::new();
        let p1 = [2, 3, 5].iter().cloned().map(|i| i * i);
        p0.alloc_slice_fill_iter(p1);
    }
}
#[cfg(test)]
mod tests_rug_59 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::default();
        let mut p1: usize = rug_fuzz_0;
        p0.alloc_slice_fill_default::<u32>(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_60 {
    use super::*;
    use crate::{Bump, alloc::Layout};
    #[test]
    fn test_alloc_layout() {
        let _rug_st_tests_rug_60_rrrruuuugggg_test_alloc_layout = 0;
        let bump = Bump::new();
        let layout = Layout::new::<u32>();
        bump.alloc_layout(layout);
        let _rug_ed_tests_rug_60_rrrruuuugggg_test_alloc_layout = 0;
    }
}
#[cfg(test)]
mod tests_rug_61 {
    use super::*;
    use crate::Bump;
    use core::alloc::Layout;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_61_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::default();
        let p1 = Layout::new::<u32>();
        Bump::try_alloc_layout(&p0, p1);
        let _rug_ed_tests_rug_61_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_62 {
    use super::*;
    use crate::Bump;
    use core::alloc::Layout;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_62_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::default();
        let p1 = Layout::new::<u32>();
        crate::Bump::try_alloc_layout_fast(&p0, p1);
        let _rug_ed_tests_rug_62_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_63 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_chunk_capacity() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0)) = <(usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::with_capacity(rug_fuzz_0);
        p0.chunk_capacity();
             }
});    }
}
#[cfg(test)]
mod tests_rug_64 {
    use super::*;
    use crate::{Bump, DEFAULT_CHUNK_SIZE_WITHOUT_FOOTER, FOOTER_SIZE};
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut p0 = Bump::default();
        let p1 = core::alloc::Layout::from_size_align(rug_fuzz_0, rug_fuzz_1).unwrap();
        p0.alloc_layout_slow(p1);
             }
});    }
}
#[cfg(test)]
mod tests_rug_65 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_65_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::new();
        Bump::iter_allocated_chunks(&mut p0);
        let _rug_ed_tests_rug_65_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_66 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_66_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::new();
        unsafe { p0.iter_allocated_chunks_raw() };
        let _rug_ed_tests_rug_66_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_67 {
    use super::*;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_67_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::new();
        p0.allocated_bytes();
        let _rug_ed_tests_rug_67_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_68 {
    use super::*;
    use core::ptr::NonNull;
    use crate::Bump;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_68_rrrruuuugggg_test_rug = 0;
        let mut p0 = Bump::default();
        let mut p1 = NonNull::<u8>::dangling();
        unsafe { p0.is_last_allocation(p1) };
        let _rug_ed_tests_rug_68_rrrruuuugggg_test_rug = 0;
    }
}
#[cfg(test)]
mod tests_rug_69 {
    use super::*;
    use crate::Bump;
    use core::ptr::NonNull;
    use core::alloc::Layout;
    #[test]
    fn test_rug() {

    extern crate bolero;
    extern crate arbitrary;
    bolero::check!()
        .for_each(|rug_data| {
            if let Ok((mut rug_fuzz_0, mut rug_fuzz_1)) = <(usize, usize) as arbitrary::Arbitrary>::arbitrary(&mut arbitrary::Unstructured::new(rug_data)){

        let mut bump = Bump::default();
        let layout = Layout::from_size_align(rug_fuzz_0, rug_fuzz_1).unwrap();
        let ptr = NonNull::<u8>::dangling();
        unsafe {
            bump.dealloc(ptr, layout);
        }
             }
});    }
}
#[cfg(test)]
mod tests_rug_74 {
    use super::*;
    use crate::alloc::Alloc;
    use crate::Bump;
    use core::alloc::Layout;
    #[test]
    fn test_rug() {
        let _rug_st_tests_rug_74_rrrruuuugggg_test_rug = 0;
        let mut p0: Bump = Bump::default();
        let p1: Layout = Layout::new::<u32>();
        p0.alloc(p1);
        let _rug_ed_tests_rug_74_rrrruuuugggg_test_rug = 0;
    }
}
