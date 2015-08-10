use super::Binary;
use super::Vendor;
use super::Mode;

pub struct Elf<'a> {
	iterator: Box<Iterator<Item=u8> + 'a>,
	vendor: Vendor,
	pc: usize,
	mode: Mode
}

impl<'a> Elf<'a> {
	pub fn new(iterator: Box<Iterator<Item=u8> + 'a>, vendor: Vendor, pc: usize, mode: Mode) -> Elf {
		Elf {
			iterator: iterator,
			vendor: vendor,
			pc: pc,
			mode: mode
		}
	}
}

impl<'a> Iterator for Elf<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.iterator.next()
	}
}

impl<'a> Binary for Elf<'a> {

	fn set_vendor(&mut self, vendor: Vendor) {
		self.vendor = vendor;
	}

	fn get_vendor(&self) -> Vendor {
		self.vendor
	}

	fn set_pc(&mut self, pc: usize) {
		self.pc = pc
	}

	fn get_pc(&self) -> usize {
		self.pc
	}

	fn set_mode(&mut self, mode: Mode) {
		self.mode = mode
	}

	fn get_mode(&self) -> Mode {
		self.mode
	}
}
