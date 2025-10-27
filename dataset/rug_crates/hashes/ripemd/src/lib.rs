//! An implementation of the [RIPEMD] cryptographic hash.
//!
//! This crate implements only the modified 1996 versions, not the original
//! one from 1992.
//!
//! Note that RIPEMD-256 provides only the same security as RIPEMD-128,
//! and RIPEMD-320 provides only the same security as RIPEMD-160.
//!
//! # Usage
//!
//! ```rust
//! use hex_literal::hex;
//! use ripemd::{Ripemd160, Ripemd320, Digest};
//!
//! // create a RIPEMD-160 hasher instance
//! let mut hasher = Ripemd160::new();
//!
//! // process input message
//! hasher.update(b"Hello world!");
//!
//! // acquire hash digest in the form of GenericArray,
//! // which in this case is equivalent to [u8; 20]
//! let result = hasher.finalize();
//! assert_eq!(result[..], hex!("7f772647d88750add82d8e1a7a3e5c0902a346a3"));
//!
//! // same for RIPEMD-320
//! let mut hasher = Ripemd320::new();
//! hasher.update(b"Hello world!");
//! let result = hasher.finalize();
//! assert_eq!(&result[..], &hex!("
//!     f1c1c231d301abcf2d7daae0269ff3e7bc68e623
//!     ad723aa068d316b056d26b7d1bb6f0cc0f28336d
//! ")[..]);
//! ```
//!
//! Also see [RustCrypto/hashes] readme.
//!
//! [RIPEMD]: https://en.wikipedia.org/wiki/RIPEMD
//! [RustCrypto/hashes]: https://github.com/RustCrypto/hashes

#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub use digest::{self, Digest};

use core::fmt;
#[cfg(feature = "oid")]
use digest::const_oid::{AssociatedOid, ObjectIdentifier};
use digest::{
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, FixedOutputCore,
        OutputSizeUser, Reset, UpdateCore,
    },
    typenum::{Unsigned, U16, U20, U32, U40, U64},
    HashMarker, Output,
};

mod c128;
mod c160;
mod c256;
mod c320;

macro_rules! impl_ripemd {
    (
        $name:ident, $wrapped_name:ident, $mod:ident,
        $alg_width:expr, $doc_name:expr, $output_size:ty $(,)?
    ) => {
        #[doc = "Core block-level"]
        #[doc = $doc_name]
        #[doc = " hasher state."]
        #[derive(Clone)]
        pub struct $name {
            h: [u32; $mod::DIGEST_BUF_LEN],
            block_len: u64,
        }

        impl HashMarker for $name {}

        impl BlockSizeUser for $name {
            type BlockSize = U64;
        }

        impl BufferKindUser for $name {
            type BufferKind = Eager;
        }

        impl OutputSizeUser for $name {
            type OutputSize = $output_size;
        }

        impl UpdateCore for $name {
            #[inline]
            fn update_blocks(&mut self, blocks: &[Block<Self>]) {
                // Assumes that `block_len` does not overflow
                self.block_len += blocks.len() as u64;
                for block in blocks {
                    $mod::compress(&mut self.h, block.as_ref());
                }
            }
        }

        impl FixedOutputCore for $name {
            #[inline]
            fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
                let bs = Self::BlockSize::U64;
                let bit_len = 8 * (buffer.get_pos() as u64 + bs * self.block_len);
                let mut h = self.h;
                buffer.len64_padding_le(bit_len, |block| $mod::compress(&mut h, block.as_ref()));

                for (chunk, v) in out.chunks_exact_mut(4).zip(h.iter()) {
                    chunk.copy_from_slice(&v.to_le_bytes());
                }
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self {
                    h: $mod::H0,
                    block_len: 0,
                }
            }
        }

        impl Reset for $name {
            #[inline]
            fn reset(&mut self) {
                *self = Default::default();
            }
        }

        impl AlgorithmName for $name {
            #[inline]
            fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!("Ripemd", $alg_width))
            }
        }

        impl fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!("Ripemd", $alg_width, "Core { ... }"))
            }
        }

        #[doc = $doc_name]
        #[doc = " hasher."]
        pub type $wrapped_name = CoreWrapper<$name>;
    };
}

impl_ripemd!(Ripemd128Core, Ripemd128, c128, "128", "RIPEMD-128", U16);
impl_ripemd!(Ripemd160Core, Ripemd160, c160, "160", "RIPEMD-160", U20);
impl_ripemd!(Ripemd256Core, Ripemd256, c256, "256", "RIPEMD-256", U32);
impl_ripemd!(Ripemd320Core, Ripemd320, c320, "320", "RIPEMD-320", U40);

#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd128Core {
    /// The OID used for the RIPEMD-160. There are two OIDs defined. The Teletrust one (which is
    /// used by almost anybody, including BouncyCastle, OpenSSL, GnuTLS, etc. and the ISO one
    /// (1.0.10118.3.0.50), which seems to be used by nobody.
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.2");
}

#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd160Core {
    /// The OID used for the RIPEMD-160. There are two OIDs defined. The Teletrust one (which is
    /// used by almost anybody, including BouncyCastle, OpenSSL, GnuTLS, etc. and the ISO one
    /// (1.0.10118.3.0.49), which seems to be used by Go and nobody else.
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.1");
}

#[cfg(feature = "oid")]
#[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
impl AssociatedOid for Ripemd256Core {
    const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.3.36.3.2.3");
}
#[cfg(test)]
mod tests_rug_192 {
    use super::*;
    use crate::digest::core_api::UpdateCore;
    use digest::generic_array::GenericArray;
    use crate::digest::generic_array::typenum::{U64};
    
    #[test]
    fn test_rug() {
        let mut p0 = Ripemd128Core::default();
        let mut p1 = GenericArray::<u8, U64>::default();

        <Ripemd128Core as UpdateCore>::update_blocks(&mut p0, &[p1]);
    }
}#[cfg(test)]
mod tests_rug_193 {
    use super::*;
    use crate::digest::core_api::FixedOutputCore;
    use digest::core_api::{UpdateCore, BlockSizeUser, BufferKindUser, OutputSizeUser};
    use digest::generic_array::{GenericArray, typenum::Unsigned};
    use digest::block_buffer::BlockBuffer;
    
    #[test]
    fn test_rug() {
        let mut p0 = Ripemd128Core::default();
        let mut p1 = BlockBuffer::<
            <Ripemd128Core as BlockSizeUser>::BlockSize,
            <Ripemd128Core as BufferKindUser>::BufferKind,
        >::default();
        let mut p2: GenericArray<u8, <Ripemd128Core as OutputSizeUser>::OutputSize> = GenericArray::default();
        
        <Ripemd128Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
    }
}#[cfg(test)]
mod tests_rug_195 {
    use super::*;
    use crate::digest::Reset;
    use crate::Ripemd128Core;

    #[test]
    fn test_reset() {
        let mut p0: Ripemd128Core = Default::default();

        <Ripemd128Core as digest::Reset>::reset(&mut p0);
        
        // Assert statements if necessary
    }
}#[cfg(test)]
mod tests_rug_197 {
    use super::*;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use digest::generic_array::GenericArray;
    
    #[test]
    fn test_rug() {
        let mut p0: Ripemd160Core = Ripemd160Core::default();
        let p1: GenericArray<u8, <Ripemd160Core as BlockSizeUser>::BlockSize> = GenericArray::default();
                
        <Ripemd160Core as UpdateCore>::update_blocks(&mut p0, &[p1]);
    }
}#[cfg(test)]
mod tests_rug_199 {
    use super::*;

    #[test]
    fn test_rug() {
        use crate::Ripemd160Core;
        use core::default::Default;

        <Ripemd160Core as Default>::default();
    }
}#[cfg(test)]
mod tests_rug_200 {
    use super::*;
    use crate::digest::Reset;
    use crate::Ripemd160Core;

    #[test]
    fn test_rug() {
        let mut p0: Ripemd160Core = Default::default();

        <Ripemd160Core as digest::Reset>::reset(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_202 {
    use super::*;
    use crate::digest::core_api::{UpdateCore, Block, BlockSizeUser};
    use digest::generic_array::GenericArray;
    use digest::generic_array::typenum::{U64};
    
    #[test]
    fn test_rug() {
        let mut p0: Ripemd256Core = Ripemd256Core {
            h: [0; 8],
            block_len: 0,
        };
        
        let p1: [Block<Ripemd256Core>; 1] = [
            GenericArray::<u8, U64>::default().into(),
        ];
        
        p0.update_blocks(&p1);
    }
}
#[cfg(test)]
mod tests_rug_203 {
    use super::*;
    use crate::digest::core_api::FixedOutputCore;
    use crate::digest::block_buffer::BlockBuffer;
    use crate::digest::generic_array::{GenericArray, ArrayLength};
    use crate::digest::{OutputSizeUser, core_api::{UpdateCore, BlockSizeUser, BufferKindUser}};

    #[test]
    fn test_rug() {
        let mut p0 = Ripemd256Core::default();
        let mut p1: BlockBuffer<<Ripemd256Core as BlockSizeUser>::BlockSize, <Ripemd256Core as BufferKindUser>::BufferKind> = Default::default();
        let mut p2: GenericArray<u8, <Ripemd256Core as OutputSizeUser>::OutputSize> = Default::default();

        <Ripemd256Core as FixedOutputCore>::finalize_fixed_core(&mut p0, &mut p1, &mut p2);
    }
}#[cfg(test)]
mod tests_rug_204 {
    use super::*;
    use crate::Ripemd256Core;
    use core::default::Default;
    
    #[test]
    fn test_rug() {
        <Ripemd256Core as core::default::Default>::default();
    }
}
#[cfg(test)]
mod tests_rug_205 {
    use super::*;
    use crate::digest::Reset;
    use crate::digest::generic_array::GenericArray;
    use crate::digest::generic_array::typenum::U32;

    #[test]
    fn test_rug() {
        let mut p0: Ripemd256Core = Ripemd256Core::default();

        <Ripemd256Core as digest::Reset>::reset(&mut p0);
    }
}

#[cfg(test)]
mod tests_rug_207 {
    use super::*;
    use crate::digest::core_api::{UpdateCore, BlockSizeUser};
    use crate::digest::generic_array::GenericArray;
    
    #[test]
    fn test_rug() {
        let mut p0 = Ripemd320Core::default();
        let p1: GenericArray<u8, <Ripemd320Core as BlockSizeUser>::BlockSize> = GenericArray::default();
        
        <Ripemd320Core as UpdateCore>::update_blocks(&mut p0, &[p1]);
    }
}
#[cfg(test)]
mod tests_rug_209 {
    use super::*;
    use crate::Ripemd320Core;
    use core::default::Default;
    
    #[test]
    fn test_rug() {
        <Ripemd320Core as core::default::Default>::default();
    }
}
#[cfg(test)]
mod tests_rug_210 {
    use super::*;
    use crate::digest::Reset;
    use crate::digest::generic_array::GenericArray;
    use crate::digest::generic_array::typenum::{U0, U64};
    use crate::{Digest, Ripemd320};

    #[test]
    fn test_rug() {
        let mut p0: Ripemd320Core = Default::default();
        <Ripemd320Core as digest::Reset>::reset(&mut p0);
        // assert statements go here
    }
}