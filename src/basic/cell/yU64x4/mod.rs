use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use ::basic::cell::UniformAccessU64;
use ::basic::cell::yU64x8::*;

#[derive(Copy, Clone)]
pub struct yU64x4
{
	pub value: (u64, u64, u64, u64),
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
		write!(f, "{:016X} {:016X} {:016X} {:016X}", self.value.3, self.value.2, self.value.1, self.value.0)
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

impl yU64x4
{
	fn letfRotateTo_yU64x8(self, sh: usize) -> yU64x8
	{
		let shn = sh / 64;
		let shx = sh % 64;

		let mut r = yU64x8
		{
			value:(0, (self.value.0>>(64-shx)), (self.value.1>>(64-shx)), (self.value.2>>(64-shx)), (self.value.3>>(64-shx)), 0, 0, 0),
		};

		r.value.0 &= (self.value.0 << shx);
		r.value.1 &= (self.value.1 << shx);
		r.value.2 &= (self.value.2 << shx);
		r.value.3 &= (self.value.3 << shx);

		match shn
		{
			0 => (),
			1 => {
					r.value.5 = r.value.4;
					r.value.4 = r.value.3;
					r.value.3 = r.value.2;
					r.value.2 = r.value.1;
					r.value.1 = r.value.0;
			   	 },
			2 => {
					r.value.5 = r.value.3;
					r.value.4 = r.value.2;
					r.value.3 = r.value.1;
					r.value.2 = r.value.0;
				 },
			3 => {
					r.value.5 = r.value.2;
					r.value.4 = r.value.1;
					r.value.3 = r.value.0;
				 },
			4 => {
					r.value.5 = r.value.1;
					r.value.4 = r.value.0;
				 },
			_ => {
					panic!("cannot hold in yU64x8!");
				 },
		};

		r
	}
}