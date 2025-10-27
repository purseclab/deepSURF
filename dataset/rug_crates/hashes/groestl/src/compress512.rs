#![allow(clippy::needless_range_loop)]
use crate::table::TABLE;
use core::{convert::TryInto, u64};

pub(crate) const COLS: usize = 8;
const ROUNDS: u64 = 10;

#[inline(always)]
fn column(x: &[u64; COLS], c: [usize; 8]) -> u64 {
    let mut t = 0;
    for i in 0..8 {
        let sl = 8 * (7 - i);
        let idx = ((x[c[i]] >> sl) & 0xFF) as usize;
        t ^= TABLE[i][idx];
    }
    t
}

#[inline(always)]
fn rndq(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= u64::MAX.wrapping_sub((i as u64) << 4) ^ r;
    }
    [
        column(&x, [1, 3, 5, 7, 0, 2, 4, 6]),
        column(&x, [2, 4, 6, 0, 1, 3, 5, 7]),
        column(&x, [3, 5, 7, 1, 2, 4, 6, 0]),
        column(&x, [4, 6, 0, 2, 3, 5, 7, 1]),
        column(&x, [5, 7, 1, 3, 4, 6, 0, 2]),
        column(&x, [6, 0, 2, 4, 5, 7, 1, 3]),
        column(&x, [7, 1, 3, 5, 6, 0, 2, 4]),
        column(&x, [0, 2, 4, 6, 7, 1, 3, 5]),
    ]
}

#[inline(always)]
fn rndp(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= ((i as u64) << 60) ^ r;
    }
    [
        column(&x, [0, 1, 2, 3, 4, 5, 6, 7]),
        column(&x, [1, 2, 3, 4, 5, 6, 7, 0]),
        column(&x, [2, 3, 4, 5, 6, 7, 0, 1]),
        column(&x, [3, 4, 5, 6, 7, 0, 1, 2]),
        column(&x, [4, 5, 6, 7, 0, 1, 2, 3]),
        column(&x, [5, 6, 7, 0, 1, 2, 3, 4]),
        column(&x, [6, 7, 0, 1, 2, 3, 4, 5]),
        column(&x, [7, 0, 1, 2, 3, 4, 5, 6]),
    ]
}

pub(crate) fn compress(h: &mut [u64; COLS], block: &[u8; 64]) {
    let mut q = [0u64; COLS];
    for (chunk, v) in block.chunks_exact(8).zip(q.iter_mut()) {
        *v = u64::from_be_bytes(chunk.try_into().unwrap());
    }
    let mut p = [0u64; COLS];
    for i in 0..COLS {
        p[i] = h[i] ^ q[i];
    }
    for i in 0..ROUNDS {
        q = rndq(q, i);
    }
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        h[i] ^= q[i] ^ p[i];
    }
}

pub(crate) fn p(h: &[u64; COLS]) -> [u64; COLS] {
    let mut p = *h;
    for i in 0..ROUNDS {
        p = rndp(p, i << 56);
    }
    for i in 0..COLS {
        p[i] ^= h[i];
    }
    p
}
#[cfg(test)]
mod tests_rug_143 {
    use super::*;
    use crate::compress512::COLS;
    use crate::compress512::TABLE;

    #[test]
    fn test_column() {
        let mut p0: [u64; COLS] = [0; COLS];
        let mut p1: [usize; 8] = [0; 8];

        // Initialize sample data for p0 and p1 here

        column(&p0, p1);
    }
}#[cfg(test)]
mod tests_rug_144 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u64; 8] = [0; 8];
        let mut p1: u64 = 0;
        
        // Initialize sample data for p0
        let p0_sample: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        p0.copy_from_slice(&p0_sample);
        
        // Initialize sample data for p1
        let p1_sample: u64 = 42;
        
        crate::compress512::rndq(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_145 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0 = [0u64; COLS];
        let mut p1 = 0u64;

        // Sample data for p0
        p0[0] = 123;
        p0[1] = 456;
        // ... initialize the other elements of p0
        
        // Sample data for p1
        p1 = 789;
        
        crate::compress512::rndp(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_146 {
    use super::*;
    use crate::compress512::{compress, COLS, ROUNDS, rndq, rndp};
    
    #[test]
    fn test_rug() {
        let mut h = [0u64; COLS];
        let mut block = [0u8; 64];
        
        // Initialize h with some sample data
        // Example:
        h[0] = 0x0123456789ABCDEF;
        h[1] = 0xFEDCBA9876543210;
        
        // Initialize block with some sample data
        // Example:
        block[0] = 0x01;
        block[1] = 0x23;
        
        compress(&mut h, &block);
        
        // Add assertions here to validate the compression
    }
}
#[cfg(test)]
mod tests_rug_147 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u64; COLS] = [0; COLS];
        p0[0] = 0x0123456789ABCDEF;
        p0[1] = 0xFEDCBA9876543210;
        p0[2] = 0x13579BDF2468ACE0;
        p0[3] = 0xECA86420BDF13579;

        crate::compress512::p(&p0);
    }
}