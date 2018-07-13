pub mod EccGroup;

trait Group<T: Copy>
{
	fn getAdditionInverseElement(&self) -> T;
	fn addElement(&self, p: T, q: T) -> T;
	fn subElement(&self, p: T, q: T) -> T;
}