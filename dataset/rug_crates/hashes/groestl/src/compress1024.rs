#![allow(clippy::needless_range_loop)]
use crate::table::TABLE;
use core::{convert::TryInto, u64};

pub(crate) const COLS: usize = 16;
const ROUNDS: u64 = 14;

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
        column(&x, [1, 3, 5, 11, 0, 2, 4, 6]),
        column(&x, [2, 4, 6, 12, 1, 3, 5, 7]),
        column(&x, [3, 5, 7, 13, 2, 4, 6, 8]),
        column(&x, [4, 6, 8, 14, 3, 5, 7, 9]),
        column(&x, [5, 7, 9, 15, 4, 6, 8, 10]),
        column(&x, [6, 8, 10, 0, 5, 7, 9, 11]),
        column(&x, [7, 9, 11, 1, 6, 8, 10, 12]),
        column(&x, [8, 10, 12, 2, 7, 9, 11, 13]),
        column(&x, [9, 11, 13, 3, 8, 10, 12, 14]),
        column(&x, [10, 12, 14, 4, 9, 11, 13, 15]),
        column(&x, [11, 13, 15, 5, 10, 12, 14, 0]),
        column(&x, [12, 14, 0, 6, 11, 13, 15, 1]),
        column(&x, [13, 15, 1, 7, 12, 14, 0, 2]),
        column(&x, [14, 0, 2, 8, 13, 15, 1, 3]),
        column(&x, [15, 1, 3, 9, 14, 0, 2, 4]),
        column(&x, [0, 2, 4, 10, 15, 1, 3, 5]),
    ]
}

#[inline(always)]
fn rndp(mut x: [u64; COLS], r: u64) -> [u64; COLS] {
    for i in 0..COLS {
        x[i] ^= ((i as u64) << 60) ^ r;
    }
    [
        column(&x, [0, 1, 2, 3, 4, 5, 6, 11]),
        column(&x, [1, 2, 3, 4, 5, 6, 7, 12]),
        column(&x, [2, 3, 4, 5, 6, 7, 8, 13]),
        column(&x, [3, 4, 5, 6, 7, 8, 9, 14]),
        column(&x, [4, 5, 6, 7, 8, 9, 10, 15]),
        column(&x, [5, 6, 7, 8, 9, 10, 11, 0]),
        column(&x, [6, 7, 8, 9, 10, 11, 12, 1]),
        column(&x, [7, 8, 9, 10, 11, 12, 13, 2]),
        column(&x, [8, 9, 10, 11, 12, 13, 14, 3]),
        column(&x, [9, 10, 11, 12, 13, 14, 15, 4]),
        column(&x, [10, 11, 12, 13, 14, 15, 0, 5]),
        column(&x, [11, 12, 13, 14, 15, 0, 1, 6]),
        column(&x, [12, 13, 14, 15, 0, 1, 2, 7]),
        column(&x, [13, 14, 15, 0, 1, 2, 3, 8]),
        column(&x, [14, 15, 0, 1, 2, 3, 4, 9]),
        column(&x, [15, 0, 1, 2, 3, 4, 5, 10]),
    ]
}

pub(crate) fn compress(h: &mut [u64; COLS], block: &[u8; 128]) {
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
mod tests_rug_138 {
    use super::*;
    
    #[test]
    fn test_column() {
        let mut p0: [u64; COLS] = [0; COLS]; // Sample data for p0
        let p1: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7]; // Sample data for p1

        crate::compress1024::column(&p0, p1);
    }
}#[cfg(test)]
mod tests_rug_139 {
    use super::*;
    use crate::compress1024::{COLS, column};
    
    #[test]
    fn test_rug() {
        let mut p0: [u64; COLS] = [0; COLS];
        let p1: u64 = 12345;
        
        crate::compress1024::rndq(p0, p1);
        
        // Add assertions or other test logic here
    }
}        
#[cfg(test)]
mod tests_rug_140 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: [u64; COLS] = [0; COLS];
        let p1: u64 = 123456789;

        crate::compress1024::rndp(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_141 {
    use super::*;
    
    #[test]
    fn test_compress() {
        let mut h = [0u64; COLS];
        let mut block = [0u8; 128];
        
        // Sample data for h
        h[0] = 12345;
        h[1] = 67890;
        
        // Sample data for block
        block[0] = 1;
        block[1] = 2;
        
        crate::compress1024::compress(&mut h, &block);
        
        // Add your assertions here based on the expected output
    }
}#[cfg(test)]
mod tests_rug_142 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u64; COLS] = [
            0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef,
            0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef,
            0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef,
            0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef, 0x0123456789abcdef,
        ];

        crate::compress1024::p(&p0);
    }
}