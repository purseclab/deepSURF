//! This module provides a reference-counted arena implementation.
//!
//! This is primarily meant for usecases where the lifetime of the arena-allocated objects is
//! overly hard (or impossible) to express in terms of Rust's lifetime facilities.
//!
//! This allows for greater flexibility, but has the drawback that unexpected behaviour can arise
//! when arena-allocated objects block the clearing of the arena even though the user expects to be
//! already invalidated.
//!
//! If you are not sure what arena to use, it's strongly suggested you try the `region` module
//! first.
use crate::common::{self, AllocHandle, ArenaBacking, ArenaError};

use std::cell::Cell;
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

/// A reference-counting arena (non-MT-safe).
///
/// This is the only object that can be used to clear the arena. All other objects referring to
/// the arena merely allow for allocation, and are present to avoid arena clearing while they are
/// live.
#[derive(Debug)]
pub struct Arena(InnerRef, ArenaBacking);

/// A non-owning object referring to the arena.
///
/// A reference to the arena that allows its holder to allocate memory from the arena. While
/// it is live, the arena cannot be cleared (as it is associated with an arena-allocated
/// object).
#[derive(Clone, Debug)]
pub struct InnerRef {
    inner: Rc<Inner>,
}

/// An arena's guts
#[derive(Debug)]
struct Inner {
    /// Head of the arena space
    head: NonNull<u8>,

    /// Offset into the last region
    pos: Cell<usize>,

    /// Total capacity of the arena
    cap: usize,
}

/// An arena allocated, fixed-size sequence of objects
pub type Slice<T> = common::Slice<T, InnerRef>;

/// An arena allocated, sequential, resizable vector
///
/// Since the arena does not support resizing, or freeing memory, this implementation just
/// creates new slices as necessary and leaks the previous arena allocation, trading memory
/// for speed.
pub type SliceVec<T> = common::SliceVec<T, InnerRef>;

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

        Ok(Arena(
            InnerRef {
                inner: Rc::new(Inner { head, pos, cap }),
            },
            backing,
        ))
    }

    /// Create another reference to the arena.
    pub fn inner(&self) -> InnerRef {
        self.0.clone()
    }

    /// Clear the arena.
    ///
    /// This only requires an immutable reference, as it (a) perfors a check that
    /// no arena-allocated object is still alive (weak reason), and because all mutable
    /// state is neatly contained in a `Cell` (slightly stronger reason).
    pub fn clear(&self) -> Result<(), ArenaError> {
        if Rc::strong_count(&self.inner) == 1 {
            self.inner.pos.set(0);

            Ok(())
        } else {
            Err(ArenaError::CannotClear)
        }
    }
}

impl Deref for Arena {
    type Target = InnerRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        match self.1 {
            ArenaBacking::MemoryMap => {
                common::destroy_mapping(self.inner.head, self.inner.cap);
            }
            ArenaBacking::SystemAllocation => {
                common::destroy_mapping_alloc(self.inner.head, self.inner.cap);
            }
        }
    }
}

impl AllocHandle for InnerRef {
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
