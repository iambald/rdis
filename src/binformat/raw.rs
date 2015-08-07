use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::Read;

use super::BinFormat;
use super::Vendor;

// use std::vec::IntoIter;


pub struct Raw<'a> {
	iterator: &'a mut Iterator<Item=&'a u8>,
	vendor: Vendor,
	pc: usize,
}

impl<'a> Raw<'a> {
	pub fn from_path(path: &Path, vendor: Vendor, pc: usize) -> Result<Raw<'a>, Error> {
		match File::open(&path) {
			Err(why) => Err(why),
			Ok(file) => Raw::from_file(&mut file, vendor, pc),
		};
	}

	pub fn from_file(file: &mut File, vendor: Vendor, pc: usize) -> Result<Raw<'a>, Error> {
		let ref mut vec = Vec::new();
		match file.read_to_end(vec) {
			Err(why) => Err(why),
			Ok(len) => Ok(Raw::from_bytes(vec, vendor, pc))
		}
	}

	pub fn from_bytes(bytes: &'a mut Vec<u8>, vendor: Vendor, pc: usize) -> Raw<'a> {
		Raw::from_iter(&mut bytes.iter(), vendor, pc)
	}

	pub fn from_iter<T: Iterator<Item=&'a u8> + 'a>(iter: &'a mut T, vendor: Vendor, pc: usize) -> Raw<'a> {
		Raw {
			iterator: iter,
			vendor: vendor,
			pc: pc,
		}
	}
}




// impl Raw {
// 	pub fn new(size: u64, iterator: Option<Box<Iterator<Item = u8>>>, vendor: Vendor, pc: u64) -> Raw {
// 		Raw {
// 			size: size,
// 			iterator: iterator,
// 			vendor: vendor,
// 			pc: pc
// 		}
// 	}
// }

// impl<'a> BinFormat<'a> for Raw {
// 	fn next_byte(&mut self) -> Option<u8> {
// 		match self.iterator {
// 			Some(ref mut f) => (*f).next(),
// 			None => None
// 		}
// 	}

// 	// fn set_iterator<T: Iterator<Item = u8> + 'a>(&mut self, it: &'a mut T) {
// 	// 	self.iterator = Some(it)
// 	// }

// 	fn clear_iterator(&mut self) {
// 		self.iterator = None
// 	}

// 	fn input_skip(&'a mut self, n: usize) {
// 		// let ref mut it = self.iterator;
// 		if let Some(ref it) = self.iterator {
// 			self.set_iterator(&mut it.skip(n));// = Some(it.skip(n));
// 			// self.iterator = Some(Box::new((*f).skip(n)));
// 		}
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