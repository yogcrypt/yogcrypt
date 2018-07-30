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
#![feature(test)] 

extern crate rand;
extern crate bit_vec;

#[macro_use]
extern crate lazy_static;

pub mod sm2;
pub mod sm3;
pub mod sm4;
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



#[cfg(test)]
mod tests 
{
	extern crate test;

	use super::*;
	use self::test::Bencher;
	use sm2::*;
	use sm3::*;
	use sm4::*;

	use basic::cell::yU64x4::*;
	use basic::group::EccGroup::Point;

	#[test]
	#[ignore]
	fn test_sm4()
	{
		let Mk: [u32;4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
		let Rk = getSm4Rk(&Mk);
		let Ptxt: [u32;4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

		let Ctxt = sm4Enc(&Rk, &Ptxt);

		println!("{:08X} {:08X} {:08X} {:08X}", Ctxt[0], Ctxt[1], Ctxt[2], Ctxt[3]);

		let Ptxt2 = sm4Dec(&Rk, &Ctxt);
		println!("{:08X} {:08X} {:08X} {:08X}", Ptxt2[0], Ptxt2[1], Ptxt2[2], Ptxt2[3]);
		assert_eq!(Ptxt, Ptxt2);
	}

	#[test]
	#[ignore]
	fn test_sm3()
	{
		let msg: [u32;16] = [
			0x61626364, 0x61626364, 0x61626364, 0x61626364,
			0x61626364, 0x61626364, 0x61626364, 0x61626364,
			0x61626364, 0x61626364, 0x61626364, 0x61626364,
			0x61626364, 0x61626364, 0x61626364, 0x61626364,];

		let hash = sm3Enc(&msg, 512);
		println!("{:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X} {:08X}",
			hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7]);


		println!();
		println!();
	}

	#[test]
	#[ignore]
	fn test_sm2()
	{
		// prepare constants

		let Xg = yU64x4::new(0x4C4E6C147FEDD43D, 0x32220B3BADD50BDC, 0x746434EBC3CC315E, 0x421DEBD61B62EAB6);
		let Yg = yU64x4::new(0xA85841B9E46E09A2, 0xE5D7FDFCBFA36EA1, 0xD47349D2153B70C4, 0x0680512BCBB42C07);
		let Da = yU64x4::new(0x0C23661D15897263, 0x2A519A55171B1B65, 0x068C8D803DFF7979, 0x128B2FA8BD433C6C);
		let Xa = yU64x4::new(0xE97C04FF4DF2548A, 0x02BB79E2A5844495, 0x471BEE11825BE462, 0x0AE4C7798AA0F119);
		let Ya = yU64x4::new(0xA9FE0C6BB798E857, 0x07353E53A176D684, 0x6352A73C17B7F16F, 0x7C0240F88F1CD4E1);

		let G = Point::new(Xg, Yg);
		let P = Point::new(Xa, Ya);

		let Msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

		let Q = sm2GetPubKey(Da);

		let mut m = sm2GenSign(&Msg, Da, Q, 4);

		for i in 0..1
		{
			let t = sm2VerSign(&Msg, Q, 4, m.0, m.1);
			println!("{}",t);
			assert!(t);
		}

	}

	#[bench]
	#[ignore]
    fn bench_sm4(b: &mut Bencher) 
	{
        b.iter(|| {
			let Mk: [u32;4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];
			let Rk = getSm4Rk(&Mk);
			let Ptxt: [u32;4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

			let Ctxt = sm4Enc(&Rk, &Ptxt);


			let Ptxt2 = sm4Dec(&Rk, &Ctxt);
			assert_eq!(Ptxt, Ptxt2);
		});
    }

	#[bench]
	#[ignore]
	fn bench_sm3(b: &mut Bencher)
	{
		b.iter(|| {
			let msg: [u32;16] = [
				0x61626364, 0x61626364, 0x61626364, 0x61626364,
				0x61626364, 0x61626364, 0x61626364, 0x61626364,
				0x61626364, 0x61626364, 0x61626364, 0x61626364,
				0x61626364, 0x61626364, 0x61626364, 0x61626364,];

			let hash = sm3Enc(&msg, 512);
		});
	}
	

	#[bench]
	#[ignore]
	fn bench_sm2_genSign(ben: &mut Bencher)
	{
		let Xg = yU64x4::new(0x4C4E6C147FEDD43D, 0x32220B3BADD50BDC, 0x746434EBC3CC315E, 0x421DEBD61B62EAB6);
		let Yg = yU64x4::new(0xA85841B9E46E09A2, 0xE5D7FDFCBFA36EA1, 0xD47349D2153B70C4, 0x0680512BCBB42C07);
		let Da = yU64x4::new(0x0C23661D15897263, 0x2A519A55171B1B65, 0x068C8D803DFF7979, 0x128B2FA8BD433C6C);
		let Xa = yU64x4::new(0xE97C04FF4DF2548A, 0x02BB79E2A5844495, 0x471BEE11825BE462, 0x0AE4C7798AA0F119);
		let Ya = yU64x4::new(0xA9FE0C6BB798E857, 0x07353E53A176D684, 0x6352A73C17B7F16F, 0x7C0240F88F1CD4E1);

		let G = Point::new(Xg, Yg);
		let P = Point::new(Xa, Ya);

		let Msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

		let Q = sm2GetPubKey(Da);
		
		ben.iter(|| {
			let m = sm2GenSign(&Msg, Da, Q, 4);
		});
	}

	#[bench]
	#[ignore]
	fn bench_sm2_verSign(ben: &mut Bencher)
	{
		let Xg = yU64x4::new(0x4C4E6C147FEDD43D, 0x32220B3BADD50BDC, 0x746434EBC3CC315E, 0x421DEBD61B62EAB6);
		let Yg = yU64x4::new(0xA85841B9E46E09A2, 0xE5D7FDFCBFA36EA1, 0xD47349D2153B70C4, 0x0680512BCBB42C07);
		let Da = yU64x4::new(0x0C23661D15897263, 0x2A519A55171B1B65, 0x068C8D803DFF7979, 0x128B2FA8BD433C6C);
		let Xa = yU64x4::new(0xE97C04FF4DF2548A, 0x02BB79E2A5844495, 0x471BEE11825BE462, 0x0AE4C7798AA0F119);
		let Ya = yU64x4::new(0xA9FE0C6BB798E857, 0x07353E53A176D684, 0x6352A73C17B7F16F, 0x7C0240F88F1CD4E1);

		let G = Point::new(Xg, Yg);
		let P = Point::new(Xa, Ya);

		let Msg = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

		let Q = sm2GetPubKey(Da);
			
		let m = sm2GenSign(&Msg, Da, Q, 4);
		
		ben.iter(|| {
			let t = sm2VerSign(&Msg, Q, 4, m.0, m.1);
			assert!(t);
		});
	}
}