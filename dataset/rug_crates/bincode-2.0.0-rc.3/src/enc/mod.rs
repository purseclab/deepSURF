//! Encoder-based structs and traits.

mod encoder;
mod impl_tuples;
mod impls;

use self::write::Writer;
use crate::{config::Config, error::EncodeError, utils::Sealed};

pub mod write;

pub use self::encoder::EncoderImpl;

/// Any source that can be encoded. This trait should be implemented for all types that you want to be able to use with any of the `encode_with` methods.
///
/// This trait will be automatically implemented if you enable the `derive` feature and add `#[derive(bincode::Encode)]` to your trait.
///
/// # Implementing this trait manually
///
/// If you want to implement this trait for your type, the easiest way is to add a `#[derive(bincode::Encode)]`, build and check your `target/generated/bincode/` folder. This should generate a `<Struct name>_Encode.rs` file.
///
/// For this struct:
///
/// ```
/// struct Entity {
///     pub x: f32,
///     pub y: f32,
/// }
/// ```
/// It will look something like:
///
/// ```
/// # struct Entity {
/// #     pub x: f32,
/// #     pub y: f32,
/// # }
/// impl bincode::Encode for Entity {
///     fn encode<E: bincode::enc::Encoder>(
///         &self,
///         encoder: &mut E,
///     ) -> core::result::Result<(), bincode::error::EncodeError> {
///         bincode::Encode::encode(&self.x, encoder)?;
///         bincode::Encode::encode(&self.y, encoder)?;
///         Ok(())
///     }
/// }
/// ```
///
/// From here you can add/remove fields, or add custom logic.

pub trait Encode {
    /// Encode a given type.
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError>;
}

/// Helper trait to encode basic types into.
pub trait Encoder: Sealed {
    /// The concrete [Writer] type
    type W: Writer;

    /// The concrete [Config] type
    type C: Config;

    /// Returns a mutable reference to the writer
    fn writer(&mut self) -> &mut Self::W;

    /// Returns a reference to the config
    fn config(&self) -> &Self::C;
}

impl<'a, T> Encoder for &'a mut T
where
    T: Encoder,
{
    type W = T::W;

    type C = T::C;

    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }
}

/// Encode the variant of the given option. Will not encode the option itself.
#[inline]
pub(crate) fn encode_option_variant<E: Encoder, T>(
    encoder: &mut E,
    value: &Option<T>,
) -> Result<(), EncodeError> {
    match value {
        None => 0u8.encode(encoder),
        Some(_) => 1u8.encode(encoder),
    }
}

/// Encodes the length of any slice, container, etc into the given encoder
#[inline]
pub(crate) fn encode_slice_len<E: Encoder>(encoder: &mut E, len: usize) -> Result<(), EncodeError> {
    (len as u64).encode(encoder)
}

#[cfg(test)]
mod tests_rug_283 {
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
        
        let mut p0 = &mut v19;
        let p1: Option<i32> = Some(10);
        
        crate::enc::encode_option_variant(p0, &p1);

    }
}

#[cfg(test)]
mod tests_rug_284 {
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
        let p0 = &mut v19;
        let p1: usize = 10;

        crate::enc::encode_slice_len(p0, p1).unwrap();
    }
}
#[cfg(test)]
mod tests_rug_285 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_writer() {
        // Construct EncoderImpl
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        // Call writer() method and assert the result
        let res: &mut <EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> as Encoder>::W = v19.writer();
        // Further assertions...
    }
}#[cfg(test)]
mod tests_rug_286 {
    use super::*;
    use crate::enc::{Encoder, EncoderImpl};
    use crate::features::IoWriter;
    use crate::config::{Configuration, BigEndian, Fixint, Limit};
    use std::vec::Vec;

    #[test]
    fn test_rug() {
        let mut writer: Vec<u8> = Vec::new();
        let mut v16: IoWriter<Vec<u8>> = IoWriter::new(&mut writer);
        let config = Configuration::<BigEndian, Fixint, Limit<100>>::default();
        let mut v19: EncoderImpl<IoWriter<Vec<u8>>, Configuration<BigEndian, Fixint, Limit<100>>> = EncoderImpl::new(v16, config);
        
        v19.config();
    }
}