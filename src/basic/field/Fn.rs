use std::option;
use ::basic::cell::yU64x4::*;
use ::basic::cell::yU64x8::*;

pub const n     : yU64x4 = yU64x4{value:(0x53BBF40939D54123, 0x7203DF6B21C6052B, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFEFFFFFFFF)};
    const rhoN  : yU64x4 = yU64x4{value:(0xAC440BF6C62ABEDD, 0x8DFC2094DE39FAD4, 0x0000000000000000, 0x0000000100000000)};
    const rhoN2 : yU64x4 = yU64x4{value:(0x901192af7c114f20, 0x3464504ade6fa2fa, 0x620fc84c3affe0d4, 0x1eb5e412a22b3d3b)};

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

pub fn transFn(mut x: yU64x4) -> yU64x4
{
	while(largerEqualThan(x, n))
	{
		x = x - n;
	}

	x
}

pub fn getAddInvModN(mut x: yU64x4) -> yU64x4
{
	n - x
}

pub fn getMulInvModN(x: yU64x4) -> yU64x4
{
	if(equalToZero(x)) {return yU64x4::new(0,0,0,0);}

	let mut u = x;
	let mut v = n;
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
				let (u,overflowFlag) = addNoMod(x1, n);
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

				let (u,overflowFlag) = addNoMod(x2, n);
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
			u = subModN(u,v);
			x1 = subModN(x1,x2);
		}
		else 
		{
			v = subModN(v,u);
			x2 = subModN(x2,x1);
		}
	}

	if(equalToOne(u))
	{
		while(largerEqualThan(x1, n))
		{
			x1 = x1 - n;
		}
		x1
	}
	else
	{
		while(largerEqualThan(x2, n))
		{
			x2 = x2 - n;
		}
		x2
	}
}

pub fn addModN(x: yU64x4, y: yU64x4) -> yU64x4
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
		m = addModN(rhoN, m);
	} 

	if largerEqualThan(m, n)
	{ m - n }
	else 
	{ m }
}

pub fn subModN(x: yU64x4, y: yU64x4) -> yU64x4
{
	addModN(x, getAddInvModN(y))
}

pub fn mulModN(x: yU64x4, y: yU64x4) -> yU64x4
{
	let x_bar = montMul(x, rhoN2);
	let y_bar = montMul(y, rhoN2);
	let t_bar = montMul(x_bar, y_bar);
	montRed(t_bar)
}


fn montMul(x: yU64x4, y:yU64x4) -> yU64x4
{
	let mut z = zero;

	for i in 0..256
	{
		z = if(y.get(i)==1) 
		{
			addModN(z,x)
		} 
		else 
		{
			z
		} ;

		if(z.value.0%2==1) 
		{
			let (u,overflowFlag) = addNoMod(z,n);
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

	if(largerEqualThan(z, n)) {z - n} else {z}
}

// get t * 2^(-256) mod p
fn montRed(mut t: yU64x4) -> yU64x4
{
	for i in 0..256
	{
		if(t.value.0%2==1) 
		{
			let (u,overflowFlag) = addNoMod(t, n);
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

	if(largerEqualThan(t, n)) {addModN(t, n)} else {t}
}