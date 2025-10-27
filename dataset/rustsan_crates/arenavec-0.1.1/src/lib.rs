//! This crate exposes a number of arena allocator implementations tailored to slightly different
//! usecases. Currently, all of them are non-MT-safe, and hence intended to be used locally per
//! thread, for instance being placed in a thread-local variable, or nested in user types.
//!
//! In addition to the allocator types, the library provides a set of data structures that are
//! allocator-agnostic (as in, compatible with all allocators provided in this crate).
#![deny(missing_debug_implementations, warnings, rust_2018_idioms)]

pub mod common;
pub mod rc;
pub mod region;

pub use crate::common::*;
