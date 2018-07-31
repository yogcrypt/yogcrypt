use ::basic::cell::yU64x4::{equalTo, equalToOne, equalToZero, yU64x4};
use ::basic::field::Fp::*;
use ::basic::field::Fn::n;

use std::fmt;
use std::fmt::Display;

use std::vec::Vec;
use bit_vec::BitVec;

pub const a: yU64x4 = yU64x4{value: [0xFFFFFFFFFFFFFFFC, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF],};
pub const b: yU64x4 = yU64x4{value: [0xDDBCBD414D940E93, 0xF39789F515AB8F92, 0x4D5A9E4BCF6509A7, 0x28E9FA9E9D9F5E34],};
pub const Gx: yU64x4 = yU64x4{value: [0x715A4589334C74C7, 0x8FE30BBFF2660BE1, 0x5F9904466A39C994, 0x32C4AE2C1F198119],};
pub const Gy: yU64x4 = yU64x4{value: [0x02DF32E52139F0A0, 0xD0A9877CC62A4740, 0x59BDCEE36B692153, 0xBC3736A2F4F6779C],};

pub const G: Point = 
Point
{
	x: Gx,
	y: Gy,
};

pub const zeroJacob: JacobPoint = 
JacobPoint
{	
	x: yU64x4{value: [1, 0, 0, 0]},
	y: yU64x4{value: [1, 0, 0, 0]},
	z: yU64x4{value: [0, 0, 0, 0]},
};

lazy_static! 
{
	static ref lowTable: Vec<Point> = {
		// save G, 2G, 4G, ... for later use
		let Gj = affineToJacob(G);
		let mut powG: Vec<JacobPoint> = vec![Gj];
		for i in 1..256
		{
			if let Some(&T) = powG.last() 
			{
				powG.push(addJacobPoint(T, T));
			} 
		}

		// find the desired values
		let mut table: Vec<Point> = Vec::new();
		for i in 0..256
		{
			let mut j = i;
			let mut T = zeroJacob;
			let mut k = 0;
			while(j != 0) 
			{
				if (j & 1 != 0)
				{
					// T = T + 2^{32p}G
					T = addJacobPoint(T, powG[k << 5]);
				}
				j >>= 1;
				k += 1;
			}
			table.push(jacobToAffine(T));
		}
		table
	};

	static ref highTable: Vec<Point> = {
		// save G, 2G, 4G, ... for later use
		let Gj = affineToJacob(G);
		let mut powG: Vec<JacobPoint> = vec![Gj];
		for i in 1..256
		{
			if let Some(&T) = powG.last() 
			{
				powG.push(addJacobPoint(T, T));
			} 
		}

		// find the desired values
		let mut table: Vec<Point> = Vec::new();
		for i in 0..256
		{
			let mut j = i;
			let mut T = zeroJacob;
			let mut k = 0;
			while(j != 0) 
			{
				if (j & 1 != 0)
				{
					// T = T + 2^{32p} * G
					T = addJacobPoint(T, powG[(k << 5) + (1 << 4)]);
				}
				j >>= 1;
				k += 1;
			}
			table.push(jacobToAffine(T));
		}
		table
	};
}

#[derive(Copy, Clone)]
pub struct Point
{
	pub x: yU64x4,
	pub y: yU64x4,
}

impl Display for Point
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f,"({} ,{})", self.x, self.y)
	}
}

#[derive(Copy, Clone)]
pub struct ProjPoint
{
	pub x: yU64x4,
	pub y: yU64x4,
	pub z: yU64x4,
}

impl Display for ProjPoint
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f,"({} ,{}, {})", self.x, self.y, self.z)
	}
}

#[derive(Copy, Clone)]
pub struct JacobPoint
{
	pub x: yU64x4,
	pub y: yU64x4,
	pub z: yU64x4,
}

impl Display for JacobPoint
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f,"({} ,{}, {})", self.x, self.y, self.z)
	}
}

impl Point
{
	pub fn new(x: yU64x4, y: yU64x4) -> Point
	{
		Point
		{
			x,
			y,
		}
	}
}

impl ProjPoint
{
	pub fn new(x: yU64x4, y: yU64x4, z: yU64x4) -> ProjPoint
	{
		ProjPoint
		{
			x,
			y,
			z,
		}
	}
}

impl JacobPoint
{
	pub fn new(x: yU64x4, y: yU64x4, z: yU64x4) -> JacobPoint
	{
		JacobPoint
		{
			x,
			y,
			z,
		}
	}
}

pub struct ECC_Fp
{
	pub a: yU64x4,
	pub b: yU64x4,
	pub G: Point,
	pub n: yU64x4, // order of G
}

pub fn isOnCurve(P: Point) -> bool
{
	// is y^2 = x^3 + ax + b ?
	equalTo(mul(P.y, P.y), add(add(mul(mul(P.x, P.x), P.x), mul(a, P.x)), b))
}

fn pointEqualToO(P: Point) -> bool
{
	equalToZero(P.x) && equalToZero(P.y)
}

fn pointEqualTo(P: Point, Q: Point) -> bool
{
	equalTo(P.x,Q.x) && equalTo(P.y, Q.y)
}

fn projPointEqualToO(P: ProjPoint) -> bool
{
	equalToZero(P.x) && equalToOne(P.y) && equalToZero(P.z)
}

fn projPointEqualTo(P: ProjPoint, Q: ProjPoint) -> bool
{
	let u = mul(P.z, getMulInv(Q.z)); //u=z1*(z2^-1)

	//return x1==x2*u && y1==y2*u
	equalTo(P.x, mul(Q.x,u)) && equalTo(P.y, mul(Q.y,u))
}

fn jacobPointEqualToO(P: JacobPoint) -> bool
{ 
	equalToOne(P.x) && equalToOne(P.y) && equalToZero(P.z)
}

fn jacobPointEuqalTo(P: JacobPoint, Q: JacobPoint) -> bool
{
	let pz2 = mul(P.z,P.z);
	let pz3 = mul(pz2,P.z);
	let qz2 = mul(Q.z,Q.z);
	let qz3 = mul(qz2,Q.z);

	let u1 = mul(P.x,qz2);
	let u2 = mul(Q.x,pz2);
	let s1 = mul(P.y,qz3);
	let s2 = mul(Q.y,pz3);
	//return x1==x2*u^2 && y1==y2*u^3
	equalTo(u1,u2) && equalTo(s1,s2)
}

pub fn affineToProj(P: Point) -> ProjPoint
{
	ProjPoint
	{
		x: P.x,
		y: P.y,
		z: yU64x4::new(1,0,0,0),
	}
}

pub fn affineToJacob(P: Point) -> JacobPoint
{
	if(!pointEqualToO(P))
	{
		JacobPoint
		{
			x: P.x,
			y: P.y,
			z: yU64x4::new(1,0,0,0),
		}
	}
	else 
	{
		zeroJacob
	}
}

pub fn projToAffine(P: ProjPoint) -> Point
{
	let u = getMulInv(P.z);

	Point
	{
		x: mul(P.x,u),
		y: mul(P.y,u),
	}
}

pub fn projToJacob(P: ProjPoint) -> JacobPoint
{
	let u = getMulInv(P.z);

	JacobPoint
	{
		x: mul(P.x,u),
		y: mul(P.y,u),
		z: yU64x4::new(1,0,0,0),
	}
}

pub fn jacobToAffine(P: JacobPoint) -> Point
{
	let u = getMulInv(P.z);
	let u2 = mul(u,u);
	let u3 = mul(u2,u);

	Point
	{
		x: mul(P.x,u2),
		y: mul(P.y,u3),
	}
}

pub fn jacobToProj(P: JacobPoint) -> ProjPoint
{
	let u = getMulInv(P.z);
	let u2 = mul(u,u);
	let u3 = mul(u2,u);

	ProjPoint
	{
		x: mul(P.x,u2),
		y: mul(P.y,u3),
		z: yU64x4::new(1,0,0,0),
	}
}

pub fn getInvPoint(P: Point) -> Point
{
	Point
	{
		x: P.x,
		y: getAddInv(P.y),
	}
}

pub fn isPointRec(P: Point, Q: Point) -> bool
{
	return equalTo(P.x,Q.x) && equalTo(P.y, getAddInv(Q.y))
}

pub fn addPoint(P: Point, Q: Point) -> Point
{
	if (pointEqualToO(P)||pointEqualToO(Q))
	{
		Point
		{
			x: P.x + Q.x,
			y: P.y + Q.y,
		}
	}
	else if (isPointRec(P, Q))
	{
		Point
		{
			x: yU64x4::new(0,0,0,0),
			y: yU64x4::new(0,0,0,0),
		}
	}
	else
	{
		let lambda = 
		if (equalTo(P.x,Q.x))
		{
			let x2 = mul(P.x, P.x); //x2 = x^2
			let tx2 = add(x2, add(x2, x2)); // tx2 = 3x^2
			let dx = add(tx2, a); // dx = 3x^2+a;
			let dy = add(P.y,P.y);
			let dyi = getMulInv(dy);
			div(dx,dy)	 //= (3x^2+a)/2y
			
		}	
		else 
		{
			let s1 = sub(Q.y,P.y);
			let s2 = sub(Q.x,P.x);
			div(s1,s2)
		};

		let lambda2 = mul(lambda, lambda);

		let X = sub(lambda2, add(P.x, Q.x));
		let Y = sub(mul(lambda, sub(P.x,X)),P.y);

		Point
		{
			x: X,
			y: Y,
		}
	}
}

pub fn getInvJacobPoint(P: JacobPoint) -> JacobPoint
{
	JacobPoint
	{
		x: P.x,
		y: getAddInv(P.y),
		z: P.z,
	}
}

pub fn isJacobReciprocal(P: JacobPoint, Q: JacobPoint) -> bool
{
	let pz2 = mul(P.z,P.z);
	let pz3 = mul(pz2,P.z);
	let qz2 = mul(Q.z,Q.z);
	let qz3 = mul(qz2,Q.z);

	let u1 = mul(P.x,qz2);
	let u2 = mul(Q.x,pz2);
	let s1 = mul(P.y,qz3);
	let s2 = mul(Q.y,pz3);

	equalTo(u1,u2) &&
	equalTo(getAddInv(s1),s2)
}

// Note: this function should
// ALWAYS be called with different point
pub fn addJacobAffine(P: JacobPoint, Q: Point) -> JacobPoint
{
	if (jacobPointEqualToO(P))
	{
		affineToJacob(Q)
	}
	else if (pointEqualToO(Q))
	{
		P
	}
	else 
	{
		let z2 = mul(P.z, P.z);
		let A = mul(Q.x, z2);
		let B = mul(mul(Q.y, z2), P.z);
		let C = sub(A, P.x);
		let D = sub(B, P.y);
		let C2 = mul(C, C);
		let C3 = mul(C2, C);
		let X1C2 = mul(P.x, C2);
		let x = sub(sub(mul(D, D), add(X1C2, X1C2)), C3);
		let y = sub(mul(D, sub(X1C2, x)), mul(P.y, C3));
		let z = mul(P.z, C);
		JacobPoint
		{
			x, y, z,
		}
	}
}

pub fn negJacob(P: JacobPoint) -> JacobPoint
{
	let x = P.x;
	let y = -P.y;
	let z = P.z;
	JacobPoint
	{
		x, y, z,
	}
}

pub fn addJacobPoint(P: JacobPoint, Q: JacobPoint) -> JacobPoint
{
	if(jacobPointEqualToO(P))
	{
		Q
	}
	else if(jacobPointEqualToO(Q))
	{
		P
	}
	else
	{
		let pz2 = mul(P.z,P.z); // pz2 = pz^2
		let qz2 = mul(Q.z,Q.z); // qz2 = qz^2
		let pz3 = mul(pz2,P.z); // pz3
		let qz3 = mul(qz2,Q.z); //
		let lambda1 = mul(P.x,qz2); //
		let lambda2 = mul(Q.x,pz2); //

		if(!equalTo(lambda1,lambda2)) //P!=Q
		{
			let lambda4 = mul(P.y,qz3); //
			let lambda5 = mul(Q.y,pz3); //
			if(!equalTo(lambda4,lambda5))
			{
				let lambda3 = sub(lambda1,lambda2); //
				let lambda6 = sub(lambda4,lambda5); //
				let lambda7 = add(lambda1,lambda2); //
				let lambda8 = add(lambda4,lambda5); //
				let l6l6 = mul(lambda6,lambda6); //
				let l7l3l3 = mul(lambda7,mul(lambda3,lambda3)); //
				let x = sub(l6l6,l7l3l3); //
				let lambda9 = sub(l7l3l3,add(x,x)); // l9 = l7l3l3 - 2x
				let l9l6 = mul(lambda9,lambda6); //
				let l8l3l3 = mul(lambda8,mul(lambda3,lambda3)); //

				let l8l3l3l3 = mul(l8l3l3,lambda3);
				let mut y = sub(l9l6,l8l3l3l3); //
				y = mul(y, inv2P); //
				let z = mul(mul(P.z, Q.z),lambda3); 	

				JacobPoint
				{
					x,
					y,
					z,
				}
			}
			else 
			{
				JacobPoint
				{
					x: yU64x4::new(1,0,0,0),
					y: yU64x4::new(1,0,0,0),
					z: yU64x4::new(0,0,0,0),
				}
			}
		}
		else //P=Q
		{
			let px2 = mul(P.x, P.x); // px2 = px^2 
			//let pz2 = mul(P.z, P.z); // pz2 = pz^2
			let pz4 = mul(pz2, pz2);	// pz4 = pz^4
			let px2_2 = add(px2,px2); // px2_2 = 2px^2
			let px2_3 = add(px2_2,px2); // px2_3 = 3px^2
			let lambda1 = add(px2_3,mul(a,pz4));
				// l1 = 3*px^2+a*pz^4
			let py2 = mul(P.y,P.y); // py2 = py^2
			let py_2 = add(P.y,P.y); // py_2 = 2*py
			let py2_2 = add(py2, py2); // py2_2 = 2*py^2
			let py2_4 = add(py2_2, py2_2); // py2_4 = 4*py^2
			let lambda2 = mul(py2_4,P.x); // l2 = 4*px*py^2
			let l2_2 = add(lambda2,lambda2); // l2 = 2*l2
			let py4_4 = mul(py2_2,py2_2); // py4_4 = 4*py^4
			let lambda3 = add(py4_4, py4_4); // l3 = 8^py^4
			let l1l1 = mul(lambda1, lambda1); // l1l1 = l1^2
			let x = sub(l1l1, l2_2); // x3 = l1^2 - 2*l2
			let m1 = sub(lambda2,x); // m1 = l2 - x3
			let m2 = mul(lambda1,m1); // m2 = l1*(l2-x3)
			let y = sub(m2,lambda3); // y = l1*(l2-x3)-l3
			let z = mul(py_2,P.z); // z = 2*py*pz

			JacobPoint
			{
				x,
				y,
				z,
			}
		}
	}
}

// Note: this function return A Jacob Point
pub fn timesPoint(P: Point, times: yU64x4) -> JacobPoint
{
	let mut T = zeroJacob;

	for blocki in (0..4).rev()
	{
		for i in (0..64).rev()
		{
			T = addJacobPoint(T, T);
			if (times.value[blocki] & (1 << i)) != 0
			{
				T = addJacobAffine(T, P);
			}
		}
	}
	T
}

//#[inline]
fn getBit(u: u64, i: usize) -> usize
{
	((u >> i) & 1) as usize
}

//#[inline]
fn toIndex(u: yU64x4, i: usize) -> usize
{
	getBit(u.value[0], i)
	+ (getBit(u.value[0], 32 + i) << 1)
	+ (getBit(u.value[1], i) << 2)
	+ (getBit(u.value[1], 32 + i) << 3)
	+ (getBit(u.value[2], i) << 4)
	+ (getBit(u.value[2], 32 + i) << 5)
	+ (getBit(u.value[3], i) << 6)
	+ (getBit(u.value[3], 32 + i) << 7)
}

// Speed up using Fixed-base comb
// described in "Software Implementation of the NIST Elliptic
// Curves Over Prime Fields" by M Brown et. al.
pub fn timesBasePoint(times: yU64x4) -> JacobPoint
{
	let mut T = zeroJacob;
	for i in (0..16).rev()
	{
		T = addJacobPoint(T, T);
		let indexLow = toIndex(times, i);
		let indexHigh = toIndex(times, i + 16);
		T = addJacobAffine(T, lowTable[indexLow]);
		T = addJacobAffine(T, highTable[indexHigh]);
	}
	T
}

#[cfg(test)]
mod tests 
{
    extern crate test;
    extern crate rand;

    use super::*;
    use ::basic::cell::yU64x4::*;

    use self::test::Bencher;
    use rand::random;

	fn rand_elem() -> yU64x4
    {
        yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>())
    }

	#[test]
	fn test_add_Jacob_affine()
	{
		let L = affineToJacob(G);
		let G2 = addJacobPoint(L, L);
		let S1 = addJacobPoint(G2, L);
		let S2 = addJacobAffine(G2, G);
		assert!(jacobPointEuqalTo(S1, S2));
	}

	#[test]
	fn test_zero_add_Jacob_affine()
	{
		let L = affineToJacob(G);
		let z = zeroJacob;
		let S1 = addJacobPoint(z, L);
		let S2 = addJacobAffine(z, G);
		assert!(jacobPointEuqalTo(S1, S2));
	}

	#[test]
	fn test_times3()
	{
		let L = affineToJacob(G);
		let G2 = addJacobPoint(L, L);
		let S1 = addJacobPoint(G2, L);
		let S2 = timesPoint(G, yU64x4::new(3, 0, 0, 0));
		assert!(jacobPointEuqalTo(S1, S2));
	}

	#[test]
	fn test_BaseTimes()
	{
		let r = rand_elem();
		let S1 = timesPoint(G, r);
		let S2 = timesBasePoint(r);
		assert!(jacobPointEuqalTo(S1, S2));
	}

    #[bench]
    fn bench_times(ben: &mut Bencher)
    {
		let r = rand_elem();
        ben.iter(||
        {
			timesPoint(G, r);
        })
    }

	#[bench]
    fn bench_timesBase(ben: &mut Bencher)
    {
		let r = rand_elem();
        ben.iter(||
        {
			timesBasePoint(r);
        })
    }
}
