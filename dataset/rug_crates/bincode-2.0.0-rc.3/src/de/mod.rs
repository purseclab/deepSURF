//! Decoder-based structs and traits.

mod decoder;
mod impl_core;
mod impl_tuples;
mod impls;

use self::read::{BorrowReader, Reader};
use crate::{
    config::{Config, InternalLimitConfig},
    error::DecodeError,
    utils::Sealed,
};

pub mod read;

pub use self::decoder::DecoderImpl;

/// Trait that makes a type able to be decoded, akin to serde's `DeserializeOwned` trait.
///
/// This trait should be implemented for types which do not have references to data in the reader. For types that contain e.g. `&str` and `&[u8]`, implement [BorrowDecode] instead.
///
/// Whenever you implement `Decode` for your type, the base trait `BorrowDecode` is automatically implemented.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to your type. Note that if the type contains any lifetimes, `BorrowDecode` will be implemented instead.
///
/// # Implementing this trait manually
///
/// If you want to implement this trait for your type, the easiest way is to add a `#[derive(bincode::Decode)]`, build and check your `target/generated/bincode/` folder. This should generate a `<Struct name>_Decode.rs` file.
///
/// For this struct:
///
/// ```
/// struct Entity {
///     pub x: f32,
///     pub y: f32,
/// }
/// ```
///
/// It will look something like:
///
/// ```
/// # struct Entity {
/// #     pub x: f32,
/// #     pub y: f32,
/// # }
/// impl bincode::Decode for Entity {
///     fn decode<D: bincode::de::Decoder>(
///         decoder: &mut D,
///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
///         Ok(Self {
///             x: bincode::Decode::decode(decoder)?,
///             y: bincode::Decode::decode(decoder)?,
///         })
///     }
/// }
/// impl<'de> bincode::BorrowDecode<'de> for Entity {
///     fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
///         decoder: &mut D,
///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
///         Ok(Self {
///             x: bincode::BorrowDecode::borrow_decode(decoder)?,
///             y: bincode::BorrowDecode::borrow_decode(decoder)?,
///         })
///     }
/// }
/// ```
///
/// From here you can add/remove fields, or add custom logic.
///
/// To get specific integer types, you can use:
/// ```
/// # struct Foo;
/// # impl bincode::Decode for Foo {
/// #     fn decode<D: bincode::de::Decoder>(
/// #         decoder: &mut D,
/// #     ) -> core::result::Result<Self, bincode::error::DecodeError> {
/// let x: u8 = bincode::Decode::decode(decoder)?;
/// let x = <u8 as bincode::Decode>::decode(decoder)?;
/// #         Ok(Foo)
/// #     }
/// # }
/// # bincode::impl_borrow_decode!(Foo);
/// ```
pub trait Decode: Sized {
    /// Attempt to decode this type with the given [Decode].
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError>;
}

/// Trait that makes a type able to be decoded, akin to serde's `Deserialize` trait.
///
/// This trait should be implemented for types that contain borrowed data, like `&str` and `&[u8]`. If your type does not have borrowed data, consider implementing [Decode] instead.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Decode)]` to a type with a lifetime.
pub trait BorrowDecode<'de>: Sized {
    /// Attempt to decode this type with the given [BorrowDecode].
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError>;
}

/// Helper macro to implement `BorrowDecode` for any type that implements `Decode`.
#[macro_export]
macro_rules! impl_borrow_decode {
    ($ty:ty $(, $param:ident),*) => {
        impl<'de $(, $param)*> $crate::BorrowDecode<'de> for $ty {
            fn borrow_decode<D: $crate::de::BorrowDecoder<'de>>(
                decoder: &mut D,
            ) -> core::result::Result<Self, $crate::error::DecodeError> {
                $crate::Decode::decode(decoder)
            }
        }
    };
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
pub trait Decoder: Sealed {
    /// The concrete [Reader] type
    type R: Reader;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the reader
    fn reader(&mut self) -> &mut Self::R;

    /// Returns a reference to the config
    fn config(&self) -> &Self::C;

    /// Claim that `n` bytes are going to be read from the decoder.
    /// This can be used to validate `Configuration::Limit<N>()`.
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError>;

    /// Claim that we're going to read a container which contains `len` entries of `T`.
    /// This will correctly handle overflowing if `len * size_of::<T>() > usize::max_value`
    fn claim_container_read<T>(&mut self, len: usize) -> Result<(), DecodeError> {
        if <Self::C as InternalLimitConfig>::LIMIT.is_some() {
            match len.checked_mul(core::mem::size_of::<T>()) {
                Some(val) => self.claim_bytes_read(val),
                None => Err(DecodeError::LimitExceeded),
            }
        } else {
            Ok(())
        }
    }

    /// Notify the decoder that `n` bytes are being reclaimed.
    ///
    /// When decoding container types, a typical implementation would claim to read `len * size_of::<T>()` bytes.
    /// This is to ensure that bincode won't allocate several GB of memory while constructing the container.
    ///
    /// Because the implementation claims `len * size_of::<T>()`, but then has to decode each `T`, this would be marked
    /// as double. This function allows us to un-claim each `T` that gets decoded.
    ///
    /// We cannot check if `len * size_of::<T>()` is valid without claiming it, because this would mean that if you have
    /// a nested container (e.g. `Vec<Vec<T>>`), it does not know how much memory is already claimed, and could easily
    /// allocate much more than the user intends.
    /// ```
    /// # use bincode::de::{Decode, Decoder};
    /// # use bincode::error::DecodeError;
    /// # struct Container<T>(Vec<T>);
    /// # impl<T> Container<T> {
    /// #     fn with_capacity(cap: usize) -> Self {
    /// #         Self(Vec::with_capacity(cap))
    /// #     }
    /// #     
    /// #     fn push(&mut self, t: T) {
    /// #         self.0.push(t);
    /// #     }
    /// # }
    /// impl<T: Decode> Decode for Container<T> {
    ///     fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
    ///         let len = u64::decode(decoder)?;
    ///         let len: usize = len.try_into().map_err(|_| DecodeError::OutsideUsizeRange(len))?;
    ///         // Make sure we don't allocate too much memory
    ///         decoder.claim_bytes_read(len * core::mem::size_of::<T>());
    ///
    ///         let mut result = Container::with_capacity(len);
    ///         for _ in 0..len {
    ///             // un-claim the memory
    ///             decoder.unclaim_bytes_read(core::mem::size_of::<T>());
    ///             result.push(T::decode(decoder)?)
    ///         }
    ///         Ok(result)
    ///     }
    /// }
    /// impl<'de, T: bincode::BorrowDecode<'de>> bincode::BorrowDecode<'de> for Container<T> {
    ///     fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
    ///         decoder: &mut D,
    ///     ) -> core::result::Result<Self, bincode::error::DecodeError> {
    ///         let len = u64::borrow_decode(decoder)?;
    ///         let len: usize = len.try_into().map_err(|_| DecodeError::OutsideUsizeRange(len))?;
    ///         // Make sure we don't allocate too much memory
    ///         decoder.claim_bytes_read(len * core::mem::size_of::<T>());
    ///
    ///         let mut result = Container::with_capacity(len);
    ///         for _ in 0..len {
    ///             // un-claim the memory
    ///             decoder.unclaim_bytes_read(core::mem::size_of::<T>());
    ///             result.push(T::borrow_decode(decoder)?)
    ///         }
    ///         Ok(result)
    ///     }
    /// }
    /// ```
    fn unclaim_bytes_read(&mut self, n: usize);
}

/// Any source that can decode basic types. This type is most notably implemented for [Decoder].
///
/// This is an extension of [Decode] that can also return borrowed data.
pub trait BorrowDecoder<'de>: Decoder {
    /// The concrete [BorrowReader] type
    type BR: BorrowReader<'de>;

    /// Rerturns a mutable reference to the borrow reader
    fn borrow_reader(&mut self) -> &mut Self::BR;
}

impl<'a, T> Decoder for &'a mut T
where
    T: Decoder,
{
    type R = T::R;

    type C = T::C;

    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }

    #[inline]
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), DecodeError> {
        T::claim_bytes_read(self, n)
    }

    #[inline]
    fn unclaim_bytes_read(&mut self, n: usize) {
        T::unclaim_bytes_read(self, n)
    }
}

impl<'a, 'de, T> BorrowDecoder<'de> for &'a mut T
where
    T: BorrowDecoder<'de>,
{
    type BR = T::BR;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        T::borrow_reader(self)
    }
}

/// Decodes only the option variant from the decoder. Will not read any more data than that.
#[inline]
pub(crate) fn decode_option_variant<D: Decoder>(
    decoder: &mut D,
    type_name: &'static str,
) -> Result<Option<()>, DecodeError> {
    let is_some = u8::decode(decoder)?;
    match is_some {
        0 => Ok(None),
        1 => Ok(Some(())),
        x => Err(DecodeError::UnexpectedVariant {
            found: x as u32,
            allowed: &crate::error::AllowedEnumVariants::Range { max: 1, min: 0 },
            type_name,
        }),
    }
}

/// Decodes the length of any slice, container, etc from the decoder
#[inline]
pub(crate) fn decode_slice_len<D: Decoder>(decoder: &mut D) -> Result<usize, DecodeError> {
    let v = u64::decode(decoder)?;

    v.try_into().map_err(|_| DecodeError::OutsideUsizeRange(v))
}
#[cfg(test)]
mod tests_rug_179 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        // Create the decoder
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        // Create the type name
        let p1: &'static str = "MyType";

        crate::de::decode_option_variant(&mut p0, p1).unwrap();
    }
}        
#[cfg(test)]
mod tests_rug_180 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_decode_slice_len() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        assert_eq!(decode_slice_len(&mut p0).unwrap(), 3usize);
    }
    
}


#[cfg(test)]
mod tests_rug_181 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        // Step 1: fill in the p0 variables
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        // Step 2: construct the p1 variable
        let p1: usize = 5;
        
        // call the target function
        crate::de::Decoder::claim_container_read::<usize>(&mut p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_182 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::BorrowDecoder;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicBool>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_183 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
            
        <std::sync::atomic::AtomicU8>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_184 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_borrow_decode() {
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicU16>::borrow_decode(&mut v12);
    }
}#[cfg(test)]
mod tests_rug_185 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
            
        <std::sync::atomic::AtomicU32>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_186 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicU64>::borrow_decode(&mut p0);
    }
}
            
#[cfg(test)]
mod tests_rug_188 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
                
        <std::sync::atomic::AtomicI8>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_189 {
    use super::*;
    use crate::de::{BorrowDecode, BorrowDecoder, DecodeError};
    use crate::Decode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        let result: Result<_, DecodeError> = <std::sync::atomic::AtomicI16>::borrow_decode(&mut p0);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests_rug_190 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
            

        <std::sync::atomic::AtomicI32>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_191 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::sync::atomic::AtomicI64>::borrow_decode(&mut p0).unwrap();
    }
}#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::sync::atomic::AtomicIsize>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::error::DecodeError;

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
                
        <std::string::String>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_194 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::boxed::Box<str>>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_195 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::ffi::CString>::borrow_decode(&mut p0);
      }
}
#[cfg(test)]
mod tests_rug_196 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::time::SystemTime>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_197 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::path::PathBuf>::borrow_decode(&mut p0);
    }
}


#[cfg(test)]
mod tests_rug_198 {
    use super::*;
    use crate::BorrowDecode;
    
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};


    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::net::IpAddr>::borrow_decode(&mut p0);

    }
}



#[cfg(test)]
mod tests_rug_199 {

    use super::*;
    use crate::BorrowDecode;

    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::net::Ipv4Addr>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_200 {
    use super::*;
    use crate::BorrowDecode;
    use crate::error::DecodeError;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: &mut DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = &mut DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::net::Ipv6Addr>::borrow_decode(p0);
    }
}
#[cfg(test)]
mod tests_rug_202 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: &mut DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            &mut DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::net::SocketAddrV4>::borrow_decode(p0);

    }
}#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::net::SocketAddrV6>::borrow_decode(&mut p0).unwrap();
    }
}#[cfg(test)]
mod tests_rug_204 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <bool>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_205 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
                
        <u8>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_206 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::num::NonZeroU8>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_207 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <u16 as BorrowDecode<'_>>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_208 {
    use super::*;
    use crate::{BorrowDecode, de::decoder::DecoderImpl, de::read::SliceReader};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        // Construct the first argument
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        // Call the target function
        <std::num::NonZeroU16>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_209 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <u32>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_210 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::num::NonZeroU32>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_211 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>>::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <u64>::borrow_decode(&mut v12);

    }
}
#[cfg(test)]
mod tests_rug_212 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::num::NonZeroU64>::borrow_decode(&mut p0);
    }
}

#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        <std::num::NonZeroU128>::borrow_decode(&mut p0);
    }
}

#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <usize>::borrow_decode(&mut p0);
    }
}

#[cfg(test)]
mod tests_rug_216 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        // Step 1: construct the sample p0 variable
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        // Step 2: fill in the p0 variable based on the sample
        let mut p0: &mut DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = &mut v12;

        <std::num::NonZeroUsize>::borrow_decode(p0);
    }
}
#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <i8>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::error::DecodeError;

    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        let result: Result<std::num::NonZeroI8, DecodeError> = <std::num::NonZeroI8 as BorrowDecode<'_>>::borrow_decode(&mut p0);
        assert!(result.is_ok());
    }
}#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <i16>::borrow_decode(&mut p0).unwrap();
    }
}#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::error::DecodeError;
    use crate::Decode;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        let _ = <std::num::NonZeroI16>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_221 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =    
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <i32>::borrow_decode(&mut p0).unwrap();
    }
}

#[cfg(test)]
mod tests_rug_222 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>>
            = DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

       <std::num::NonZeroI32>::borrow_decode(&mut p0);

    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <i64>::borrow_decode(&mut p0).unwrap();
    }
}#[cfg(test)]
mod tests_rug_224 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::num::NonZeroI64>::borrow_decode(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_225 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_borrow_decode() {
        let mut p0: &mut DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            &mut DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <i128>::borrow_decode(p0);
    }
}                
#[cfg(test)]
mod tests_rug_226 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <std::num::NonZeroI128>::borrow_decode(&mut v12).unwrap();
    }
}#[cfg(test)]
mod tests_rug_227 {
    use super::*;
    use crate::{BorrowDecode, de::decoder::DecoderImpl};
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
                
        <isize>::borrow_decode(&mut v12);
    }
}
#[cfg(test)]
mod tests_rug_228 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <std::num::NonZeroIsize>::borrow_decode(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_229 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::{DecoderImpl, read::SliceReader};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
                
        <f32>::borrow_decode(&mut p0).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_230 {
    use super::*;
    use crate::de::BorrowDecode;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::{Decode, error::DecodeError, config::{Configuration, BigEndian, Fixint, Limit}};

    #[test]
    fn test_rug() {
        let mut v12: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <f64>::borrow_decode(&mut v12).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_231 {
    use super::*;
    use crate::BorrowDecode;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    
    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        
        <char>::borrow_decode(&mut p0).unwrap();
    
    }
}#[cfg(test)]
mod tests_rug_232 {
    use super::*;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        <()>::borrow_decode(&mut p0).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_234 {
    use super::*;
    use crate::de::{BorrowDecode, decoder::DecoderImpl, read::SliceReader};
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use crate::error::DecodeError;
    use std::time::Duration;

    #[test]
    fn test_borrow_decode() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        let result: Result<Duration, DecodeError> = Duration::borrow_decode(&mut p0);
        assert!(result.is_ok());
    }
}#[cfg(test)]
mod tests_rug_235 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_reader() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        p0.reader();
    }
}#[cfg(test)]
mod tests_rug_236 {
    use super::*;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        p0.config();
    }
}#[cfg(test)]
mod tests_rug_237 {
    use super::*;
    use crate::de::{Decoder, DecodeError};
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_claim_bytes_read() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        let p1: usize = 10;

        p0.claim_bytes_read(p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_238 {
    use super::*;
    use crate::de::Decoder;
    use crate::de::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_rug() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> =
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());
        let p1: usize = 10;

        p0.unclaim_bytes_read(p1);

    }
}
#[cfg(test)]
mod tests_rug_239 {
    use super::*;
    use crate::de::BorrowDecoder;
    use crate::de::decoder::DecoderImpl;
    use crate::de::read::SliceReader;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};

    #[test]
    fn test_borrow_reader() {
        let mut p0: DecoderImpl<SliceReader<'_>, Configuration<BigEndian, Fixint, Limit<100>>> = 
            DecoderImpl::new(SliceReader::new(&[0u8, 1u8, 2u8]), Configuration::<BigEndian, Fixint, Limit<100>>::default());

        p0.borrow_reader();
    }
}