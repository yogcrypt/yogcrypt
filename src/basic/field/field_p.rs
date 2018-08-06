use basic::cell::u64x4::*;

pub const MODULO_P: U64x4 = U64x4 {
    value: [
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFF00000000,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFEFFFFFFFF,
    ],
};
const RHO_P: U64x4 = U64x4 {
    value: [
        0x0000000000000001,
        0x00000000FFFFFFFF,
        0x0000000000000000,
        0x0000000100000000,
    ],
};
pub const INV_2P: U64x4 = U64x4 {
    value: [
        0x8000000000000000,
        0xFFFFFFFF80000000,
        0xFFFFFFFFFFFFFFFF,
        0x7FFFFFFF7FFFFFFF,
    ],
};

macro_rules! overflowing_add {
    ($x:expr, $y:expr, $result:ident, $overflow_flag:ident) => {
        let car = if $overflow_flag { 1 } else { 0 };

        let r1 = u64::overflowing_add($x, $y);
        let r2 = u64::overflowing_add(r1.0, car);
        $result = r2.0;
        $overflow_flag = r1.1 | r2.1;
    };
}

pub fn to_mod_p(mut x: U64x4) -> U64x4 {
    while greater_equal(x, MODULO_P) {
        x = x - MODULO_P;
    }

    x
}

pub fn get_add_inv(x: U64x4) -> U64x4 {
    MODULO_P - x
}

pub fn get_mul_inv(x: U64x4) -> U64x4 {
    if equal_to_zero(x) {
        return U64x4::new(0, 0, 0, 0);
    }

    let mut u = x;
    let mut v = MODULO_P;
    let mut x1 = U64x4::new(1, 0, 0, 0);
    let mut x2 = U64x4::new(0, 0, 0, 0);

    while (!equal_to_one(u)) && (!equal_to_one(v)) {
        while u.value[0] % 2 == 0 {
            u.right_shift_by_one();

            if x1.value[0] % 2 == 0 {
                x1.right_shift_by_one();
            } else {
                let (u, overflow_flag) = add_no_mod(x1, MODULO_P);
                x1 = u;
                x1.right_shift_by_one();
                if overflow_flag {
                    x1.value[3] |= 0x8000000000000000;
                }
            }
        }

        while v.value[0] % 2 == 0 {
            v.right_shift_by_one();

            if x2.value[0] % 2 == 0 {
                x2.right_shift_by_one();
            } else {
                let (u, overflow_flag) = add_no_mod(x2, MODULO_P);
                x2 = u;
                x2.right_shift_by_one();
                if overflow_flag {
                    x2.value[3] |= 0x8000000000000000;
                }
            }
        }

        if greater_equal(u, v) {
            u = sub(u, v);
            x1 = sub(x1, x2);
        } else {
            v = sub(v, u);
            x2 = sub(x2, x1);
        }
    }

    if equal_to_one(u) {
        to_mod_p(x1)
    } else {
        to_mod_p(x2)
    }
}

pub fn add(x: U64x4, y: U64x4) -> U64x4 {
    let res0: u64;
    let res1: u64;
    let res2: u64;
    let res3: u64;
    let mut overflow_flag = false;

    overflowing_add!(x.value[0], y.value[0], res0, overflow_flag);
    overflowing_add!(x.value[1], y.value[1], res1, overflow_flag);
    overflowing_add!(x.value[2], y.value[2], res2, overflow_flag);
    overflowing_add!(x.value[3], y.value[3], res3, overflow_flag);

    let mut m = U64x4 {
        value: [res0, res1, res2, res3],
    };

    //overflow
    if overflow_flag {
        m = add(RHO_P, m);
    }

    if greater_equal(m, MODULO_P) {
        m - MODULO_P
    } else {
        m
    }
}

pub fn sub(x: U64x4, y: U64x4) -> U64x4 {
    add(x, get_add_inv(y))
}

fn helper_mul(x: u64, y: u64) -> (u128, u128) {
    /* a helper overflowing multiplication for u64 */
    let z: u128 = u128::from(x) * u128::from(y);
    let carry = z >> 64;
    let rst = z ^ (carry << 64);

    (rst, carry)
}

fn raw_mul(x: U64x4, y: U64x4) -> [u64; 8] {
    /* Perform long multiplication */
    let mut result: [u64; 8] = [0; 8];
    let mut carry: u128 = 0;

    // for each result block
    for (block_i, cell) in result.iter_mut().enumerate().take(7) {
        // temporary value
        let mut cur: u128 = carry;
        carry = 0;

        // enumerate each block of y
        let low = if block_i > 3 { block_i - 3 } else { 0 };
        let high = if block_i > 3 { 3 } else { block_i };

        for y_i in low..=high {
            let (rst, c) = helper_mul(x.value[block_i - y_i], y.value[y_i]);
            carry += c;
            cur += rst;
        }

        // check addition overlow carry
        let c = cur >> 64;
        carry += c;

        *cell = (cur ^ (c << 64)) as u64;
    }
    result[7] = carry as u64;

    result
}

fn helper_split_u64(x: u64) -> (u32, u32) {
    let high = x >> 32;
    let low = x ^ (high << 32);
    (low as u32, high as u32)
}

fn reduce(n: [u64; 8]) -> U64x4 {
    /* fast reduction 256bit to 128bit*/
    /* ref: http://cacr.uwaterloo.ca/techreports/1999/corr99-39.pdf */

    // first split the input
    let mut a: [u32; 16] = [0; 16];
    for i in 0..8 {
        let (low, high) = helper_split_u64(n[i]);
        a[2 * i] = low;
        a[(2 * i) ^ 1] = high;
    }

    // prepare the summands
    // given by LFSR with [1,0,0,0,1,-1,0,1] and proper re-combination
    // of mantissa
    let s = U64x4::new(n[0], n[1], n[2], n[3]); // lower parts of n

    // the following should be added twice (suffix d)
    let s15_d = U64x4::from_u32([a[15], a[15], 0, 0, 0, a[15], 0, a[15]]);
    let s14_d = U64x4::from_u32([a[14], a[14], 0, 0, a[14], 0, 0, a[14]]);
    let s13_d = U64x4::from_u32([a[13], 0, 0, a[13], 0, 0, 0, a[13]]);
    let s12_d = U64x4::from_u32([0, 0, 0, 0, 0, 0, 0, a[12]]);
    // find the sum
    let sum_d = add(add(s15_d, s14_d), add(s13_d, s12_d));

    // find other sum (hard coded by sight)
    let s8_13 = U64x4::from_u32([a[8], a[13], 0, a[8], a[13], a[13], 0, a[8]]);
    let s9_14 = U64x4::from_u32([a[9], a[9], 0, a[14], a[9], a[14], a[14], a[9]]);
    let s10_12 = U64x4::from_u32([a[10], a[10], 0, a[12], a[12], a[10], 0, a[10]]);
    let s11 = U64x4::from_u32([a[11], a[11], 0, a[11], 0, 0, a[11], a[11]]);
    let s15_12 = U64x4::from_u32([a[12], a[12], 0, a[15], a[15], 0, a[15], a[15]]);

    // sum all the stuffs together
    let s = add(
        add(add(s, sum_d), add(add(s8_13, s9_14), add(s10_12, s11))),
        add(s15_12, sum_d),
    );

    // find the subtrahend
    let subtra: u64 = u64::from(a[8]) + u64::from(a[9]) + u64::from(a[13]) + u64::from(a[14]);
    let upper = subtra >> 32;
    let lower = subtra ^ (upper << 32);

    let s = sub(
        s,
        U64x4::from_u32([0, 0, lower as u32, upper as u32, 0, 0, 0, 0]),
    );

    if greater_equal(s, MODULO_P) {
        s - MODULO_P
    } else {
        s
    }
}

pub fn mul(x: U64x4, y: U64x4) -> U64x4 {
    reduce(raw_mul(x, y))
}

pub fn div(x: U64x4, y: U64x4) -> U64x4 {
    let q = get_mul_inv(y);
    mul(x, q)
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate test;

    use super::*;

    use self::test::Bencher;
    use rand::random;

    fn rand_elem() -> U64x4 {
        U64x4::new(
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        )
    }

    #[test]
    fn test_helper_mul() {
        let a = 1327187218989_u64;
        let b = 532432746434597_u64;
        let (r, c) = helper_mul(a, b);
        assert_eq!(c, 0x2488467);
        assert_eq!(r, 0x454d_d932_de46_f081);
    }

    #[test]
    fn test_raw_mul() {
        let a = U64x4::new(
            0x1351534EF350E2BB,
            0x14E68D77BC131F7B,
            0x6A7171A01A638E75,
            0x4F9EA7A816AB7908,
        );
        let b = U64x4::new(
            0x141CC66D0595B6F0,
            0xC85BF76622E07301,
            0x5B261629F8AD4D45,
            0x7DE9CF63BC635636,
        );
        let rst = raw_mul(a, b);

        assert_eq!(
            rst,
            [
                0x866d99203adc8150,
                0xc623d9758ed1332c,
                0x3b1dab20b950e375,
                0xbc165cad5d713996,
                0x63e9be904aa539b5,
                0x7edc6525c6a1f17c,
                0x2a99a65d2ec61248,
                0x27292fc3f99184ca
            ]
        );
    }

    #[bench]
    fn bench_raw_mul(ben: &mut Bencher) {
        let a = rand_elem();
        let b = rand_elem();
        ben.iter(|| {
            raw_mul(a, b);
        })
    }

    #[test]
    fn test_mul() {
        let ra = u64::from(random::<u32>());
        let rb = u64::from(random::<u32>());
        let (mut a, f1) = add_no_mod(MODULO_P, U64x4::new(ra, 0, 0, 0));
        let (mut b, f2) = add_no_mod(MODULO_P, U64x4::new(rb, 0, 0, 0));
        a = if f1 {
            println!("OVER A");
            a + RHO_P
        } else {
            a
        };
        b = if f2 {
            println!("OVER B");
            b + RHO_P
        } else {
            b
        };
        let c = mul(a, b);
        assert!(equal_to(c, U64x4::new(ra * rb, 0, 0, 0)));
    }

    #[bench]
    fn bench_mul(ben: &mut Bencher) {
        let a = rand_elem();

        let b = rand_elem();

        ben.iter(|| {
            mul(a, b);
        })
    }

    #[test]
    fn test_inversion() {
        let a = rand_elem();
        let b = get_mul_inv(a);
        assert!(equal_to(mul(a, b), U64x4::new(1, 0, 0, 0)));
    }

    #[bench]
    fn bench_inversion(ben: &mut Bencher) {
        let a = rand_elem();
        ben.iter(|| {
            get_mul_inv(a);
        })
    }

    #[bench]
    fn bench_add(ben: &mut Bencher) {
        let a = rand_elem();
        let b = rand_elem();
        ben.iter(|| {
            add(a, b);
        })
    }
}
