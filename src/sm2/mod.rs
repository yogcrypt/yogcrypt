//! An implementation of the SM2 signature standard.
//!
//! ## Usage
//! ```
//! extern crate yogcrypt;
//! use yogcrypt::sm2::{get_sec_key, get_pub_key, sm2_gen_sign, sm2_ver_sign};
//!
//! let sk = get_sec_key();
//! let msg = b"Hello World!";
//!
//! let pk = get_pub_key(sk);
//!
//! let mut tag = sm2_gen_sign(msg, sk, pk);
//!
//! let t = sm2_ver_sign(msg, pk, &tag);
//!
//! // Signature is accepted
//! assert!(t);
//! ```
//!
//! ## Reference
//! Most variable's name in the source code are in accordance with the document.
//!
//! [OSCCA: SM2 document](http://www.oscca.gov.cn/sca/xxgk/2010-12/17/1002386/files/b791a9f908bb4803875ab6aeeb7b4e03.pdf)
use basic::cell::u64x4::*;
use basic::field::field_n::*;
use basic::field::field_p::MODULO_P;
use basic::group::ecc_group::*;
use basic::helper::bytes_to_u32_blocks;
use rand::random;
use sm3::*;

pub type PubKey = Point;
pub type SecKey = U64x4;
pub struct Signature {
    pub r: U64x4,
    pub s: U64x4,
}

/// Randomly sample secret key uniformly from [0,..n), where n is the order of the base point
pub fn get_sec_key() -> SecKey {
    // TODO replace with random
    U64x4::new(0, 0, 0, 0)
}

/// Compute public key from secret key
///
/// By definition public key is computed by pk = [sk] G, where G is the base point.
pub fn get_pub_key(d: SecKey) -> PubKey {
    let rst_jacobi = times_base_point(d);
    jacobi_to_affine(rst_jacobi)
}

/// Compute context `Z` as specified in standard document
fn get_z(q: PubKey) -> [u32; 8] {
    let _len: usize = 2 + 14 + 6 * 32;
    let mut s: [u32; 52] = [0; 52];

    s[0] = 0x00D00102;
    s[1] = 0x03040506;
    s[2] = 0x0708090A;
    s[3] = 0x0B0C0D0E;

    s[4] = (ECC_A.value(3) >> 32) as u32;
    s[5] = ECC_A.value(3) as u32;
    s[6] = (ECC_A.value(2) >> 32) as u32;
    s[7] = ECC_A.value(2) as u32;
    s[8] = (ECC_A.value(1) >> 32) as u32;
    s[9] = ECC_A.value(1) as u32;
    s[10] = (ECC_A.value(0) >> 32) as u32;
    s[11] = ECC_A.value(0) as u32;

    s[12] = (ECC_B.value(3) >> 32) as u32;
    s[13] = ECC_B.value(3) as u32;
    s[14] = (ECC_B.value(2) >> 32) as u32;
    s[15] = ECC_B.value(2) as u32;
    s[16] = (ECC_B.value(1) >> 32) as u32;
    s[17] = ECC_B.value(1) as u32;
    s[18] = (ECC_B.value(0) >> 32) as u32;
    s[19] = ECC_B.value(0) as u32;

    s[20] = (ECC_G.x.value(3) >> 32) as u32;
    s[21] = ECC_G.x.value(3) as u32;
    s[22] = (ECC_G.x.value(2) >> 32) as u32;
    s[23] = ECC_G.x.value(2) as u32;
    s[24] = (ECC_G.x.value(1) >> 32) as u32;
    s[25] = ECC_G.x.value(1) as u32;
    s[26] = (ECC_G.x.value(0) >> 32) as u32;
    s[27] = ECC_G.x.value(0) as u32;

    s[28] = (ECC_G.y.value(3) >> 32) as u32;
    s[29] = ECC_G.y.value(3) as u32;
    s[30] = (ECC_G.y.value(2) >> 32) as u32;
    s[31] = ECC_G.y.value(2) as u32;
    s[32] = (ECC_G.y.value(1) >> 32) as u32;
    s[33] = ECC_G.y.value(1) as u32;
    s[34] = (ECC_G.y.value(0) >> 32) as u32;
    s[35] = ECC_G.y.value(0) as u32;

    s[36] = (q.x.value(3) >> 32) as u32;
    s[37] = q.x.value(3) as u32;
    s[38] = (q.x.value(2) >> 32) as u32;
    s[39] = q.x.value(2) as u32;
    s[40] = (q.x.value(1) >> 32) as u32;
    s[41] = q.x.value(1) as u32;
    s[42] = (q.x.value(0) >> 32) as u32;
    s[43] = q.x.value(0) as u32;

    s[44] = (q.y.value(3) >> 32) as u32;
    s[45] = q.y.value(3) as u32;
    s[46] = (q.y.value(2) >> 32) as u32;
    s[47] = q.y.value(2) as u32;
    s[48] = (q.y.value(1) >> 32) as u32;
    s[49] = q.y.value(1) as u32;
    s[50] = (q.y.value(0) >> 32) as u32;
    s[51] = q.y.value(0) as u32;

    //Z = sm3Enc(&s[0..52], 52 * 32)
    sm3_enc_inner(&s[0..52], 52 * 32)
}

/// Generate a valid signature for a message using a pair of keys.
///
/// **Note**: The underlying hash function is `sm3`.
pub fn sm2_gen_sign(msg: &[u8], d: SecKey, q: PubKey) -> Signature {
    let (msg, bit_len) = bytes_to_u32_blocks(msg);
    sm2_gen_sign_inner(&msg[..], d, q, bit_len)
}

/// Core function for generation with specified input length
pub(crate) fn sm2_gen_sign_inner(msg: &[u32], d: SecKey, q: PubKey, len: usize) -> Signature {
    // verify that Q is indeed on the curve
    // to prevent false curve attack
    assert!(is_on_curve(q), "Public key not on curve!");

    let z = get_z(q);

    let m = [msg, &z].concat();

    // compute the hash value
    let e = sm3_enc_inner(&m, (len + 8) * 32);
    let mut e = U64x4::new(
        u64::from(e[7]) | (u64::from(e[6]) << 32),
        u64::from(e[5]) | (u64::from(e[4]) << 32),
        u64::from(e[3]) | (u64::from(e[2]) << 32),
        u64::from(e[1]) | (u64::from(e[0]) << 32),
    );

    // ephemeral key
    let mut k = U64x4::new(
        random::<u64>(),
        random::<u64>(),
        random::<u64>(),
        random::<u64>(),
    );
    while greater_equal(k, MODULO_P) {
        k = U64x4::new(
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        );
    }

    let mut p_jacobi = times_base_point(k);
    let mut p = jacobi_to_affine(p_jacobi);

    e = to_mod_n(e);
    let mut r = add_mod_n(e, p.x.num);
    let d = to_mod_n(d);

    // Calculate s = (1+d)^-1 * (k-r*d);
    let s = mul_mod_n(
        get_mul_inv_mod_n(add_mod_n(d, U64x4::new(1, 0, 0, 0))), //(1+d)^-1
        sub_mod_n(k, mul_mod_n(r, d)),                           //k-r*d
    );

    while equal_to_zero(r) || equal_to_zero(add_mod_n(r, k)) || equal_to_zero(s) {
        k = U64x4::new(
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        );
        while greater_equal(k, MODULO_P) {
            k = U64x4::new(
                random::<u64>(),
                random::<u64>(),
                random::<u64>(),
                random::<u64>(),
            );
        }
        p_jacobi = times_base_point(k);
        p = jacobi_to_affine(p_jacobi);
        r = add_mod_n(e, p.x.num);
    }

    Signature { r, s }
}

/// Verify a signature on a given message using public key
///
/// **Note**: The underlying hash function is `sm3`.
pub fn sm2_ver_sign(msg: &[u8], q: PubKey, sig: &Signature) -> bool {
    let (msg, bit_len) = bytes_to_u32_blocks(msg);
    sm2_ver_sign_inner(&msg[..], q, bit_len, sig)
}

/// Core function for verification with specified input length
pub(crate) fn sm2_ver_sign_inner(msg: &[u32], q: PubKey, len: usize, sig: &Signature) -> bool {
    // verify that Q is indeed on the curve
    // to prevent false curve attack
    assert!(is_on_curve(q), "public key not on curve!");
    let r = sig.r;
    let s = sig.s;

    if greater_equal(r, MODULO_N) || equal_to_zero(r) {
        return false;
    }
    if greater_equal(s, MODULO_N) || equal_to_zero(s) {
        return false;
    }
    let z_a = get_z(q);
    let m = [msg, &z_a].concat();

    let e = sm3_enc_inner(&m, (len + 8) * 32);
    let e = U64x4::new(
        u64::from(e[7]) | (u64::from(e[6]) << 32),
        u64::from(e[5]) | (u64::from(e[4]) << 32),
        u64::from(e[3]) | (u64::from(e[2]) << 32),
        u64::from(e[1]) | (u64::from(e[0]) << 32),
    );

    let t = add_mod_n(r, s);
    if equal_to_zero(t) {
        return false;
    }

    let p_jacobi = add_jacobi_point(times_base_point(s), times_point(q, t));
    let p = jacobi_to_affine(p_jacobi);

    let e1 = to_mod_n(e);
    let x1 = to_mod_n(p.x.num);
    let r2 = add_mod_n(e1, x1);

    // accept if r2 = r
    equal_to(r2, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_u64x4() -> U64x4 {
        U64x4::new(
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        )
    }

    #[test]
    fn test() {
        for _ in 0..10000 {
            let d_a = rand_u64x4();

            let msg = [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
                0x32, 0x10,
            ];

            let q = get_pub_key(d_a);

            let mut m = sm2_gen_sign(&msg, d_a, q);

            let t = sm2_ver_sign(&msg, q, &m);
            assert!(t);
        }
    }
}
