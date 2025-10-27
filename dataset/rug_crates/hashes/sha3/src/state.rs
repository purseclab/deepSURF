use core::convert::TryInto;
#[cfg(feature = "zeroize")]
use zeroize::{Zeroize, ZeroizeOnDrop};

const PLEN: usize = 25;
const DEFAULT_ROUND_COUNT: usize = 24;

#[derive(Clone)]
pub(crate) struct Sha3State {
    pub state: [u64; PLEN],
    round_count: usize,
}

impl Default for Sha3State {
    fn default() -> Self {
        Self {
            state: [0u64; PLEN],
            round_count: DEFAULT_ROUND_COUNT,
        }
    }
}

#[cfg(feature = "zeroize")]
impl Drop for Sha3State {
    fn drop(&mut self) {
        self.state.zeroize();
    }
}

#[cfg(feature = "zeroize")]
impl ZeroizeOnDrop for Sha3State {}

impl Sha3State {
    pub(crate) fn new(round_count: usize) -> Self {
        Self {
            state: [0u64; PLEN],
            round_count,
        }
    }

    #[inline(always)]
    pub(crate) fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len() % 8, 0);

        for (b, s) in block.chunks_exact(8).zip(self.state.iter_mut()) {
            *s ^= u64::from_le_bytes(b.try_into().unwrap());
        }

        keccak::p1600(&mut self.state, self.round_count);
    }

    #[inline(always)]
    pub(crate) fn as_bytes(&self, out: &mut [u8]) {
        for (o, s) in out.chunks_mut(8).zip(self.state.iter()) {
            o.copy_from_slice(&s.to_le_bytes()[..o.len()]);
        }
    }

    #[inline(always)]
    pub(crate) fn permute(&mut self) {
        keccak::p1600(&mut self.state, self.round_count);
    }
}
#[cfg(test)]
mod tests_rug_283 {
    use super::*;
    use crate::state::Sha3State;
    use std::default::Default;
    
    #[test]
    fn test_default() {
        <Sha3State as Default>::default();
    }
}
#[cfg(test)]
mod tests_rug_284 {
    use super::*;
    use crate::state::Sha3State;

    #[test]
    fn test_rug() {
        let round_count: usize = 12;
        
        Sha3State::new(round_count);
    }
}
#[cfg(test)]
mod tests_rug_285 {
    use super::*;
    use crate::state::Sha3State;

    #[test]
    fn test_absorb_block() {
        let mut p0 = Sha3State::default();
        let p1: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        
        p0.absorb_block(p1);
    }
}#[cfg(test)]
mod tests_rug_286 {
    use super::*;
    use crate::state::Sha3State;

    #[test]
    fn test_rug() {
        let mut p0 = Sha3State::default();
        let mut p1 = [0u8; 64];

        Sha3State::as_bytes(&p0, &mut p1);
    }
}
#[cfg(test)]
mod tests_rug_287 {
    use super::*;
    use crate::state::Sha3State;

    #[test]
    fn test_rug() {
        let mut p0 = Sha3State::default();

        Sha3State::permute(&mut p0);
    }
}
