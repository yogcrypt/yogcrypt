extern crate rand;
#[macro_use]
extern crate criterion;
extern crate yogcrypt;

use criterion::Criterion;
use rand::random;
use yogcrypt::basic::cell::u64x4::*;

fn rand_u64x4() -> U64x4 {
    U64x4::new(
        random::<u64>(),
        random::<u64>(),
        random::<u64>(),
        random::<u64>(),
    )
}

mod sm2_benches {
    use super::*;
    use yogcrypt::sm2::*;

    fn bench_gen_sign(c: &mut Criterion) {
        c.bench_function("sm2::gen_sign", move |b| {
            b.iter_with_setup(
                || {
                    let d_a = rand_u64x4();

                    let msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
                    let q = get_pub_key(d_a);
                    (d_a, msg, q)
                },
                |(d_a, msg, q)| sm2_gen_sign(&msg, d_a, q),
            )
        });
    }

    fn bench_ver_sign(c: &mut Criterion) {
        c.bench_function("sm2::ver_sign", move |b| {
            b.iter_with_setup(
                || {
                    let d_a = rand_u64x4();

                    let msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

                    let q = get_pub_key(d_a);

                    (msg, q, sm2_gen_sign(&msg, d_a, q))
                },
                |(msg, q, signature)| {
                    let t = sm2_ver_sign(&msg, q,  &signature);
                    assert!(t);
                },
            )
        });
    }

    criterion_group!(
        benches,
        sm2_benches::bench_gen_sign,
        sm2_benches::bench_ver_sign
    );
}

mod sm3_benches {
    use super::*;
    use yogcrypt::sm3::*;

    fn bench(c: &mut Criterion) {
        c.bench_function("sm3::hash", move |b| {
            b.iter(|| {
                let msg= b"abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd";

                sm3_enc(msg)
            });
        });
    }
    criterion_group!(benches, sm3_benches::bench);
}

mod sm4_benches {
    use super::*;
    use yogcrypt::sm4::*;

    fn bench_enc(c: &mut Criterion) {
        let m: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
        let p_txt: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

        c.bench_function("sm4::enc", move |b| {
            b.iter(|| {
                sm4_enc(&m, &p_txt);
            });
        });
    }

    fn bench_dec(c: &mut Criterion) {
        let m: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
        let p_txt: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

        c.bench_function("sm4::dec", move |b| {
            b.iter_with_setup(
                || {
                    sm4_enc(&m, &p_txt)
                },
                |c_txt| {
                    let p_txt2 = sm4_dec(&m, &c_txt);
                    assert_eq!(p_txt, p_txt2);
                },
            )
        });
    }
    criterion_group!(benches, sm4_benches::bench_enc, sm4_benches::bench_dec);

}

mod ecc_group_benches {
    use super::*;
    use yogcrypt::basic::group::ecc_group::*;

    fn bench_times(c: &mut Criterion) {
        c.bench_function("ecc_group::times_point", move |b| {
            b.iter_with_setup(
                || rand_u64x4(),
                |r| {
                    times_point(ECC_G, r);
                },
            )
        });
    }

    fn bench_times_base(c: &mut Criterion) {
        c.bench_function("ecc_group::times_base_point", move |b| {
            b.iter_with_setup(
                || rand_u64x4(),
                |r| {
                    times_base_point(r);
                },
            )
        });
    }
    criterion_group!(
        benches,
        ecc_group_benches::bench_times,
        ecc_group_benches::bench_times_base
    );
}

mod field_p_benches {
    use super::*;
    use yogcrypt::basic::field::field_p::*;

    fn rand_elem() -> FieldElement {
        FieldElement::from_u64([
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
            random::<u64>(),
        ])
    }

    fn bench_mul(c: &mut Criterion) {
        c.bench_function("field_p::mul", move |b| {
            b.iter_with_setup(|| (rand_elem(), rand_elem()), |(a, b)| a * b)
        });
    }

    fn bench_inversion(c: &mut Criterion) {
        c.bench_function("field_p::inv", move |b| {
            b.iter_with_setup(
                || rand_elem(),
                |a| {
                    get_mul_inv(a);
                },
            )
        });
    }

    fn bench_add(c: &mut Criterion) {
        c.bench_function("field_p:add", move |b| {
            b.iter_with_setup(|| (rand_elem(), rand_elem()), |(a, b)| a + b)
        });
    }

    criterion_group!(
        benches,
        field_p_benches::bench_mul,
        field_p_benches::bench_inversion,
        field_p_benches::bench_add
    );
}

criterion_main!(
    sm2_benches::benches,
    sm3_benches::benches,
    sm4_benches::benches,
    ecc_group_benches::benches,
    field_p_benches::benches
);
