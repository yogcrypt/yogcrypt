use basic::cell::u64x4::*;
use basic::field::field_p::*;

use std::fmt;
use std::fmt::Display;

pub const ECC_A: U64x4 = U64x4 {
    value: [
        0xFFFFFFFFFFFFFFFC,
        0xFFFFFFFF00000000,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFEFFFFFFFF,
    ],
};
pub const ECC_B: U64x4 = U64x4 {
    value: [
        0xDDBCBD414D940E93,
        0xF39789F515AB8F92,
        0x4D5A9E4BCF6509A7,
        0x28E9FA9E9D9F5E34,
    ],
};
pub const ECC_GX: U64x4 = U64x4 {
    value: [
        0x715A4589334C74C7,
        0x8FE30BBFF2660BE1,
        0x5F9904466A39C994,
        0x32C4AE2C1F198119,
    ],
};
pub const ECC_GY: U64x4 = U64x4 {
    value: [
        0x02DF32E52139F0A0,
        0xD0A9877CC62A4740,
        0x59BDCEE36B692153,
        0xBC3736A2F4F6779C,
    ],
};

pub const ECC_G: Point = Point {
    x: ECC_GX,
    y: ECC_GY,
};

pub const ZERO_JACOBI: JacobiPoint = JacobiPoint {
    x: U64x4 {
        value: [1, 0, 0, 0],
    },
    y: U64x4 {
        value: [1, 0, 0, 0],
    },
    z: U64x4 {
        value: [0, 0, 0, 0],
    },
};

lazy_static! {
    static ref lowTable: Vec<Point> = {
        // save G, 2G, 4G, ... for later use
        let g_jacobi = affine_to_jacobi(ECC_G);
        let mut pow_g: Vec<JacobiPoint> = vec![g_jacobi];
        for _ in 1..256
        {
            if let Some(&t) = pow_g.last()
            {
                pow_g.push(add_jacobi_point(t, t));
            }
        }

        // find the desired values
        let mut table: Vec<Point> = Vec::new();
        for i in 0..256
        {
            let mut j = i;
            let mut t = ZERO_JACOBI;
            let mut k = 0;
            while j != 0
            {
                if j & 1 != 0
                {
                    // t = t + 2^{32p} * G
                    t = add_jacobi_point(t, pow_g[k << 5]);
                }
                j >>= 1;
                k += 1;
            }
            table.push(jacobi_to_affine(t));
        }
        table
    };

    static ref highTable: Vec<Point> = {
        // save G, 2G, 4G, ... for later use
        let g_jacobi = affine_to_jacobi(ECC_G);
        let mut pow_g: Vec<JacobiPoint> = vec![g_jacobi];
        for _ in 1..256
        {
            if let Some(&t) = pow_g.last()
            {
                pow_g.push(add_jacobi_point(t, t));
            }
        }

        // find the desired values
        let mut table: Vec<Point> = Vec::new();
        for i in 0..256
        {
            let mut j = i;
            let mut t = ZERO_JACOBI;
            let mut k = 0;
            while j != 0
            {
                if j & 1 != 0
                {
                    // T = T + 2^{32p + 16} * G
                    t = add_jacobi_point(t, pow_g[(k << 5) + (1 << 4)]);
                }
                j >>= 1;
                k += 1;
            }
            table.push(jacobi_to_affine(t));
        }
        table
    };
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: U64x4,
    pub y: U64x4,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ,{})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
pub struct JacobiPoint {
    pub x: U64x4,
    pub y: U64x4,
    pub z: U64x4,
}

impl Display for JacobiPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ,{}, {})", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn new(x: U64x4, y: U64x4) -> Point {
        Point { x, y }
    }
}

impl JacobiPoint {
    pub fn new(x: U64x4, y: U64x4, z: U64x4) -> JacobiPoint {
        JacobiPoint { x, y, z }
    }
}

pub fn is_on_curve(p: Point) -> bool {
    // is y^2 = x^3 + ax + b ?
    equal_to(
        mul(p.y, p.y),
        add(add(mul(mul(p.x, p.x), p.x), mul(ECC_A, p.x)), ECC_B),
    )
}

pub fn point_equal_to_zero(p: Point) -> bool {
    equal_to_zero(p.x) && equal_to_zero(p.y)
}

pub fn point_equal_to(p: Point, q: Point) -> bool {
    equal_to(p.x, q.x) && equal_to(p.y, q.y)
}

pub fn jacobi_point_equal_to_zero(p: JacobiPoint) -> bool {
    equal_to_one(p.x) && equal_to_one(p.y) && equal_to_zero(p.z)
}

pub fn jacobi_point_equal_to(p: JacobiPoint, q: JacobiPoint) -> bool {
    let pz2 = mul(p.z, p.z);
    let pz3 = mul(pz2, p.z);
    let qz2 = mul(q.z, q.z);
    let qz3 = mul(qz2, q.z);

    let u1 = mul(p.x, qz2);
    let u2 = mul(q.x, pz2);
    let s1 = mul(p.y, qz3);
    let s2 = mul(q.y, pz3);
    //return x1==x2*u^2 && y1==y2*u^3
    equal_to(u1, u2) && equal_to(s1, s2)
}

pub fn affine_to_jacobi(p: Point) -> JacobiPoint {
    if !point_equal_to_zero(p) {
        JacobiPoint {
            x: p.x,
            y: p.y,
            z: U64x4::new(1, 0, 0, 0),
        }
    } else {
        ZERO_JACOBI
    }
}

pub fn jacobi_to_affine(p: JacobiPoint) -> Point {
    let u = get_mul_inv(p.z);
    let u2 = mul(u, u);
    let u3 = mul(u2, u);

    Point {
        x: mul(p.x, u2),
        y: mul(p.y, u3),
    }
}

pub fn get_inv_point(p: Point) -> Point {
    Point {
        x: p.x,
        y: get_add_inv(p.y),
    }
}

pub fn is_point_rec(p: Point, q: Point) -> bool {
    equal_to(p.x, q.x) && equal_to(p.y, get_add_inv(q.y))
}

pub fn add_point(p: Point, q: Point) -> Point {
    if point_equal_to_zero(p) || point_equal_to_zero(q) {
        Point {
            x: p.x + q.x,
            y: p.y + q.y,
        }
    } else if is_point_rec(p, q) {
        Point {
            x: U64x4::new(0, 0, 0, 0),
            y: U64x4::new(0, 0, 0, 0),
        }
    } else {
        let lambda = if equal_to(p.x, q.x) {
            let x2 = mul(p.x, p.x); //x2 = x^2
            let tx2 = add(x2, add(x2, x2)); // tx2 = 3x^2
            let dx = add(tx2, ECC_A); // dx = 3x^2+a;
            let dy = add(p.y, p.y);
            div(dx, dy) //= (3x^2+a)/2y
        } else {
            let s1 = sub(q.y, p.y);
            let s2 = sub(q.x, p.x);
            div(s1, s2)
        };

        let lambda2 = mul(lambda, lambda);

        let x = sub(lambda2, add(p.x, q.x));
        let y = sub(mul(lambda, sub(p.x, x)), p.y);

        Point { x, y }
    }
}

pub fn get_inv_jacobi_point(p: JacobiPoint) -> JacobiPoint {
    JacobiPoint {
        x: p.x,
        y: get_add_inv(p.y),
        z: p.z,
    }
}

pub fn is_jacobi_reciprocal(p: JacobiPoint, q: JacobiPoint) -> bool {
    let pz2 = mul(p.z, p.z);
    let pz3 = mul(pz2, p.z);
    let qz2 = mul(q.z, q.z);
    let qz3 = mul(qz2, q.z);

    let u1 = mul(p.x, qz2);
    let u2 = mul(q.x, pz2);
    let s1 = mul(p.y, qz3);
    let s2 = mul(q.y, pz3);

    equal_to(u1, u2) && equal_to(get_add_inv(s1), s2)
}

// Note: this function should
// ALWAYS be called with different point
pub fn add_jacobi_affine(p: JacobiPoint, q: Point) -> JacobiPoint {
    if jacobi_point_equal_to_zero(p) {
        affine_to_jacobi(q)
    } else if point_equal_to_zero(q) {
        p
    } else {
        let z2 = mul(p.z, p.z);
        let a = mul(q.x, z2);
        let b = mul(mul(q.y, z2), p.z);
        let c = sub(a, p.x);
        let d = sub(b, p.y);
        let c2 = mul(c, c);
        let c3 = mul(c2, c);
        let x1c2 = mul(p.x, c2);
        let x = sub(sub(mul(d, d), add(x1c2, x1c2)), c3);
        let y = sub(mul(d, sub(x1c2, x)), mul(p.y, c3));
        let z = mul(p.z, c);
        JacobiPoint { x, y, z }
    }
}

pub fn neg_jacobiacob(p: JacobiPoint) -> JacobiPoint {
    let x = p.x;
    let y = -p.y;
    let z = p.z;
    JacobiPoint { x, y, z }
}

pub fn add_jacobi_point(p: JacobiPoint, q: JacobiPoint) -> JacobiPoint {
    if jacobi_point_equal_to_zero(p) {
        q
    } else if jacobi_point_equal_to_zero(q) {
        p
    } else {
        let pz2 = mul(p.z, p.z);
        let qz2 = mul(q.z, q.z);
        let pz3 = mul(pz2, p.z);
        let qz3 = mul(qz2, q.z);
        let lambda1 = mul(p.x, qz2);
        let lambda2 = mul(q.x, pz2);

        //P != Q
        if !equal_to(lambda1, lambda2) {
            let lambda4 = mul(p.y, qz3);
            let lambda5 = mul(q.y, pz3);
            if !equal_to(lambda4, lambda5) {
                let lambda3 = sub(lambda1, lambda2);
                let lambda6 = sub(lambda4, lambda5);
                let lambda7 = add(lambda1, lambda2);
                let lambda8 = add(lambda4, lambda5);
                let l6l6 = mul(lambda6, lambda6);
                let l7l3l3 = mul(lambda7, mul(lambda3, lambda3));
                let x = sub(l6l6, l7l3l3);
                let lambda9 = sub(l7l3l3, add(x, x));
                let l9l6 = mul(lambda9, lambda6);
                let l8l3l3 = mul(lambda8, mul(lambda3, lambda3));

                let l8l3l3l3 = mul(l8l3l3, lambda3);
                let mut y = sub(l9l6, l8l3l3l3);
                y = mul(y, INV_2P);
                let z = mul(mul(p.z, q.z), lambda3);

                JacobiPoint { x, y, z }
            } else {
                ZERO_JACOBI
            }
        } else {
            //P=Q
            let px2 = mul(p.x, p.x); // px2 = px^2
            let pz4 = mul(pz2, pz2); // pz4 = pz^4
            let px2_2 = add(px2, px2); // px2_2 = 2px^2
            let px2_3 = add(px2_2, px2); // px2_3 = 3px^2
            let lambda1 = add(px2_3, mul(ECC_A, pz4)); // l1 = 3*px^2+a*pz^4
            let py2 = mul(p.y, p.y); // py2 = py^2
            let py_2 = add(p.y, p.y); // py_2 = 2*py
            let py2_2 = add(py2, py2); // py2_2 = 2*py^2
            let py2_4 = add(py2_2, py2_2); // py2_4 = 4*py^2
            let lambda2 = mul(py2_4, p.x); // l2 = 4*px*py^2
            let l2_2 = add(lambda2, lambda2); // l2 = 2*l2
            let py4_4 = mul(py2_2, py2_2); // py4_4 = 4*py^4
            let lambda3 = add(py4_4, py4_4); // l3 = 8^py^4
            let l1l1 = mul(lambda1, lambda1); // l1l1 = l1^2
            let x = sub(l1l1, l2_2); // x3 = l1^2 - 2*l2
            let m1 = sub(lambda2, x); // m1 = l2 - x3
            let m2 = mul(lambda1, m1); // m2 = l1*(l2-x3)
            let y = sub(m2, lambda3); // y = l1*(l2-x3)-l3
            let z = mul(py_2, p.z); // z = 2*py*pz

            JacobiPoint { x, y, z }
        }
    }
}

// Note: this function return A Jacobi Point
pub fn times_point(p: Point, times: U64x4) -> JacobiPoint {
    let mut t = ZERO_JACOBI;

    for blocki in (0..4).rev() {
        for i in (0..64).rev() {
            t = add_jacobi_point(t, t);
            if (times.value[blocki] & (1 << i)) != 0 {
                t = add_jacobi_affine(t, p);
            }
        }
    }
    t
}

//#[inline]
fn get_bit(u: u64, i: usize) -> usize {
    ((u >> i) & 1) as usize
}

//#[inline]
fn to_index(u: U64x4, i: usize) -> usize {
    get_bit(u.value[0], i)
        + (get_bit(u.value[0], 32 + i) << 1)
        + (get_bit(u.value[1], i) << 2)
        + (get_bit(u.value[1], 32 + i) << 3)
        + (get_bit(u.value[2], i) << 4)
        + (get_bit(u.value[2], 32 + i) << 5)
        + (get_bit(u.value[3], i) << 6)
        + (get_bit(u.value[3], 32 + i) << 7)
}

// Speed up using Fixed-base comb
// described in "Software Implementation of the NIST Elliptic
// Curves Over Prime Fields" by M Brown et. al.
pub fn times_base_point(times: U64x4) -> JacobiPoint {
    let mut t = ZERO_JACOBI;
    for i in (0..16).rev() {
        t = add_jacobi_point(t, t);
        let index_low = to_index(times, i);
        let index_high = to_index(times, i + 16);
        t = add_jacobi_affine(t, lowTable[index_low]);
        t = add_jacobi_affine(t, highTable[index_high]);
    }
    t
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
    fn test_add_jacobi_affine() {
        let l = affine_to_jacobi(ECC_G);
        let g2 = add_jacobi_point(l, l);
        let s1 = add_jacobi_point(g2, l);
        let s2 = add_jacobi_affine(g2, ECC_G);
        assert!(jacobi_point_equal_to(s1, s2));
    }

    #[test]
    fn test_zero_add_jacobi_affine() {
        let l = affine_to_jacobi(ECC_G);
        let z = ZERO_JACOBI;
        let s1 = add_jacobi_point(z, l);
        let s2 = add_jacobi_affine(z, ECC_G);
        assert!(jacobi_point_equal_to(s1, s2));
    }

    #[test]
    fn test_times3() {
        let l = affine_to_jacobi(ECC_G);
        let g2 = add_jacobi_point(l, l);
        let s1 = add_jacobi_point(g2, l);
        let s2 = times_point(ECC_G, U64x4::new(3, 0, 0, 0));
        assert!(jacobi_point_equal_to(s1, s2));
    }

    #[test]
    fn test_base_times() {
        let r = rand_elem();
        let s1 = times_point(ECC_G, r);
        let s2 = times_base_point(r);
        assert!(jacobi_point_equal_to(s1, s2));
    }

    #[bench]
    fn bench_times(ben: &mut Bencher) {
        let r = rand_elem();
        ben.iter(|| {
            times_point(ECC_G, r);
        })
    }

    #[bench]
    fn bench_times_base(ben: &mut Bencher) {
        let r = rand_elem();
        ben.iter(|| {
            times_base_point(r);
        })
    }
}
