use ::basic::cell::yU64x4::*;
use ::basic::group::EccGroup::*;
use ::basic::field::primeField::*;
use ::basic::field::theField;
use rand::random;
use std::vec::Vec;
use ::sm3::*;

const prime: yU64x4 = 
yU64x4
{
	value: (0xFFFFFFFFFFFFFFFF , 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF),
};

const negPrime: yU64x4 = 
yU64x4
{
	value: (0x0000000000000001, 0x00000000FFFFFFFF, 0x0000000000000000, 0x0000000100000000),
};

const rho: yU64x4 = 
yU64x4
{
	value: (0x0000000000000001, 0x00000000FFFFFFFF, 0x0000000000000000, 0x0000000100000000),
};

const rho2: yU64x4 = 
yU64x4
{
	value: (0x0000000200000003, 0x00000002FFFFFFFF, 0x0000000100000001, 0x0000000400000002),
};

const inv2: yU64x4 = 
yU64x4
{
	value: (0x8000000000000000, 0xFFFFFFFF80000000, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFF7FFFFFFF),
};

const a: yU64x4 = 
yU64x4
{
	value: (0xFFFFFFFFFFFFFFFC, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF),
};

const b: yU64x4 = 
yU64x4
{
	value: (0xDDBCBD414D940E93, 0xF39789F515AB8F92, 0x4D5A9E4BCF6509A7, 0x28E9FA9E9D9F5E34),
};

const Gx: yU64x4 = 
yU64x4
{
	value: (0x715A4589334C74C7, 0x8FE30BBFF2660BE1, 0x5F9904466A39C994, 0x32C4AE2C1F198119),
};

const Gy: yU64x4 = 
yU64x4
{
	value: (0x02DF32E52139F0A0, 0xD0A9877CC62A4740, 0x59BDCEE36B692153, 0xBC3736A2F4F6779C),
};

const n: yU64x4 =
yU64x4
{
	value: (0x53BBF40939D54123, 0x7203DF6B21C6052B, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF),
};

const zero: yU64x4 = 
yU64x4
{
	value: (0, 0, 0, 0),
};

const Px: yU64x4 =
yU64x4
{
	value: (0x6BB08FF356F35020, 0x72179FAD1833FC07, 0x50DD7D161E4BC5C6, 0x09F9DF311E5421A1),
};

const Py: yU64x4 =
yU64x4
{
	value: (0x6632F6072DA9AD13, 0x0AED05FBF35E084A, 0x2DC6EA718CC1AA60, 0xCCEA490CE26775A5),
};

const Fp: primeField = 
primeField
{
	prime,
	negPrime,
	rho,
	rho2,
	inv2,
};

// const Fn: yU64x4 = 
// {

// }

const G: Point = 
Point
{
	x: Gx,
	y: Gy,
};

const P: Point = 
Point
{
	x: Px,
	y: Py,
};

const Eccp: ECC_Fp = 
ECC_Fp
{
	a,
	b,
	Fp,
	G,
	l: 0,
	EFp: zero, // order of ECC_Fp
	n, // order of G
};

pub struct Sm2
{
	pub Ecc: ECC_Fp,
	pub P: Point, //public key
	pub D: yU64x4, //private key
}

pub fn sm2GetPubKey(d: yU64x4) -> Point
{
	Eccp.timesPoint(Eccp.G, d)
}

fn sm2GetZ(Q: Point) -> [u32;8]
{
	let mut len: usize = 2 + 14 + 6*32;


	let mut s: [u32;52] = [0;52];

	s[0] = 0x00D00102;
	s[1] = 0x03040506;
	s[2] = 0x0708090A;
	s[3] = 0x0B0C0D0E;

	s[4]  = (Eccp.a.value.3 >> 32) as u32;
	s[5]  = Eccp.a.value.3 as u32;
	s[6]  = (Eccp.a.value.2 >> 32) as u32;
	s[7]  = Eccp.a.value.2 as u32;
	s[8]  = (Eccp.a.value.1 >> 32) as u32;
	s[9] =	Eccp.a.value.1 as u32;
	s[10] = (Eccp.a.value.0 >> 32) as u32;
	s[11] =	Eccp.a.value.0 as u32;

	s[12] = (Eccp.b.value.3 >> 32) as u32;
	s[13] = Eccp.b.value.3 as u32;
	s[14] = (Eccp.b.value.2 >> 32) as u32;
	s[15] = Eccp.b.value.2 as u32;
	s[16] = (Eccp.b.value.1 >> 32) as u32;
	s[17] =	Eccp.b.value.1 as u32;
	s[18] = (Eccp.b.value.0 >> 32) as u32;
	s[19] =	Eccp.b.value.0 as u32;

	s[20] = (Eccp.G.x.value.3 >> 32) as u32;
	s[21] = Eccp.G.x.value.3 as u32;
	s[22] = (Eccp.G.x.value.2 >> 32) as u32;
	s[23] = Eccp.G.x.value.2 as u32;
	s[24] = (Eccp.G.x.value.1 >> 32) as u32;
	s[25] =	Eccp.G.x.value.1 as u32;
	s[26] = (Eccp.G.x.value.0 >> 32) as u32;
	s[27] =	Eccp.G.x.value.0 as u32;

	s[28] = (Eccp.G.y.value.3 >> 32) as u32;
	s[29] = Eccp.G.y.value.3 as u32;
	s[30] = (Eccp.G.y.value.2 >> 32) as u32;
	s[31] = Eccp.G.y.value.2 as u32;
	s[32] = (Eccp.G.y.value.1 >> 32) as u32;
	s[33] =	Eccp.G.y.value.1 as u32;
	s[34] = (Eccp.G.y.value.0 >> 32) as u32;
	s[35] =	Eccp.G.y.value.0 as u32;

	s[36] = (Q.x.value.3 >> 32) as u32;
	s[37] = Q.x.value.3 as u32;
	s[38] = (Q.x.value.2 >> 32) as u32;
	s[39] = Q.x.value.2 as u32;
	s[40] = (Q.x.value.1 >> 32) as u32;
	s[41] =	Q.x.value.1 as u32;
	s[42] = (Q.x.value.0 >> 32) as u32;
	s[43] =	Q.x.value.0 as u32;

	s[44] = (Q.y.value.3 >> 32) as u32;
	s[45] = Q.y.value.3 as u32;
	s[46] = (Q.y.value.2 >> 32) as u32;
	s[47] = Q.y.value.2 as u32;
	s[48] = (Q.y.value.1 >> 32) as u32;
	s[49] =	Q.y.value.1 as u32;
	s[50] = (Q.y.value.0 >> 32) as u32;
	s[51] =	Q.y.value.0 as u32;

	let Z = sm3Enc(&s[0..52],52*32);

	Z
}

pub fn sm2GenSign(Msg: &[u32], D: yU64x4, Q: Point, len: usize) -> (yU64x4, yU64x4)
{
	let Z = sm2GetZ(Q);

	let M = [Msg, &Z].concat();

	let E = sm3Enc(&M, (len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));
	
	let mut k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	if(primeField::largerEqualThan(k,Eccp.Fp.prime))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	}

	let mut P1 = Eccp.timesPoint(Eccp.G, k);

	let mut Fn = primeField::new(Eccp.n);
	e = Fn.transformToElement(e);
	let mut r = Fn.addElement(e,P1.x);
	let mut d = Fn.transformToElement(D);

	// Calculate s;
	let m1 = Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
	let m2 = Fn.subElement(k,Fn.mulElement(r,d));//k-r*d

	let mut s = Fn.mulElement(Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))),Fn.subElement(k,Fn.mulElement(r,d)));

	while(primeField::equalToZero(r)||primeField::equalToZero(Fn.addElement(r,k))||primeField::equalToZero(s))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(primeField::largerEqualThan(k,Eccp.Fp.prime))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}
		P1 = Eccp.timesPoint(Eccp.G, k);
		Fn = primeField::new(Eccp.n);
		r = Fn.addElement(e,P1.x);
	}

	(r, s)
}

pub fn sm2VerSign(Msg: &[u32], Q: Point, len: usize, r: yU64x4, s: yU64x4) -> bool
{
	if(primeField::largerEqualThan(r,Eccp.n)||primeField::equalToZero(r)) {return false;}
	if(primeField::largerEqualThan(s,Eccp.n)||primeField::equalToZero(s)) {return false;}
	let Za = sm2GetZ(Q);
	let M = [Msg, &Za].concat();

	let E = sm3Enc(&M,(len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

	let Fn = primeField::new(Eccp.n);

	if(primeField::equalToZero(r)||primeField::largerEqualThan(r,Eccp.n))
	{
		return false;
	}
	if(primeField::equalToZero(s)||primeField::largerEqualThan(s,Eccp.n))
	{
		return false;
	}

	let t = Fn.addElement(r,s);
	let P1 = Eccp.addPoint(Eccp.timesPoint(Eccp.G,s),Eccp.timesPoint(Q,t));

	let e1 = Fn.transformToElement(e);
	let x1 = Fn.transformToElement(P1.x);
	let R = Fn.addElement(e1,x1);


	if(primeField::equalTo(R,r))
	{
		true
	}
	else 
	{
		false
	}
}

pub fn sm2GenSignJ(Msg: &[u32], D: yU64x4, Q: Point, len: usize) -> (yU64x4, yU64x4)
{
	let Z = sm2GetZ(Q);

	let M = [Msg, &Z].concat();

	let E = sm3Enc(&M, (len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));
	
	let mut k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	if(primeField::largerEqualThan(k,Eccp.Fp.prime))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	}

	let GJacob = Eccp.affineToJacob(Eccp.G);
	let mut P1Jacob = Eccp.timesJacobPoint(GJacob, k);
	let mut P1 = Eccp.jacobToAffine(P1Jacob);
	let mut P2 = Eccp.timesPoint(Eccp.G,k);

	let mut Fn = primeField::new(Eccp.n);
	e = Fn.transformToElement(e);
	let mut r = Fn.addElement(e,P1.x);
	let mut d = Fn.transformToElement(D);

	// Calculate s;
	let m1 = Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
	let m2 = Fn.subElement(k,Fn.mulElement(r,d));//k-r*d

	let mut s = Fn.mulElement(Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))),Fn.subElement(k,Fn.mulElement(r,d)));

	while(primeField::equalToZero(r)||primeField::equalToZero(Fn.addElement(r,k))||primeField::equalToZero(s))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(primeField::largerEqualThan(k,Eccp.Fp.prime))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}
		P1Jacob = Eccp.timesJacobPoint(GJacob, k);
		P1 = Eccp.jacobToAffine(P1Jacob);
		Fn = primeField::new(Eccp.n);
		r = Fn.addElement(e,P1.x);
	}

	(r, s)
}

pub fn sm2VerSignJ(Msg: &[u32], Q: Point, len: usize, r: yU64x4, s: yU64x4) -> bool
{
	if(primeField::largerEqualThan(r,Eccp.n)||primeField::equalToZero(r)) {return false;}
	if(primeField::largerEqualThan(s,Eccp.n)||primeField::equalToZero(s)) {return false;}
	let Za = sm2GetZ(Q);
	let M = [Msg, &Za].concat();

	let E = sm3Enc(&M,(len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

	let Fn = primeField::new(Eccp.n);

	if(primeField::equalToZero(r)||primeField::largerEqualThan(r,Eccp.n))
	{
		return false;
	}
	if(primeField::equalToZero(s)||primeField::largerEqualThan(s,Eccp.n))
	{
		return false;
	}

	let PaJacob = Eccp.affineToJacob(Q);
	let GJacob = Eccp.affineToJacob(Eccp.G);

	let t = Fn.addElement(r,s);
	let P1Jacob = Eccp.addJacobPoint(Eccp.timesJacobPoint(GJacob,s), Eccp.timesJacobPoint(PaJacob,t));
	let P1 = Eccp.jacobToAffine(P1Jacob);

	let e1 = Fn.transformToElement(e);
	let x1 = Fn.transformToElement(P1.x);
	let R = Fn.addElement(e1,x1);


	if(primeField::equalTo(R,r))
	{
		true
	}
	else 
	{
		false
	}
}

impl Sm2
{
	pub fn new(a1: yU64x4, b1: yU64x4, p1: yU64x4, x0: yU64x4, y0: yU64x4, n1: yU64x4, D1: yU64x4) -> Sm2
	{
		let ecc = ECC_Fp::new(a1,b1,p1,x0,y0,n1);
		let p = ecc.timesPoint(ecc.G,D1);

		Sm2
		{
			Ecc: ecc,
			P: p,
			D: D1,
		}
	}

	pub fn getPubKey(&self) -> Point
	{
		self.P
	}

	fn getZ(&self, Pa: Point) -> [u32;8]
	{
		let mut len: usize = 2 + 14 + 6*32;


		let mut s: [u32;52] = [0;52];

		s[0] = 0x00D00102;
		s[1] = 0x03040506;
		s[2] = 0x0708090A;
		s[3] = 0x0B0C0D0E;

		s[4]  = (self.Ecc.a.value.3 >> 32) as u32;
		s[5]  = self.Ecc.a.value.3 as u32;
		s[6]  = (self.Ecc.a.value.2 >> 32) as u32;
		s[7]  = self.Ecc.a.value.2 as u32;
		s[8]  = (self.Ecc.a.value.1 >> 32) as u32;
		s[9] =	self.Ecc.a.value.1 as u32;
		s[10] = (self.Ecc.a.value.0 >> 32) as u32;
		s[11] =	self.Ecc.a.value.0 as u32;

		s[12] = (self.Ecc.b.value.3 >> 32) as u32;
		s[13] = self.Ecc.b.value.3 as u32;
		s[14] = (self.Ecc.b.value.2 >> 32) as u32;
		s[15] = self.Ecc.b.value.2 as u32;
		s[16] = (self.Ecc.b.value.1 >> 32) as u32;
		s[17] =	self.Ecc.b.value.1 as u32;
		s[18] = (self.Ecc.b.value.0 >> 32) as u32;
		s[19] =	self.Ecc.b.value.0 as u32;

		s[20] = (self.Ecc.G.x.value.3 >> 32) as u32;
		s[21] = self.Ecc.G.x.value.3 as u32;
		s[22] = (self.Ecc.G.x.value.2 >> 32) as u32;
		s[23] = self.Ecc.G.x.value.2 as u32;
		s[24] = (self.Ecc.G.x.value.1 >> 32) as u32;
		s[25] =	self.Ecc.G.x.value.1 as u32;
		s[26] = (self.Ecc.G.x.value.0 >> 32) as u32;
		s[27] =	self.Ecc.G.x.value.0 as u32;

		s[28] = (self.Ecc.G.y.value.3 >> 32) as u32;
		s[29] = self.Ecc.G.y.value.3 as u32;
		s[30] = (self.Ecc.G.y.value.2 >> 32) as u32;
		s[31] = self.Ecc.G.y.value.2 as u32;
		s[32] = (self.Ecc.G.y.value.1 >> 32) as u32;
		s[33] =	self.Ecc.G.y.value.1 as u32;
		s[34] = (self.Ecc.G.y.value.0 >> 32) as u32;
		s[35] =	self.Ecc.G.y.value.0 as u32;

		s[36] = (Pa.x.value.3 >> 32) as u32;
		s[37] = Pa.x.value.3 as u32;
		s[38] = (Pa.x.value.2 >> 32) as u32;
		s[39] = Pa.x.value.2 as u32;
		s[40] = (Pa.x.value.1 >> 32) as u32;
		s[41] =	Pa.x.value.1 as u32;
		s[42] = (Pa.x.value.0 >> 32) as u32;
		s[43] =	Pa.x.value.0 as u32;

		s[44] = (Pa.y.value.3 >> 32) as u32;
		s[45] = Pa.y.value.3 as u32;
		s[46] = (Pa.y.value.2 >> 32) as u32;
		s[47] = Pa.y.value.2 as u32;
		s[48] = (Pa.y.value.1 >> 32) as u32;
		s[49] =	Pa.y.value.1 as u32;
		s[50] = (Pa.y.value.0 >> 32) as u32;
		s[51] =	Pa.y.value.0 as u32;

		let Z = sm3Enc(&s[0..52],52*32);

		Z
	}

	// len: the Msg length (words)
	pub fn genSign(&self, Msg: &[u32], len: usize) -> (yU64x4, yU64x4)
	{
		let Z = self.getZ(self.P);

		let M = [Msg, &Z].concat();
	
		let E = sm3Enc(&M, (len+8)*32);
		let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));
		
		let mut k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(primeField::largerEqualThan(k,self.Ecc.Fp.prime))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}

		let mut P1 = self.Ecc.timesPoint(self.Ecc.G, k);

		let mut Fn = primeField::new(self.Ecc.n);
		e = Fn.transformToElement(e);
		let mut r = Fn.addElement(e,P1.x);
		let mut d = Fn.transformToElement(self.D);

		// Calculate s;
		let m1 = Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
		let m2 = Fn.subElement(k,Fn.mulElement(r,d));//k-r*d

		let mut s = Fn.mulElement(Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))),Fn.subElement(k,Fn.mulElement(r,d)));

		while(primeField::equalToZero(r)||primeField::equalToZero(Fn.addElement(r,k))||primeField::equalToZero(s))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
			if(primeField::largerEqualThan(k,self.Ecc.Fp.prime))
			{
				k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
			}
			P1 = self.Ecc.timesPoint(self.Ecc.G, k);
			Fn = primeField::new(self.Ecc.n);
			r = Fn.addElement(e,P1.x);
		}

		(r, s)
	}

	pub fn verSign(&self, Msg: &[u32], len: usize, r: yU64x4, s: yU64x4, Pa: Point) -> bool
	{
		if(primeField::largerEqualThan(r,self.Ecc.n)||primeField::equalToZero(r)) {return false;}
		if(primeField::largerEqualThan(s,self.Ecc.n)||primeField::equalToZero(s)) {return false;}
		let Za = self.getZ(Pa);
		let M = [Msg, &Za].concat();

		let E = sm3Enc(&M,(len+8)*32);
		let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

		let Fn = primeField::new(self.Ecc.n);

		if(primeField::equalToZero(r)||primeField::largerEqualThan(r,self.Ecc.n))
		{
			return false;
		}
		if(primeField::equalToZero(s)||primeField::largerEqualThan(s,self.Ecc.n))
		{
			return false;
		}

		let t = Fn.addElement(r,s);

		let Aj = self.Ecc.timesJacobPoint(self.Ecc.affineToJacob(self.Ecc.G),s);
		let Bj = self.Ecc.timesJacobPoint(self.Ecc.affineToJacob(Pa), t);
		let A = self.Ecc.jacobToAffine(Aj);
		let B = self.Ecc.jacobToAffine(Bj);

		let P1 = self.Ecc.addPoint(A, B);

		let e1 = Fn.transformToElement(e);
		let x1 = Fn.transformToElement(P1.x);
		let R = Fn.addElement(e1,x1);


		if(primeField::equalTo(R,r))
		{
			true
		}
		else 
		{
			false
		}
	}

	pub fn genSignJ(&self, Msg: &[u32], len: usize) -> (yU64x4, yU64x4)
	{
		let Z = self.getZ(self.P);

		let M = [Msg, &Z].concat();

		let E = sm3Enc(&M, (len+8)*32);
		let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));
		
		let mut k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(primeField::largerEqualThan(k,self.Ecc.Fp.prime))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}

		let GJacob = self.Ecc.affineToJacob(self.Ecc.G);
		let mut P1Jacob = self.Ecc.timesJacobPoint(GJacob, k);
		let mut P1 = self.Ecc.jacobToAffine(P1Jacob);

		let mut Fn = primeField::new(self.Ecc.n);
		e = Fn.transformToElement(e);
		let mut r = Fn.addElement(e,P1.x);
		let mut d = Fn.transformToElement(self.D);

		// Calculate s;
		let m1 = Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
		let m2 = Fn.subElement(k,Fn.mulElement(r,d));//k-r*d

		let mut s = Fn.mulElement(Fn.getMultiplicationInverseElement(Fn.addElement(d,yU64x4::new(1,0,0,0))),Fn.subElement(k,Fn.mulElement(r,d)));

		while(primeField::equalToZero(r)||primeField::equalToZero(Fn.addElement(r,k))||primeField::equalToZero(s))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
			if(primeField::largerEqualThan(k,self.Ecc.Fp.prime))
			{
				k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
			}
			P1Jacob = self.Ecc.timesJacobPoint(GJacob, k);
			P1 = self.Ecc.jacobToAffine(P1Jacob);
			Fn = primeField::new(self.Ecc.n);
			r = Fn.addElement(e,P1.x);
		}

		(r, s)
	}

	pub fn verSignJ(&self, Msg: &[u32], len: usize, r: yU64x4, s: yU64x4, Pa: Point) -> bool
	{
		if(primeField::largerEqualThan(r,self.Ecc.n)||primeField::equalToZero(r)) {return false;}
		if(primeField::largerEqualThan(s,self.Ecc.n)||primeField::equalToZero(s)) {return false;}
		let Za = self.getZ(Pa);
		let M = [Msg, &Za].concat();

		let E = sm3Enc(&M,(len+8)*32);
		let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

		let Fn = primeField::new(self.Ecc.n);

		if(primeField::equalToZero(r)||primeField::largerEqualThan(r,self.Ecc.n))
		{
			return false;
		}
		if(primeField::equalToZero(s)||primeField::largerEqualThan(s,self.Ecc.n))
		{
			return false;
		}

		let PaJacob = self.Ecc.affineToJacob(Pa);
		let GJacob = self.Ecc.affineToJacob(self.Ecc.G);

		let t = Fn.addElement(r,s);
		let P1Jacob = self.Ecc.addJacobPoint(self.Ecc.timesJacobPoint(GJacob,s),self.Ecc.timesJacobPoint(PaJacob,t));
		let P1 = self.Ecc.jacobToAffine(P1Jacob);

		let e1 = Fn.transformToElement(e);
		let x1 = Fn.transformToElement(P1.x);
		let R = Fn.addElement(e1,x1);


		if(primeField::equalTo(R,r))
		{
			true
		}
		else 
		{
			false
		}
	}

}

