use core::{convert::TryInto, u32};

use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{
    config::Endian,
    de::read::Reader,
    error::{DecodeError, IntegerType},
};

#[inline(never)]
#[cold]
fn deserialize_varint_cold_u16<R>(read: &mut R, endian: Endian) -> Result<u16, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u16),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes),
                Endian::Little => u16::from_le_bytes(bytes),
            })
        }
        U32_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U32),
        U64_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U16, IntegerType::Reserved),
    }
}

#[inline(never)]
#[cold]
fn deserialize_varint_cold_u32<R>(read: &mut R, endian: Endian) -> Result<u32, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u32),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u32,
                Endian::Little => u16::from_le_bytes(bytes) as u32,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes),
                Endian::Little => u32::from_le_bytes(bytes),
            })
        }
        U64_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
    }
}

#[inline(never)]
#[cold]
fn deserialize_varint_cold_u64<R>(read: &mut R, endian: Endian) -> Result<u64, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u64),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u64,
                Endian::Little => u16::from_le_bytes(bytes) as u64,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as u64,
                Endian::Little => u32::from_le_bytes(bytes) as u64,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes),
                Endian::Little => u64::from_le_bytes(bytes),
            })
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::U64, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U64, IntegerType::Reserved),
    }
}

#[inline(never)]
#[cold]
fn deserialize_varint_cold_usize<R>(read: &mut R, endian: Endian) -> Result<usize, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as usize),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as usize,
                Endian::Little => u16::from_le_bytes(bytes) as usize,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as usize,
                Endian::Little => u32::from_le_bytes(bytes) as usize,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes) as usize,
                Endian::Little => u64::from_le_bytes(bytes) as usize,
            })
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::Usize, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::Usize, IntegerType::Reserved),
    }
}

#[inline(never)]
#[cold]
fn deserialize_varint_cold_u128<R>(read: &mut R, endian: Endian) -> Result<u128, DecodeError>
where
    R: Reader,
{
    let mut bytes = [0u8; 1];
    read.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u128),
        U16_BYTE => {
            let mut bytes = [0u8; 2];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u16::from_be_bytes(bytes) as u128,
                Endian::Little => u16::from_le_bytes(bytes) as u128,
            })
        }
        U32_BYTE => {
            let mut bytes = [0u8; 4];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u32::from_be_bytes(bytes) as u128,
                Endian::Little => u32::from_le_bytes(bytes) as u128,
            })
        }
        U64_BYTE => {
            let mut bytes = [0u8; 8];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u64::from_be_bytes(bytes) as u128,
                Endian::Little => u64::from_le_bytes(bytes) as u128,
            })
        }
        U128_BYTE => {
            let mut bytes = [0u8; 16];
            read.read(&mut bytes)?;
            Ok(match endian {
                Endian::Big => u128::from_be_bytes(bytes),
                Endian::Little => u128::from_le_bytes(bytes),
            })
        }
        _ => invalid_varint_discriminant(IntegerType::U128, IntegerType::Reserved),
    }
}

#[inline(never)]
#[cold]
fn invalid_varint_discriminant<T>(
    expected: IntegerType,
    found: IntegerType,
) -> Result<T, DecodeError> {
    Err(DecodeError::InvalidIntegerType { expected, found })
}

pub fn varint_decode_u16<R: Reader>(read: &mut R, endian: Endian) -> Result<u16, DecodeError> {
    if let Some(bytes) = read.peek_read(3) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u16, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };

                (val, 3)
            }
            U32_BYTE => return invalid_varint_discriminant(IntegerType::U16, IntegerType::U32),
            U64_BYTE => return invalid_varint_discriminant(IntegerType::U16, IntegerType::U64),
            U128_BYTE => return invalid_varint_discriminant(IntegerType::U16, IntegerType::U128),
            _ => return invalid_varint_discriminant(IntegerType::U16, IntegerType::Reserved),
        };

        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u16(read, endian)
    }
}

pub fn varint_decode_u32<R: Reader>(read: &mut R, endian: Endian) -> Result<u32, DecodeError> {
    if let Some(bytes) = read.peek_read(5) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u32, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };

                (val as u32, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };

                (val, 5)
            }
            U64_BYTE => return invalid_varint_discriminant(IntegerType::U32, IntegerType::U64),
            U128_BYTE => return invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
            _ => return invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
        };

        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u32(read, endian)
    }
}

pub fn varint_decode_u64<R: Reader>(read: &mut R, endian: Endian) -> Result<u64, DecodeError> {
    if let Some(bytes) = read.peek_read(9) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u64, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };

                (val as u64, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };

                (val as u64, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };

                (val, 9)
            }
            U128_BYTE => return invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
            _ => return invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
        };

        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u64(read, endian)
    }
}

pub fn varint_decode_usize<R: Reader>(read: &mut R, endian: Endian) -> Result<usize, DecodeError> {
    if let Some(bytes) = read.peek_read(9) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as usize, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };

                (val as usize, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };

                (val as usize, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };

                (val as usize, 9)
            }
            U128_BYTE => return invalid_varint_discriminant(IntegerType::Usize, IntegerType::U128),
            _ => return invalid_varint_discriminant(IntegerType::Usize, IntegerType::Reserved),
        };

        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_usize(read, endian)
    }
}

pub fn varint_decode_u128<R: Reader>(read: &mut R, endian: Endian) -> Result<u128, DecodeError> {
    if let Some(bytes) = read.peek_read(17) {
        let (discriminant, bytes) = bytes.split_at(1);
        let (out, used) = match discriminant[0] {
            byte @ 0..=SINGLE_BYTE_MAX => (byte as u128, 1),
            U16_BYTE => {
                let val = match endian {
                    Endian::Big => u16::from_be_bytes(bytes[..2].try_into().unwrap()),
                    Endian::Little => u16::from_le_bytes(bytes[..2].try_into().unwrap()),
                };

                (val as u128, 3)
            }
            U32_BYTE => {
                let val = match endian {
                    Endian::Big => u32::from_be_bytes(bytes[..4].try_into().unwrap()),
                    Endian::Little => u32::from_le_bytes(bytes[..4].try_into().unwrap()),
                };

                (val as u128, 5)
            }
            U64_BYTE => {
                let val = match endian {
                    Endian::Big => u64::from_be_bytes(bytes[..8].try_into().unwrap()),
                    Endian::Little => u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                };

                (val as u128, 9)
            }
            U128_BYTE => {
                let val = match endian {
                    Endian::Big => u128::from_be_bytes(bytes[..16].try_into().unwrap()),
                    Endian::Little => u128::from_le_bytes(bytes[..16].try_into().unwrap()),
                };

                (val, 17)
            }
            _ => return invalid_varint_discriminant(IntegerType::Usize, IntegerType::Reserved),
        };

        read.consume(used);
        Ok(out)
    } else {
        deserialize_varint_cold_u128(read, endian)
    }
}

#[test]
fn test_decode_u16() {
    let cases: &[(&[u8], u16, u16)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U32_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U32,
            },
        ),
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U16,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd { additional: 2 }),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd { additional: 1 }),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u16(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u32() {
    let cases: &[(&[u8], u32, u32)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U64_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U64,
            },
        ),
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U32,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd { additional: 2 }),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd { additional: 1 }),
        (&[U32_BYTE], DecodeError::UnexpectedEnd { additional: 4 }),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd { additional: 3 }),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u32(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u64() {
    let cases: &[(&[u8], u64, u64)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10],
            720_575_940_379_279_360,
            10,
        ),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (
            &[U128_BYTE],
            DecodeError::InvalidIntegerType {
                expected: IntegerType::U64,
                found: IntegerType::U128,
            },
        ),
        (&[U16_BYTE], DecodeError::UnexpectedEnd { additional: 2 }),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd { additional: 1 }),
        (&[U32_BYTE], DecodeError::UnexpectedEnd { additional: 4 }),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd { additional: 3 }),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
        (&[U64_BYTE], DecodeError::UnexpectedEnd { additional: 8 }),
        (&[U64_BYTE, 0], DecodeError::UnexpectedEnd { additional: 7 }),
        (
            &[U64_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 6 },
        ),
        (
            &[U64_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 5 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 4 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 3 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u64(&mut reader, Endian::Little).unwrap_err();
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}

#[test]
fn test_decode_u128() {
    let cases: &[(&[u8], u128, u128)] = &[
        (&[0], 0, 0),
        (&[10], 10, 10),
        (&[U16_BYTE, 0, 10], 2560, 10),
        (&[U32_BYTE, 0, 0, 0, 10], 167_772_160, 10),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0, 10],
            720_575_940_379_279_360,
            10,
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
            13_292_279_957_849_158_729_038_070_602_803_445_760,
            10,
        ),
    ];
    for &(slice, expected_le, expected_be) in cases {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap();
        assert_eq!(expected_le, found);

        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Big).unwrap();
        assert_eq!(expected_be, found);
    }

    let errors: &[(&[u8], DecodeError)] = &[
        (&[U16_BYTE], DecodeError::UnexpectedEnd { additional: 2 }),
        (&[U16_BYTE, 0], DecodeError::UnexpectedEnd { additional: 1 }),
        (&[U32_BYTE], DecodeError::UnexpectedEnd { additional: 4 }),
        (&[U32_BYTE, 0], DecodeError::UnexpectedEnd { additional: 3 }),
        (
            &[U32_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U32_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
        (&[U64_BYTE], DecodeError::UnexpectedEnd { additional: 8 }),
        (&[U64_BYTE, 0], DecodeError::UnexpectedEnd { additional: 7 }),
        (
            &[U64_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 6 },
        ),
        (
            &[U64_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 5 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 4 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 3 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U64_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
        (&[U128_BYTE], DecodeError::UnexpectedEnd { additional: 16 }),
        (
            &[U128_BYTE, 0],
            DecodeError::UnexpectedEnd { additional: 15 },
        ),
        (
            &[U128_BYTE, 0, 0],
            DecodeError::UnexpectedEnd { additional: 14 },
        ),
        (
            &[U128_BYTE, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 13 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 12 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 11 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 10 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 9 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 8 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 7 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 6 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 5 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 4 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 3 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 2 },
        ),
        (
            &[U128_BYTE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            DecodeError::UnexpectedEnd { additional: 1 },
        ),
    ];

    for (slice, expected) in errors {
        let mut reader = crate::de::read::SliceReader::new(slice);
        let found = varint_decode_u128(&mut reader, Endian::Little).unwrap_err();
        std::dbg!(slice);
        assert_eq!(std::format!("{:?}", expected), std::format!("{:?}", found));
    }
}
#[cfg(test)]
mod tests_rug_93 {
    use super::*;
    use std::io::BufReader;
    use crate::varint::decode_unsigned::{deserialize_varint_cold_u16, Endian};

    #[test]
    fn test_deserialize_varint_cold_u16() {
        let data: &[u8] = &[1, 2, 3, 4, 5];
        let mut reader = BufReader::new(data);
        let endian = Endian::Big;

        let result = deserialize_varint_cold_u16(&mut reader, endian);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}

#[cfg(test)]
mod tests_rug_94 {
    use super::*;
    use std::io::{BufReader};
    use crate::varint::decode_unsigned::{Endian};
    
    #[test]
    fn test_deserialize_varint_cold_u32() {
        let mut p0: BufReader<&[u8]> = BufReader::new(&[1, 2, 3, 4, 5]);
        let p1: Endian = Endian::Big;

        deserialize_varint_cold_u32(&mut p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_96 {
    use super::*;
    use std::io::BufReader;
    use crate::varint::decode_unsigned::{deserialize_varint_cold_usize, Endian, DecodeError};
    
    #[test]
    fn test_deserialize_varint_cold_usize() {
        
        let mut p0: BufReader<&[u8]> = BufReader::new(&[1, 2, 3, 4, 5]);
        let p1: Endian = Endian::Big;
        
        let result: Result<usize, DecodeError> = deserialize_varint_cold_usize(&mut p0, p1);
        
        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), 1);
    }
}
#[cfg(test)]
mod tests_rug_97 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::varint::decode_unsigned::{deserialize_varint_cold_u128, Endian};

    #[test]
    fn test_deserialize_varint_cold_u128() {
        let mut p0: SliceReader<'_> = SliceReader::new(&[0u8, 1u8, 2u8]);
        let p1: Endian = Endian::Big;

        let result = deserialize_varint_cold_u128(&mut p0, p1);
        assert_eq!(result.is_ok(), true);
    }
}
#[cfg(test)]
mod tests_rug_98 {
    use super::*;
    use crate::varint::decode_unsigned::invalid_varint_discriminant;
    use crate::error::DecodeError;
    use crate::error::IntegerType;

    #[test]
    fn test_rug() {
        let mut p0: IntegerType = IntegerType::U8;
        let mut p1: IntegerType = IntegerType::U8;

        invalid_varint_discriminant::<()>(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_100 {
    use super::*;
    use crate::varint::decode_unsigned::{varint_decode_u32, Endian};
    use crate::de::read::SliceReader;

    #[test]
    fn test_varint_decode_u32() {
        let mut p0: SliceReader<'_> = SliceReader::new(&[0u8, 1u8, 2u8]);
        let p1: Endian = Endian::Big;

        varint_decode_u32(&mut p0, p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_101 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::varint::decode_unsigned::{varint_decode_u64, Endian, DecodeError};

    #[test]
    fn test_varint_decode_u64() {
        let mut p0: SliceReader<'_> = SliceReader::new(&[0u8, 1u8, 2u8]);
        let p1: Endian = Endian::Little;

        let result: Result<u64, DecodeError> = varint_decode_u64(&mut p0, p1);

        // Add assertions here
        assert!(result.is_ok());
    }
}#[cfg(test)]
mod tests_rug_102 {
    use super::*;
    use crate::de::read::SliceReader;
    use crate::varint::decode_unsigned::{varint_decode_usize, Endian};

    #[test]
    fn test_rug() {
        let mut p0: SliceReader<'_> = SliceReader::new(&[0u8, 1u8, 2u8]);
        let p1: Endian = Endian::Big;

        varint_decode_usize(&mut p0, p1);

    }
}#[cfg(test)]
mod tests_rug_103 {
    use super::*;
    use crate::varint::decode_unsigned::{
        varint_decode_u128, DecodeError, Endian, SINGLE_BYTE_MAX, U16_BYTE, U32_BYTE, U64_BYTE,
        U128_BYTE,
    };
    use crate::de::read::SliceReader;
    use std::convert::TryInto;

    #[test]
    fn test_varint_decode_u128() {
        let mut reader: SliceReader<'_> = SliceReader::new(&[0u8, 1u8, 2u8]);
        let endian: Endian = Endian::Little;

        let result: Result<u128, DecodeError> = varint_decode_u128(&mut reader, endian);

        // Add assertion statements
        // ...
    }
}