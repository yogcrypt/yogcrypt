pub mod prime_field;
pub mod power2_field;

use ::basic::cell::yU64x4::*;
use std::option;

pub trait theField
{
	fn getAdditionInverseElement(&self, x: yU64x4) -> yU64x4;
	fn getMultiplicationInverseElement(&self, x: yU64x4) -> yU64x4;
	fn addElement(&self, x: yU64x4, y: yU64x4) -> yU64x4;
	fn subElement(&self, x: yU64x4, y: yU64x4) -> yU64x4;
	fn mulElement(&self, x: yU64x4, y: yU64x4) -> yU64x4;
	fn divElement(&self, x: yU64x4, y: yU64x4) -> yU64x4;
}

trait Field<T: Copy>
{
	fn getAdditionInverseElement(&self) -> T;
	fn getMultiplicationInverseElement(&self) -> T;
	fn addElement(&self, x: T, y: T) -> T;
	fn subElement(&self, x: T, y: T) -> T;
	fn mulElement(&self, x: T, y: T) -> T;
	fn divElement(&self, x: T, y: T) -> T;
}