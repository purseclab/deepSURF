use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute_pod_pedantic};
use self::super::LeToNative;


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&[]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 0,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&[0x00]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 1,
                   reason: ErrorReason::InexactByteCount,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()),
               Ok(0x01000000));
}

#[test]
fn too_much() {
    assert_eq!(guarded_transmute_pod_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01, 0x00]),
               Err(Error::Guard(GuardError {
                   required: 32 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}
