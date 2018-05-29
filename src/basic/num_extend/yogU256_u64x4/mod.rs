/*// just for test 

pub struct yoU256
{
	pub value: u64x4,
}

impl yoU256
{
	pub fn new(x0: u64, x1: u64, x2: u64, x3: u64) -> Self
	{
		Self
		{
			value: u64x4::new(x0, x1, x2, x3),
		}
	}

	fn make (x: u64x4) -> Self
	{
		Self
		{
			value: x,
		}
	}
}

// Performance Optimize: use memcpy??
impl Clone for yoU256
{
	fn clone(&self) -> Self 
	{
		Self
		{
			value: self.value,
		}
	}
}

impl Not for yoU256
{
	type Output = Self;

	fn not(self) -> Self
	{
		Self
		{
			value: !self.value,
		}
	}
}

impl BitAnd<yoU256> for yoU256
{
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value & rhs.value,
		}
	}
}

impl BitOr<yoU256> for yoU256
{
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value | rhs.value,
		}
	}
}

impl BitXor<yoU256> for yoU256
{
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self
	{
		Self
		{
			value: self.value ^ rhs.value,
		}
	}
}

impl BitAndAssign<yoU256> for yoU256
{
	fn bitand_assign(&mut self, rhs: Self)
	{
		self.value &= rhs.value;
	}
}

impl BitOrAssign<yoU256> for yoU256
{
	fn bitor_assign(&mut self, rhs: Self)
	{
		self.value |= rhs.value;
	}
}

impl BitXorAssign<yoU256> for yoU256
{
	fn bitxor_assign(&mut self, rhs: Self)
	{
		self.value ^= rhs.value;
	}
}

macro_rules! YOU256_SHRN_ROTATE {
	($x:ident, $n:expr) => 
	(
		unsafe
		{
			let mut p: [u64;4] = [0;4]; // warning: do not change this to ensure no memory leakage 
			$x.value.store_unaligned_unchecked(&mut p);  //unsafe

			u64x4::new(p[$n%4], p[(1+$n)%4], p[(2+$n)%4], p[(3+$n)%4])
		}
	)
}

impl Shr<usize> for yoU256
{
	type Output = Self;

	fn shr(self, rhs: usize) -> Self
	{
		let shrn = rhs / 64;
		let mut x: u64x4 = YOU256_SHRN_ROTATE!(self, shrn);

		printu64x4!(x);

		let shrx = rhs % 64; 

		let mut y: u64x4 = x << (64-shrx);
		y = u64x4::new(y.extract(1), y.extract(2), y.extract(3), y.extract(0));

		yoU256::make((x>>shrx) | y)
	}
}*/

