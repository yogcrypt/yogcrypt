extern crate rand;
use self::rand::random;

use std::option;
use ::basic::field::*;
use ::basic::cell::yU64x4::*;
use ::basic::cell::yU64x8::*;


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

impl power2_field
{
	fn getNewElement(&self, x: yU64x4) -> yU64x4
	{
		x & self.moduler
	}

	fn getNewRandomElement(&self) -> yU64x4
	{
		yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>())  & self.moduler
	}
}

impl theField for power2_field
{

	fn getAdditionInverseElement(&self, mut p: yU64x4) -> yU64x4
	{
		p & self.moduler
	}

	// unfinished
	fn getMultiplicationInverseElement(&self, mut p: yU64x4) -> yU64x4
	{
		p = p & self.moduler;

		p
	}

	fn addElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		(p ^ q)
	}

	fn subElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		(p ^ q)
	}

	fn mulElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		let mut x = yU64x8::new(0, 0, 0, 0, 0, 0, 0, 0);

		let i = q.value.0;
		for m in 0..64 
		{
			if(i%2==1) {x ^= p.letfRotateTo_yU64x8(m)};
			i >> 1;
		}

		p^q
	}

	fn divElement(&self, p: yU64x4, q: yU64x4) -> yU64x4
	{
		p
	}
}