use ::basic::cell::yU64x4::*;
use ::basic::field::theField;
use ::basic::field::prime_field::*;

use std::fmt;
use std::fmt::Display;

use std::vec::Vec;
use bit_vec::BitVec;

trait ECC 
{

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
	pub Fp: prime_field,
	pub G: Point,
	pub l: u32, //the byte length of p
	pub EFp: yU64x4, // order of ECC_Fp
	pub n: yU64x4, // order of G
}

impl ECC_Fp
{
	fn pointEqualToO(&self, P: Point) -> bool
	{
		prime_field::equalToZero(P.x) && prime_field::equalToZero(P.y)
	}

	fn pointEqualTo(&self, P: Point, Q: Point) -> bool
	{
		prime_field::equalTo(P.x,Q.x) && prime_field::equalTo(P.y, Q.y)
	}

	fn projPointEqualToO(&self, P: ProjPoint) -> bool
	{
		prime_field::equalToZero(P.x) && prime_field::equalToOne(P.y) && prime_field::equalToZero(P.z)
	}

	fn projPointEqualTo(&self, P: ProjPoint, Q: ProjPoint) -> bool
	{
		let u = self.Fp.mulElement(P.z,self.Fp.getMultiplicationInverseElement(Q.z)); //u=z1*(z2^-1)

		//return x1==x2*u && y1==y2*u
		prime_field::equalTo(P.x,self.Fp.mulElement(Q.x,u)) && prime_field::equalTo(P.y,self.Fp.mulElement(Q.y,u))
	}

	fn jacobPointEqualToO(&self, P: JacobPoint) -> bool
	{ 
		prime_field::equalToOne(P.x) && prime_field::equalToOne(P.y) && prime_field::equalToZero(P.z)
	}

	fn jacobPointEuqalTo(&self, P: JacobPoint, Q: JacobPoint) -> bool
	{
		let pz2 = self.Fp.mulElement(P.z,P.z);
		let pz3 = self.Fp.mulElement(pz2,P.z);
		let qz2 = self.Fp.mulElement(Q.z,Q.z);
		let qz3 = self.Fp.mulElement(qz2,Q.z);

		let u1 = self.Fp.mulElement(P.x,qz2);
		let u2 = self.Fp.mulElement(Q.x,pz2);
		let s1 = self.Fp.mulElement(P.y,qz3);
		let s2 = self.Fp.mulElement(Q.y,pz3);
		//return x1==x2*u^2 && y1==y2*u^3
		prime_field::equalTo(u1,u2) && prime_field::equalTo(s1,s2)
	}

	pub fn affineToProj(&self, P: Point) -> ProjPoint
	{
		ProjPoint
		{
			x: P.x,
			y: P.y,
			z: yU64x4::new(1,0,0,0),
		}
	}

	pub fn affineToJacob(&self, P: Point) -> JacobPoint
	{
		JacobPoint
		{
			x: P.x,
			y: P.y,
			z: yU64x4::new(1,0,0,0),
		}
	}

	pub fn projToAffine(&self, P: ProjPoint) -> Point
	{
		let u = self.Fp.getMultiplicationInverseElement(P.z);

		Point
		{
			x: self.Fp.mulElement(P.x,u),
			y: self.Fp.mulElement(P.y,u),
		}
	}

	pub fn projToJacob(&self, P: ProjPoint) -> JacobPoint
	{
		let u = self.Fp.getMultiplicationInverseElement(P.z);

		JacobPoint
		{
			x: self.Fp.mulElement(P.x,u),
			y: self.Fp.mulElement(P.y,u),
			z: yU64x4::new(1,0,0,0),
		}
	}

	pub fn jacobToAffine(&self, P: JacobPoint) -> Point
	{
		let u = self.Fp.getMultiplicationInverseElement(P.z);
		let u2 = self.Fp.mulElement(u,u);
		let u3 = self.Fp.mulElement(u2,u);

		Point
		{
			x: self.Fp.mulElement(P.x,u2),
			y: self.Fp.mulElement(P.y,u3),
		}
	}

	pub fn jacobToProj(&self, P: JacobPoint) -> ProjPoint
	{
				let u = self.Fp.getMultiplicationInverseElement(P.z);
		let u2 = self.Fp.mulElement(u,u);
		let u3 = self.Fp.mulElement(u2,u);

		ProjPoint
		{
			x: self.Fp.mulElement(P.x,u2),
			y: self.Fp.mulElement(P.y,u3),
			z: yU64x4::new(1,0,0,0),
		}
	}
}

impl ECC_Fp
{
	// Create a Ecc(Fp): y^2 = x^3 + ax + b
	pub fn new(a: yU64x4, b: yU64x4, p: yU64x4, x0: yU64x4, y0: yU64x4, n: yU64x4) -> ECC_Fp
	{
		ECC_Fp
		{
			a,
			b,
			Fp: prime_field::new(p),
			G: Point::new(x0,y0),
			l: 32,
			EFp: yU64x4::new(0,0,0,0),
			n,
		}
	}
}

// Caculations on affine coordinate
impl ECC_Fp
{
	pub fn isPointInGroup(&self, P: Point) -> bool
	{
		let left = self.Fp.addElement(self.Fp.mulElement(self.Fp.addElement(self.Fp.mulElement(P.x,P.x),self.a),P.x),self.b);
		let right = self.Fp.mulElement(P.y,P.y);

		prime_field::equalTo(left,right)
	}

	pub fn getInvPoint(&self, P: Point) -> Point
	{
		Point
		{
			x: P.x,
			y: self.Fp.getAdditionInverseElement(P.y),
		}
	}

	pub fn isReciprocal(&self, P: Point, Q: Point) -> bool
	{
		return prime_field::equalTo(P.x,Q.x) && prime_field::equalTo(P.y, self.Fp.getAdditionInverseElement(Q.y))
	}

	pub fn addPoint(&self, P: Point, Q: Point) -> Point
	{
		if (self.pointEqualToO(P)||self.pointEqualToO(Q))
		{
			Point
			{
				x: prime_field::add_yU64x4(P.x, Q.x),
				y: prime_field::add_yU64x4(P.y, Q.y),
			}
		}
		else if (self.isReciprocal(P, Q))
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
			if (prime_field::equalTo(P.x,Q.x))
			{
				let x2 = self.Fp.mulElement(P.x, P.x); //x2 = x^2
				let tx2 = self.Fp.addElement(x2, self.Fp.addElement(x2, x2)); // tx2 = 3x^2
				let dx = self.Fp.addElement(tx2, self.a); // dx = 3x^2+a;
				let dy = self.Fp.addElement(P.y,P.y);
				let dyi = self.Fp.getMultiplicationInverseElement(dy);
				self.Fp.divElement(dx,dy)	 //= (3x^2+a)/2y
				
			}	
			else 
			{
				let s1 = self.Fp.subElement(Q.y,P.y);
				let s2 = self.Fp.subElement(Q.x,P.x);
				self.Fp.divElement(s1,s2)
			};

			let lambda2 = self.Fp.mulElement(lambda, lambda);

			let X = self.Fp.subElement(lambda2, self.Fp.addElement(P.x, Q.x));
			let Y = self.Fp.subElement(self.Fp.mulElement(lambda, self.Fp.subElement(P.x,X)),P.y);

			Point
			{
				x: X,
				y: Y,
			}
		}
	}

	pub fn doublePoint(&self, P: Point) -> Point
	{
		if (self.pointEqualToO(P))
		{
			Point
			{
				x: yU64x4::new(0,0,0,0),
				y: yU64x4::new(0,0,0,0),
			}
		}
		else 
		{
			let x2 = self.Fp.mulElement(P.x, P.x);
			let tx2 = self.Fp.addElement(x2, self.Fp.addElement(x2, x2));
			let lambda = self.Fp.divElement(self.Fp.subElement(tx2, self.a),self.Fp.addElement(P.y,P.y));

			let lambda2 = self.Fp.mulElement(lambda, lambda);

			let X = self.Fp.subElement(lambda2, self.Fp.addElement(P.x, P.x));
			let Y = self.Fp.subElement(self.Fp.mulElement(lambda, self.Fp.subElement(P.x,X)),P.y);

			Point
			{
				x: X,
				y: Y,
			}
		}
	}

	pub fn timesPoint(&self, mut P: Point, mut times: yU64x4) -> Point
	{
		let mut T = Point
		{
			x: yU64x4::new(0,0,0,0),
			y: yU64x4::new(0,0,0,0),
		};

		while (!prime_field::equalToOne(times))
		{
			if(times.value.0%2==0)
			{
				times.rightShift1();
				P = self.addPoint(P,P);
			}
			else 
			{
				times.value.0 -= 1;
				T = self.addPoint(T,P);	
			}
		}

		self.addPoint(P,T)
	}
}

// Calculations on Projective Coordinate
impl ECC_Fp
{

}

// Calculations on Jacobian Projective Coordinate
impl ECC_Fp
{
	pub fn getInvJacobPoint(&self, P: JacobPoint) -> JacobPoint
	{
		JacobPoint
		{
			x: P.x,
			y: self.Fp.getAdditionInverseElement(P.y),
			z: P.z,
		}
	}

	pub fn isJacobReciprocal(&self, P: JacobPoint, Q: JacobPoint) -> bool
	{
		let pz2 = self.Fp.mulElement(P.z,P.z);
		let pz3 = self.Fp.mulElement(pz2,P.z);
		let qz2 = self.Fp.mulElement(Q.z,Q.z);
		let qz3 = self.Fp.mulElement(qz2,Q.z);

		let u1 = self.Fp.mulElement(P.x,qz2);
		let u2 = self.Fp.mulElement(Q.x,pz2);
		let s1 = self.Fp.mulElement(P.y,qz3);
		let s2 = self.Fp.mulElement(Q.y,pz3);

		prime_field::equalTo(u1,u2) &&
		prime_field::equalTo(self.Fp.getAdditionInverseElement(s1),s2)
	}

	pub fn addJacobPoint(&self, P: JacobPoint, Q: JacobPoint) -> JacobPoint
	{
		if(self.jacobPointEqualToO(P))
		{
			Q
		}
		else if(self.jacobPointEqualToO(Q))
		{
			P
		}
		else
		{
			let pz2 = self.Fp.mulElement(P.z,P.z); //
			let qz2 = self.Fp.mulElement(Q.z,Q.z); //
			let pz3 = self.Fp.mulElement(pz2,P.z); //
			let qz3 = self.Fp.mulElement(qz2,Q.z); //
			let lambda1 = self.Fp.mulElement(P.x,qz2); //
			let lambda2 = self.Fp.mulElement(Q.x,pz2); //

			if(prime_field::equalTo(lambda1,lambda2)) //P=Q)
			{
				let lambda4 = self.Fp.mulElement(P.y,qz3); //
				let lambda5 = self.Fp.mulElement(Q.y,pz3); //
				if(prime_field::equalTo(lambda4,lambda5))
				{
					let lambda3 = self.Fp.subElement(lambda1,lambda2); //
					let lambda6 = self.Fp.subElement(lambda4,lambda5); //
					let lambda7 = self.Fp.addElement(lambda1,lambda2); //
					let lambda8 = self.Fp.addElement(lambda4,lambda5); //
					let l6l6 = self.Fp.mulElement(lambda6,lambda6); //
					let l7l3l3 = self.Fp.mulElement(lambda7,self.Fp.mulElement(lambda3,lambda3)); //
					let x = self.Fp.subElement(l6l6,l7l3l3); //
					let lambda9 = self.Fp.subElement(l7l3l3,self.Fp.addElement(x,x)); //
					let l9l6 = self.Fp.mulElement(lambda9,lambda6); //
					let l8l3l3 = self.Fp.mulElement(lambda8,self.Fp.mulElement(lambda3,lambda3)); //

					let l8l3l3l3 = self.Fp.mulElement(l8l3l3,lambda3);
					let mut y = self.Fp.subElement(l9l6,l8l3l3l3); //
					y = self.Fp.mulElement(y, self.Fp.inv2); //
					let z = self.Fp.mulElement(self.Fp.mulElement(P.z, Q.z),lambda3); 		

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
			else //P!=Q
			{
				let px2 = self.Fp.mulElement(P.x, P.x); // px2 = px^2 
				//let pz2 = self.Fp.mulElement(P.z, P.z); // pz2 = pz^2
				let pz4 = self.Fp.mulElement(pz2, pz2);	// pz4 = pz^4
				let px2_2 = self.Fp.addElement(px2,px2); // px2_2 = 2px^2
				let px2_3 = self.Fp.addElement(px2_2,px2); // px2_3 = 3px^2
				let lambda1 = self.Fp.addElement(px2_3,self.Fp.mulElement(self.a,pz4));
					// l1 = 3*px^2+a*pz^4
				let py2 = self.Fp.mulElement(P.y,P.y); // py2 = py^2
				let py_2 = self.Fp.addElement(P.y,P.y); // py_2 = 2*py
				let py2_2 = self.Fp.addElement(py2, py2); // py2_2 = 2*py^2
				let py2_4 = self.Fp.addElement(py2_2, py2_2); // py2_4 = 4*py^2
				let lambda2 = self.Fp.mulElement(py2_4,P.x); // l2 = 4*px*py^2
				let l2_2 = self.Fp.addElement(lambda2,lambda2); // l2 = 2*l2
				let py4_4 = self.Fp.mulElement(py2_2,py2_2); // py4_4 = 4*py^4
				let lambda3 = self.Fp.addElement(py4_4, py4_4); // l3 = 8^py^4
				let l1l1 = self.Fp.mulElement(lambda1, lambda1); // l1l1 = l1^2
				let x = self.Fp.subElement(l1l1, l2_2); // x3 = l1^2 - 2*l2
				let m1 = self.Fp.subElement(lambda2,x); // m1 = l2 - x3
				let m2 = self.Fp.mulElement(lambda1,m1); // m2 = l1*(l2-x3)
				let y = self.Fp.subElement(m2,lambda3); // y = l1*(l2-x3)-l3
				let z = self.Fp.mulElement(py_2,P.z); // z = 2*py*pz

				JacobPoint
				{
					x,
					y,
					z,
				}
			}
		}
	}

	pub fn timesJacobPoint(&self, mut P: JacobPoint, mut times: yU64x4) -> JacobPoint
	{
		let mut T = JacobPoint
		{
			x: yU64x4::new(1,0,0,0),
			y: yU64x4::new(1,0,0,0),
			z: yU64x4::new(0,0,0,0),
		};

		while (!prime_field::equalToOne(times))
		{
			if(times.value.0%2==0)
			{
				times.rightShift1();
				P = self.addJacobPoint(P,P);
			}
			else 
			{
				times.value.0 -= 1;
				T = self.addJacobPoint(T,P);	
			}
		}

		self.addJacobPoint(P,T)
	}
}

// Convert
impl ECC_Fp
{
	fn bytes2Int()
	{

	}

	fn int2Bytes(&self, int: yU64x4, len: usize) -> Vec<u8>
	{
		if(len<32)
		{
			panic!("Cannot convert yU64x4 to Vec<u8> shorter than 32");
		}

		let mut vec = Vec::new();

		for i in 0..32
		{
			vec.push(int.getByte(i));
		}

		for i in 32..len
		{
			vec.push(0u8);
		}

		vec
	}

	fn bits2Bytes()
	{

	}

	fn bytes2Bits()
	{

	}

	fn element2Bytes()
	{

	}

	fn bytes2Element(&self, x: yU64x4) -> yU64x4
	{
		assert!(prime_field::largerEqualThan(x,self.Fp.prime));

		x
	}

	fn element2Int(&self, x: yU64x4) -> yU64x4
	{
		x
	}

	fn point2Bytes(&self, P: Point)
	{
		
	}

	fn bytes2Point()
	{

	}
}