use safe_transmute::{ErrorReason, GuardError, Error, guarded_transmute_pod_many_pedantic};
use self::super::LeToNative;


#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 0,
                   reason: ErrorReason::NotEnoughBytes,
               })));
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x00]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 1,
                   reason: ErrorReason::NotEnoughBytes,
               })));
}

#[test]
fn just_enough() {
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x00, 0x01].le_to_native::<u16>()),
               Ok([0x0100u16].into_iter().as_slice()));
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
               Ok([0x0100u16, 0x0200u16].into_iter().as_slice()));
}

#[test]
fn too_much() {
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x00, 0x01, 0x00]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 3,
                   reason: ErrorReason::InexactByteCount,
               })));
    assert_eq!(guarded_transmute_pod_many_pedantic::<u16>(&[0x00, 0x01, 0x00, 0x02, 0x00]),
               Err(Error::Guard(GuardError {
                   required: 16 / 8,
                   actual: 5,
                   reason: ErrorReason::InexactByteCount,
               })));
}
