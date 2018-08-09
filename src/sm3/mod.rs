// Most variable's name are same as those in the Document written by the Encryption Administration
use std::num::Wrapping;

static IV: [u32; 8] = [
    0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e,
];

#[inline]
fn sm3_t(j: u32) -> u32 {
    if j <= 15 {
        0x79cc4519
    } else {
        0x7a879d8a
    }
}

#[inline]
fn sm3_ff(x: u32, y: u32, z: u32, j: u32) -> u32 {
    if j <= 15 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

#[inline]
fn sm3_gg(x: u32, y: u32, z: u32, j: u32) -> u32 {
    if j <= 15 {
        x ^ y ^ z
    } else {
        (x & y) | ((!x) & z)
    }
}

#[inline]
fn sm3_p_0(x: u32) -> u32 {
    x ^ x.rotate_left(9) ^ x.rotate_left(17)
}

#[inline]
fn sm3_p_1(x: u32) -> u32 {
    x ^ x.rotate_left(15) ^ x.rotate_left(23)
}

fn sm3_extend(b: [u32; 16]) -> ([u32; 68], [u32; 64]) {
    let mut w: [u32; 68] = [0; 68];
    let mut w_p: [u32; 64] = [0; 64];
    w[..16].clone_from_slice(&b[..16]);
    for j in 16..68 {
        w[j] = sm3_p_1(w[j - 16] ^ w[j - 9] ^ w[j - 3].rotate_left(15))
            ^ w[j - 13].rotate_left(7)
            ^ w[j - 6];
    }
    for j in 0..64 {
        w_p[j] = w[j] ^ w[j + 4];
    }

    (w, w_p)
}

fn sm3_cf(vi: [u32; 8], bi: [u32; 16]) -> [u32; 8] {
    let ws = sm3_extend(bi);
    let w = ws.0;
    let w_p = ws.1;

    let mut a = vi[0];
    let mut b = vi[1];
    let mut c = vi[2];
    let mut d = vi[3];
    let mut e = vi[4];
    let mut f = vi[5];
    let mut g = vi[6];
    let mut h = vi[7];

    let mut ss1;
    let mut ss2;
    let mut tt1;
    let mut tt2;

    for j in 0..64 {
        ss1 = (Wrapping(a.rotate_left(12)) + Wrapping(e) + Wrapping(sm3_t(j).rotate_left(j % 32)))
            .0
            .rotate_left(7);
        ss2 = ss1 ^ (a.rotate_left(12));
        tt1 = (Wrapping(sm3_ff(a, b, c, j))
            + Wrapping(d)
            + Wrapping(ss2)
            + Wrapping(w_p[j as usize])).0;
        tt2 =
            (Wrapping(sm3_gg(e, f, g, j)) + Wrapping(h) + Wrapping(ss1) + Wrapping(w[j as usize]))
                .0;
        d = c;
        c = b.rotate_left(9);
        b = a;
        a = tt1;
        h = g;
        g = f.rotate_left(19);
        f = e;
        e = sm3_p_0(tt2);
    }

    let mut vs: [u32; 8] = [0; 8];
    vs[0] = a ^ vi[0];
    vs[1] = b ^ vi[1];
    vs[2] = c ^ vi[2];
    vs[3] = d ^ vi[3];
    vs[4] = e ^ vi[4];
    vs[5] = f ^ vi[5];
    vs[6] = g ^ vi[6];
    vs[7] = h ^ vi[7];

    vs
}

pub fn sm3_enc(msg: &[u32], prim_len: usize) -> [u32; 8] {
    let mut msg_len = prim_len;
    msg_len += 1; // Add "1" to the end of msg

    // to long
    if msg_len % 512 > 448 {
        msg_len += (msg_len % 512) + 512;
    } else {
        msg_len += msg_len % 512;
    }

    let msg_len1: u32 = (prim_len / 0x0000_0001_0000_0000) as u32;
    let msg_len2: u32 = (prim_len % 0x0000_0001_0000_0000) as u32;

    // set V to IV
    let mut v: [u32; 8] = IV;

    // msg blocks' index;
    // the operations are the same except the last block
    for i in 0..msg_len / 512 + 1 {
        //println!("i={}",i);
        let mut b: [u32; 16] = [0; 16];

        // words' index in a block
        for j in 0..16 {
            if (i * 16 + j) < msg.len() as usize {
                b[j as usize] = msg[(i * 16 + j) as usize];
            }
        }

        // add "1" somewhere in this block
        if prim_len >= 512 * i && prim_len < 512 * (i + 1) {
            let mut bias = prim_len % 512;

            let mut bias_of_word = (bias / 32) as u32;
            let mut bias_of_bit = (bias % 32) as u32;
            b[bias_of_word as usize] += 0x80000000u32.rotate_right(bias_of_bit);
        }

        // the last block should store the length of msg
        if i == (msg_len / 512) {
            b[14] = msg_len1;
            b[15] = msg_len2;
        }

        v = sm3_cf(v, b);
    }

    v
}

pub fn sm3_enc_to_u8(msg: &[u32], prim_len: usize) -> [u8; 32] {
    let l = sm3_enc(msg, prim_len);

    [
        (l[0] >> 24) as u8,
        (l[0] >> 16) as u8,
        (l[0] >> 8) as u8,
        l[0] as u8,
        (l[1] >> 24) as u8,
        (l[1] >> 16) as u8,
        (l[1] >> 8) as u8,
        l[1] as u8,
        (l[2] >> 24) as u8,
        (l[2] >> 16) as u8,
        (l[2] >> 8) as u8,
        l[2] as u8,
        (l[3] >> 24) as u8,
        (l[3] >> 16) as u8,
        (l[3] >> 8) as u8,
        l[3] as u8,
        (l[4] >> 24) as u8,
        (l[4] >> 16) as u8,
        (l[4] >> 8) as u8,
        l[4] as u8,
        (l[5] >> 24) as u8,
        (l[5] >> 16) as u8,
        (l[5] >> 8) as u8,
        l[5] as u8,
        (l[6] >> 24) as u8,
        (l[6] >> 16) as u8,
        (l[6] >> 8) as u8,
        l[6] as u8,
        (l[7] >> 24) as u8,
        (l[7] >> 16) as u8,
        (l[7] >> 8) as u8,
        l[7] as u8,
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let msg: [u32; 16] = [
            0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364,
            0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364,
            0x61626364, 0x61626364,
        ];

        let hash = sm3_enc(&msg, 512);
        println!(
            "{:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]
        );

        println!();
        println!();
    }
}
