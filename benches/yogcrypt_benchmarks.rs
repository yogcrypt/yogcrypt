#![feature(test)]
extern crate test;
extern crate yogcrypt;
extern crate rand;

use test::Bencher;
use rand::random;
use yogcrypt::basic::cell::u64x4::*;

mod sm2_benches {
    use yogcrypt::sm2::*;
    use super::*;

    #[bench]
    fn bench_gen_sign(ben: &mut Bencher) {
        let d_a = U64x4::new(
            0x0C23661D15897263,
            0x2A519A55171B1B65,
            0x068C8D803DFF7979,
            0x128B2FA8BD433C6C,
        );

        let msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

        let q = get_pub_key(d_a);

        ben.iter(|| {
            sm2_gen_sign(&msg, d_a, q, 4);
        });
    }

    #[bench]
    fn bench_ver_sign(ben: &mut Bencher) {
        let d_a = U64x4::new(
            0x0C23661D15897263,
            0x2A519A55171B1B65,
            0x068C8D803DFF7979,
            0x128B2FA8BD433C6C,
        );

        let msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

        let q = get_pub_key(d_a);

        let m = sm2_gen_sign(&msg, d_a, q, 4);

        ben.iter(|| {
            let t = sm2_ver_sign(&msg, q, 4, m.0, m.1);
            assert!(t);
        });
    }
}

mod sm3_benches {
    use yogcrypt::sm3::*;
    use super::*;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            let msg: [u32; 16] = [
                0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364,
                0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364, 0x61626364,
                0x61626364, 0x61626364,
            ];

            sm3_enc(&msg, 512)
        });
    }
}

mod sm4_benches {
    use yogcrypt::sm4::*;
    use super::*;

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            let m: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
            let r = get_sm4_r_k(&m);
            let p_txt: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

            let c_txt = sm4_enc(&r, &p_txt);

            let p_txt2 = sm4_dec(&r, &c_txt);
            assert_eq!(p_txt, p_txt2);
        });
    }
}

mod ecc_group_benches {
    use yogcrypt::basic::group::ecc_group::*;
    use super::*;

    fn rand_u64x4() -> U64x4 {
        U64x4::new(
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        )
    }

    #[bench]
    fn bench_times(ben: &mut Bencher) {
        let r = rand_u64x4();
        ben.iter(|| {
            times_point(ECC_G, r);
        })
    }

    #[bench]
    fn bench_times_base(ben: &mut Bencher) {
        let r = rand_u64x4();
        ben.iter(|| {
            times_base_point(r);
        })
    }
}

mod field_p_benches {
    use yogcrypt::basic::field::field_p::*;
    use super::*;

    fn rand_elem() -> FieldElement {
        FieldElement::from_u64([
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        ])
    }

    #[bench]
    fn bench_mul(ben: &mut Bencher) {
        let a = rand_elem();

        let b = rand_elem();

        ben.iter(|| a * b)
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
        ben.iter(|| a + b)
    }
}
