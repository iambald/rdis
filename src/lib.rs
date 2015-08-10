mod rdis;
mod operand;
mod instruction;
mod format;

use format::Binary;
use format::Vendor;
use format::Raw;
use format::Mode;
use rdis::Rdis;
use operand::Operand;
use instruction::Instruction;

#[test]
fn it_works() {
    let mut vec = 0..10;
	let mut elf = Raw::new(Box::new(vec), Vendor::Generic, 0, Mode::B32);
    // for i in 0..10 {
    let mut elf = elf.skip(5);
        match elf.next() {
            None => println!("Error!"),
            Some(n) => assert_eq!(5, n)
        }
    // }

}
