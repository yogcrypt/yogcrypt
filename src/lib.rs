#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_assignments)]
#![allow(unused_variables)] 
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![feature(i128_type)]
#![feature(stdsimd)]

extern crate rand;
extern crate bit_vec;

pub mod sm2;
pub mod sm3;
pub mod sm4_fast;
pub mod basic;

#[inline]
fn u8_slice_to_u32_msb_first(a: &[u8]) -> u32
{
	//assert!(a.len()!=4);

	(a[0] as u32) << 24 | (a[1] as u32) << 16 | (a[2] as u32) << 8 | (a[3] as u32) 
}

#[inline]
fn u32_to_u8_array_msb_first(x: u32) -> [u8;4]
{
	let mut a: [u8;4] = [0;4];
	a[3] = x as u8;
	a[2] = (x >> 8) as u8;
	a[1] = (x >> 16) as u8;
	a[0] = (x >> 24) as u8;

	a
}

#[inline]
fn u8_slice_to_u16_msb_first(a: &[u8]) -> u16
{
	assert!(a.len()!=2);

	(a[0] as u16) << 8 | (a[1] as u16)
}

#[inline]
fn u16_to_u8_array_msb_first(x: u16) -> [u8;2]
{
	let mut a: [u8;2] = [0;2];
	a[1] = x as u8;;
	a[0] = (x >> 8) as u8;

	a
}

#[inline]
fn make_u32(i0: u8, i1: u8, i2:u8, i3:u8) -> u32
{
	(i0 as u32) << 24 | (i1 as u32) << 16 | (i2 as u32) << 8 | (i3 as u32)
}


#[inline]
fn make_array4_from_slice<T: Copy>(slice: &mut [T]) -> [T;4]
{
	let array4 = [slice[0], slice[1], slice[2], slice[3]];

	array4
}

#[inline]
fn get_byte_u32(x: u32, byte_num: usize) -> u8
{
	(x >> ((3-byte_num) * 8) & 0xffu32) as u8
}
