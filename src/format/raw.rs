use super::Binary;
use super::Vendor;
use super::Mode;

pub struct Raw<'a> {
	iterator: Box<Iterator<Item=u8> + 'a>,
	pub vendor: Vendor,
	pub mode: Mode,
	offset: usize	// Current offset (from pc)
}

impl<'a> Raw<'a> {
	pub fn new(iterator: Box<Iterator<Item=u8> + 'a>, vendor: Vendor, mode: Mode) -> Raw {
		Raw {
			iterator: iterator,
			vendor: vendor,
			mode: mode,
			offset: 0
		}
	}

	pub fn offset(&self) -> usize {
		self.offset
	}
}

impl<'a> Iterator for Raw<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		match self.iterator.next() {
			None => None,
			Some(b) => {
				self.offset += 1;
				Some(b)
			}			
		}
	}
}

impl<'a> Binary for Raw<'a> {

	fn set_vendor(&mut self, vendor: Vendor) {
		self.vendor = vendor;
	}

	fn vendor(&self) -> Vendor {
		self.vendor
	}

	fn set_mode(&mut self, mode: Mode) {
		self.mode = mode
	}

	fn mode(&self) -> Mode {
		self.mode
	}
}
