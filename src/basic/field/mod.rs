pub mod prime_field;
pub mod power2_field;

use ::basic::cell::yU64x4::*;
use std::option;

trait Field
{
	fn getNewElement(&self, x: yU64x4) -> yU64x4;
	fn getNewRandomElement(&self) -> yU64x4;
	fn getAdditionInverseElement(&self, p: yU64x4) -> yU64x4;
	fn getMultiplicationInverseElement(&self, p: yU64x4) -> Option<yU64x4>;
	fn addElement(&self, p: yU64x4, q: yU64x4) -> yU64x4;
	fn subElement(&self, p: yU64x4, q: yU64x4) -> yU64x4;
	fn mulElement(&self, p: yU64x4, q: yU64x4) -> yU64x4;
	fn divElement(&self, p: yU64x4, q: yU64x4) -> Option<yU64x4>;
}