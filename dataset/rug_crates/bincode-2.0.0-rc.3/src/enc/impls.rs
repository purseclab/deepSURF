use super::{write::Writer, Encode, Encoder};
use crate::{
    config::{Endian, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::EncodeError,
};
use core::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ops::{Bound, Range, RangeInclusive},
    time::Duration,
};

impl Encode for () {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl<T> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        u8::from(*self).encode(encoder)
    }
}

impl Encode for u8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self])
    }
}

impl Encode for NonZeroU8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroU128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for usize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&(*self as u64).to_be_bytes()),
                Endian::Little => encoder.writer().write(&(*self as u64).to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroUsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encoder.writer().write(&[*self as u8])
    }
}

impl Encode for NonZeroI8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for i128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&self.to_be_bytes()),
                Endian::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroI128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for isize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_isize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endian::Big => encoder.writer().write(&(*self as i64).to_be_bytes()),
                Endian::Little => encoder.writer().write(&(*self as i64).to_le_bytes()),
            },
        }
    }
}

impl Encode for NonZeroIsize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for f32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

impl Encode for f64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match E::C::ENDIAN {
            Endian::Big => encoder.writer().write(&self.to_be_bytes()),
            Endian::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

impl Encode for char {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        encode_utf8(encoder.writer(), *self)
    }
}

// BlockedTODO: https://github.com/rust-lang/rust/issues/37653
//
// We'll want to implement encoding for both &[u8] and &[T: Encode],
// but those implementations overlap because u8 also implements Encode
// impl Encode for &'_ [u8] {
//     fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//         encoder.writer().write(*self)
//     }
// }

impl<T> Encode for [T]
where
    T: Encode + 'static,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        super::encode_slice_len(encoder, self.len())?;

        if core::any::TypeId::of::<T>() == core::any::TypeId::of::<u8>() {
            let t: &[u8] = unsafe { core::mem::transmute(self) };
            encoder.writer().write(t)?;
            return Ok(());
        }

        for item in self {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

fn encode_utf8(writer: &mut impl Writer, c: char) -> Result<(), EncodeError> {
    let code = c as u32;

    if code < MAX_ONE_B {
        writer.write(&[c as u8])
    } else if code < MAX_TWO_B {
        let mut buf = [0u8; 2];
        buf[0] = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
        buf[1] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else if code < MAX_THREE_B {
        let mut buf = [0u8; 3];
        buf[0] = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
        buf[1] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    } else {
        let mut buf = [0u8; 4];
        buf[0] = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
        buf[1] = (code >> 12 & 0x3F) as u8 | TAG_CONT;
        buf[2] = (code >> 6 & 0x3F) as u8 | TAG_CONT;
        buf[3] = (code & 0x3F) as u8 | TAG_CONT;
        writer.write(&buf)
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_bytes().encode(encoder)
    }
}

impl<T, const N: usize> Encode for [T; N]
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        super::encode_option_variant(encoder, self)?;
        if let Some(val) = self {
            val.encode(encoder)?;
        }
        Ok(())
    }
}

impl<T, U> Encode for Result<T, U>
where
    T: Encode,
    U: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Ok(val) => {
                0u32.encode(encoder)?;
                val.encode(encoder)
            }
            Err(err) => {
                1u32.encode(encoder)?;
                err.encode(encoder)
            }
        }
    }
}

impl<T> Encode for Cell<T>
where
    T: Encode + Copy,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(&self.get(), encoder)
    }
}

impl<T> Encode for RefCell<T>
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        let borrow_guard = self
            .try_borrow()
            .map_err(|e| EncodeError::RefCellAlreadyBorrowed {
                inner: e,
                type_name: core::any::type_name::<RefCell<T>>(),
            })?;
        T::encode(&borrow_guard, encoder)
    }
}

impl Encode for Duration {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_secs().encode(encoder)?;
        self.subsec_nanos().encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for Range<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start.encode(encoder)?;
        self.end.encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for RangeInclusive<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.start().encode(encoder)?;
        self.end().encode(encoder)?;
        Ok(())
    }
}

impl<T> Encode for Bound<T>
where
    T: Encode,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        match self {
            Self::Unbounded => {
                0u32.encode(encoder)?;
            }
            Self::Included(val) => {
                1u32.encode(encoder)?;
                val.encode(encoder)?;
            }
            Self::Excluded(val) => {
                2u32.encode(encoder)?;
                val.encode(encoder)?;
            }
        }
        Ok(())
    }
}

impl<'a, T> Encode for &'a T
where
    T: Encode + ?Sized,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        T::encode(self, encoder)
    }
}

#[cfg(test)]
mod tests_rug_240 {
    use super::*;
    use crate::enc::impls::{Writer, encode_utf8, EncodeError, TAG_TWO_B, TAG_CONT, TAG_THREE_B, TAG_FOUR_B};
    use crate::features::IoWriter;
    use std::vec::Vec;
    
    #[test]
    fn test_encode_utf8() {
        let mut p0: Vec<u8> = Vec::new();
        let mut p1: char = '\u{1F600}';
        
        let mut writer: IoWriter<Vec<u8>> = IoWriter::new(&mut p0);
        encode_utf8(&mut writer, p1).unwrap();
        
        // Additional assertions or checks if needed
    }
}
#[cfg(test)]
mod tests_rug_241 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        let p0: () = ();
        let p1 = &mut v19;
        
        p0.encode(p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_242 {
    use super::*;
    use crate::enc::{Encode, Encoder, EncodeError};
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::marker::PhantomData;
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        let p0: PhantomData<u32> = PhantomData;
        let p1: &mut EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = &mut v19;

        
        p0.encode(p1);
    }
}#[cfg(test)]
mod tests_rug_243 {
    use super::*;
    use crate::enc::Encoder;
    use crate::enc::EncodeError;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let p0: bool = true;

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut v19).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_244 {
    use super::*;
    use crate::Encode;
    use crate::enc::{Encoder, EncodeError};
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0: u8 = 42;

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        <u8>::encode(&p0, &mut p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_245 {
    use super::*;
    use crate::Encode;
    use crate::enc::encoder::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use std::num::NonZeroU8;

    #[test]
    fn test_rug() {
        let mut p0: NonZeroU8 = NonZeroU8::new(99).unwrap();
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        p0.encode(&mut p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_247 {
    use super::*;
    use crate::enc::{Encode, EncodeError};
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroU16;
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0 = NonZeroU16::new(100).unwrap();

        let mut writer: Vec<u8> = Vec::new();
        let mut p1: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder_impl: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(p1, config);

        p0.encode(&mut encoder_impl).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_248 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0: u32 = 42;

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        <u32>::encode(&p0, &mut p1).unwrap();
    }
}

#[cfg(test)]
mod tests_rug_249 {
    use super::*;
    use crate::enc::Encoder;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use std::num::NonZeroU32;

    #[test]
    fn test_rug() {
        let mut p0: NonZeroU32 = NonZeroU32::new(42).unwrap();
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

                
        <std::num::NonZeroU32>::encode(&mut p0, &mut p1);

    }
}


#[cfg(test)]
mod tests_rug_250 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::Encode;

    #[test]
    fn test_rug() {
        let p0: u64 = 42;

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut p1).unwrap();
        
        // Add assertions here if needed
    }
}#[cfg(test)]
mod tests_rug_251 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use std::num::NonZeroU64;

    #[test]
    fn test_rug() {
        let mut p0 = NonZeroU64::new(42).unwrap();

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        <std::num::NonZeroU64>::encode(&p0, &mut p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_252 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use crate::Encode;

    #[test]
    fn test_rug() {
        let p0: u128 = 1234567890;
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut p1);
    }
}#[cfg(test)]
mod tests_rug_253 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroU128;
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0 = NonZeroU128::new(42).unwrap();
        
        let mut writer: Vec<u8> = Vec::new();
        let mut p1: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(p1, config);
        
        <std::num::NonZeroU128>::encode(&p0, &mut encoder).unwrap();

    }
}#[cfg(test)]
mod tests_rug_254 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        let p0: usize = 42;
        let p1: &mut EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = &mut v19;

        p0.encode(p1);

    }
}#[cfg(test)]
mod tests_rug_256 {
    use super::*;
    use crate::enc::{Encoder, EncodeError};
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        let p0: i8 = 42;
        let p1: &mut EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = &mut v19;

        p0.encode(p1);
    }
}#[cfg(test)]
mod tests_rug_257 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroI8;
    
    #[test]
    fn test_encode() {
        let mut p0 = NonZeroI8::new(42).expect("42 is not zero");
        
        let mut writer: Vec<u8> = Vec::new();
        let mut p1: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(p1, config);
        
        p0.encode(&mut encoder);
        
    }
}        
#[cfg(test)]
mod tests_rug_258 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::enc::Encoder;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let p0: i16 = -12345;
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        p0.encode(&mut p1).unwrap();

        // Further assertions...
    }
}

#[cfg(test)]
mod tests_rug_259 {
    use super::*;
    use crate::Encode;
    use std::num::NonZeroI16;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    
    #[test]
    fn test_rug() {
        
        let mut v107: NonZeroI16 = NonZeroI16::new(42).unwrap();
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

                
        <std::num::NonZeroI16>::encode(&mut v107, &mut v19);

    }
}
#[cfg(test)]
mod tests_rug_261 {
    use super::*;
    use crate::{Encode, enc::{Encoder, EncodeError}, enc::EncoderImpl, features::IoWriter, config::{Configuration, BigEndian, Fixint, Limit}};
    use std::num::NonZeroI32;
    use std::vec::Vec;
    
    #[test]
    fn test_rug() {
        let mut p0 = NonZeroI32::new(123).unwrap();
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        <std::num::NonZeroI32>::encode(&p0, &mut p1).unwrap();
    }
}

#[cfg(test)]
mod tests_rug_263 {
    use super::*;
    use crate::enc::{Encode, EncodeError};
    use crate::enc::Encoder;
    use std::num::NonZeroI64;
    use crate::enc::encoder::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut p0 = NonZeroI64::new(42).unwrap();
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        <std::num::NonZeroI64 as Encode>::encode(&p0, &mut p1).unwrap();
        
        // Additional assertions if required
		assert_eq!(writer[0], 42);
		// ...
   }
}
#[cfg(test)]
mod tests_rug_264 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::enc::{Encode, EncodeError};
    
    #[test]
    fn test_rug() {
        let p0: i128 = 123456789012345678901234567890;
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
                
        p0.encode(&mut p1).unwrap();

    }
}        
#[cfg(test)]
mod tests_rug_265 {
    use super::*;
    use crate::enc::{Encode, Encoder};
    use crate::enc::encoder::EncoderImpl;
    use crate::features::{IoWriter};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use std::num::NonZeroI128;

    #[test]
    fn test_encode() {
        let mut p0 = NonZeroI128::new(110).unwrap();
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        <std::num::NonZeroI128>::encode(&p0, &mut p1).unwrap();
        
        // add assertions or other verifications here
    }
}
                            
#[cfg(test)]
mod tests_rug_267 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::num::NonZeroIsize;
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0 = NonZeroIsize::new(42).unwrap();

        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
                
        <std::num::NonZeroIsize>::encode(&p0,&mut p1).unwrap();

    }
}
#[cfg(test)]
mod tests_rug_268 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let p0: f32 = 1.23;  // Fill in with a sample value for f32
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        <f32>::encode(&p0, &mut p1);
    }
}
                    
#[cfg(test)]
mod tests_rug_272 {
    use super::*;
    use crate::Encode;
    use crate::enc::{Encoder, EncodeError};
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        let p0: &str = "sample string";
        let p1 = &mut v19;

        
        <str>::encode(&p0, p1).unwrap();

    }
}           #[cfg(test)]
mod tests_rug_273 {
    use super::*;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        // Construct the first argument
        let mut p0: [i32; 5] = [0, 1, 2, 3, 4];
        // Construct the second argument
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        <[i32; 5]>::encode(&p0, &mut p1);
    }
}#[cfg(test)]
mod tests_rug_274 {
    use super::*;
    use crate::enc::{Encode, EncodeError};
    use crate::enc::encoder::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            EncoderImpl::new(v16, config);

        let p0: Option<i32> = Some(42);

        p0.encode(&mut encoder).unwrap();
        // Further assertions...
    }
}#[cfg(test)]
mod tests_rug_277 {
    use super::*;
    use crate::Encode;
    use crate::enc::impls::EncodeError;
    use std::cell::RefCell;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_encode() {
        let mut p0: RefCell<i32> = RefCell::new(42);
        let mut writer: Vec<u8> = Vec::new();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(IoWriter::new(&mut writer), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        p0.encode(&mut p1);
    }
}#[cfg(test)]
mod tests_rug_278 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::time::Duration;
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut p0: Duration = Duration::new(10, 0);
        let mut writer: Vec<u8> = Vec::new();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(IoWriter::new(&mut writer), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        <std::time::Duration as crate::Encode>::encode(&p0, &mut p1);
    }
}#[cfg(test)]
mod tests_rug_280 {
    use super::*;
    use crate::Encode;
    use std::ops::RangeInclusive;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0: RangeInclusive<i32> = 1..=20;

        let mut writer: Vec<u8> = Vec::new();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>>
            = EncoderImpl::new(IoWriter::new(&mut writer), Configuration::<BigEndian, Fixint, Limit<100>>::default());             
                
        p0.encode(&mut p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_282 {
    use super::*;
    use crate::enc::{Encode, Encoder};
    use crate::enc::encoder::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;
    use std::marker::PhantomData;

    #[test]
    fn test_rug() {
        // Construct the PhantomData
        let mut phantom: PhantomData<()> = PhantomData;

        // Construct the EncoderImpl
        let mut writer: Vec<u8> = Vec::new();
        let mut io_writer: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder_impl: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(io_writer, config);

        // Call the encode function
        phantom.encode(&mut encoder_impl).unwrap();
    }
}