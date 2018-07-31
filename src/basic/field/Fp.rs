use std::option;
use ::basic::field::*;
use ::basic::cell::yU64x4::*;
use ::basic::cell::yU64x8::*;

pub const p: yU64x4 = yU64x4{value:[0xFFFFFFFFFFFFFFFF, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF]};
const rhoP  : yU64x4 = yU64x4{value:[0x0000000000000001, 0x00000000FFFFFFFF, 0x0000000000000000, 0x0000000100000000]};
const rhoP2 : yU64x4 = yU64x4{value:[0x0000000200000003, 0x00000002FFFFFFFF, 0x0000000100000001, 0x0000000400000002]};
pub const inv2P : yU64x4 = yU64x4{value:[0x8000000000000000, 0xFFFFFFFF80000000, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFF7FFFFFFF]};

macro_rules! OVERFLOWING_ADD
{
	($x:expr, $y:expr, $result:ident, $overflowFlag:ident) => 
	(
		let car = if ($overflowFlag==true) {1} else {0};

		let r1 = u64::overflowing_add($x, $y);
		let r2 = u64::overflowing_add(r1.0, car);
		$result = r2.0;
		$overflowFlag = r1.1|r2.1;
	)
}

pub fn toElement(mut x: yU64x4) -> yU64x4
{
	while(largerEqualThan(x, p))
	{
		x = x - p;
	}

	x
}

pub fn getAddInv(mut x: yU64x4) -> yU64x4
{
	p - x
}

pub fn getMulInv(x: yU64x4) -> yU64x4
{
	if(equalToZero(x)) {return yU64x4::new(0,0,0,0);}

	let mut u = x;
	let mut v = p;
	let mut x1 = yU64x4::new(1,0,0,0);
	let mut x2 = yU64x4::new(0,0,0,0);

	while((!equalToOne(u))&&(!equalToOne(v)))
	{
		while(u.value[0]%2==0)
		{
			u.rightShift1();

			if(x1.value[0]%2==0) 
			{
				x1.rightShift1();
			}
			else 
			{
				let (u,overflowFlag) = addNoMod(x1, p);
				x1 = u;
				x1.rightShift1();
				if(overflowFlag)
				{
					x1.value[3] |= 0x8000000000000000;
				}
			}
		}

		while(v.value[0]%2==0)
		{
			v.rightShift1();

			if(x2.value[0]%2==0) 
			{
				x2.rightShift1();
			} 
			else 
			{

				let (u,overflowFlag) = addNoMod(x2, p);
				x2 = u;
				x2.rightShift1();
				if(overflowFlag)
				{
					x2.value[3] |= 0x8000000000000000;
				}
			}
		}

		if(largerEqualThan(u,v))
		{
			u = sub(u,v);
			x1 = sub(x1,x2);
		}
		else 
		{
			v = sub(v,u);
			x2 = sub(x2,x1);
		}
	}

	if(equalToOne(u))
	{
		while(largerEqualThan(x1, p))
		{
			x1 = x1 - p;
		}
		x1
	}
	else
	{
		while(largerEqualThan(x2, p))
		{
			x2 = x2 - p;
		}
		x2
	}
}

pub fn add(x: yU64x4, y: yU64x4) -> yU64x4
{
	let res0: u64;
	let res1: u64;
	let res2: u64;
	let res3: u64;
	let mut overflowFlag = false;

	OVERFLOWING_ADD!(x.value[0], y.value[0], res0, overflowFlag);
	OVERFLOWING_ADD!(x.value[1], y.value[1], res1, overflowFlag);
	OVERFLOWING_ADD!(x.value[2], y.value[2], res2, overflowFlag);
	OVERFLOWING_ADD!(x.value[3], y.value[3], res3, overflowFlag);
	

	let mut m = yU64x4
	{
		value: [res0, res1, res2, res3],
	};

	if overflowFlag==true  //overflow
	{
		m = add(rhoP, m);
	} 

	if largerEqualThan(m,p)
	{ m - p }
	else 
	{ m }
}

pub fn sub(x: yU64x4, y: yU64x4) -> yU64x4
{
	add(x, getAddInv(y))
}

fn helperMul(x: u64, y: u64) -> (u128, u128)
{
	/* a helper overflowing multiplication for u64 */
	let z: u128 = (x as u128) * (y as u128);
	let carry = z >> 64;
	let rst = z ^ (carry << 64);

	(rst, carry)
}

fn rawMul(x: yU64x4, y: yU64x4) -> [u64;8]
{
	/* Perform long multiplication */
	let mut result: [u64; 8] = [0; 8];
	let mut carry: u128 = 0;

	// for each result block
	for blocki in 0..7
	{
		// temporary value
		let mut cur: u128 = carry;
		carry = 0;

		// enumerate each block of y
		let mut low: usize = 0;
		let mut high: usize = 0;
		if blocki > 3 
		{ 
			low = blocki - 3;
			high = 3;
		} else {
			low = 0;
			high = blocki;
		}

		for yi in low..=high
		{
			let (rst, c) = helperMul(x.value[blocki - yi], y.value[yi]);
			carry += c;
			cur += rst;
		}

		// check addition overlow carry
		let c = cur >> 64;
		carry += c;

		result[blocki] = (cur ^ (c << 64)) as u64;
	}
	result[7] = carry as u64;

	result
}

fn helperSplitU64(x: u64) -> (u32, u32)
{
	let high = x >> 32;
	let low = x ^ (high << 32);
	(low as u32, high as u32)
}

fn reduce(n: [u64;8]) -> yU64x4
{
	/* fast reduction 256bit to 128bit*/
	/* ref: http://cacr.uwaterloo.ca/techreports/1999/corr99-39.pdf */

	// first split the input
	let mut A: [u32;16] = [0;16];
	for i in 0..8
	{
		let (a, b) = helperSplitU64(n[i]);
		A[2 * i] = a;
		A[(2 * i) ^ 1] = b;
	}

	// prepare the summands
	// given by LFSR with [1,0,0,0,1,-1,0,1] and proper re-combination
	// of mantissa
	let S = yU64x4::new(n[0],n[1],n[2],n[3]); // lower parts of n

	// the following should be added twice (suffix D)
	let S15D = yU64x4::
		fromU32([A[15], A[15], 0, 0, 0, A[15], 0, A[15]]);
	let S14D = yU64x4::
		fromU32([A[14], A[14], 0, 0, A[14], 0, 0, A[14]]);
	let S13D = yU64x4::
		fromU32([A[13], 0, 0, A[13], 0, 0, 0, A[13]]);
	let S12D = yU64x4::
		fromU32([0, 0, 0, 0, 0, 0, 0, A[12]]);
	// find the sum
	let sumD = add(add(S15D, S14D), add(S13D, S12D));

	// find other sum (hard coded by sight)
	let S8_13 = yU64x4::
		fromU32([A[8], A[13], 0, A[8], A[13], A[13], 0, A[8]]);
	let S9_14 = yU64x4::
		fromU32([A[9], A[9], 0, A[14], A[9], A[14], A[14], A[9]]);
	let S10_12 = yU64x4::
		fromU32([A[10], A[10], 0, A[12], A[12], A[10], 0, A[10]]);
	let S11 = yU64x4::
		fromU32([A[11], A[11], 0, A[11], 0, 0, A[11], A[11]]);
	let S15_12 = yU64x4::
		fromU32([A[12], A[12], 0, A[15], A[15], 0, A[15], A[15]]);
	
	// sum all the stuffs together
	let S = add(add(add(S, sumD), add(add(S8_13, S9_14), add(S10_12, S11))), add(S15_12, sumD));

	// find the subtrahend
	let A: u64 = A[8] as u64 + A[9] as u64 + A[13] as u64 + A[14] as u64;
	let upper = A >> 32;
	let lower = A ^ (upper << 32);

	
	let S = sub(S, yU64x4::fromU32([0, 0, lower as u32, upper as u32, 0, 0, 0, 0]));

	if largerEqualThan(S, p)
	{
		S - p
	} else {
		S
	}
}

pub fn mul(x: yU64x4, y: yU64x4) -> yU64x4
{
	reduce(rawMul(x, y))
}

pub fn div(x: yU64x4, y: yU64x4) -> yU64x4
{	
	let q = getMulInv(y);
	mul(x, q)
}

#[cfg(test)]
mod tests 
{
    extern crate test;
    extern crate rand;

    use super::Fp::*;
    use ::basic::cell::yU64x4::*;

    use self::test::Bencher;
    use rand::random;

    fn rand_elem() -> yU64x4
    {
        yU64x4::new(random::<u64>(), random::<u64>(), random::<u64>(), random::<u64>())
    }

	#[test]
	fn test_helper_mul()
	{
		let a = 1327187218989_u64;
		let b = 532432746434597_u64;
		let (r, c) = helperMul(a, b);
		assert_eq!(c, 0x2488467);
		assert_eq!(r, 0x454d_d932_de46_f081);
	}

	#[test]
    fn test_raw_mul() 
    {
		let a = yU64x4::new(0x1351534EF350E2BB, 0x14E68D77BC131F7B,
			0x6A7171A01A638E75, 0x4F9EA7A816AB7908);
		let b = yU64x4::new(0x141CC66D0595B6F0, 0xC85BF76622E07301,
			0x5B261629F8AD4D45, 0x7DE9CF63BC635636);
        let rst = rawMul(a, b);

        assert_eq!(rst, [0x866d99203adc8150, 0xc623d9758ed1332c, 0x3b1dab20b950e375, 0xbc165cad5d713996,
			0x63e9be904aa539b5, 0x7edc6525c6a1f17c, 0x2a99a65d2ec61248, 0x27292fc3f99184ca]);
    }

	#[bench]
	fn bench_raw_mul(ben: &mut Bencher)
	{
		let a = rand_elem();
		let b = rand_elem();
		ben.iter(||
		{
			rawMul(a, b);
		})
	}

    #[test]
    fn test_mul() 
    {
		let ra = random::<u32>() as u64;
		let rb = random::<u32>() as u64;
        let (mut a, f1) = addNoMod(p, yU64x4::new(ra, 0, 0, 0));
        let (mut b, f2) = addNoMod(p, yU64x4::new(rb, 0, 0, 0));
		a = if f1 { println!("OVER A"); a + rhoP } else {a};
		b = if f2 { println!("OVER B"); b + rhoP } else {b};
        let c = mul(a, b);
        assert!(equalTo(c, yU64x4::new(ra * rb, 0, 0, 0)));
    }
	

    #[bench]
	fn bench_mul(ben: &mut Bencher)
	{
        let a = rand_elem();
            
        let b = rand_elem();
            
        ben.iter(|| 
        {
            let c = mul(a, b);
        })
	}

    #[test]
	fn test_inversion()
	{
        let a = rand_elem();
        let b = getMulInv(a);
        assert!(equalTo(mul(a, b), yU64x4::new(1,0,0,0)));
	}

    #[bench]
	fn bench_inversion(ben: &mut Bencher)
	{
        let a = rand_elem();
        ben.iter(|| 
        {
            let b = getMulInv(a);
        })
	}

    #[bench]
    fn bench_add(ben: &mut Bencher)
    {
        let a = rand_elem();
        let b = rand_elem();
        ben.iter(||
        {
            let c = add(a,b);
        })
    }
}
