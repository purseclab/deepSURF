#![allow(clippy::many_single_char_names)]
use super::BLOCK_SIZE;
use core::convert::TryInto;

const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];

#[inline(always)]
fn add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ]
}

#[inline(always)]
fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

#[inline]
pub fn sha1_first_add(e: u32, w0: [u32; 4]) -> [u32; 4] {
    let [a, b, c, d] = w0;
    [e.wrapping_add(a), b, c, d]
}

fn sha1msg1(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let [_, _, w2, w3] = a;
    let [w4, w5, _, _] = b;
    [a[0] ^ w2, a[1] ^ w3, a[2] ^ w4, a[3] ^ w5]
}

fn sha1msg2(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    let [x0, x1, x2, x3] = a;
    let [_, w13, w14, w15] = b;

    let w16 = (x0 ^ w13).rotate_left(1);
    let w17 = (x1 ^ w14).rotate_left(1);
    let w18 = (x2 ^ w15).rotate_left(1);
    let w19 = (x3 ^ w16).rotate_left(1);

    [w16, w17, w18, w19]
}

#[inline]
fn sha1_first_half(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    sha1_first_add(abcd[0].rotate_left(30), msg)
}

fn sha1_digest_round_x4(abcd: [u32; 4], work: [u32; 4], i: i8) -> [u32; 4] {
    match i {
        0 => sha1rnds4c(abcd, add(work, [K[0]; 4])),
        1 => sha1rnds4p(abcd, add(work, [K[1]; 4])),
        2 => sha1rnds4m(abcd, add(work, [K[2]; 4])),
        3 => sha1rnds4p(abcd, add(work, [K[3]; 4])),
        _ => unreachable!("unknown icosaround index"),
    }
}

fn sha1rnds4c(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => {
            $c ^ ($a & ($b ^ $c))
        };
    } // Choose, MD5F, SHA1C

    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_202!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_202!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_202!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_202!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    [b, c, d, e]
}

fn sha1rnds4p(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_150 {
        ($a:expr, $b:expr, $c:expr) => {
            $a ^ $b ^ $c
        };
    } // Parity, XOR, MD5H, SHA1P

    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_150!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_150!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_150!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_150!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    [b, c, d, e]
}

fn sha1rnds4m(abcd: [u32; 4], msg: [u32; 4]) -> [u32; 4] {
    let [mut a, mut b, mut c, mut d] = abcd;
    let [t, u, v, w] = msg;
    let mut e = 0u32;

    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) ^ ($a & $c) ^ ($b & $c)
        };
    } // Majority, SHA1M

    e = e
        .wrapping_add(a.rotate_left(5))
        .wrapping_add(bool3ary_232!(b, c, d))
        .wrapping_add(t);
    b = b.rotate_left(30);

    d = d
        .wrapping_add(e.rotate_left(5))
        .wrapping_add(bool3ary_232!(a, b, c))
        .wrapping_add(u);
    a = a.rotate_left(30);

    c = c
        .wrapping_add(d.rotate_left(5))
        .wrapping_add(bool3ary_232!(e, a, b))
        .wrapping_add(v);
    e = e.rotate_left(30);

    b = b
        .wrapping_add(c.rotate_left(5))
        .wrapping_add(bool3ary_232!(d, e, a))
        .wrapping_add(w);
    d = d.rotate_left(30);

    [b, c, d, e]
}

macro_rules! rounds4 {
    ($h0:ident, $h1:ident, $wk:expr, $i:expr) => {
        sha1_digest_round_x4($h0, sha1_first_half($h1, $wk), $i)
    };
}

macro_rules! schedule {
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr) => {
        sha1msg2(xor(sha1msg1($v0, $v1), $v2), $v3)
    };
}

macro_rules! schedule_rounds4 {
    (
        $h0:ident, $h1:ident,
        $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i:expr
    ) => {
        $w4 = schedule!($w0, $w1, $w2, $w3);
        $h1 = rounds4!($h0, $h1, $w4, $i);
    };
}

#[inline(always)]
fn sha1_digest_block_u32(state: &mut [u32; 5], block: &[u32; 16]) {
    let mut w0 = [block[0], block[1], block[2], block[3]];
    let mut w1 = [block[4], block[5], block[6], block[7]];
    let mut w2 = [block[8], block[9], block[10], block[11]];
    let mut w3 = [block[12], block[13], block[14], block[15]];
    #[allow(clippy::needless_late_init)]
    let mut w4;

    let mut h0 = [state[0], state[1], state[2], state[3]];
    let mut h1 = sha1_first_add(state[4], w0);

    // Rounds 0..20
    h1 = sha1_digest_round_x4(h0, h1, 0);
    h0 = rounds4!(h1, h0, w1, 0);
    h1 = rounds4!(h0, h1, w2, 0);
    h0 = rounds4!(h1, h0, w3, 0);
    schedule_rounds4!(h0, h1, w0, w1, w2, w3, w4, 0);

    // Rounds 20..40
    schedule_rounds4!(h1, h0, w1, w2, w3, w4, w0, 1);
    schedule_rounds4!(h0, h1, w2, w3, w4, w0, w1, 1);
    schedule_rounds4!(h1, h0, w3, w4, w0, w1, w2, 1);
    schedule_rounds4!(h0, h1, w4, w0, w1, w2, w3, 1);
    schedule_rounds4!(h1, h0, w0, w1, w2, w3, w4, 1);

    // Rounds 40..60
    schedule_rounds4!(h0, h1, w1, w2, w3, w4, w0, 2);
    schedule_rounds4!(h1, h0, w2, w3, w4, w0, w1, 2);
    schedule_rounds4!(h0, h1, w3, w4, w0, w1, w2, 2);
    schedule_rounds4!(h1, h0, w4, w0, w1, w2, w3, 2);
    schedule_rounds4!(h0, h1, w0, w1, w2, w3, w4, 2);

    // Rounds 60..80
    schedule_rounds4!(h1, h0, w1, w2, w3, w4, w0, 3);
    schedule_rounds4!(h0, h1, w2, w3, w4, w0, w1, 3);
    schedule_rounds4!(h1, h0, w3, w4, w0, w1, w2, 3);
    schedule_rounds4!(h0, h1, w4, w0, w1, w2, w3, 3);
    schedule_rounds4!(h1, h0, w0, w1, w2, w3, w4, 3);

    let e = h1[0].rotate_left(30);
    let [a, b, c, d] = h0;

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}

pub fn compress(state: &mut [u32; 5], blocks: &[[u8; BLOCK_SIZE]]) {
    let mut block_u32 = [0u32; BLOCK_SIZE / 4];
    // since LLVM can't properly use aliasing yet it will make
    // unnecessary state stores without this copy
    let mut state_cpy = *state;
    for block in blocks.iter() {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(4)) {
            *o = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        sha1_digest_block_u32(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_rug_212 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0, 1, 2, 3];
        let mut p1: [u32; 4] = [4, 5, 6, 7];

        crate::compress::soft::add(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_213 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0x12345678, 0x87654321, 0xABCDEF01, 0x01234567];
        let mut p1: [u32; 4] = [0x98765432, 0x56781234, 0xDCBAFED0, 0x76543210];

        crate::compress::soft::xor(p0, p1);

    }
}#[cfg(test)]
mod tests_rug_214 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: u32 = 123;
        let mut p1: [u32; 4] = [456, 789, 101112, 131415];
        
        sha1_first_add(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_215 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0x01234567, 0x89abcdef, 0xfedcba98, 0x76543210];
        let mut p1: [u32; 4] = [0xa5a5a5a5, 0x5a5a5a5a, 0xc3c3c3c3, 0x3c3c3c3c];
        
        crate::compress::soft::sha1msg1(p0, p1);
    }
}#[cfg(test)]
        mod tests_rug_216 {
            use super::*;
            
            #[test]
            fn test_rug() {
                let mut p0 = [0x12345678, 0xabcdef12, 0x3456789a, 0x98765432];
                let mut p1 = [0x87654321, 0xabcdef12, 0x9abcdef1, 0x98765432];
                
                crate::compress::soft::sha1msg2(p0, p1);

            }
        }#[cfg(test)]
mod tests_rug_217 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0 = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];
        let mut p1 = [0x12345678, 0x9ABCDEF0, 0xFEDCBA09, 0x87654321];
        
        crate::compress::soft::sha1_first_half(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_218 {
    use super::*;
    use crate::compress::soft::{add, sha1rnds4c, sha1rnds4m, sha1rnds4p, K};

    #[test]
    fn test_sha1_digest_round_x4() {
        let abcd: [u32; 4] = [0x01234567, 0x89abcdef, 0xfedcba98, 0x76543210];
        let work: [u32; 4] = [0x10203040, 0x50607080, 0x90a0b0c0, 0xd0e0f000];
        let i: i8 = 1;
        
        let result = match i {
            0 => sha1rnds4c(abcd, add(work, [K[0]; 4])),
            1 => sha1rnds4p(abcd, add(work, [K[1]; 4])),
            2 => sha1rnds4m(abcd, add(work, [K[2]; 4])),
            3 => sha1rnds4p(abcd, add(work, [K[3]; 4])),
            _ => unreachable!("unknown icosaround index"),
        };
        
        // assert the result here
    }
}
#[cfg(test)]
mod tests_rug_219 {
    use super::*;
    use crate::compress::soft::sha1rnds4c;

    #[test]
    fn test_sha1rnds4c() {
        let p0: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];
        let p1: [u32; 4] = [0xaabbccdd, 0xeeff0011, 0x22334455, 0x66778899];

        let result = sha1rnds4c(p0, p1);
        
        // Add assertions here to verify the output
        
    }
}

#[cfg(test)]
mod tests_rug_220 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let p0: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];
        let p1: [u32; 4] = [0x89ABCDEF, 0x01234567, 0xFEDCBA98, 0x76543210];
        
        let _ = sha1rnds4p(p0, p1);
    }
}
#[cfg(test)]
mod tests_rug_221 {
    use super::*;
    use crate::compress::soft::sha1rnds4m;

    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];
        let mut p1: [u32; 4] = [0xFFFFFFFF, 0x00000000, 0x11111111, 0x22222222];

        sha1rnds4m(p0, p1);

        // Add assertions here
    }
}        
#[cfg(test)]
mod tests_rug_222 {
    use super::*;
    use crate::compress::soft::{sha1_digest_block_u32, sha1_first_add, sha1_digest_round_x4};

    #[test]
    fn test_rug() {
        let mut p0: [u32; 5] = [0; 5];
        let mut p1: [u32; 16] = [0; 16];

        sha1_digest_block_u32(&mut p0, &mut p1);
    }
}
#[cfg(test)]
mod tests_rug_223 {
    use super::*;
    
    #[test]
    fn test_compress() {
        let mut state = [0u32; 5];
        let blocks = [
            [0u8; BLOCK_SIZE],
            [0u8; BLOCK_SIZE],
            [0u8; BLOCK_SIZE],
        ];
        
        compress(&mut state, &blocks);
        
        // Add assertions here to verify the result
        
    }
}