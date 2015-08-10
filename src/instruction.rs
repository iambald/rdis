use super::rdis::Rdis;
use super::operand::Operand;
use super::rdis::Mnemonic;
use super::rdis::Syntax;
use super::rdis::Prefixes;
use super::rdis::Opcode;
use std::fmt;

pub struct Instruction<'a> {
    length: u64,
    offset: u64,
    operands: Vec<Operand<'a>>,
    bytes: &'a [u8],
    prefixes: Prefixes,
    opcode: Opcode,
    modrm: u8,	//TODO: replace with enum/struct with fields
    sib: u8,	//TODO: replace with enum/struct with fields
    displacement: &'a [u8],	//TODO: ...
    immediate: &'a [u8],

    pub syntax: Syntax
}

impl<'a> Instruction<'a> {

    pub fn operands(&self) -> &Vec<Operand> {
        &self.operands
    }

    pub fn operand(&self, n: usize) -> &Operand {
        &(self.operands[n])
    }

    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    // Todo: implement....
    pub fn mnemonic(&self) -> Mnemonic {
        Mnemonic::Imov
    }
}

impl<'a> fmt::Display for Instruction<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for byte in self.bytes {
			write!(f, "{} ", byte);
		}
		write!(f, "\n")
	}
}