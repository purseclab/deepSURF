#![allow(clippy::many_single_char_names)]
use crate::consts::{BLOCK_LEN, K64X2};
use core::convert::TryInto;

fn add(a: [u64; 2], b: [u64; 2]) -> [u64; 2] {
    [a[0].wrapping_add(b[0]), a[1].wrapping_add(b[1])]
}

/// Not an intrinsic, but works like an unaligned load.
fn sha512load(v0: [u64; 2], v1: [u64; 2]) -> [u64; 2] {
    [v1[1], v0[0]]
}

/// Performs 2 rounds of the SHA-512 message schedule update.
pub fn sha512_schedule_x2(v0: [u64; 2], v1: [u64; 2], v4to5: [u64; 2], v7: [u64; 2]) -> [u64; 2] {
    // sigma 0
    fn sigma0(x: u64) -> u64 {
        ((x << 63) | (x >> 1)) ^ ((x << 56) | (x >> 8)) ^ (x >> 7)
    }

    // sigma 1
    fn sigma1(x: u64) -> u64 {
        ((x << 45) | (x >> 19)) ^ ((x << 3) | (x >> 61)) ^ (x >> 6)
    }

    let [w1, w0] = v0;
    let [_, w2] = v1;
    let [w10, w9] = v4to5;
    let [w15, w14] = v7;

    let w16 = sigma1(w14)
        .wrapping_add(w9)
        .wrapping_add(sigma0(w1))
        .wrapping_add(w0);
    let w17 = sigma1(w15)
        .wrapping_add(w10)
        .wrapping_add(sigma0(w2))
        .wrapping_add(w1);

    [w17, w16]
}

/// Performs one round of the SHA-512 message block digest.
pub fn sha512_digest_round(
    ae: [u64; 2],
    bf: [u64; 2],
    cg: [u64; 2],
    dh: [u64; 2],
    wk0: u64,
) -> [u64; 2] {
    macro_rules! big_sigma0 {
        ($a:expr) => {
            ($a.rotate_right(28) ^ $a.rotate_right(34) ^ $a.rotate_right(39))
        };
    }
    macro_rules! big_sigma1 {
        ($a:expr) => {
            ($a.rotate_right(14) ^ $a.rotate_right(18) ^ $a.rotate_right(41))
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

    let [a0, e0] = ae;
    let [b0, f0] = bf;
    let [c0, g0] = cg;
    let [d0, h0] = dh;

    // a round
    let x0 = big_sigma1!(e0)
        .wrapping_add(bool3ary_202!(e0, f0, g0))
        .wrapping_add(wk0)
        .wrapping_add(h0);
    let y0 = big_sigma0!(a0).wrapping_add(bool3ary_232!(a0, b0, c0));
    let (a1, _, _, _, e1, _, _, _) = (
        x0.wrapping_add(y0),
        a0,
        b0,
        c0,
        x0.wrapping_add(d0),
        e0,
        f0,
        g0,
    );

    [a1, e1]
}

/// Process a block with the SHA-512 algorithm.
pub fn sha512_digest_block_u64(state: &mut [u64; 8], block: &[u64; 16]) {
    let k = &K64X2;

    macro_rules! schedule {
        ($v0:expr, $v1:expr, $v4:expr, $v5:expr, $v7:expr) => {
            sha512_schedule_x2($v0, $v1, sha512load($v4, $v5), $v7)
        };
    }

    macro_rules! rounds4 {
        ($ae:ident, $bf:ident, $cg:ident, $dh:ident, $wk0:expr, $wk1:expr) => {{
            let [u, t] = $wk0;
            let [w, v] = $wk1;

            $dh = sha512_digest_round($ae, $bf, $cg, $dh, t);
            $cg = sha512_digest_round($dh, $ae, $bf, $cg, u);
            $bf = sha512_digest_round($cg, $dh, $ae, $bf, v);
            $ae = sha512_digest_round($bf, $cg, $dh, $ae, w);
        }};
    }

    let mut ae = [state[0], state[4]];
    let mut bf = [state[1], state[5]];
    let mut cg = [state[2], state[6]];
    let mut dh = [state[3], state[7]];

    // Rounds 0..20
    let (mut w1, mut w0) = ([block[3], block[2]], [block[1], block[0]]);
    rounds4!(ae, bf, cg, dh, add(k[0], w0), add(k[1], w1));
    let (mut w3, mut w2) = ([block[7], block[6]], [block[5], block[4]]);
    rounds4!(ae, bf, cg, dh, add(k[2], w2), add(k[3], w3));
    let (mut w5, mut w4) = ([block[11], block[10]], [block[9], block[8]]);
    rounds4!(ae, bf, cg, dh, add(k[4], w4), add(k[5], w5));
    let (mut w7, mut w6) = ([block[15], block[14]], [block[13], block[12]]);
    rounds4!(ae, bf, cg, dh, add(k[6], w6), add(k[7], w7));
    let mut w8 = schedule!(w0, w1, w4, w5, w7);
    let mut w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[8], w8), add(k[9], w9));

    // Rounds 20..40
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[10], w0), add(k[11], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[12], w2), add(k[13], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[14], w4), add(k[15], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[16], w6), add(k[17], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[18], w8), add(k[19], w9));

    // Rounds 40..60
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[20], w0), add(k[21], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[22], w2), add(k[23], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[24], w4), add(k[25], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[26], w6), add(k[27], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[28], w8), add(k[29], w9));

    // Rounds 60..80
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, add(k[30], w0), add(k[31], w1));
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, add(k[32], w2), add(k[33], w3));
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, add(k[34], w4), add(k[35], w5));
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, add(k[36], w6), add(k[37], w7));
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, add(k[38], w8), add(k[39], w9));

    let [a, e] = ae;
    let [b, f] = bf;
    let [c, g] = cg;
    let [d, h] = dh;

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn compress(state: &mut [u64; 8], blocks: &[[u8; 128]]) {
    let mut block_u32 = [0u64; BLOCK_LEN];
    // since LLVM can't properly use aliasing yet it will make
    // unnecessary state stores without this copy
    let mut state_cpy = *state;
    for block in blocks {
        for (o, chunk) in block_u32.iter_mut().zip(block.chunks_exact(8)) {
            *o = u64::from_be_bytes(chunk.try_into().unwrap());
        }
        sha512_digest_block_u64(&mut state_cpy, &block_u32);
    }
    *state = state_cpy;
}
#[cfg(test)]
mod tests_rug_250 {
    use super::*;

    #[test]
    fn test_rug() {
        let mut p0: [u64; 2] = [0; 2];
        let mut p1: [u64; 2] = [0; 2];

        // Sample data for p0
        p0[0] = 1;
        p0[1] = 2;

        // Sample data for p1
        p1[0] = 3;
        p1[1] = 4;

         crate::sha512::soft::add(p0, p1);

    }
}#[cfg(test)]
mod tests_rug_251 {
    use super::*;
    use crate::sha512::soft::sha512load;
    
    #[test]
    fn test_sha512load() {
        let p0: [u64; 2] = [0x1122334455667788, 0x99AABBCCDDEEFF00];
        let p1: [u64; 2] = [0xFFEEDDCCBBAA9988, 0x7766554433221100];
        
        let result = sha512load(p0, p1);
        
        assert_eq!(result, [0x7766554433221100, 0x1122334455667788]);
    }
}#[cfg(test)]
mod tests_rug_252 {
    use super::*;
    
    #[test]
    fn test_rug() {
        let mut p0 = [0x1122334455667788, 0x99AABBBCCCDDDEEE];
        let mut p1 = [0x1020304050607080, 0x90A0B0C0D0E0F000];
        let mut p2 = [0xA1A2A3A4A5A6A7A8, 0xB1B2B3B4B5B6B7B8];
        let mut p3 = [0xC1C2C3C4C5C6C7C8, 0xD1D2D3D4D5D6D7D8];

        crate::sha512::soft::sha512_schedule_x2(p0, p1, p2, p3);
    }
}#[cfg(test)]
mod tests_rug_255 {
    use super::*;

    #[test]
    fn test_sha512_digest_round() {
        let ae: [u64; 2] = [0x0123456789abcdef, 0xfedcba9876543210];
        let bf: [u64; 2] = [0xabcdef0123456789, 0x3210fedcba987654];
        let cg: [u64; 2] = [0x89abcdef01234567, 0x76543210fedcba98];
        let dh: [u64; 2] = [0xba9876543210fedc, 0xdcba9876543210fe];
        let wk0: u64 = 0x0123456789abcdef;

        sha512_digest_round(ae, bf, cg, dh, wk0);
    }
}#[cfg(test)]
mod tests_rug_256 {
    use super::*;
    
    #[test]
    fn test_sha512_digest_block_u64() {
        let mut p0: [u64; 8] = [0; 8];
        let mut p1: [u64; 16] = [0; 16];
        
        crate::sha512::soft::sha512_digest_block_u64(&mut p0, &p1);
        
        // Add assertions here
    }
}#[cfg(test)]
mod tests_rug_257 {
    use super::*;
    
    #[test]
    fn test_compress() {
        let mut state = [0u64; 8];
        let blocks = [
            [0u8; 128],
            [0u8; 128],
        ];
        
        compress(&mut state, &blocks);
    }
}