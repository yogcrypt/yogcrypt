use ::basic::cell::yU64x4::*;
use ::basic::group::EccGroup::*;
use ::basic::field::primeField::*;
use ::basic::field::theField;
use rand::random;
use std::vec::Vec;
use ::sm3::*;


pub struct Sm2Cryptor
{
	pub Ecc: ECC_Fp,
	pub P: Point, //public key
	pub D: yU64x4, //private key
}

impl Sm2Cryptor
{
	pub fn new(a: yU64x4, b: yU64x4, p: yU64x4, x0: yU64x4, y0: yU64x4, n: yU64x4, D: yU64x4) -> Sm2Cryptor
	{
		let ecc = ECC_Fp::new(a,b,p,x0,y0,n);
		let p = ecc.timesPoint(ecc.G,D);

		Sm2Cryptor
		{
			Ecc: ecc,
			P: p,
			D,
		}
	}	

	pub fn getZ(&self, Pa: Point) -> [u32;8]
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
		let P1 = self.Ecc.addPoint(self.Ecc.timesPoint(self.Ecc.G,s),self.Ecc.timesPoint(Pa,t));

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

	pub fn generateSignatureJacob(&self, Msg: &[u32], len: usize) -> (yU64x4, yU64x4)
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
		let mut P2 = self.Ecc.timesPoint(self.Ecc.G, k);

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

	pub fn verifySignatureJacob(&self, Msg: &[u32], len: usize, r: yU64x4, s: yU64x4, Pa: Point) -> bool
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

