macro_rules! impl_sha3 {
    (
        $name:ident, $full_name:ident, $output_size:ident,
        $rate:ident, $pad:expr, $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " hasher state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            state: Sha3State,
        }

        impl HashMarker for $name {}

        impl BlockSizeUser for $name {
            type BlockSize = $rate;
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
                for block in blocks {
                    self.state.absorb_block(block)
                }
            }
        }

        impl FixedOutputCore for $name {
            #[inline]
            fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
                let pos = buffer.get_pos();
                let block = buffer.pad_with_zeros();
                block[pos] = $pad;
                let n = block.len();
                block[n - 1] |= 0x80;

                self.state.absorb_block(block);

                self.state.as_bytes(out);
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self {
                    state: Default::default(),
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
            fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(stringify!($full_name))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), " { ... }"))
            }
        }

        #[doc = $alg_name]
        #[doc = " hasher state."]
        pub type $full_name = CoreWrapper<$name>;
    };
    (
        $name:ident, $full_name:ident, $output_size:ident,
        $rate:ident, $pad:expr, $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_sha3!($name, $full_name, $output_size, $rate, $pad, $alg_name);

        #[cfg(feature = "oid")]
        #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
        impl AssociatedOid for $name {
            const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
        }
    };
}

macro_rules! impl_shake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
        $rate:ident, $pad:expr, $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " hasher state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            state: Sha3State,
        }

        impl HashMarker for $name {}

        impl BlockSizeUser for $name {
            type BlockSize = $rate;
        }

        impl BufferKindUser for $name {
            type BufferKind = Eager;
        }

        impl UpdateCore for $name {
            #[inline]
            fn update_blocks(&mut self, blocks: &[Block<Self>]) {
                for block in blocks {
                    self.state.absorb_block(block)
                }
            }
        }

        impl ExtendableOutputCore for $name {
            type ReaderCore = $reader;

            #[inline]
            fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
                let pos = buffer.get_pos();
                let block = buffer.pad_with_zeros();
                block[pos] = $pad;
                let n = block.len();
                block[n - 1] |= 0x80;

                self.state.absorb_block(block);
                $reader {
                    state: self.state.clone(),
                }
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self {
                    state: Default::default(),
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
            fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(stringify!($full_name))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), " { ... }"))
            }
        }

        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " reader state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $reader {
            state: Sha3State,
        }

        impl BlockSizeUser for $reader {
            type BlockSize = $rate;
        }

        impl XofReaderCore for $reader {
            #[inline]
            fn read_block(&mut self) -> Block<Self> {
                let mut block = Block::<Self>::default();
                self.state.as_bytes(&mut block);
                self.state.permute();
                block
            }
        }

        #[doc = $alg_name]
        #[doc = " hasher state."]
        pub type $full_name = CoreWrapper<$name>;

        #[doc = $alg_name]
        #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper<$reader>;
    };
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
        $rate:ident, $pad:expr, $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_shake!(
            $name,
            $full_name,
            $reader,
            $reader_full,
            $rate,
            $pad,
            $alg_name
        );

        #[cfg(feature = "oid")]
        #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
        impl AssociatedOid for $name {
            const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
        }
    };
}

macro_rules! impl_turbo_shake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
        $rate:ident, $alg_name:expr $(,)?
    ) => {
        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " hasher state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            domain_separation: u8,
            state: Sha3State,
        }

        impl $name {
            /// Creates a new TurboSHAKE instance with the given domain separation.
            /// Note that the domain separation needs to be a byte with a value in
            /// the range [0x01, . . . , 0x7F]
            pub fn new(domain_separation: u8) -> Self {
                assert!((0x01..=0x7F).contains(&domain_separation));
                Self {
                    domain_separation,
                    state: Sha3State::new(TURBO_SHAKE_ROUND_COUNT),
                }
            }
        }

        impl HashMarker for $name {}

        impl BlockSizeUser for $name {
            type BlockSize = $rate;
        }

        impl BufferKindUser for $name {
            type BufferKind = Eager;
        }

        impl UpdateCore for $name {
            #[inline]
            fn update_blocks(&mut self, blocks: &[Block<Self>]) {
                for block in blocks {
                    self.state.absorb_block(block)
                }
            }
        }

        impl ExtendableOutputCore for $name {
            type ReaderCore = $reader;

            #[inline]
            fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
                let pos = buffer.get_pos();
                let block = buffer.pad_with_zeros();
                block[pos] = self.domain_separation;
                let n = block.len();
                block[n - 1] |= 0x80;

                self.state.absorb_block(block);
                $reader {
                    state: self.state.clone(),
                }
            }
        }

        impl Reset for $name {
            #[inline]
            fn reset(&mut self) {
                *self = Self::new(self.domain_separation);
            }
        }

        impl AlgorithmName for $name {
            fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(stringify!($full_name))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), " { ... }"))
            }
        }

        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " reader state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $reader {
            state: Sha3State,
        }

        impl BlockSizeUser for $reader {
            type BlockSize = $rate;
        }

        impl XofReaderCore for $reader {
            #[inline]
            fn read_block(&mut self) -> Block<Self> {
                let mut block = Block::<Self>::default();
                self.state.as_bytes(&mut block);
                self.state.permute();
                block
            }
        }

        #[doc = $alg_name]
        #[doc = " hasher state."]
        pub type $full_name = CoreWrapper<$name>;

        #[doc = $alg_name]
        #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper<$reader>;
    };
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
        $rate:ident, $alg_name:expr, $oid:literal $(,)?
    ) => {
        impl_turbo_shake!($name, $full_name, $reader, $reader_full, $rate, $alg_name);

        #[cfg(feature = "oid")]
        #[cfg_attr(docsrs, doc(cfg(feature = "oid")))]
        impl AssociatedOid for $name {
            const OID: ObjectIdentifier = ObjectIdentifier::new_unwrap($oid);
        }
    };
}

macro_rules! impl_cshake {
    (
        $name:ident, $full_name:ident, $reader:ident, $reader_full:ident,
        $rate:ident, $shake_pad:expr, $cshake_pad:expr, $alg_name:expr,
    ) => {
        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " hasher state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            padding: u8,
            state: Sha3State,
            #[cfg(feature = "reset")]
            initial_state: Sha3State,
        }

        impl $name {
            /// Creates a new CSHAKE instance with the given customization.
            pub fn new(customization: &[u8]) -> Self {
                Self::new_with_function_name(&[], customization)
            }

            /// Creates a new CSHAKE instance with the given function name and customization.
            /// Note that the function name is intended for use by NIST and should only be set to
            /// values defined by NIST. You probably don't need to use this function.
            pub fn new_with_function_name(function_name: &[u8], customization: &[u8]) -> Self {
                let mut state = Sha3State::default();
                if function_name.is_empty() && customization.is_empty() {
                    return Self {
                        padding: $shake_pad,
                        state: state.clone(),
                        #[cfg(feature = "reset")]
                        initial_state: state,
                    };
                }

                let mut buffer = Buffer::<Self>::default();
                let mut b = [0u8; 9];
                buffer.digest_blocks(left_encode($rate::to_u64(), &mut b), |blocks| {
                    for block in blocks {
                        state.absorb_block(block);
                    }
                });
                buffer.digest_blocks(
                    left_encode((function_name.len() * 8) as u64, &mut b),
                    |blocks| {
                        for block in blocks {
                            state.absorb_block(block);
                        }
                    },
                );
                buffer.digest_blocks(function_name, |blocks| {
                    for block in blocks {
                        state.absorb_block(block);
                    }
                });
                buffer.digest_blocks(
                    left_encode((customization.len() * 8) as u64, &mut b),
                    |blocks| {
                        for block in blocks {
                            state.absorb_block(block);
                        }
                    },
                );
                buffer.digest_blocks(customization, |blocks| {
                    for block in blocks {
                        state.absorb_block(block);
                    }
                });
                state.absorb_block(buffer.pad_with_zeros());

                Self {
                    padding: $cshake_pad,
                    state: state.clone(),
                    #[cfg(feature = "reset")]
                    initial_state: state,
                }
            }
        }

        impl HashMarker for $name {}

        impl BlockSizeUser for $name {
            type BlockSize = $rate;
        }

        impl BufferKindUser for $name {
            type BufferKind = Eager;
        }

        impl UpdateCore for $name {
            #[inline]
            fn update_blocks(&mut self, blocks: &[Block<Self>]) {
                for block in blocks {
                    self.state.absorb_block(block)
                }
            }
        }

        impl ExtendableOutputCore for $name {
            type ReaderCore = $reader;

            #[inline]
            fn finalize_xof_core(&mut self, buffer: &mut Buffer<Self>) -> Self::ReaderCore {
                let pos = buffer.get_pos();
                let block = buffer.pad_with_zeros();
                block[pos] = self.padding;
                let n = block.len();
                block[n - 1] |= 0x80;

                self.state.absorb_block(block);
                $reader {
                    state: self.state.clone(),
                }
            }
        }

        #[cfg(feature = "reset")]
        impl Reset for $name {
            #[inline]
            fn reset(&mut self) {
                self.state = self.initial_state.clone();
            }
        }

        impl AlgorithmName for $name {
            fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(stringify!($full_name))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(concat!(stringify!($name), " { ... }"))
            }
        }

        #[doc = "Core "]
        #[doc = $alg_name]
        #[doc = " reader state."]
        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub struct $reader {
            state: Sha3State,
        }

        impl BlockSizeUser for $reader {
            type BlockSize = $rate;
        }

        impl XofReaderCore for $reader {
            #[inline]
            fn read_block(&mut self) -> Block<Self> {
                let mut block = Block::<Self>::default();
                self.state.as_bytes(&mut block);
                self.state.permute();
                block
            }
        }

        #[doc = $alg_name]
        #[doc = " hasher state."]
        pub type $full_name = CoreWrapper<$name>;

        #[doc = $alg_name]
        #[doc = " reader state."]
        pub type $reader_full = XofReaderCoreWrapper<$reader>;
    };
}
#[cfg(test)]
mod tests_rug_291 {
    use super::*;
    use crate::digest::Reset;
    use crate::Keccak224Core;
    
    #[test]
    fn test_rug() {
        let mut p0: Keccak224Core = Keccak224Core::default();
        
        <Keccak224Core as digest::Reset>::reset(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_293 {
    use super::*;
    use crate::digest::core_api::{BlockSizeUser, UpdateCore};
    use crate::{digest::generic_array::GenericArray, Keccak256Core};
    
    #[test]
    fn test_rug() {
        let mut p0 = Keccak256Core::default();
        let mut p1: [GenericArray<u8, <Keccak256Core as BlockSizeUser>::BlockSize>; 1] = Default::default();
        
        p0.update_blocks(&p1);
    }
}#[cfg(test)]
mod tests_rug_295 {
    use super::*;
    use crate::Keccak256Core;
    use std::default::Default;
    
    #[test]
    fn test_default() {
        let result: Keccak256Core = <Keccak256Core as Default>::default();
        // add assertions if necessary
        
        // example assertions
        // assert_eq!(result.state, 0);
    }
}
#[cfg(test)]
mod tests_rug_301 {
    use super::*;
    use crate::digest::Reset;
    use crate::Keccak384Core;

    #[test]
    fn test_rug() {
        let mut p0: Keccak384Core = Default::default();
        <Keccak384Core as digest::Reset>::reset(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_305 {
    use super::*;
    use crate::Keccak512Core;
    use std::default::Default;
    
    #[test]
    fn test_default() {
        <Keccak512Core as Default>::default();
    }
}#[cfg(test)]
mod tests_rug_306 {
    use super::*;
    use crate::digest::Reset;
    use crate::Keccak512Core;
    
    #[test]
    fn test_reset() {
        let mut p0: Keccak512Core = Default::default();
        
        <Keccak512Core as digest::Reset>::reset(&mut p0);
    
        // Add assertions here
        
    }
}
#[cfg(test)]
mod tests_rug_310 {
    use super::*;
    use crate::Keccak256FullCore;
    use std::default::Default;
    
    #[test]
    fn test_default() {
        let _: Keccak256FullCore = Default::default();
    }
}

use crate::digest::Reset;
use crate::Sha3_224Core;

#[cfg(test)]
mod tests_rug_316 {
    use super::*;

    #[test]
    fn test_reset() {
        let mut p0: Sha3_224Core = Sha3_224Core::default();

        <Sha3_224Core as digest::Reset>::reset(&mut p0);
    }
}
#[cfg(test)]
mod tests_rug_320 {
    use super::*;
    use crate::Sha3_256Core; // Import the relevant struct from the sha3 crate
    use std::default::Default; // Import the Default trait from std

    #[test]
    fn test_rug() {
        <Sha3_256Core as Default>::default();
    }
}#[cfg(test)]
mod tests_rug_321 {
    use super::*;
    use crate::digest::Reset;
    use crate::Sha3_256Core;

    #[test]
    fn test_rug() {
        let mut p0: Sha3_256Core = Sha3_256Core::default();

        p0.reset();
    }
}
#[cfg(test)]
mod tests_rug_326 {
    use super::*;
    use crate::digest::Reset;
    use crate::Sha3_384Core;
    
    #[test]
    fn test_reset() {
        let mut p0: Sha3_384Core = Default::default();

        p0.reset();
    }
}
#[cfg(test)]
mod tests_rug_330 {
    use super::*;
    use crate::Sha3_512Core;
    
    #[test]
    fn test_default() {
        <Sha3_512Core as std::default::Default>::default();
    }
}
#[cfg(test)]
mod tests_rug_331 {
    use super::*;
    use crate::digest::Reset;
    use crate::digest::Digest;
    use crate::Sha3_512Core;

    #[test]
    fn test_rug() {
        let mut p0: Sha3_512Core = Default::default();

        p0.reset();
        
        // Add assertions here if needed

    }
}
#[cfg(test)]
mod tests_rug_336 {
    use super::*;
    use crate::digest::Reset;
    use crate::Shake128Core;

    #[test]
    fn test_rug() {
        let mut p0: Shake128Core = Default::default();
        <Shake128Core as digest::Reset>::reset(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_345 {
    use super::*;
    use crate::TurboShake128Core;

    #[test]
    fn test_rug() {
        let mut p0: u8 = 0x7F;

        TurboShake128Core::new(p0);
    }
}#[cfg(test)]
        mod tests_rug_351 {
            use super::*;
            use crate::{TurboShake256Core, Sha3State, TURBO_SHAKE_ROUND_COUNT};
            
            #[test]
            fn test_rug() {
                let mut p0: u8 = 0x01;  // Sample data

                TurboShake256Core::new(p0);

            }
        }#[cfg(test)]
mod tests_rug_354 {
    use super::*;
    use crate::digest::Reset;
    use crate::TurboShake256Core;
    
    #[test]
    fn test_rug() {
        let mut p0 = TurboShake256Core::new(0);

        <TurboShake256Core as digest::Reset>::reset(&mut p0);
    }
}#[cfg(test)]
mod tests_rug_357 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: &[u8] = &[0x01, 0x02, 0x03];
        
        crate::CShake128Core::new(p0);

    }
}#[cfg(test)]
mod tests_rug_358 {
    use super::*;
    use crate::CShake128Core;

    #[test]
    fn test_rug() {
        let mut p0 = b"Function Name";
        let mut p1 = b"Customization";

        CShake128Core::new_with_function_name(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_363 {
    use super::*;
    use crate::CShake256Core;

    #[test]
    fn test_rug() {
        let mut p0: &[u8] = b"customization data";

        <CShake256Core>::new(p0);

    }
}
#[cfg(test)]
mod tests_rug_364 {
    use super::*;
    use crate::{CShake256Core, Sha3State, Buffer};

    #[test]
    fn test_rug() {
        let p0: &[u8] = b"function_name";
        let p1: &[u8] = b"customization";

        CShake256Core::new_with_function_name(p0, p1);
    }
}