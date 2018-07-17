use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};
use std::ops::{Add, Sub, Neg};

use ::basic::cell::UniformAccessU64;
use ::basic::cell::yU64x8::*;

#[derive(Copy, Clone)]
pub struct yU64x4
{
	pub value: (u64, u64, u64, u64),
}


macro_rules! OVERFLOWING_ADD
{
	($x:expr, $y:expr, $result:ident, $overflowFlag:ident) => 
	(
		let car = if ($overflowFlag==true) {1} else {0};

		let r1 = u64::overflowing_add($x, $y);
		let r2 = u64::overflowing_add(r1.0, car);
		$result = r2.0;
		$overflowFlag = r1.1|r2.1;
	)
}

impl yU64x4
{
	pub fn new(x0: u64, x1: u64, x2: u64, x3: u64) -> Self	
	{
		Self
		{
			value: (x0, x1, x2, x3),
		}
	}
}

impl UniformAccessU64 for yU64x4
{
	fn get(&self, i: usize) -> u64
	{
		match i
		{
			0 => (self.value.0),
			1 => (self.value.1),
			2 => (self.value.2),
			3 => (self.value.3),
			_ => (0xFFFFFFFFFFFFFFFF),
		}
	}

	fn set(&mut self, i: usize, x: u64)
	{
		match i
		{
			0 => (self.value.0 = x),
			1 => (self.value.1 = x),
			2 => (self.value.2 = x),
			3 => (self.value.3 = x),
			_ => (),
		}
	}
}

impl Display for yU64x4
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:016X}{:016X}{:016X}{:016X}", self.value.3, self.value.2, self.value.1, self.value.0)
	}
}

impl Not for yU64x4
{
	type Output = Self;

	fn not(self) -> Self
	{
		Self
		{
			value: (!self.value.0, !self.value.1, !self.value.2, !self.value.3),
		}
	}
}

impl BitAnd for yU64x4
{
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 & rhs.value.0, self.value.1 & rhs.value.1, self.value.2 & rhs.value.2, self.value.3 & rhs.value.3),
		}
	}
}

impl BitOr for yU64x4
{
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 | rhs.value.0, self.value.1 | rhs.value.1, self.value.2 | rhs.value.2, self.value.3 | rhs.value.3),
		}
	}
}

impl BitXor for yU64x4
{
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 ^ rhs.value.0, self.value.1 ^ rhs.value.1, self.value.2 ^ rhs.value.2, self.value.3 ^ rhs.value.3),
		}
	}
}

impl BitAndAssign for yU64x4
{
	fn bitand_assign(&mut self, rhs: Self)
	{
		self.value.0 &= rhs.value.0;
		self.value.1 &= rhs.value.1;
		self.value.2 &= rhs.value.2;
		self.value.3 &= rhs.value.3;	
	}
}

impl BitOrAssign for yU64x4
{
	fn bitor_assign(&mut self, rhs: Self)
	{
		self.value.0 |= rhs.value.0;
		self.value.1 |= rhs.value.1;
		self.value.2 |= rhs.value.2;
		self.value.3 |= rhs.value.3;
	}
}

impl BitXorAssign for yU64x4
{
	fn bitxor_assign(&mut self, rhs: Self)
	{
		self.value.0 ^= rhs.value.0;
		self.value.1 ^= rhs.value.1;
		self.value.2 ^= rhs.value.2;
		self.value.3 ^= rhs.value.3;
	}
}

impl Neg for yU64x4
{
	type Output = Self;

	fn neg(self) -> yU64x4
	{
		let mut x = self;

		if x.value.0!=0
		{
			x.value.0 = u64::wrapping_neg(x.value.0);
			x.value.1 = !x.value.1;
			x.value.2 = !x.value.2;
			x.value.3 = !x.value.3;
		}
		else if x.value.1!=0
		{
			x.value.1 = u64::wrapping_neg(x.value.1);
			x.value.2 = !x.value.2;
			x.value.3 = !x.value.3;
		}
		else if x.value.2!=0
		{
			x.value.2 = u64::wrapping_neg(x.value.2);
			x.value.3 = !x.value.3;
		}
		else if x.value.3!=0
		{
			x.value.3 = u64::wrapping_neg(x.value.3);
		}

		x
	}
}

impl Add for yU64x4
{
	type Output = Self;

	fn add(self, rhs: yU64x4) -> yU64x4
	{
		let res0: u64;
		let res1: u64;
		let res2: u64;
		let res3: u64;
		let mut overflowFlag = false;

		OVERFLOWING_ADD!(self.value.0, rhs.value.0, res0, overflowFlag);
		OVERFLOWING_ADD!(self.value.1, rhs.value.1, res1, overflowFlag);
		OVERFLOWING_ADD!(self.value.2, rhs.value.2, res2, overflowFlag);
		OVERFLOWING_ADD!(self.value.3, rhs.value.3, res3, overflowFlag);
		
		yU64x4
		{
			value: (res0, res1, res2, res3),
		}
	}
}

impl Sub for yU64x4
{
	type Output = Self;

	fn sub(self, rhs: yU64x4) -> yU64x4
	{
		self + (-rhs)
	}
}

impl yU64x4
{
	pub fn letfRotateTo_yU64x8(self, sh: usize) -> yU64x8
	{
		let shn = sh / 64;
		let shx = sh % 64;

		let t = (64-shx) as u32;

		let mut r = yU64x8
		{
			value:(0, 
				if(t!=64){self.value.0>>t} else {0}, 
				if(t!=64){self.value.1>>t} else {0}, 
				if(t!=64){self.value.2>>t} else {0}, 
				if(t!=64){self.value.3>>t} else {0}, 0, 0, 0),
		};

		r.value.0 |= (self.value.0 << shx);
		r.value.1 |= (self.value.1 << shx);
		r.value.2 |= (self.value.2 << shx);
		r.value.3 |= (self.value.3 << shx);

		match shn
		{
			0 => (),
			1 => {
					r.value.5 = r.value.4;
					r.value.4 = r.value.3;
					r.value.3 = r.value.2;
					r.value.2 = r.value.1;
					r.value.1 = r.value.0;
					r.value.0 = 0;
			   	 },
			2 => {
					r.value.5 = r.value.3;
					r.value.4 = r.value.2;
					r.value.3 = r.value.1;
					r.value.2 = r.value.0;
					r.value.1 = 0;
					r.value.0 = 0;
				 },
			3 => {
					r.value.5 = r.value.2;
					r.value.4 = r.value.1;
					r.value.3 = r.value.0;
					r.value.2 = 0;
					r.value.1 = 0;
					r.value.0 = 0;
				 },
			4 => {
					r.value.5 = r.value.1;
					r.value.4 = r.value.0;
					r.value.3 = 0;
					r.value.2 = 0;
					r.value.1 = 0;
					r.value.0 = 0;
				 },
			_ => {
					panic!("cannot hold in yU64x8!");
				 },
		};

		r
	}
}

impl yU64x4
{
	pub fn leftShift1(&mut self)
	{
		self.value.3 <<= 1;
		self.value.3 |= (self.value.2 >> 63);
		self.value.2 <<= 1;
		self.value.2 |= (self.value.1 >> 63);
		self.value.1 <<= 1;
		self.value.1 |= (self.value.0 >> 63);
		self.value.0 <<= 1;
	}

	pub fn rightShift1(&mut self)
	{
		self.value.0 >>= 1;
		self.value.0 |= (self.value.1 << 63);
		self.value.1 >>= 1;
		self.value.1 |= (self.value.2 << 63);
		self.value.2 >>= 1;
		self.value.2 |= (self.value.3 << 63);
		self.value.3 >>= 1;
	}

	pub fn get(&self, i: usize) -> u64
	{
		let n = i/64;
		let x = i%64;
		match n 
		{
			0 => ((self.value.0>>x)%2),
			1 => ((self.value.1>>x)%2),
			2 => ((self.value.2>>x)%2),
			3 => ((self.value.3>>x)%2),
			_ => (panic!("unknown n")),
		}
	}
}

// Access
impl yU64x4
{
	// get the i-th lowest byte
	pub fn getByte(&self, mut i: usize) -> u8
	{
		i %= 32;
		let x = i%8;

		let q;
		match (i/8)
		{
			0 => (q=self.value.0),
			1 => (q=self.value.1),
			2 => (q=self.value.2),
			3 => (q=self.value.3),
			_ => (q=0),
		}

		(q>>(x*8)) as u8
	}

	pub fn toU8Slice(&self) -> [u8;32]
	{
		let arr = [
			(self.value.3 >> 56) as u8, (self.value.3 >> 48) as u8, (self.value.3 >> 40) as u8, (self.value.3 >> 32) as u8,
			(self.value.3 >> 24) as u8, (self.value.3 >> 16) as u8, (self.value.3 >> 8) as u8, self.value.3 as u8,
			(self.value.2 >> 56) as u8, (self.value.2 >> 48) as u8, (self.value.2 >> 40) as u8, (self.value.2 >> 32) as u8,
			(self.value.2 >> 24) as u8, (self.value.2 >> 16) as u8, (self.value.2 >> 8) as u8, self.value.2 as u8,
			(self.value.1 >> 56) as u8, (self.value.1 >> 48) as u8, (self.value.1 >> 40) as u8, (self.value.1 >> 32) as u8,
			(self.value.1 >> 24) as u8, (self.value.1 >> 16) as u8, (self.value.1 >> 8) as u8, self.value.1 as u8,
			(self.value.0 >> 56) as u8, (self.value.0 >> 48) as u8, (self.value.0 >> 40) as u8, (self.value.0 >> 32) as u8,
			(self.value.0 >> 24) as u8, (self.value.0 >> 16) as u8, (self.value.0 >> 8) as u8, self.value.0 as u8,
		];

		arr
	}
}