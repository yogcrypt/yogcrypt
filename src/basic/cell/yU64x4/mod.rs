use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

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