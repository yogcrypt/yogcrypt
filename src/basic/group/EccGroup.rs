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
	JacobPoint
	{
		x: P.x,
		y: P.y,
		z: yU64x4::new(1,0,0,0),
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

pub fn timesPoint(mut P: Point, mut times: yU64x4) -> Point
{
	let mut T = Point
	{
		x: yU64x4::new(0,0,0,0),
		y: yU64x4::new(0,0,0,0),
	};

	while (!equalToOne(times))
	{
		if(times.value[0]%2==0)
		{
			times.rightShift1();
			P = addPoint(P,P);
		}
		else 
		{
			times.value[0] -= 1;
			T = addPoint(T,P);	
		}
	}

	addPoint(P,T)
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

pub fn timesJacobPoint(mut P: JacobPoint, mut times: yU64x4) -> JacobPoint
{
	let mut T = JacobPoint
	{
		x: yU64x4::new(1,0,0,0),
		y: yU64x4::new(1,0,0,0),
		z: yU64x4::new(0,0,0,0),
	};

	while (!equalToOne(times))
	{
		if(times.value[0]%2==0)
		{
			times.rightShift1();
			P = addJacobPoint(P,P);
		}
		else 
		{
			times.value[0] -= 1;
			T = addJacobPoint(T,P);	
		}
	}

	addJacobPoint(P,T)
}
