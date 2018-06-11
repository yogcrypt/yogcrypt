use std::option;
use ::basic::field::*;
use ::basic::cell::yU64x4::*;
use ::basic::cell::yU64x8::*;

const ZERO: yU64x4 =  yU64x4{value:(0,0,0,0)};

pub struct prime_field
{
	prime: yU64x4,
	negPrime: yU64x4,
	rho: yU64x4,
	rho2: yU64x4,
}

// accessor
impl prime_field
{
	pub fn getRho(&self) -> yU64x4
	{
		self.rho
	}

	pub fn getRho2(&self) -> yU64x4
	{
		self.rho2
	}
}

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

impl prime_field
{
	pub fn new(prime: yU64x4) -> prime_field
	{
		let mut field = prime_field
		{
			prime,
			negPrime: prime_field::getNeg(prime),
			rho: yU64x4::new(1,0,0,0),
			rho2: yU64x4::new(1,0,0,0),
		};

		//compute rho
		for i in 0..256
		{
			field.rho.leftShift1();
			if prime_field::largerEqualThan(field.rho, prime)
			{
				field.rho = prime_field::sub_yU64x4(field.rho, prime);
			}
		}

		//compute rho2 (=rho^2 mod p)
		field.rho2 = field.rho;
		for i in 0..256
		{
			field.rho2.leftShift1();
			if prime_field::largerEqualThan(field.rho2, prime)
			{
				field.rho2 = prime_field::sub_yU64x4(field.rho2, prime);
			}
		}

		field
	}
	
}

impl theField for prime_field
{
	fn getAdditionInverseElement(&self, mut x: yU64x4) -> yU64x4
	{
		prime_field::sub_yU64x4(self.prime, x)
	}
	
	fn getMultiplicationInverseElement(&self, x: yU64x4) -> yU64x4
	{
		let mut u = x;
		let mut v = self.prime;
		let mut x1 = yU64x4::new(1,0,0,0);
		let mut x2 = yU64x4::new(0,0,0,0);

		while((!prime_field::equalToOne(u))&&(!prime_field::equalToOne(v)))
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
					x1 = prime_field::add_yU64x4(x1,self.prime);
					x1.rightShift1();
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
					x2 = prime_field::add_yU64x4(x2,self.prime);
					x2.rightShift1();
				}
			}

			if(prime_field::largerEqualThan(u,v))
			{
				u = self.subElement(u,v);
				x1 = self.subElement(x1,x2);
			}
			else 
			{
				v = self.subElement(v,u);
				x2 = self.subElement(x2,x1);
			}
		}

		if(prime_field::equalToOne(u))
		{
			while(prime_field::largerEqualThan(x1,self.prime))
			{
				x1 = prime_field::sub_yU64x4(x1, self.prime);
			}
			x1
		}
		else
		{
			while(prime_field::largerEqualThan(x2, self.prime))
			{
				x2 = prime_field::sub_yU64x4(x2, self.prime);
			}
			x2
		}
	
/*		let mut t = yU64x4::new(1,0,0,0);

		let mut u = prime_field::sub_yU64x4(self.prime, yU64x4::new(2,0,0,0)); // u = p - 2

		while(prime_field::equalToOne(u)==false)
		{
			if(u.value.0%2==1)
			{
				t = self.mulElement(t, x);
				u.value.0 -= 1;
			}
			else 
			{
				x = self.mulElement(x, x);
				u.rightShift1();	
			}
		}

		self.mulElement(x, t)*/
	}

	fn addElement(&self, x: yU64x4, y: yU64x4) -> yU64x4
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
		
		if overflowFlag==true
		{
			panic!("get wrong result because of overflowFlag==true");
		}

		let m = yU64x4
		{
			value: (res0, res1, res2, res3),
		};

		if prime_field::largerEqualThan(m,self.prime)
		{ prime_field::sub_yU64x4(m,self.prime) }
		else 
		{ m }
	}

	fn subElement(&self, x: yU64x4, y: yU64x4) -> yU64x4
	{
		self.addElement(x, self.getAdditionInverseElement(y))
	}

	fn mulElement(&self, x: yU64x4, y: yU64x4) -> yU64x4
	{
		let x_bar = self.montMul(x, self.rho2);
		let y_bar = self.montMul(y, self.rho2);
		let t_bar = self.montMul(x_bar, y_bar);
		self.montRed(t_bar)
	}

	fn divElement(&self, x: yU64x4, y: yU64x4) -> yU64x4
	{
		self.mulElement(x, self.getMultiplicationInverseElement(y))
	}
}

// Mong
impl prime_field
{
	// get x * y * 2^(-256) mod p
	pub fn montMul(&self, x: yU64x4, y:yU64x4) -> yU64x4
	{
		let mut z = yU64x4::new(0, 0, 0, 0);

		for i in 0..256
		{
			z = if(y.get(i)==1) {prime_field::add_yU64x4(z,x)} else {z} ;
			if(z.value.0%2==1) {z = prime_field::add_yU64x4(z, self.prime);}
			z.rightShift1();
		};

		if(prime_field::largerEqualThan(z, self.prime)) {self.subElement(z, self.prime)} else {z}
	}

	// get t * 2^(-256) mod p
	pub fn montRed(&self, mut t: yU64x4) -> yU64x4
	{
		for i in 0..256
		{
			if(t.value.0%2==1) {t = prime_field::add_yU64x4(t, self.prime)};
			t.rightShift1();
		}

		if(prime_field::largerEqualThan(t, self.prime)) {self.subElement(t, self.prime)} else {t}
	}
}

// Arithmetic
impl prime_field
{
	fn getNeg(mut x: yU64x4) -> yU64x4
	{

		if x.value.0!=0
		{
			x.value.0 = u64::wrapping_neg(x.value.0);
			x.value.1 = !x.value.1;
			x.value.2 = !x.value.2;
			x.value.3 = !x.value.3;
		}
		else if x.value.1!=0
		{
			x.value.1 = u64::wrapping_neg(x.value.1);
			x.value.2 = !x.value.2;
			x.value.3 = !x.value.3;
		}
		else if x.value.2!=0
		{
			x.value.2 = u64::wrapping_neg(x.value.2);
			x.value.3 = !x.value.3;
		}
		else if x.value.3!=0
		{
			x.value.3 = u64::wrapping_neg(x.value.3);
		}

		x
	}

	fn add_yU64x4(x: yU64x4, y: yU64x4) -> yU64x4
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
		
		yU64x4
		{
			value: (res0, res1, res2, res3),
		}
	}

	fn sub_yU64x4(x: yU64x4, y: yU64x4) -> yU64x4
	{
		prime_field::add_yU64x4(x, prime_field::getNeg(y))
	}
}

// Order
impl prime_field
{
	fn largerEqualThan(x: yU64x4, y: yU64x4) -> bool
	{
		if(x.value.3>y.value.3) {return true;};
		if(x.value.3<y.value.3) {return false;};
		if(x.value.2>y.value.2) {return true;};
		if(x.value.2<y.value.2) {return false;};
		if(x.value.1>y.value.1) {return true;};
		if(x.value.1<y.value.1) {return false;};
		if(x.value.0>=y.value.0) {return true;};
		return false;
	}

	fn equalTo(x: yU64x4, y: yU64x4) -> bool
	{
		x.value.0==y.value.0 && x.value.1==y.value.1 && x.value.2==y.value.2 && x.value.3==y.value.3
	}

	fn equalToZero(x: yU64x4) -> bool
	{
		x.value.0==0 && x.value.1==0 && x.value.2==0 && x.value.3==0
	}

	fn equalToOne(x: yU64x4) -> bool
	{
		x.value.0==1 && x.value.1==0 && x.value.2==0 && x.value.3==0
	}
}