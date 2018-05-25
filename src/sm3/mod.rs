use std::num::Wrapping;

static IV: [u32; 8] = [0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e];

#[inline]
fn SM3_T(j: u32) ->u32 
{
	if j<=15
	{
		0x79cc4519
	}
	else 
	{
		0x7a879d8a
	}
}

#[inline]
fn SM3_FF(X: u32, Y: u32, Z: u32, j:u32) -> u32
{
	if j<=15
	{
		X ^ Y ^ Z
	}
	else 
	{
		(X & Y) | (X & Z) | (Y & Z)
	}
}

#[inline]
fn SM3_GG(X: u32, Y: u32, Z: u32, j: u32) -> u32
{
	if j<=15
	{
		X ^ Y ^ Z
	}
	else 
	{
		(X & Y) | ((!X) & Z)
	}	
}

#[inline]
fn SM3_P_0(X: u32) -> u32
{
	X ^ X.rotate_left(9) ^ X.rotate_left(17)
}

#[inline]
fn SM3_P_1(X: u32) -> u32
{
	X ^ X.rotate_left(15) ^ X.rotate_left(23)
}


fn SM3_extend(B: [u32;16]) -> ([u32;68],[u32;64])
{
	let mut W: [u32;68] = [0;68];
	let mut W_p: [u32;64] = [0;64];
	for j in 0..16
	{
		W[j] = B[j];
	}
	for j in 16..68
	{
		W[j] = SM3_P_1(W[j-16] ^ W[j-9] ^ W[j-3].rotate_left(15)) ^ W[j-13].rotate_left(7) ^ W[j-6];
	}
	for j in 0..64
	{
		W_p[j] = W[j] ^ W[j+4];
	}

	(W,W_p)
}

fn SM3_CF(Vi: [u32;8], Bi: [u32;16]) -> [u32;8]
{
	let Ws = SM3_extend(Bi);
	let W = Ws.0;
	let W_p = Ws.1;

	let mut A = Vi[0];
	let mut B = Vi[1];
	let mut C = Vi[2];
	let mut D = Vi[3];
	let mut E = Vi[4];
	let mut F = Vi[5];
	let mut G = Vi[6];
	let mut H = Vi[7];

	let mut SS1 = 0;
	let mut SS2 = 0;
	let mut TT1 = 0;
	let mut TT2 = 0;

	for j in 0..64
	{
		SS1 = (Wrapping(A.rotate_left(12))+Wrapping(E)+Wrapping(SM3_T(j).rotate_left(j%32))).0.rotate_left(7);
		SS2 = SS1 ^ (A.rotate_left(12));
		TT1 = (Wrapping(SM3_FF(A,B,C,j))+Wrapping(D)+Wrapping(SS2)+Wrapping(W_p[j as usize])).0;
		TT2 = (Wrapping(SM3_GG(E,F,G,j))+Wrapping(H)+Wrapping(SS1)+Wrapping(W[j as usize])).0;
		D = C;
		C = B.rotate_left(9);
		B = A;
		A = TT1;
		H = G;
		G = F.rotate_left(19);
		F = E;
		E = SM3_P_0(TT2);
	}

	let mut Vs: [u32;8] = [0;8];
	Vs[0] = A ^ Vi[0];
	Vs[1] = B ^ Vi[1];
	Vs[2] = C ^ Vi[2];
	Vs[3] = D ^ Vi[3];
	Vs[4] = E ^ Vi[4];
	Vs[5] = F ^ Vi[5];
	Vs[6] = G ^ Vi[6];
	Vs[7] = H ^ Vi[7];

	Vs
}

macro_rules! copyArray 
{
	($v1:ident, $v2:ident,$($i:expr),* ) => ($(v1[i] = v2[i];)*)
}

pub struct Sm3Cryptor
{

}

impl Sm3Cryptor
{
	pub fn new() -> Sm3Cryptor
	{
		Sm3Cryptor
		{

		}
	}

	pub fn encrypt(&self, msg: &[u32], primLen: usize) -> [u32;8]
	{
		let mut msgLen = primLen;
		msgLen += 1; // Add "1" to the end of msg

		if msgLen%512>448 // too long
		{
			msgLen += (msgLen%512)+512;
		}
		else 
		{
			msgLen += msgLen%512;
		}

		let msgLen1: u32 = (primLen/0x0000_0001_0000_0000) as u32;
		let msgLen2: u32 = (primLen%0x0000_0001_0000_0000) as u32;

		// set V to IV
		let mut V: [u32;8] = [0x7380166f, 0x4914b2b9, 0x172442d7, 0xda8a0600, 0xa96f30bc, 0x163138aa, 0xe38dee4d, 0xb0fb0e4e];

		for i in 0..msgLen/512+1
		// msg blocks' index;
		// the operations are the same except the last block
		{
			//println!("i={}",i);
			let mut B: [u32;16] = [0;16];
			for j in 0..16 // words' index in a block
			{
				if (i*16+j)<msg.len() as usize
				{
					B[j as usize] = msg[(i*16+j) as usize];
				}
			}

			if primLen+1>512*i&&primLen+1<=512*(i+1) // add "1" somewhere in this block
			{
				let mut bias = primLen % 512;

				let mut biasOfWord = (bias / 32) as u32;
				let mut biasOfBit = (bias % 32) as u32;
				B[biasOfWord as usize] += 0x80000000u32.rotate_right(biasOfBit);
			}

			if i==(msgLen/512) // the last block should store the length of msg
			{
				B[14] = msgLen1;
				B[15] = msgLen2;
			}

			V = SM3_CF(V, B);

		}

		V
	}

}