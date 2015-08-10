pub use self::raw::Raw;
pub use self::elf::Elf;
// pub use self::pe::Pe;

mod elf;
mod raw;
mod pe;

#[derive(Clone, Copy)]
pub enum Vendor {
	AMD,
	Intel,
	Generic
}

#[derive(Clone, Copy)]
pub enum Mode {
	B16,
	B32,
	B64
}

pub trait Binary : Iterator<Item=u8> {
	fn set_vendor(&mut self, vendor: Vendor);
	fn vendor(&self) -> Vendor;

	fn set_mode(&mut self, mode: Mode);
	fn mode(&self) -> Mode;
}
