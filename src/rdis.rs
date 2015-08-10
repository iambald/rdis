use super::format::Binary;
use std::error::Error;

#[derive(Clone, Copy)]
pub enum Syntax {
    Intel,
    Att
}

pub struct Operand {
    size: usize,
}

pub struct Instruction {
    length: u64,
    offset: u64,
    operands: Box<Vec<Operand>>,
    syntax: Syntax
}

pub struct Rdis {
    binary: Box<Binary<Item=u8>>,
    syntax: Syntax
}

impl Rdis {

    /*
     * Disassembles the next instruction in the input stream
     */
    pub fn disassemble(&mut self) -> Result<Instruction, String> {
        panic!()
    }

    pub fn set_syntax(&mut self, syntax: Syntax) {
        self.syntax = syntax
    }

    pub fn get_syntax(&self) -> Syntax {
        self.syntax
    }
}
