//! This module provides a statically-borrow-checked arena implementation.
//!
//! It is relatively intuitive to use -- all objects in the arena are tied to the lifetime of a
//! so-called generation token that is handed exclusively by the arena. As soon as the token is
//! dropped, the arena is cleared. This way, the borrow-checker statically verifies that no objects
//! in the arena live beyond a certain point.
//!
//! The drawback of this approach is that some scenarios cannot be handled with statically-known
//! lifetimes, for instance if the arena-allocated objects have dynamic lifetimes depending on user
//! input or other factors only known at runtime. In such cases the reference-counted arena found
//! in the `rc` module might be a better fit.
use crate::common::{self, AllocHandle, ArenaBacking, ArenaError};

use std::cell::Cell;
use std::ptr::NonNull;

/// A statically checked arena (non-MT-safe).
///
/// Can be stored in a thread-local variable to be accessible everywhere in the owning thread.
/// This arena has an explicit notion of generations of objects. That is, at every point in time,
/// all live objects allocated in the arena belong to the same generation, which is tracked using
/// the lifetime of a token object (which is just a borrow on the arena). Once the token is
/// dropped, the arena is cleared. Furthermore, all objects allocated in the arena are restricted
/// to the token lifetime, so the borrow-checker will stop you when your objects can outlive the
/// generation they are allocated in.
#[derive(Debug)]
pub struct Arena {
    /// Head of the arena space
    head: NonNull<u8>,

    /// Offset into the last region
    pos: Cell<usize>,

    /// Total capacity of the arena
    cap: usize,

    /// The type of backing storage used in the arena
    backing: ArenaBacking,

    /// Whether an exclusive allocation token has been handed out
    locked: Cell<bool>,
}

/// A proxy for an arena that actually allows allocation.
///
/// The intention is to ensure exclusive allocation access and to tag allocated objects with
/// the token lifetime. Only one such object referring to an arena instance is allowed to exist at
/// any time.
#[derive(Debug)]
pub struct ArenaToken<'a> {
    inner: &'a Arena,
}

/// A handle to the arena for the current generation.
///
/// Allows for allocation, but doesn't cause the generation of objects to die when dropped.
#[derive(Debug, Clone)]
pub struct ArenaHandle<'a>(&'a ArenaToken<'a>);

/// An arena allocated, fixed-size sequence of objects
pub type Slice<'a, T> = common::Slice<T, ArenaHandle<'a>>;

/// An arena allocated, sequential, resizable vector
///
/// Since the arena does not support resizing, or freeing memory, this implementation just
/// creates new slices as necessary and leaks the previous arena allocation, trading memory
/// for speed.
pub type SliceVec<'a, T> = common::SliceVec<T, ArenaHandle<'a>>;

impl Arena {
    /// Create an `Arena` with specified capacity.
    ///
    /// Capacity must be a power of 2. The capacity cannot be grown after the fact.
    pub fn init_capacity(backing: ArenaBacking, cap: usize) -> Result<Self, ArenaError> {
        let head = NonNull::new(match backing {
            ArenaBacking::MemoryMap => common::create_mapping(cap),
            ArenaBacking::SystemAllocation => common::create_mapping_alloc(cap),
        })
        .ok_or(ArenaError::AllocationFailed)?;
        let pos = Cell::new(0);
        let locked = Cell::new(false);

        Ok(Arena {
            head,
            pos,
            cap,
            backing,
            locked,
        })
    }

    /// Return a fresh generation token for the arena.
    ///
    /// If a generation of objects is currently live, an error is returned instead.
    pub fn generation_token<'a>(&'a self) -> Result<ArenaToken<'a>, ArenaError> {
        if self.locked.get() {
            Err(ArenaError::AlreadyLocked)
        } else {
            self.locked.set(true);
            Ok(ArenaToken { inner: self })
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        match self.backing {
            ArenaBacking::MemoryMap => {
                common::destroy_mapping(self.head, self.cap);
            }
            ArenaBacking::SystemAllocation => {
                common::destroy_mapping_alloc(self.head, self.cap);
            }
        }
    }
}

impl<'a> ArenaToken<'a> {
    /// Create an arena handle using the current generation's token.
    pub fn weak(&'a self) -> ArenaHandle<'a> {
        ArenaHandle(self)
    }
}

impl<'a> AllocHandle for ArenaToken<'a> {
    fn allocate<T>(&self, count: usize) -> NonNull<T> {
        common::allocate_inner(self.inner.head, &self.inner.pos, self.inner.cap, count)
    }

    fn allocate_or_extend<T>(&self, ptr: NonNull<T>, old_count: usize, count: usize) -> NonNull<T> {
        common::allocate_or_extend_inner(
            self.inner.head,
            &self.inner.pos,
            self.inner.cap,
            ptr,
            old_count,
            count)
    }
}

impl<'a> AllocHandle for ArenaHandle<'a> {
    fn allocate<T>(&self, count: usize) -> NonNull<T> {
        self.0.allocate(count)
    }

    fn allocate_or_extend<T>(&self, ptr: NonNull<T>, old_count: usize, count: usize) -> NonNull<T> {
        self.0.allocate_or_extend(ptr, old_count, count)
    }
}

impl<'a> Drop for ArenaToken<'a> {
    fn drop(&mut self) {
        self.inner.pos.set(0);
        self.inner.locked.set(false);
    }
}
