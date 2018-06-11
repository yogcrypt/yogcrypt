
use std::clone::Clone;
use std::convert::From;

use std::fmt;
use std::fmt::Display;

use std::ops::{BitAnd, BitOr, BitXor, Not};  // Done: BitAdd, BitOr, BitXor, Not
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign}; // Done: BitAndAssign, BitOrAssign, BitXorAssign
use std::ops::{Shl, Shr};
use std::ops::{ShlAssign, ShrAssign};
use std::ops::{Add, Sub, Mul, Div, Rem, Neg}; //Done: 
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};

#[derive(Copy, Clone)]
struct yU256
{
	pub value: (u64, u64, u64, u64),
}

// Constructors
impl yU256
{
	pub fn new(x0: u64, x1: u64, x2: u64, x3: u64) -> yU256
	{
		yU256
		{
			value: (x0, x1, x2, x3),
		}
	}
}

impl Display for yU256
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{:016X} {:016X} {:016X} {:016X}", self.value.3, self.value.2, self.value.1, self.value.0)
	}
}

impl yU256
{
	fn FromU8(x: u8) -> yU256
	{
		yU256
		{
			value: (x as u64, 0u64, 0u64, 0u64),
		}
	}
}

impl From<u8> for yU256
{
	fn from(x: u8) -> Self
	{
		yU256::FromU8(x)
	}
}

impl Not for yU256
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

impl BitAnd for yU256
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

impl BitOr for yU256
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

impl BitXor for yU256
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

impl BitAndAssign for yU256
{
	fn bitand_assign(&mut self, rhs: Self)
	{
		self.value.0 &= rhs.value.0;
		self.value.1 &= rhs.value.1;
		self.value.2 &= rhs.value.2;
		self.value.3 &= rhs.value.3;	
	}
}

impl BitOrAssign for yU256
{
	fn bitor_assign(&mut self, rhs: Self)
	{
		self.value.0 |= rhs.value.0;
		self.value.1 |= rhs.value.1;
		self.value.2 |= rhs.value.2;
		self.value.3 |= rhs.value.3;
	}
}

impl BitXorAssign for yU256
{
	fn bitxor_assign(&mut self, rhs: Self)
	{
		self.value.0 ^= rhs.value.0;
		self.value.1 ^= rhs.value.1;
		self.value.2 ^= rhs.value.2;
		self.value.3 ^= rhs.value.3;
	}
}

macro_rules! OVERFLOWING_ADD
{
	($x:expr, $y:expr, $result:ident, $overflowFlag:ident, $index:expr) => 
	(
		let mut car = if $overflowFlag==true 
			{
				1
			}
			else 
			{
				0
			};

		let r1 = u64::overflowing_add($x, $y);
		let r2 = u64::overflowing_add(r1.0, car);
		$result[$index] = r2.0;
		$overflowFlag = r1.1|r2.1;
	)
}

impl Add for yU256
{
	type Output = Self;


	fn add(self, rhs: Self) -> Self
	{
		let mut result = [0u64;4];
		let mut overflowFlag = false;

		OVERFLOWING_ADD!(self.value.0, rhs.value.0, result, overflowFlag, 0);
		OVERFLOWING_ADD!(self.value.1, rhs.value.1, result, overflowFlag, 1);
		OVERFLOWING_ADD!(self.value.2, rhs.value.2, result, overflowFlag, 2);
		OVERFLOWING_ADD!(self.value.3, rhs.value.3, result, overflowFlag, 3);

		Self
		{
			value: (result[0], result[1], result[2], result[3]),
		}
	}
}

impl Neg for yU256
{
	type Output = Self;

	fn neg(self) -> Self
	{
		let mut v = [self.value.0, self.value.1, self.value.2, self.value.3];
		if v[0]!=0
		{
			v[0] = u64::wrapping_neg(v[0]);
			v[1] = !v[1];
			v[2] = !v[2];
			v[3] = !v[3];
		}
		else if v[1]!=0
		{
			v[1] = u64::wrapping_neg(v[1]);
			v[2] = !v[2];
			v[3] = !v[3];
		}
		else if v[2]!=0
		{
			v[2] = u64::wrapping_neg(v[2]);
			v[3] = !v[3];
		}
		else if v[3]!=0
		{
			v[3] = u64::wrapping_neg(v[3]);
		}


		Self
		{
			value: (v[0], v[1], v[2], v[3])
		}
	}
}

impl Sub for yU256
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self
	{
		self+(-rhs)
	}
}

impl AddAssign for yU256
{
	fn add_assign(&mut self, rhs: Self)
	{
		*self = *self + rhs;
	}
}


impl SubAssign for yU256
{
	fn sub_assign(&mut self, rhs: Self)
	{
		*self = *self - rhs;
	}
}
 
macro_rules! U64_MUL {
	($x:expr, $y:expr) => 
		{{

			let p:u128 = ($x as u128) * ($y as u128);

			(p as u64, (p>>64) as u64)

		}}
}

macro_rules! U64_MUL_CAR {
	($x:expr, $y:expr, $c:expr) => 
	{{
		let (res0, car0) = U64_MUL!($x, $y);

		let (res1, overflowFlag) = u64::overflowing_add(res0, $c);

		if overflowFlag==true
		{
			(res1, car0+1) 
		}
		else 
		{
			(res1, car0)
		}

	}}
}

macro_rules! yU256_MUL_U64 {
	($x:ident, $z:expr) => 
		{{
			
			let (res0, car0) = U64_MUL!($x.value.0, $z);
			let (res1, car1) = U64_MUL_CAR!($x.value.1, $z, car0);
			let (res2, car2) = U64_MUL_CAR!($x.value.2, $z, car1);
			let (res3, car3) = U64_MUL_CAR!($x.value.3, $z, car2);

			yU256
			{
				value:(res0, res1, res2, res3),
			}

		}}
}

// shift by (64b)s
macro_rules! yU256_LSH_64 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.2;
		$x.value.2 = $x.value.1;
		$x.value.1 = $x.value.0;
		$x.value.0 = 0u64;
	)
}

macro_rules! yU256_LSH_128 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.1;
		$x.value.2 = $x.value.0;
		$x.value.1 = 0u64;
		$x.value.0 = 0u64;
	)
}

macro_rules! yU256_LSH_192 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.0;
		$x.value.2 = 0u64;
		$x.value.1 = 0u64;
		$x.value.0 = 0u64;
	)
}

macro_rules! yU256_RSH_64 {
	($x:ident) => 
	(
		$x.value.3 = 0u64;
		$x.value.2 = $x.value.3;
		$x.value.1 = $x.value.2;
		$x.value.0 = $x.value.1;
	)
}

macro_rules! yU256_RSH_128 {
	($x:ident) => 
	(
		$x.value.3 = 0u64;
		$x.value.2 = 0u64;
		$x.value.1 = $x.value.3;
		$x.value.0 = $x.value.2;
	)
}

macro_rules! yU256_RSH_192 {
	($x:ident) => 
	(
		$x.value.3 = 0u64;
		$x.value.2 = 0u64;
		$x.value.1 = 0u64;
		$x.value.0 = $x.value.3;
	)
}

// rotate shift by (64b)s
macro_rules! yU256_LSHR_64 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.2;
		$x.value.2 = $x.value.1;
		$x.value.1 = $x.value.0;
		$x.value.0 = $x.value.3;
	)
}

macro_rules! yU256_LSHR_128 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.1;
		$x.value.2 = $x.value.0;
		$x.value.1 = $x.value.3;
		$x.value.0 = $x.value.2;
	)
}

macro_rules! yU256_LSHR_192 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.0;
		$x.value.2 = $x.value.3;
		$x.value.1 = $x.value.2;
		$x.value.0 = $x.value.1;
	)
}

macro_rules! yU256_RSHR_64 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.2;
		$x.value.2 = $x.value.1;
		$x.value.1 = $x.value.0;
		$x.value.0 = $x.value.3;
	)
}

macro_rules! yU256_RSHR_128 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.1;
		$x.value.2 = $x.value.0;
		$x.value.1 = $x.value.3;
		$x.value.0 = $x.value.2;
	)
}

macro_rules! yU256_RSHR_192 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.2;
		$x.value.2 = $x.value.1;
		$x.value.1 = $x.value.0;
		$x.value.0 = $x.value.3;
	)
}

impl Mul for yU256
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self
	{
		let mut res0 = yU256_MUL_U64!(self, rhs.value.0);
		let mut res1 = yU256_MUL_U64!(self, rhs.value.1);
		let mut res2 = yU256_MUL_U64!(self, rhs.value.2);
		let mut res3 = yU256_MUL_U64!(self, rhs.value.3);

		yU256_LSH_64!(res1);
		yU256_LSH_128!(res2);
		yU256_LSH_192!(res3);

		res0 += res1;
		res0 += res2;
		res0 += res3;

		res0
	}
}

impl MulAssign for yU256
{
	fn mul_assign(&mut self, rhs: Self)
	{
		*self = *self * rhs;
	}
}

macro_rules! yU256_LSHR_64N {
	($x:ident, $n:expr) => 
	(
		match $n
		{
			1 => (yU256_LSHR_64($x)),
			2 => (yU256_LSHR_128($x)),
			3 => (yU256_LSHR_192($x)),
			_ => (),
		}
	)
}

impl Shl<usize> for yU256
{
	type Output = Self;

	fn shl(self, rhs: usize) -> yU256
 	{
 		let mut res = self;

		let s = if rhs>=256
		{
			rhs % 256
		}
		else 
		{
			rhs
		};

		let m = s % 64;

		res
	}
}

impl Div for yU256
{
	type Output = Self;

	fn div(self, rhs: Self) -> Self
	{
		self
	}
}

impl Rem for yU256
{
	type Output = Self;

	fn rem(self, rhs: Self) -> Self
	{
		self
	}
}

impl PartialEq for yU256
{
	fn eq(&self, other: &Self) -> bool
	{
		(self.value.0 == other.value.0) && (self.value.1 == other.value.1) && (self.value.2 == other.value.2) && (self.value.3 == other.value.3)
	}
}

/*impl PartialOrd for yU256
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		match self.value.3.cmp(&other.value.3)
		{
			Ordering::Equal => 
				(
					match self.value.2.cmp(&other.value.2)
					{
						Ordering::Equal => 
						(
							Some(self.value.2.cmp(&other.value.2))
						),
						_ => (Some(Ordering::Equal)),
					}
				),
			_ => (Some(Ordering::Equal)),
		}
	}
}*/