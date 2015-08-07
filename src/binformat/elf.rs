use super::BinFormat;
use super::Vendor;

pub struct Elf {
	size: u64,
	iterator: Option<Box<Iterator<Item = u8>>>,
	vendor: Vendor,
	pc: u64,
}

impl Elf {
	pub fn new(size: u64, iterator: Option<Box<Iterator<Item = u8>>>, vendor: Vendor, pc: u64) -> Elf {
		Elf {
			size: size,
			iterator: iterator,
			vendor: vendor,
			pc: pc
		}
	}
}

// impl BinFormat for Elf {
// 	fn next_byte(&mut self) -> Option<u8> {
// 		match self.iterator {
// 			Some(ref mut f) => (*f).next(),
// 			None => None
// 		}
// 	}

// 	fn set_iterator<T: Iterator<Item = u8> + 'static>(&mut self, it: T) {
// 		self.iterator = Some(Box::new(it))
// 	}

// 	fn clear_iterator(&mut self) {
// 		self.iterator = None
// 	}

// 	fn input_skip(&mut self, n: u64) {

// 	}
// 	fn input_end(&self) -> bool {
// 		true
// 	}

// 	fn set_vendor(&mut self, vendor: Vendor) {
// 		self.vendor = vendor
// 	}
// 	fn get_vendor(&self) -> Vendor {
// 		self.vendor
// 	}

// 	fn set_pc(&mut self, pc: u64) {
// 		self.pc = pc
// 	}
// 	fn get_pc(&self) -> u64 {
// 		self.pc
// 	}
// }