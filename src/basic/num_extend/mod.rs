

use std::simd::u64x4;

use std::clone::Clone;
use std::convert::From;

use std::ops::{BitAnd, BitOr, BitXor, Not};  // Done: BitAdd, BitOr, BitXor, Not
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign}; // Done: BitAndAssign, BitOrAssign, BitXorAssign
use std::ops::{Shl, Shr};
use std::ops::{ShlAssign, ShrAssign};
use std::ops::{Add, Sub, Mul, Div, Rem, Neg}; //Done: 
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

// just for test 
macro_rules! printyo {
	($x:ident) => 
	{{
		let mut v = [0u64;4];
		$x.value.store_unaligned(&mut v);
		println!("{:016X} {:016X} {:016X} {:016X}", v[3], v[2], v[1], v[0]);
	}}
}

macro_rules! printu64x4 {
	($x:ident) => (println!("{:016X} {:016X} {:016X} {:016X}", $x.extract(3), $x.extract(2), $x.extract(1), $x.extract(0));)
}

pub struct yoU256
{
	pub value: u64x4,
}

impl yoU256
{
	pub fn new(x0: u64, x1: u64, x2: u64, x3: u64) -> Self
	{
		Self
		{
			value: u64x4::new(x0, x1, x2, x3),
		}
	}

	fn make (x: u64x4) -> Self
	{
		Self
		{
			value: x,
		}
	}
}

// Performance Optimize: use memcpy??
impl Clone for yoU256
{
	fn clone(&self) -> Self 
	{
		Self
		{
			value: self.value,
		}
	}
}

impl Not for yoU256
{
	type Output = Self;

	fn not(self) -> Self
	{
		Self
		{
			value: !self.value,
		}
	}
}

impl BitAnd<yoU256> for yoU256
{
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value & rhs.value,
		}
	}
}

impl BitOr<yoU256> for yoU256
{
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value | rhs.value,
		}
	}
}

impl BitXor<yoU256> for yoU256
{
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value ^ rhs.value,
		}
	}
}

impl BitAndAssign<yoU256> for yoU256
{
	fn bitand_assign(&mut self, rhs: Self)
	{
		self.value &= rhs.value;
	}
}

impl BitOrAssign<yoU256> for yoU256
{
	fn bitor_assign(&mut self, rhs: Self)
	{
		self.value |= rhs.value;
	}
}

impl BitXorAssign<yoU256> for yoU256
{
	fn bitxor_assign(&mut self, rhs: Self)
	{
		self.value ^= rhs.value;
	}
}

macro_rules! YOU256_SHRN_ROTATE {
	($x:ident, $n:expr) => 
	(
		unsafe
		{
			let mut p: [u64;4] = [0;4]; // warning: do not change this to ensure no memory leakage 
			$x.value.store_unaligned_unchecked(&mut p);  //unsafe

			u64x4::new(p[$n%4], p[(1+$n)%4], p[(2+$n)%4], p[(3+$n)%4])
		}
	)
}

impl Shr<usize> for yoU256
{
	type Output = Self;

	fn shr(self, rhs: usize) -> Self
	{
		let shrn = rhs / 64;
		let mut x: u64x4 = YOU256_SHRN_ROTATE!(self, shrn);

		printu64x4!(x);

		let shrx = rhs % 64; 

		let mut y: u64x4 = x << (64-shrx);
		y = u64x4::new(y.extract(1), y.extract(2), y.extract(3), y.extract(0));

		yoU256::make((x>>shrx) | y)
	}
}





/*
#[derive(Copy, Clone)]
pub struct yU256
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

// Implement from traits 
// This is just a test
// Whether this will be implemented is undecided
// If this will be implemented, use macros to reuse the code
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


// If there's a $op in the macros system, these impls can be re-wright in macros_rules!
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
		//println!("==== index {} ====", $index);
		let mut car = if $overflowFlag==true 
			{
				1
			}
			else 
			{
				0
			};

		let r1 = u64::overflowing_add($x, $y);
		//println!("x = {:016X}   y = {:016X}", $x, $y);
		//println!("r1 = {:016X} {}", r1.0, r1.1);
		let r2 = u64::overflowing_add(r1.0, car);
		//println!("r2 = {:016X} {}", r2.0, r1.1);
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

/*macro_rules! U64_SPLIT {
	($x:expr) => 
		{{
			let x0 = $x & 0x00000000FFFFFFFF;
			let x1 = ($x & 0xFFFFFFFF00000000) >> 32;
			(x0, x1)
		}}
}*/

// 
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

macro_rules! YU256_MUL_U64 {
	($x:ident, $v:ident, $z:expr) => 
		{{
			
			let (res0, car0) = U64_MUL!($x.$v.0, $z);
			let (res1, car1) = U64_MUL_CAR!($x.$v.1, $z, car0);
			let (res2, car2) = U64_MUL_CAR!($x.$v.2, $z, car1);
			let (res3, car3) = U64_MUL_CAR!($x.$v.3, $z, car2);

			yU256
			{
				value:(res0, res1, res2, res3),
			}

		}}
}

macro_rules! YU256_LSH_64 {
	($x:ident,$v:ident) => 
	(
		$x.$v.3 = $x.$v.2;
		$x.$v.2 = $x.$v.1;
		$x.$v.1 = $x.$v.0;
		$x.$v.0 = 0u64;
	)
}

macro_rules! YU256_LSH_128 {
	($x:ident,$v:ident) => 
	(
		$x.$v.3 = $x.$v.1;
		$x.$v.2 = $x.$v.0;
		$x.$v.1 = 0u64;
		$x.$v.0 = 0u64;
	)
}

macro_rules! YU256_LSH_192 {
	($x:ident,$v:ident) => 
	(
		$x.$v.3 = $x.$v.0;
		$x.$v.2 = 0u64;
		$x.$v.1 = 0u64;
		$x.$v.0 = 0u64;
	)
}


// Not finished yet
impl Mul for yU256
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self
	{
		let mut res0 = YU256_MUL_U64!(self, value, rhs.value.0);
		let mut res1 = YU256_MUL_U64!(self, value, rhs.value.1);
		let mut res2 = YU256_MUL_U64!(self, value, rhs.value.2);
		let mut res3 = YU256_MUL_U64!(self, value, rhs.value.3);

		YU256_LSH_64!(res1, value);
		YU256_LSH_128!(res2, value);
		YU256_LSH_192!(res3, value);

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

// The wrong way of using tuple
macro_rules! YU256_LSHR_64N {
	($x:ident,$v:ident, $n:expr) => 
	(
		match $n
		{
			
			1 =>
				{
					$x.$v.3 = $x.$v.2;
					$x.$v.2 = $x.$v.1;
					$x.$v.1 = $x.$v.0;
					$x.$v.0 = $x.$v.3;
				}
			2 =>
				{
					$x.$v.3 = $x.$v.1;
					$x.$v.2 = $x.$v.0;
					$x.$v.1 = $x.$v.3;
					$x.$v.0 = $x.$v.2;
				}
			3 =>
				{
					$x.$v.3 = $x.$v.0;
					$x.$v.2 = $x.$v.3;
					$x.$v.1 = $x.$v.2;
					$x.$v.0 = $x.$v.1;
				}
			_ => {}	
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
}*/

