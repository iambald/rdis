pub mod elf;
pub mod raw;
pub mod pe;

#[derive(Clone, Copy)]
pub enum Vendor {
	AMD,
	Intel,
	Generic
}

pub trait BinFormat<'a> {
	fn next_byte(&mut self) -> Option<u8>;

	// fn set_iterator<T: Iterator<Item = u8> + 'a>(&mut self, it: &'a mut T);
	fn clear_iterator(&mut self);

	fn input_skip(&'a mut self, n: usize);
	fn input_end(&self) -> bool;

	fn set_vendor(&mut self, vendor: Vendor);
	fn get_vendor(&self) -> Vendor;

	fn set_pc(&mut self, pc: u64);
	fn get_pc(&self) -> u64;
}