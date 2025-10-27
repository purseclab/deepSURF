//! This module contains shared data structures and other functionality for use with the allocators
//! implemented in this crate.
use std::alloc::{alloc, dealloc, Layout};
use std::cell::Cell;
use std::cmp;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
use std::slice;

#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};

/// An error type representing errors possible during arena creation or other arena operations.
#[derive(Debug)]
pub enum ArenaError {
    /// The backing storage for the arena could not be allocated.
    AllocationFailed,
    /// If an arena is locked by some token type, it refuses locking when already locked.
    AlreadyLocked,
    /// The arena is blocked from clearing by objects that are still live.
    CannotClear,
}

/// The kind of backing requested for an arena.
#[derive(Debug)]
pub enum ArenaBacking {
    /// Create a virtual memory mapping via `mmap()` or `VirtualAlloc()`.
    MemoryMap,
    /// Ask the system allocator for the memory.
    SystemAllocation,
}

/// Every arena-allocated object has some form of handle to the arena containing it.
///
/// Depending on the type of arena, the actual functionality of the handle can be different,
/// but it at least allows for allocation and object resizing.
///
/// To be useful, handles need to implement `Clone`.
pub trait AllocHandle {
    /// Allocate memory from the arena.
    ///
    /// Allocate `count` objects of type `T` from the arena, and panic if this is not possible.
    fn allocate<T>(&self, count: usize) -> NonNull<T>;
    /// Reallocate memory in the arena.
    ///
    /// Resize the object sequence pointed to by `ptr` of `old_count` elements of type `T` to
    /// `count` objects, and copy the sequence if no resizing in place is possible.
    ///
    /// `ptr` must point into the arena.
    fn allocate_or_extend<T>(&self, ptr: NonNull<T>, old_count: usize, count: usize) -> NonNull<T>;
}

/// An arena allocated, fixed-size sequence of objects.
pub struct Slice<T, H> {
    ptr: NonNull<T>,
    len: usize,
    handle: H,
}

/// An arena allocated, sequential, resizable vector
///
/// Since the arena does not support resizing, or freeing memory, this implementation just
/// creates new slices as necessary and leaks the previous arena allocation, trading memory
/// for speed.
pub struct SliceVec<T, H> {
    slice: Slice<T, H>,
    // owo what's this
    capacity: usize,
}

impl<T, H: AllocHandle> Slice<T, H> {
    /// Create a new slice of default-initialized objects using the provided handle.
    pub fn new(handle: H, len: usize) -> Self
    where
        T: Default,
    {
        let mut res = unsafe { Self::new_empty(handle, len) };
        res.len = len;

        for i in 0..len {
            unsafe {
                ptr::write(res.ptr.as_ptr().add(i), T::default());
            }
        }

        res
    }

    /// Create a new slice of size `real_len`, but initialize length to `0`.
    unsafe fn new_empty(handle: H, real_len: usize) -> Self {
        let ptr: NonNull<T> = if real_len == 0 {
            NonNull::dangling()
        } else {
            handle.allocate(real_len)
        };

        Slice {
            ptr,
            len: 0,
            handle,
        }
    }
}

impl<T: Clone, H: AllocHandle + Clone> Clone for Slice<T, H> {
    fn clone(&self) -> Self {
        let ptr: NonNull<T> = self.handle.allocate(self.len);

        for i in 0..self.len {
            unsafe {
                ptr::write(ptr.as_ptr().add(i), (*self.ptr.as_ptr().add(i)).clone());
            }
        }

        Slice {
            ptr,
            len: self.len,
            handle: self.handle.clone(),
        }
    }
}

impl<T: fmt::Debug, H> fmt::Debug for Slice<T, H> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}

impl<T, H> Deref for Slice<T, H> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T, H> DerefMut for Slice<T, H> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T: Eq, H> Eq for Slice<T, H> {}

impl<T: PartialEq, H> PartialEq for Slice<T, H> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T: PartialOrd, H> PartialOrd for Slice<T, H> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<'a, T, H> IntoIterator for &'a Slice<T, H> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}

impl<'a, T, H> IntoIterator for &'a mut Slice<T, H> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}

#[cfg(feature = "serde")]
impl<T, H> Serialize for Slice<T, H>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

/* #[cfg(feature = "serde")]
impl<'de, T, H> Deserialize<'de> for Slice<T, H>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut res: Vec<T> = Deserialize::deserialize(deserializer)?;
        let mut slice = Slice::new(res.len());

        unsafe {
            let ptr = res.as_mut_ptr();
            ptr::copy_nonoverlapping(slice.ptr, ptr, slice.len);
            dealloc(ptr);
        }

        mem::forget(res);

        slice
    }
} */

impl<T, H> Drop for Slice<T, H> {
    fn drop(&mut self) {
        unsafe {
            ptr::drop_in_place(&mut self[..]);
        }
    }
}

impl<T, H> SliceVec<T, H> {
    /// Create an immutable iterator over the elements of the vector.
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
        self.slice.iter()
    }

    /// Create an mutable iterator over the elements of the vector.
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
        self.slice.iter_mut()
    }
}

impl<T, H: AllocHandle> SliceVec<T, H> {
    /// Create a new empty vector of capacity `0` using the given handle.
    pub fn new(handle: H) -> Self {
        Self::with_capacity(handle, 0)
    }

    /// Create a new vector of given capacity using the given handle.
    pub fn with_capacity(handle: H, capacity: usize) -> Self {
        SliceVec {
            slice: unsafe { Slice::new_empty(handle, capacity) },
            capacity,
        }
    }

    /// Return the current capacity of the vector.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Reseve enough space in the vector for at least `size` additional elements.
    pub fn reserve(&mut self, additional: usize) {
        let ptr = self.slice.ptr;
        let size = self.slice.len + additional;

        if self.capacity >= size {
            return;
        }

        let mut new_capacity = if self.capacity > 0 { self.capacity } else { 4 };

        while new_capacity < size {
            new_capacity *= 2;
        }

        let new_ptr: NonNull<T> = self.slice.handle.allocate_or_extend(ptr, self.capacity, new_capacity);

        if ptr != new_ptr {
            unsafe {
                ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr(), self.slice.len());
            }

            self.slice.ptr = new_ptr;
        }

        self.capacity = new_capacity;
    }

    // TODO: shrink_to_fit

    // TODO: shrink_to

    /// Shorten the vector, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no effect.
    pub fn truncate(&mut self, len: usize) {
        let old_len = self.slice.len;

        if len < old_len {
            unsafe {
                ptr::drop_in_place(&mut self.slice[len..old_len]);
            }

            self.slice.len = len;
        }
    }

    /// Remove an element from the vector and return it.
    ///
    /// The removed element is replaced by the last element of the vector.
    /// This does not preserve ordering, but is O(1).
    pub fn swap_remove(&mut self, index: usize) -> T {
        let hole: *mut T = &mut self[index];
        self.slice.len -= 1;

        unsafe {
            let last = ptr::read(self.slice.ptr.as_ptr().add(self.slice.len));
            ptr::replace(hole, last)
        }
    }

    // TODO: insert

    // TODO: remove

    // TODO: retain

    // TODO: dedup_by_key

    // TODO: dedup_by

    /// Push an element into the vector.
    pub fn push(&mut self, elem: T) {
        if self.slice.len == self.capacity {
            let new_capacity = if self.capacity == 0 {
                4
            } else {
                self.capacity * 2
            };

            self.reserve(new_capacity - self.capacity);
        }

        unsafe {
            ptr::write(self.slice.ptr.as_ptr().add(self.slice.len()), elem);
        }

        self.slice.len = self.slice.len() + 1;
    }

    /// Remove the last element from the vector and return it, or `None` if the vector is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            self.slice.len -= 1;
            Some(ptr::read(self.slice.ptr.as_ptr().add(self.slice.len)))
        }
    }

    /// Move all elements of `other` into `self`, leaving `other` empty.
    pub fn append(&mut self, other: &mut Self) {
        let count = other.len();
        self.reserve(count);
        let len = self.len();

        unsafe {
            ptr::copy_nonoverlapping(
                other.slice.ptr.as_ptr(),
                self.slice.ptr.as_ptr().add(len),
                count);
        }

        other.slice.len = 0;
    }

    // TODO: drain

    /// Clear the vector.
    pub fn clear(&mut self) {
        unsafe {
            ptr::drop_in_place(&mut self.slice[..]);
        }

        self.slice.len = 0;
    }

    /// Return the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.slice.len
    }

    /// Return `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.slice.len == 0
    }

    /// Splits the vector into two at the given index.
    ///
    /// Retruns a newly allocated `Self`. `self` contains elements `[0, at)`, and the returned
    /// `Self` contains elements `[at, len)`.
    ///
    /// The capacity of `self` remains unchanged.
    pub fn split_off(&mut self, at: usize) -> Self
    where
        H: Clone,
    {
        let mut ret = Self::with_capacity(self.slice.handle.clone(), self.slice.len - at);
        ret.slice.len = self.slice.len - at;

        unsafe {
            ptr::copy_nonoverlapping(
                self.slice.ptr.as_ptr().add(at),
                ret.slice.ptr.as_ptr(),
                ret.len());
        }

        ret
    }

    /// Resize the vector to hold `len` elements, initialized to the return value of `f` if necessary.
    pub fn resize_with<F>(&mut self, len: usize, mut f: F)
    where
        F: FnMut() -> T,
    {
        let old_len = self.slice.len;

        if self.capacity < len {
            self.reserve(len - old_len);
        }

        for i in old_len..len.saturating_sub(1) {
            unsafe { ptr::write(self.slice.ptr.as_ptr().add(i), f()) }
        }

        if len > old_len {
            unsafe {
                ptr::write(self.slice.ptr.as_ptr().add(len - 1), f());
            }
        } else if len < old_len {
            unsafe {
                ptr::drop_in_place(&mut self.slice[len..old_len]);
            }
        }

        self.slice.len = len;
    }

    /// Resize the vector to hold `len` elements, initialized to `value` if necessary.
    pub fn resize(&mut self, len: usize, value: T)
    where
        T: Clone,
    {
        let old_len = self.slice.len;

        if self.capacity < len {
            self.reserve(len - old_len);
        }

        for i in old_len..len.saturating_sub(1) {
            unsafe { ptr::write(self.slice.ptr.as_ptr().add(i), value.clone()) }
        }

        if len > old_len {
            unsafe {
                ptr::write(self.slice.ptr.as_ptr().add(len - 1), value);
            }
        } else if len < old_len {
            unsafe {
                ptr::drop_in_place(&mut self.slice[len..old_len]);
            }
        }

        self.slice.len = len;
    }

    /// Clone and append all elements in a slice to the vector.
    pub fn extend_from_slice(&mut self, other: &[T])
        where
            T: Clone
    {
        for e in other {
            self.push(e.clone());
        }
    }

    // TODO: dedup

    // TODO: remove_item

    // TODO: splice

    // TODO: drain_filter
}

impl<T: Clone, H: AllocHandle + Clone> Clone for SliceVec<T, H> {
    fn clone(&self) -> Self {
        let mut vec: SliceVec<T, H> =
            SliceVec::with_capacity(self.slice.handle.clone(), self.capacity);

        for i in 0..self.slice.len() {
            unsafe {
                ptr::write(
                    vec.slice.ptr.as_ptr().add(i),
                    (*self.slice.ptr.as_ptr().add(i)).clone(),
                );
            }
        }

        vec.slice.len = self.slice.len();

        vec
    }
}

impl<T: fmt::Debug, H> fmt::Debug for SliceVec<T, H> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.slice.fmt(fmt)
    }
}

impl<T, H> Deref for SliceVec<T, H> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.slice.deref()
    }
}

impl<T, H> DerefMut for SliceVec<T, H> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.slice.deref_mut()
    }
}

impl<T: Eq, H> Eq for SliceVec<T, H> { }

impl<T: PartialEq, H> PartialEq for SliceVec<T, H> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T: PartialOrd, H> PartialOrd for SliceVec<T, H> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.deref().partial_cmp(other.deref())
    }
}

impl<'a, T: 'a, H> IntoIterator for &'a SliceVec<T, H> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a, H> IntoIterator for &'a mut SliceVec<T, H> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/* impl<T, H> FromIterator<T> for SliceVec<T, H> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item=T>
    {
        let iter = iter.into_iter();
        let (min, max) = iter.size_hint();
        let cap = if let Some(m) = max { m } else { min };

        let mut res = SliceVec::new(cap);

        for e in iter {
            res.push(e);
        }

        res
    }
} */

#[cfg(feature = "serde")]
impl<T, H> Serialize for SliceVec<T, H>
where
    T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.slice.serialize(serializer)
    }
}

/// Get the page size of the system we are running on.
#[cfg(unix)]
pub(crate) fn get_page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

/// Get the page size of the system we are running on.
#[cfg(windows)]
pub(crate) fn get_page_size() -> usize {
    use std::mem;
    use winapi::um::sysinfoapi::GetSystemInfo;

    unsafe {
        let mut info = mem::zeroed();
        GetSystemInfo(&mut info);

        info.dwPageSize as usize
    }
}

/// Create a virtual memory mapping of size `capacity`.
#[cfg(unix)]
pub(crate) fn create_mapping(capacity: usize) -> *mut u8 {
    let ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            capacity,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        )
    };

    ptr as *mut u8
}

/// Create a virtual memory mapping of size `capacity`.
#[cfg(windows)]
pub(crate) fn create_mapping(capacity: usize) -> *mut u8 {
    use std::ptr;
    use winapi::shared::basetsd::SIZE_T;
    use winapi::shared::minwindef::LPVOID;
    use winapi::um::memoryapi::VirtualAlloc;
    use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};

    let lpAddress: LPVOID = ptr::null_mut();
    let page_size = get_page_size();
    let len = if capacity % page_size == 0 {
        capacity
    } else {
        capacity + page_size - (capacity % page_size)
    };
    let flAllocationType = MEM_COMMIT | MEM_RESERVE;
    let flProtect = PAGE_READWRITE;

    let r = unsafe { VirtualAlloc(lpAddress, len as SIZE_T, flAllocationType, flProtect) };

    r as *mut u8
}

/// Request `capacity` bytes from the system allocator.
pub(crate) fn create_mapping_alloc(capacity: usize) -> *mut u8 {
    unsafe { alloc(Layout::from_size_align_unchecked(capacity, get_page_size())) }
}

/// Destroy a virtual memory mapping.
#[cfg(unix)]
pub(crate) fn destroy_mapping(base: NonNull<u8>, capacity: usize) {
    let res = unsafe { libc::munmap(base.as_ptr() as *mut libc::c_void, capacity) };

    // TODO: Do something on error
    debug_assert_eq!(res, 0);
}

/// Destroy a virtual memory mapping.
#[cfg(windows)]
pub(crate) fn destroy_mapping(base: NonNull<u8>, capacity: usize) {
    use winapi::shared::minwindef::LPVOID;
    use winapi::um::memoryapi::VirtualFree;
    use winapi::um::winnt::MEM_RELEASE;

    let res = unsafe { VirtualFree(base.as_ptr() as LPVOID, 0, MEM_RELEASE) };

    // TODO: Do something on error
    debug_assert_ne!(res, 0);
}

/// Return memory to the system allocator.
pub(crate) fn destroy_mapping_alloc(base: NonNull<u8>, capacity: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(capacity, get_page_size());
        dealloc(base.as_ptr(), layout);
    }
}

pub(crate) fn allocate_inner<T>(
    head: NonNull<u8>,
    position: &Cell<usize>,
    cap: usize,
    count: usize) -> NonNull<T>
{
    let layout = Layout::new::<T>();
    let mask = layout.align() - 1;
    let pos = position.get();

    debug_assert!(layout.align() >= (pos & mask));

    // let align = Ord::max(layout.align(), 64);
    let mut skip = 64 - (pos & mask);

    if skip == layout.align() {
        skip = 0;
    }

    let additional = skip + layout.size() * count;

    assert!(
        pos + additional <= cap,
        "arena overflow: {} > {}",
        pos + additional,
        cap
    );

    position.set(pos + additional);

    let ret = unsafe { head.as_ptr().add(pos + skip) as *mut T };

    assert!((ret as usize) >= head.as_ptr() as usize);
    assert!((ret as usize) < (head.as_ptr() as usize + cap));

    unsafe { NonNull::new_unchecked(ret) }
}

pub(crate) fn allocate_or_extend_inner<T>(
    head: NonNull<u8>,
    position: &Cell<usize>,
    cap: usize,
    ptr: NonNull<T>,
    old_count: usize,
    count: usize) -> NonNull<T>
{
    let pos = position.get();
    let next = unsafe { head.as_ptr().add(pos) };
    let end = unsafe { ptr.as_ptr().add(old_count) };
    if next == end as *mut u8 {
        position.set(pos + (count - old_count) * mem::size_of::<T>());

        ptr
    } else {
        allocate_inner(head, position, cap, count)
    }
}
