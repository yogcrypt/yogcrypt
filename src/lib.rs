// make the linter allow the following usage

// literals declartion are used in S boxes which are
// not intended for human reading
#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
// Or (`|`) operators are used in overflowing addition
// which is not a mistake
#![cfg_attr(feature = "cargo-clippy", allow(suspicious_arithmetic_impl))]
// single characters names are used in accordance to
// documentation of cryptographic schemes
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
extern crate rand;

#[macro_use]
extern crate lazy_static;

pub mod basic;
pub mod sm2;
pub mod sm3;
pub mod sm4;
