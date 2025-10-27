use crate::{de::Decode, enc::Encode, impl_borrow_decode};
use core::sync::atomic::Ordering;

#[cfg(target_has_atomic = "ptr")]
use core::sync::atomic::{AtomicIsize, AtomicUsize};

#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{AtomicBool, AtomicI8, AtomicU8};

#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};

#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};

#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};

#[cfg(target_has_atomic = "8")]
impl Encode for AtomicBool {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "8")]
impl Decode for AtomicBool {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicBool::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicBool);

#[cfg(target_has_atomic = "8")]
impl Encode for AtomicU8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "8")]
impl Decode for AtomicU8 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU8::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicU8);

#[cfg(target_has_atomic = "16")]
impl Encode for AtomicU16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "16")]
impl Decode for AtomicU16 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU16::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "16")]
impl_borrow_decode!(AtomicU16);

#[cfg(target_has_atomic = "32")]
impl Encode for AtomicU32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "32")]
impl Decode for AtomicU32 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU32::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "32")]
impl_borrow_decode!(AtomicU32);

#[cfg(target_has_atomic = "64")]
impl Encode for AtomicU64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "64")]
impl Decode for AtomicU64 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicU64::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "64")]
impl_borrow_decode!(AtomicU64);

#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicUsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicUsize {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicUsize::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl_borrow_decode!(AtomicUsize);

#[cfg(target_has_atomic = "8")]
impl Encode for AtomicI8 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "8")]
impl Decode for AtomicI8 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI8::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "8")]
impl_borrow_decode!(AtomicI8);

#[cfg(target_has_atomic = "16")]
impl Encode for AtomicI16 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "16")]
impl Decode for AtomicI16 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI16::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "16")]
impl_borrow_decode!(AtomicI16);

#[cfg(target_has_atomic = "32")]
impl Encode for AtomicI32 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "32")]
impl Decode for AtomicI32 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI32::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "32")]
impl_borrow_decode!(AtomicI32);

#[cfg(target_has_atomic = "64")]
impl Encode for AtomicI64 {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "64")]
impl Decode for AtomicI64 {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicI64::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "64")]
impl_borrow_decode!(AtomicI64);

#[cfg(target_has_atomic = "ptr")]
impl Encode for AtomicIsize {
    fn encode<E: crate::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), crate::error::EncodeError> {
        self.load(Ordering::SeqCst).encode(encoder)
    }
}

#[cfg(target_has_atomic = "ptr")]
impl Decode for AtomicIsize {
    fn decode<D: crate::de::Decoder>(decoder: &mut D) -> Result<Self, crate::error::DecodeError> {
        Ok(AtomicIsize::new(Decode::decode(decoder)?))
    }
}
#[cfg(target_has_atomic = "ptr")]
impl_borrow_decode!(AtomicIsize);
#[cfg(test)]
mod tests_rug_307 {
    use super::*;
    use crate::enc::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::sync::atomic::AtomicBool;
    
    #[test]
    fn test_encode() {
        let mut p0 = AtomicBool::new(false);
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_308 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicBool>::decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_309 {
    use super::*;
    use crate::enc::Encode;
    use std::sync::atomic::AtomicU8;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0 = AtomicU8::new(0);
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut p1);
    }
}    
#[cfg(test)]
mod tests_rug_310 {

    use super::*;
    use crate::Decode;
    use crate::de::read::SliceReader;
    use crate::de::DecoderImpl;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicU8>::decode(&mut p0).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_312 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicU16>::decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_313 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::error::EncodeError;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::sync::atomic::AtomicU32;
    
    #[test]
    fn test_rug() {
        let mut p0 = AtomicU32::new(0);
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        p0.encode(&mut p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_314 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicU32 as Decode>::decode(&mut p0);
        
    }
}

#[cfg(test)]
mod tests_rug_315 {
    use super::*;
    use crate::Encode;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::sync::atomic::AtomicU64;
    use std::vec::Vec;

    #[test]
    fn test_encode() {
        let mut p0 = AtomicU64::new(42);
        
        let mut writer: Vec<u8> = Vec::new();
        let mut p1: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut encoder: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(p1, config);
        
        p0.encode(&mut encoder).unwrap();
    }
}#[cfg(test)]
mod tests_rug_318 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicUsize>::decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_319 {
    use super::*;
    use crate::enc::Encode;
    use crate::enc::Encoder;
    use crate::error::EncodeError;
    
    use std::sync::atomic::AtomicI8;
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_encode() {
        let mut p0 = AtomicI8::new(0);
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        assert!(p0.encode(&mut p1).is_ok());
    
    }
}#[cfg(test)]
mod tests_rug_320 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicI8>::decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_321 {
    use super::*;
    use crate::{enc::{Encoder, EncoderImpl}, error::EncodeError, features::IoWriter, config::{Configuration, BigEndian, Fixint, Limit}};
    use std::sync::atomic::AtomicI16;
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut p0 = AtomicI16::new(0);
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);

        p0.encode(&mut p1).unwrap();
    }
}#[cfg(test)]
mod tests_rug_322 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicI16>::decode(&mut p0).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_324 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicI32>::decode(&mut p0);
        
    }
}

#[cfg(test)]
mod tests_rug_325 {
    use super::*;
    use crate::{
        enc::{Encoder, Encode},
        error::EncodeError,
        atomic::{AtomicI64},
    };
    
    use crate::enc::EncoderImpl;
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0 = AtomicI64::new(0);
        
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut p1: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        p0.encode(&mut p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_326 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicI64>::decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_328 {
    use super::*;
    use crate::Decode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicIsize>::decode(&mut p0);
    }
}
