use std::option;
use ::basic::cell::yU64x4::*;

pub struct prime_field
{
	order: yU64x4,
}

impl prime_field
{
	//unfinished
	pub fn isPrime(x: yU64x4) -> bool
	{
		true
	}

	pub fn new(x: yU64x4) -> Option<Self>
	{
		if prime_field::isPrime(x) 
		{
			Option::Some(prime_field{order:x,})
		}
		else 
		{
			Option::None
		}
	}
}