use std::option;
use ::basic::field::*;
use ::basic::cell::yU64x4::*;
use ::basic::cell::yU64x8::*;

pub const p: yU64x4 = yU64x4{value:(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF)};
const rhoP  : yU64x4 = yU64x4{value:(0x0000000000000001, 0x00000000FFFFFFFF, 0x0000000000000000, 0x0000000100000000)};
const rhoP2 : yU64x4 = yU64x4{value:(0x0000000200000003, 0x00000002FFFFFFFF, 0x0000000100000001, 0x0000000400000002)};
pub const inv2P : yU64x4 = yU64x4{value:(0x8000000000000000, 0xFFFFFFFF80000000, 0xFFFFFFFFFFFFFFFF, 0x7FFFFFFF7FFFFFFF)};

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
		while(u.value.0%2==0)
		{
			u.rightShift1();

			if(x1.value.0%2==0) 
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
					x1.value.3 |= 0x8000000000000000;
				}
			}
		}

		while(v.value.0%2==0)
		{
			v.rightShift1();

			if(x2.value.0%2==0) 
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
					x2.value.3 |= 0x8000000000000000;
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

	OVERFLOWING_ADD!(x.value.0, y.value.0, res0, overflowFlag);
	OVERFLOWING_ADD!(x.value.1, y.value.1, res1, overflowFlag);
	OVERFLOWING_ADD!(x.value.2, y.value.2, res2, overflowFlag);
	OVERFLOWING_ADD!(x.value.3, y.value.3, res3, overflowFlag);
	

	let mut m = yU64x4
	{
		value: (res0, res1, res2, res3),
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

pub fn mul(x: yU64x4, y: yU64x4) -> yU64x4
{
	let x_bar = montMul(x, rhoP2);
	let y_bar = montMul(y, rhoP2);
	let t_bar = montMul(x_bar, y_bar);
	montRed(t_bar)
}

pub fn div(x: yU64x4, y: yU64x4) -> yU64x4
{	
	let q = getMulInv(y);
	mul(x, q)
}

fn montMul(x: yU64x4, y:yU64x4) -> yU64x4
{
	let mut z = yU64x4::new(0, 0, 0, 0);

	for i in 0..256
	{
		z = if(y.get(i)==1) 
		{
			add(z,x)
		} 
		else 
		{
			z
		} ;

		if(z.value.0%2==1) 
		{
			let (u,overflowFlag) = addNoMod(z,p);
			z = u;
			z.rightShift1();
			if(overflowFlag)
			{
				z.value.3 |= 0x8000000000000000;
			}
		}
		else 
		{
			z.rightShift1();
		}

		
	};

	if(largerEqualThan(z, p)) {z - p} else {z}
}

// get t * 2^(-256) mod p
fn montRed(mut t: yU64x4) -> yU64x4
{
	for i in 0..256
	{
		if(t.value.0%2==1) 
		{
			let (u,overflowFlag) = addNoMod(t, p);
			t = u;
			t.rightShift1();
			if(overflowFlag)
			{
				t.value.3 |= 0x8000000000000000;
			}
		}	
		else
		{
			t.rightShift1();
		}
	}

	if(largerEqualThan(t, p)) {sub(t, p)} else {t}
}

pub fn div2(mut x: yU64x4) -> yU64x4
{
	if(x.value.0%2==1)
	{
		x = add(x, p);
	}
	x.rightShift1();

	x
}
