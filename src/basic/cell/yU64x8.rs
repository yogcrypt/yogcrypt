use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use ::basic::cell::UniformAccessU64;

#[derive(Copy, Clone)]
pub struct yU64x8
{
	pub value: (u64, u64, u64, u64, u64, u64, u64, u64),
}

impl yU64x8
{
	pub fn new(x0: u64, x1: u64, x2: u64, x3: u64, x4: u64, x5: u64, x6: u64, x7: u64) -> Self	
	{
		Self
		{
			value: (x0, x1, x2, x3, x4, x5, x6, x7),
		}
	}
}

impl UniformAccessU64 for yU64x8
{
	fn get(&self, i: usize) -> u64
	{
		match i
		{
			0 => (self.value.0),
			1 => (self.value.1),
			2 => (self.value.2),
			3 => (self.value.3),
			4 => (self.value.4),
			5 => (self.value.5),
			6 => (self.value.6),
			7 => (self.value.7),
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
			4 => (self.value.4 = x),
			5 => (self.value.5 = x),
			6 => (self.value.6 = x),
			7 => (self.value.7 = x),
			_ => (),
		}
	}
}

impl Display for yU64x8
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:016X} {:016X} {:016X} {:016X} {:016X} {:016X} {:016X} {:016X}", self.value.7, self.value.6, self.value.5, self.value.4, self.value.3, self.value.2, self.value.1, self.value.0)
	}
}

impl Not for yU64x8
{
	type Output = Self;

	fn not(self) -> Self
	{
		Self
		{
			value: (!self.value.0, !self.value.1, !self.value.2, !self.value.3, !self.value.4, !self.value.5, !self.value.6, !self.value.7),
		}
	}
}

impl BitAnd for yU64x8
{
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 & rhs.value.0, self.value.1 & rhs.value.1, self.value.2 & rhs.value.2, self.value.3 & rhs.value.3, self.value.4 & rhs.value.4, self.value.5 & rhs.value.5, self.value.6 & rhs.value.6, self.value.7 & rhs.value.7),
		}
	}
}

impl BitOr for yU64x8
{
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 | rhs.value.0, self.value.1 | rhs.value.1, self.value.2 | rhs.value.2, self.value.3 | rhs.value.3, self.value.4 | rhs.value.4, self.value.5 | rhs.value.5, self.value.6 | rhs.value.6, self.value.7 | rhs.value.7),
		}
	}
}

impl BitXor for yU64x8
{
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self
	{
		Self
		{
			value: (self.value.0 ^ rhs.value.0, self.value.1 ^ rhs.value.1, self.value.2 ^ rhs.value.2, self.value.3 ^ rhs.value.3, self.value.4 ^ rhs.value.4, self.value.5 ^ rhs.value.5, self.value.6 ^ rhs.value.6, self.value.7 ^ rhs.value.7),
		}
	}
}

impl BitAndAssign for yU64x8
{
	fn bitand_assign(&mut self, rhs: Self)
	{
		self.value.0 &= rhs.value.0;
		self.value.1 &= rhs.value.1;
		self.value.2 &= rhs.value.2;
		self.value.3 &= rhs.value.3;			
		self.value.4 &= rhs.value.4;
		self.value.5 &= rhs.value.5;
		self.value.6 &= rhs.value.6;
		self.value.7 &= rhs.value.7;	
	}
}

impl BitOrAssign for yU64x8
{
	fn bitor_assign(&mut self, rhs: Self)
	{
		self.value.0 |= rhs.value.0;
		self.value.1 |= rhs.value.1;
		self.value.2 |= rhs.value.2;
		self.value.3 |= rhs.value.3;
		self.value.4 |= rhs.value.4;
		self.value.5 |= rhs.value.5;
		self.value.6 |= rhs.value.6;
		self.value.7 |= rhs.value.7;
	}
}

impl BitXorAssign for yU64x8
{
	fn bitxor_assign(&mut self, rhs: Self)
	{
		self.value.0 ^= rhs.value.0;
		self.value.1 ^= rhs.value.1;
		self.value.2 ^= rhs.value.2;
		self.value.3 ^= rhs.value.3;
		self.value.4 ^= rhs.value.4;
		self.value.5 ^= rhs.value.5;
		self.value.6 ^= rhs.value.6;
		self.value.7 ^= rhs.value.7;
	}
}