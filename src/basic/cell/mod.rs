pub mod yU64x4;
pub mod yU64x8;

trait UniformAccessU64
{
	fn get(&self, i: usize) -> u64;
	fn set(&mut self, i: usize, x: u64);
}