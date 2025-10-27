#![allow(clippy::many_single_char_names)]
use crate::consts::BLOCK_LEN;
use core::convert::TryInto;

#[inline(always)]
fn shl(v: [u32; 4], o: u32) -> [u32; 4] {
    [v[0] >> o, v[1] >> o, v[2] >> o, v[3] >> o]
}

#[inline(always)]
fn shr(v: [u32; 4], o: u32) -> [u32; 4] {
    [v[0] << o, v[1] << o, v[2] << o, v[3] << o]
}

#[inline(always)]
fn or(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] | b[0], a[1] | b[1], a[2] | b[2], a[3] | b[3]]
}

#[inline(always)]
fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

#[inline(always)]
fn add(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    ]
}

fn sha256load(v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    [v3[3], v2[0], v2[1], v2[2]]
}

fn sha256swap(v0: [u32; 4]) -> [u32; 4] {
    [v0[2], v0[3], v0[0], v0[1]]
}

fn sha256msg1(v0: [u32; 4], v1: [u32; 4]) -> [u32; 4] {
    // sigma 0 on vectors
    #[inline]
    fn sigma0x4(x: [u32; 4]) -> [u32; 4] {
        let t1 = or(shl(x, 7), shr(x, 25));
        let t2 = or(shl(x, 18), shr(x, 14));
        let t3 = shl(x, 3);
        xor(xor(t1, t2), t3)
    }

    add(v0, sigma0x4(sha256load(v0, v1)))
}

fn sha256msg2(v4: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    macro_rules! sigma1 {
        ($a:expr) => {
            $a.rotate_right(17) ^ $a.rotate_right(19) ^ ($a >> 10)
        };
    }

    let [x3, x2, x1, x0] = v4;
    let [w15, w14, _, _] = v3;

    let w16 = x0.wrapping_add(sigma1!(w14));
    let w17 = x1.wrapping_add(sigma1!(w15));
    let w18 = x2.wrapping_add(sigma1!(w16));
    let w19 = x3.wrapping_add(sigma1!(w17));

    [w19, w18, w17, w16]
}

fn sha256_digest_round_x2(cdgh: [u32; 4], abef: [u32; 4], wk: [u32; 4]) -> [u32; 4] {
    macro_rules! big_sigma0 {
        ($a:expr) => {
            ($a.rotate_right(2) ^ $a.rotate_right(13) ^ $a.rotate_right(22))
        };
    }
    macro_rules! big_sigma1 {
        ($a:expr) => {
            ($a.rotate_right(6) ^ $a.rotate_right(11) ^ $a.rotate_right(25))
        };
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => {
            $c ^ ($a & ($b ^ $c))
        };
    } // Choose, MD5F, SHA1C
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) ^ ($a & $c) ^ ($b & $c)
        };
    } // Majority, SHA1M

    let [_, _, wk1, wk0] = wk;
    let [a0, b0, e0, f0] = abef;
    let [c0, d0, g0, h0] = cdgh;

    // a round
    let x0 = big_sigma1!(e0)
        .wrapping_add(bool3ary_202!(e0, f0, g0))
        .wrapping_add(wk0)
        .wrapping_add(h0);
    let y0 = big_sigma0!(a0).wrapping_add(bool3ary_232!(a0, b0, c0));
    let (a1, b1, c1, d1, e1, f1, g1, h1) = (
        x0.wrapping_add(y0),
        a0,
        b0,
        c0,
        x0.wrapping_add(d0),
        e0,
        f0,
        g0,
    );

    // a round
    let x1 = big_sigma1!(e1)
        .wrapping_add(bool3ary_202!(e1, f1, g1))
        .wrapping_add(wk1)
        .wrapping_add(h1);
    let y1 = big_sigma0!(a1).wrapping_add(bool3ary_232!(a1, b1, c1));
    let (a2, b2, _, _, e2, f2, _, _) = (
        x1.wrapping_add(y1),
        a1,
        b1,
        c1,
        x1.wrapping_add(d1),
        e1,
        f1,
        g1,
    );

    [a2, b2, e2, f2]
}

fn schedule(v0: [u32; 4], v1: [u32; 4], v2: [u32; 4], v3: [u32; 4]) -> [u32; 4] {
    let t1 = sha256msg1(v0, v1);
    let t2 = sha256load(v2, v3);
    let t3 = add(t1, t2);
    sha256msg2(t3, v3)
}

macro_rules! rounds4 {
    ($abef:ident, $cdgh:ident, $rest:expr, $i:expr) => {{
        let t1 = add($rest, crate::consts::K32X4[$i]);
        $cdgh = sha256_digest_round_x2($cdgh, $abef, t1);
        let t2 = sha256swap(t1);
        $abef = sha256_digest_round_x2($abef, $cdgh, t2);
    }};
}

macro_rules! schedule_rounds4 {
    (
        $abef:ident, $cdgh:ident,
        $w0:expr, $w1:expr, $w2:expr, $w3:expr, $w4:expr,
        $i: expr
    ) => {{
        $w4 = schedule($w0, $w1, $w2, $w3);
        rounds4!($abef, $cdgh, $w4, $i);
    }};
}

/// Process a block with the SHA-256 algorithm.
fn sha256_digest_block_u32(state: &mut [u32; 8], block: &[u32; 16]) {
    let mut abef = [state[0], state[1], state[4], state[5]];
    let mut cdgh = [state[2], state[3], state[6], state[7]];

    // Rounds 0..64
    let mut w0 = [block[3], block[2], block[1], block[0]];
    let mut w1 = [block[7], block[6], block[5], block[4]];
    let mut w2 = [block[11], block[10], block[9], block[8]];
    let mut w3 = [block[15], block[14], block[13], block[12]];
    let mut w4;

    rounds4!(abef, cdgh, w0, 0);
    rounds4!(abef, cdgh, w1, 1);
    rounds4!(abef, cdgh, w2, 2);
    rounds4!(abef, cdgh, w3, 3);
    schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 4);
    schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 5);
    schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 6);
    schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 7);
    schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 8);
    schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 9);
    schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 10);
    schedule_rounds4!(abef, cdgh, w2, w3, w4, w0, w1, 11);
    schedule_rounds4!(abef, cdgh, w3, w4, w0, w1, w2, 12);
    schedule_rounds4!(abef, cdgh, w4, w0, w1, w2, w3, 13);
    schedule_rounds4!(abef, cdgh, w0, w1, w2, w3, w4, 14);
    schedule_rounds4!(abef, cdgh, w1, w2, w3, w4, w0, 15);

    let [a, b, e, f] = abef;
    let [c, d, g, h] = cdgh;

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn compress(state: &mut [u32; 8], blocks: &[[u8; 64]]) {
    let mut block_u32 = [0u32; BLOCK_LEN];
    // since LLVM can't properly use aliasing yet it will make
    // unnecessary state stores without this copy
    let mut state_cpy = *state;
    for block in blocks {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(4)) {
            *o = u32::from_be_bytes(chunk.try_into().unwrap());
        }
        sha256_digest_block_u32(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_rug_232 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [1, 2, 3, 4];
        let mut p1: u32 = 2;
        
        crate::sha256::soft::shl(p0, p1);
        
    }
}                    
#[cfg(test)]
mod tests_rug_233 {
    use super::*;
    use crate::sha256::soft::shr;
    
    #[test]
    fn test_rug() {
        let v: [u32; 4] = [1, 2, 3, 4];
        let o: u32 = 10;

        shr(v, o);
    }
}
                            #[cfg(test)]
mod tests_rug_234 {
    use super::*;

    #[test]
    fn test_rug() {
        let p0: [u32; 4] = [0x11111111, 0x22222222, 0x33333333, 0x44444444];
        let p1: [u32; 4] = [0x55555555, 0x66666666, 0x77777777, 0x88888888];

        let result = crate::sha256::soft::or(p0, p1);

        assert_eq!(result[0], 0x55555555 | 0x11111111);
        assert_eq!(result[1], 0x66666666 | 0x22222222);
        assert_eq!(result[2], 0x77777777 | 0x33333333);
        assert_eq!(result[3], 0x88888888 | 0x44444444);
    }
}
#[cfg(test)]
mod tests_rug_235 {
    use super::*;
    use crate::sha256::soft::xor;
    
    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0xDEADBEEF, 0xBAADF00D, 0xC0FFEE, 0xFEEDBEEF];
        let mut p1: [u32; 4] = [0xCAFEBABE, 0xDEADBEEF, 0xBAADF00D, 0xC0FFEE];
        
        xor(p0, p1);
    }
}

#[cfg(test)]
mod tests_rug_236 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0, 1, 2, 3];
        let mut p1: [u32; 4] = [4, 5, 6, 7];

        crate::sha256::soft::add(p0, p1);

    }
}
#[cfg(test)]
mod tests_rug_237 {
    use super::*;

    #[test]
    fn test_sha256load() {
        let mut p0: [u32; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
        let mut p1: [u32; 4] = [0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
                
        assert_eq!(crate::sha256::soft::sha256load(p0, p1), [p1[3], p0[0], p0[1], p0[2]]);
    }
}#[cfg(test)]
mod tests_rug_238 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let p0: [u32; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
        
        crate::sha256::soft::sha256swap(p0);
        
        // Add your assertions here
        // ...
    }
}#[cfg(test)]
mod tests_rug_239 {
    use super::*;
    use crate::sha256::soft::{add, sha256load, shl, shr, or, xor};

    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0; 4];
        let mut p1: [u32; 4] = [0; 4];
        // Initialize p0 and p1 with sample data
        // For example:
        p0 = [1, 2, 3, 4];
        p1 = [5, 6, 7, 8];

        crate::sha256::soft::sha256msg1(p0, p1);
    }
}#[cfg(test)]
mod tests_rug_241 {
    use super::*;
    use crate::sha256::soft::sha256msg2;

    #[test]
    fn test_sha256msg2() {
        let v4: [u32; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
        let v3: [u32; 4] = [0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

        let result = sha256msg2(v4, v3);

        assert_eq!(result[0], 0xdff5b10b);
        assert_eq!(result[1], 0xf648bd4e);
        assert_eq!(result[2], 0xf47c5aca);
        assert_eq!(result[3], 0x1abba8db);
    }
}
#[cfg(test)]
mod tests_rug_242 {
    use super::*;
    
    #[test]
    fn test_sha256_digest_round_x2() {
        let mut p0: [u32; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
        let mut p1: [u32; 4] = [0x2a907bbb, 0xeb55a8e8, 0x14f8385a, 0x08de7719];
        let mut p2: [u32; 4] = [0xaaffffff, 0x3399baaa, 0xffeeddcc, 0x11223344];

        sha256_digest_round_x2(p0, p1, p2);
    }
}#[cfg(test)]
mod tests_rug_243 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0: [u32; 4] = [0; 4]; // Sample data for p0
        let mut p1: [u32; 4] = [0; 4]; // Sample data for p1
        let mut p2: [u32; 4] = [0; 4]; // Sample data for p2
        let mut p3: [u32; 4] = [0; 4]; // Sample data for p3

        crate::sha256::soft::schedule(p0, p1, p2, p3);
    }
}        
#[cfg(test)]
mod tests_rug_244 {
    use super::*;
    
    #[test]
    fn test_sha256_digest_block_u32() {
        let mut p0: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
        let mut p1: [u32; 16] = [
            0xf3bcc908, 0x6c7eae46, 0xb44ca70a, 0xb8ff64f8,
            0xfad03212, 0xb0452d2e, 0xd5b1be16, 0xf7ee1122,
            0xcce24c92, 0x7fc780cf, 0xa480f74d, 0xc4547c6b,
            0xe73bf036, 0xc83179b1, 0xa50093d4, 0xd8a12964,
        ];
        
        sha256_digest_block_u32(&mut p0,&p1);
    }
}
#[cfg(test)]
mod tests_rug_245 {
    use super::*;
    use crate::Digest;

    #[test]
    fn test_compress() {
        let mut state = [0u32; 8];
        let blocks = [[0u8; 64], [0u8; 64]]; // Sample blocks data

        compress(&mut state, &blocks);

        // Add assertions here
    }
}