use super::rdis::Rdis;
use super::instruction::Instruction;

pub struct Operand<'a> {
    size: usize,
    bytes: &'a [u8]
}

impl<'a> Operand<'a> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}