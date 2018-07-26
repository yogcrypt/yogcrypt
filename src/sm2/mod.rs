use ::basic::cell::yU64x4::*;
use ::basic::group::EccGroup::{G, Point, a, addJacobPoint, addPoint, affineToJacob, b, jacobToAffine, timesJacobPoint, timesPoint};
use ::basic::field::Fp::p;
use ::basic::field::Fn::*;
use rand::random;
use std::vec::Vec;
use ::sm3::*;

const Px: yU64x4 =yU64x4{value: [0x6BB08FF356F35020, 0x72179FAD1833FC07, 0x50DD7D161E4BC5C6, 0x09F9DF311E5421A1],};
const Py: yU64x4 =yU64x4{value: [0x6632F6072DA9AD13, 0x0AED05FBF35E084A, 0x2DC6EA718CC1AA60, 0xCCEA490CE26775A5],};

const P: Point = 
Point
{
	x: Px,
	y: Py,
};

pub fn sm2GetPubKey(d: yU64x4) -> Point
{
	timesPoint(G, d)
}

fn sm2GetZ(Q: Point) -> [u32;8]
{
	let mut len: usize = 2 + 14 + 6*32;


	let mut s: [u32;52] = [0;52];

	s[0] = 0x00D00102;
	s[1] = 0x03040506;
	s[2] = 0x0708090A;
	s[3] = 0x0B0C0D0E;

	s[4]  = (a.value[3] >> 32) as u32;
	s[5]  = a.value[3] as u32;
	s[6]  = (a.value[2] >> 32) as u32;
	s[7]  = a.value[2] as u32;
	s[8]  = (a.value[1] >> 32) as u32;
	s[9] =	a.value[1] as u32;
	s[10] = (a.value[0] >> 32) as u32;
	s[11] =	a.value[0] as u32;

	s[12] = (b.value[3] >> 32) as u32;
	s[13] = b.value[3] as u32;
	s[14] = (b.value[2] >> 32) as u32;
	s[15] = b.value[2] as u32;
	s[16] = (b.value[1] >> 32) as u32;
	s[17] =	b.value[1] as u32;
	s[18] = (b.value[0] >> 32) as u32;
	s[19] =	b.value[0] as u32;

	s[20] = (G.x.value[3] >> 32) as u32;
	s[21] = G.x.value[3] as u32;
	s[22] = (G.x.value[2] >> 32) as u32;
	s[23] = G.x.value[2] as u32;
	s[24] = (G.x.value[1] >> 32) as u32;
	s[25] =	G.x.value[1] as u32;
	s[26] = (G.x.value[0] >> 32) as u32;
	s[27] =	G.x.value[0] as u32;

	s[28] = (G.y.value[3] >> 32) as u32;
	s[29] = G.y.value[3] as u32;
	s[30] = (G.y.value[2] >> 32) as u32;
	s[31] = G.y.value[2] as u32;
	s[32] = (G.y.value[1] >> 32) as u32;
	s[33] =	G.y.value[1] as u32;
	s[34] = (G.y.value[0] >> 32) as u32;
	s[35] =	G.y.value[0] as u32;

	s[36] = (Q.x.value[3] >> 32) as u32;
	s[37] = Q.x.value[3] as u32;
	s[38] = (Q.x.value[2] >> 32) as u32;
	s[39] = Q.x.value[2] as u32;
	s[40] = (Q.x.value[1] >> 32) as u32;
	s[41] =	Q.x.value[1] as u32;
	s[42] = (Q.x.value[0] >> 32) as u32;
	s[43] =	Q.x.value[0] as u32;

	s[44] = (Q.y.value[3] >> 32) as u32;
	s[45] = Q.y.value[3] as u32;
	s[46] = (Q.y.value[2] >> 32) as u32;
	s[47] = Q.y.value[2] as u32;
	s[48] = (Q.y.value[1] >> 32) as u32;
	s[49] =	Q.y.value[1] as u32;
	s[50] = (Q.y.value[0] >> 32) as u32;
	s[51] =	Q.y.value[0] as u32;

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
	if(largerEqualThan(k, p))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	}

	let mut P1 = timesPoint(G, k);

	e = transFn(e);
	let mut r = addModN(e,P1.x);
	let mut d = transFn(D);

	// Calculate s;
	let m1 = getMulInvModN(addModN(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
	let m2 = subModN(k,mulModN(r,d));//k-r*d

	let mut s = mulModN(getMulInvModN(addModN(d,yU64x4::new(1,0,0,0))),subModN(k,mulModN(r,d)));

	while(equalToZero(r)||equalToZero(addModN(r,k))||equalToZero(s))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(largerEqualThan(k, p))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}
		P1 = timesPoint(G, k);
		r = addModN(e,P1.x);
	}

	(r, s)
}

pub fn sm2VerSign(Msg: &[u32], Q: Point, len: usize, r: yU64x4, s: yU64x4) -> bool
{
	if(largerEqualThan(r,n)||equalToZero(r)) {return false;}
	if(largerEqualThan(s,n)||equalToZero(s)) {return false;}
	let Za = sm2GetZ(Q);
	let M = [Msg, &Za].concat();

	let E = sm3Enc(&M,(len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

	if(equalToZero(r)||largerEqualThan(r,n))
	{
		return false;
	}
	if(equalToZero(s)||largerEqualThan(s,n))
	{
		return false;
	}

	let t = addModN(r,s);
	let P1 = addPoint(timesPoint(G,s),timesPoint(Q,t));

	let e1 = transFn(e);
	let x1 = transFn(P1.x);
	let R = addModN(e1,x1);


	if(equalTo(R,r))
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
	if(largerEqualThan(k, p))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
	}

	let GJacob = affineToJacob(G);
	let mut P1Jacob = timesJacobPoint(GJacob, k);
	let mut P1 = jacobToAffine(P1Jacob);
	let mut P2 = timesPoint(G,k);

	e = transFn(e);
	let mut r = addModN(e,P1.x);
	let mut d = transFn(D);

	// Calculate s;
	let m1 = getMulInvModN(addModN(d,yU64x4::new(1,0,0,0))); //(1+d)^-1
	let m2 = subModN(k,mulModN(r,d));//k-r*d

	let mut s = mulModN(getMulInvModN(addModN(d,yU64x4::new(1,0,0,0))),subModN(k,mulModN(r,d)));

	while(equalToZero(r)||equalToZero(addModN(r,k))||equalToZero(s))
	{
		k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		if(largerEqualThan(k, p))
		{
			k = yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>());
		}
		P1Jacob = timesJacobPoint(GJacob, k);
		P1 = jacobToAffine(P1Jacob);
		r = addModN(e,P1.x);
	}

	(r, s)
}

pub fn sm2VerSignJ(Msg: &[u32], Q: Point, len: usize, r: yU64x4, s: yU64x4) -> bool
{
	if(largerEqualThan(r,n)||equalToZero(r)) {return false;}
	if(largerEqualThan(s,n)||equalToZero(s)) {return false;}
	let Za = sm2GetZ(Q);
	let M = [Msg, &Za].concat();

	let E = sm3Enc(&M,(len+8)*32);
	let mut e = yU64x4::new(E[7] as u64|((E[6] as u64)<<32),E[5] as u64|((E[4] as u64)<<32) ,E[3] as u64|((E[2] as u64)<<32),E[1] as u64|((E[0] as u64)<<32));

	if(equalToZero(r)||largerEqualThan(r,n))
	{
		return false;
	}
	if(equalToZero(s)||largerEqualThan(s,n))
	{
		return false;
	}

	let PaJacob = affineToJacob(Q);
	let GJacob = affineToJacob(G);

	let t = addModN(r,s);
	let P1Jacob = addJacobPoint(timesJacobPoint(GJacob,s), timesJacobPoint(PaJacob,t));
	let P1 = jacobToAffine(P1Jacob);

	let e1 = transFn(e);
	let x1 = transFn(P1.x);
	let R = addModN(e1,x1);


	if(equalTo(R,r))
	{
		true
	}
	else 
	{
		false
	}
}
