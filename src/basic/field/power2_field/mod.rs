extern crate rand;
use self::rand::random;

use std::option;
use ::basic::field::*;
use ::basic::cell::yU64x4::*;


pub struct power2_field
{
	m: u8,    // Field of order 2^m
	B: yU64x4, // the Base of the field
	moduler: yU64x4,
}

impl power2_field
{
	// unfinished
	pub fn check(m: u8, B: yU64x4) -> bool
	{
		true
	}

	pub fn new(m: u8, B: yU64x4) -> Option<power2_field>
	{
		if power2_field::check(m, B)
		{
			let mut y = yU64x4::new(0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF, 0xFFFF_FFFF_FFFF_FFFF);

			if m>=64 {y.value.0 = 0x0;}
			if m>=128 {y.value.1 = 0x0;}
			if m>=192 {y.value.2 = 0x0;}

			let z: u64 = 0xFFFF_FFFF_FFFF_FFFF % (1<<(m%64));

			match m/64
			{
				0 => (y.value.0 = z),
				1 => (y.value.1 = z),
				2 => (y.value.2 = z),
				3 => (y.value.3 = z),
				_ => (panic!("Unrecognized bias found when construct power2_field.")),
			}

			let newField = power2_field
			{
				m, 
				B,
				moduler: y,
			};
			Option::Some(newField)
		}
		else 
		{
			Option::None	
		}
	}
}

macro_rules! OVERFLOWING_ADD
{
	($x:expr, $y:expr, $result:ident, $overflowFlag:ident) => 
	{{
		let car = if $overflowFlag==true 
			{
				1
			}
			else 
			{
				0
			};

		let r1 = u64::overflowing_add($x, $y);
		let r2 = u64::overflowing_add(r1.0, car);
		$result = r2.0;
		$overflowFlag = r1.1|r2.1;
	}}
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

macro_rules! YU64x4_MUL_U64 {
	($x:ident, $z:expr) => 
		{{
			
			let (res0, car0) = U64_MUL!($x.value.0, $z);
			let (res1, car1) = U64_MUL_CAR!($x.value.1, $z, car0);
			let (res2, car2) = U64_MUL_CAR!($x.value.2, $z, car1);
			let (res3, car3) = U64_MUL_CAR!($x.value.3, $z, car2);

			yU64x4
			{
				value:(res0, res1, res2, res3),
			}

		}}
}

macro_rules! YU64x4_LSH_64 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.2;
		$x.value.2 = $x.value.1;
		$x.value.1 = $x.value.0;
		$x.value.0 = 0u64;
	)
}

macro_rules! YU64x4_LSH_128 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.1;
		$x.value.2 = $x.value.0;
		$x.value.1 = 0u64;
		$x.value.0 = 0u64;
	)
}

macro_rules! YU64x4_LSH_192 {
	($x:ident) => 
	(
		$x.value.3 = $x.value.0;
		$x.value.2 = 0u64;
		$x.value.1 = 0u64;
		$x.value.0 = 0u64;
	)
}

impl Field for power2_field
{
	fn getNewElement(&self, x: yU64x4) -> yU64x4
	{
		x & self.moduler
	}

	fn getNewRandomElement(&self) -> yU64x4
	{
		yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>())  & self.moduler
	}

	fn getAdditionInverseElement(&self, mut p: yU64x4) -> yU64x4
	{
		if p.value.0!=0
		{
			p.value.0 = u64::wrapping_neg(p.value.0);
			p.value.1 = !p.value.1;
			p.value.2 = !p.value.2;
			p.value.3 = !p.value.3;
		}
		else if p.value.1!=0
		{
			p.value.1 = u64::wrapping_neg(p.value.1);
			p.value.2 = !p.value.2;
			p.value.3 = !p.value.3;
		}
		else if p.value.2!=0
		{
			p.value.2 = u64::wrapping_neg(p.value.2);
			p.value.3 = !p.value.3;
		}
		else if p.value.3!=0
		{
			p.value.3 = u64::wrapping_neg(p.value.3);
		}

		p  & self.moduler
	}

	// unfinished
	fn getMultiplicationInverseElement(&self, mut p: yU64x4) -> Option<yU64x4>
	{
		p = p & self.moduler;

		if(p.value.0==0&&p.value.1==0&&p.value.2==0&&p.value.3==0)
		{
			Option::None
		}
		else 
		{	
			Option::Some(yU64x4::new(0,0,0,0))
		}
	}

	fn addElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		let mut x0:u64;
		let mut x1:u64;
		let mut x2:u64;
		let mut x3:u64;
		let mut overflowFlag = false;

		OVERFLOWING_ADD!(p.value.0, q.value.0, x0, overflowFlag);
		OVERFLOWING_ADD!(p.value.1, q.value.1, x1, overflowFlag);
		OVERFLOWING_ADD!(p.value.2, q.value.2, x2, overflowFlag);
		OVERFLOWING_ADD!(p.value.3, q.value.3, x3, overflowFlag);

		yU64x4::new(x0, x1, x2, x3) & self.moduler
	}

	fn subElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		self.addElement(p, self.getAdditionInverseElement(q))
	}

	fn mulElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		let mut res0 = YU64x4_MUL_U64!(p, q.value.0);
		let mut res1 = YU64x4_MUL_U64!(p, q.value.1);
		let mut res2 = YU64x4_MUL_U64!(p, q.value.2);
		let mut res3 = YU64x4_MUL_U64!(p, q.value.3);

		YU64x4_LSH_64!(res1);
		YU64x4_LSH_128!(res2);
		YU64x4_LSH_192!(res3);

		res0 = self.addElement(res0, res1);
		res0 = self.addElement(res0, res2);
		res0 = self.addElement(res0, res3);

		res0
	}

	fn divElement(&self, p: yU64x4, q: yU64x4) -> Option<yU64x4>
	{
		Option::None
	}
}