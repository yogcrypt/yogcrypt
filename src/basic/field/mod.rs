pub mod prime_field;
pub mod power2_field;

use ::basic::cell::yU64x4::*;
use std::option;

trait Field
{


	fn getNewElement() -> yU64x4;
	fn getAdditionInverseElement(p: yU64x4) -> yU64x4;
	fn getMultiplicationInverseElement(p: yU64x4) -> Option<yU64x4>;
	fn addElement(p: yU64x4, q: yU64x4) -> yU64x4;
	fn subElement(p: yU64x4, q: yU64x4) -> yU64x4;
	fn mulElement(p: yU64x4, q: yU64x4) -> yU64x4;
	fn divElement(p: yU64x4, q: yU64x4) -> Option<yU64x4>;
}