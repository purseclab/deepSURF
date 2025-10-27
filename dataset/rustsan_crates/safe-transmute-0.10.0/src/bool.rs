//! Functions for safe transmutation to `bool`.
//! Transmuting to `bool' is not undefined behavior if the transmuted value is
//! either 0 or 1. These functions will return an error if the integer value
//! behind the `bool` value is neither one.


use self::super::{Error, guarded_transmute_many_permissive, guarded_transmute_many_pedantic};
#[cfg(feature = "std")]
use self::super::{guarded_transmute_vec_permissive, guarded_transmute_vec_pedantic};
use core::mem::{size_of, transmute};


/// Makes sure that the bytes represent a sequence of valid boolean values.
///
/// # Panics
///
/// This shouldn't happen on all currently supported platforms, but this
/// function panics if the size of `bool` isn't 1.
#[inline]
pub fn bytes_are_bool(v: &[u8]) -> bool {
    // TODO make this a static assert once available
    assert_eq!(size_of::<bool>(),
               1,
               "unsupported platform due to invalid bool size {}, please report over at https://github.com/nabijaczleweli/safe-transmute-rs/issues/new",
               size_of::<bool>());

    v.iter().cloned().all(byte_is_bool)
}

#[inline]
fn byte_is_bool(b: u8) -> bool {
    unsafe { b == transmute::<_, u8>(false) || b == transmute::<_, u8>(true) }
}

/// View a byte slice as a slice of boolean values.
///
/// The resulting slice will have as many instances of `bool` as will fit, can be empty.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, guarded_transmute_bool_permissive};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(guarded_transmute_bool_permissive(&[0x00, 0x01, 0x00, 0x01])?,
///            &[false, true, false, true]);
/// assert_eq!(guarded_transmute_bool_permissive(&[])?, &[]);
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn guarded_transmute_bool_permissive(bytes: &[u8]) -> Result<&[bool], Error> {
    check_bool(bytes)?;
    unsafe { Ok(guarded_transmute_many_permissive(bytes)) }
}

/// View a byte slice as a slice of boolean values.
///
/// The byte slice must have at least enough bytes to fill a single `bool`.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, guarded_transmute_bool_pedantic};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(guarded_transmute_bool_pedantic(&[0x01, 0x01, 0x01, 0x01])?,
///            &[true, true, true, true]);
/// assert!(guarded_transmute_bool_pedantic(&[]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
pub fn guarded_transmute_bool_pedantic(bytes: &[u8]) -> Result<&[bool], Error> {
    check_bool(bytes)?;
    unsafe { guarded_transmute_many_pedantic(bytes) }
}

/// Trasform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// have as many instances of a type as will fit, rounded down.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, guarded_transmute_bool_vec_permissive};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(guarded_transmute_bool_vec_permissive(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
/// assert_eq!(guarded_transmute_bool_vec_permissive(vec![0x01, 0x00, 0x00, 0x00, 0x01])?,
///            vec![true, false, false, false, true]);
/// assert_eq!(guarded_transmute_bool_vec_permissive(vec![]), Ok(vec![]));
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "std")]
pub fn guarded_transmute_bool_vec_permissive(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    check_bool(&bytes)?;
    unsafe { Ok(guarded_transmute_vec_permissive(bytes)) }
}

/// Transform a byte vector into a vector of bool.
///
/// The vector's allocated byte buffer will be reused when possible, and
/// should not have extraneous data.
///
/// # Examples
///
/// ```
/// # use safe_transmute::{Error, guarded_transmute_bool_vec_pedantic};
/// # fn run() -> Result<(), Error> {
/// assert_eq!(guarded_transmute_bool_vec_pedantic(vec![0x00, 0x01, 0x00, 0x01])?,
///            vec![false, true, false, true]);
///
/// assert!(guarded_transmute_bool_vec_pedantic(vec![]).is_err());
///
/// assert!(guarded_transmute_bool_vec_pedantic(vec![0x04, 0x00, 0xED]).is_err());
/// # Ok(())
/// # }
/// # run().unwrap()
/// ```
#[cfg(feature = "std")]
pub fn guarded_transmute_bool_vec_pedantic(bytes: Vec<u8>) -> Result<Vec<bool>, Error> {
    check_bool(&bytes)?;
    unsafe { guarded_transmute_vec_pedantic(bytes) }
}


fn check_bool(bytes: &[u8]) -> Result<(), Error> {
    if bytes_are_bool(bytes) {
        Ok(())
    } else {
        Err(Error::InvalidValue)
    }
}
