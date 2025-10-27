use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute_many};
use self::super::LeToNative;


#[test]
fn too_short() {
    unsafe {
        assert_eq!(guarded_transmute_many::<u16>(&[]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 0,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
        assert_eq!(guarded_transmute_many::<u16>(&[0x00]),
                   Err(Error::Guard(GuardError {
                       required: 16 / 8,
                       actual: 1,
                       reason: ErrorReason::NotEnoughBytes,
                   })));
    }
}

#[test]
fn just_enough() {
    unsafe {
        assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01].le_to_native::<u16>()),
                   Ok([0x0100u16].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
                   Ok([0x0100u16, 0x0200u16].into_iter().as_slice()));
    }
}

#[test]
fn too_much() {
    unsafe {
        assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00].le_to_native::<u16>()),
                   Ok([0x0100u16].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00].le_to_native::<u16>()),
                   Ok([0x0100u16, 0x0200u16].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].le_to_native::<u16>()),
                   Ok([0x0100u16, 0x0200u16, 0x0300u16].into_iter().as_slice()));
    }
}

#[cfg(feature = "test-unaligned")]
#[cfg(target_endian = "little")]
#[test]
fn unaligned_slicing() {
    let bytes = &[0xFF, 0x01, 0xEE, 0x02, 0xDD, 0x03, 0xCC];
    unsafe {
        assert_eq!(guarded_transmute_many::<u16>(bytes),
                   Ok([0x01FF, 0x02EE, 0x03DD].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u16>(&bytes[1..]),
                   Ok([0xEE01, 0xDD02, 0xCC03].into_iter().as_slice()));
    }

    let bytes: &[u8] = &[0xFF, 0x01, 0xEE, 0x02, 0xDD, 0x03, 0xCC, 0x04, 0xBB, 0x05, 0xAA, 0x06];
    unsafe {
        assert_eq!(guarded_transmute_many::<u32>(bytes),
                   Ok([0x02EE01FF, 0x04CC03DD, 0x06AA05BB].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[1..]),
                   Ok([0xDD02EE01, 0xBB04CC03].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[2..]),
                   Ok([0x03DD02EE, 0x05BB04CC].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[3..]),
                   Ok([0xCC03DD02, 0xAA05BB04].into_iter().as_slice()));
    }
}


#[cfg(feature = "test-unaligned")]
#[cfg(target_endian = "big")]
#[test]
fn unaligned_slicing() {
    let bytes = &[0xFF, 0x01, 0xEE, 0x02, 0xDD, 0x03, 0xCC];
    unsafe {
        assert_eq!(guarded_transmute_many::<u16>(bytes),
                   Ok([0xFF01, 0xEE02, 0xDD03].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u16>(&bytes[1..]),
                   Ok([0x01EE, 0x02DD, 0x03CC].into_iter().as_slice()));
    }

    let bytes: &[u8] = &[0xFF, 0x01, 0xEE, 0x02, 0xDD, 0x03, 0xCC, 0x04, 0xBB, 0x05, 0xAA, 0x06];
    unsafe {
        assert_eq!(guarded_transmute_many::<u32>(bytes),
                   Ok([0xFF01EE02, 0xDD03CC04, 0xBB05AA06].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[1..]),
                   Ok([0x01EE02DD, 0x03CC04BB].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[2..]),
                   Ok([0xEE02DD03, 0xCC04BB05].into_iter().as_slice()));
        assert_eq!(guarded_transmute_many::<u32>(&bytes[3..]),
                   Ok([0x02DD03CC, 0x04BB05AA].into_iter().as_slice()));
    }
}
