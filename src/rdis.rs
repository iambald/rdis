use super::format::Binary;
use std::error::Error;

use super::instruction::Instruction;
use super::operand::Operand;

use std::iter::Peekable;

#[derive(Clone, Copy)]
pub enum Syntax {
    Intel,
    Att
}

// Prefixes of g1, g2, g3, g4, respectively
pub type Prefixes = (Option<u8>, Option<u8>, Option<u8>, Option<u8>);

// Todo: add all these
#[derive(Clone, Copy)]
pub enum Mnemonic {
    Imov,
    Ixor
}

// Todo: add all these
#[derive(Clone, Copy)]
pub enum Opcode {

}

pub struct Rdis {
    binary: Peekable<Box<Binary<Item=u8>>>,
    pub syntax: Syntax
}

impl Rdis {

    /*
     * Disassembles the next instruction in the input stream
     */
    pub fn disassemble(&mut self) -> Result<Instruction, &'static str> {
        let ref mut cur_length : usize = 0;
        let prefixes = self.decode_prefixes(cur_length);
        let opcode = self.decode_opcode(cur_length);
        panic!()
    }

    /*
     * Decodes the prefixes from the instruction stream
     * Returns Ok(num), where num=the number of prefixes, or Err(String) with the error description
     */
    fn decode_prefixes(&mut self, cur_length: &mut usize) -> Result<Prefixes, &'static str> {
        let mut prefixes = (None, None, None, None);

        while *cur_length < 16 {
            if let Some(byte) = self.binary.peek() {
                let byte = *byte;
                match byte {
                    0xF0 | 0xF2 | 0xF3 => {
                        prefixes.0 = Some(byte);
                        *cur_length += 1;
                    },
                    0x2E | 0x36 | 0x3E | 0x26 | 0x64 | 0x65 => {
                        prefixes.1 = Some(byte);
                        *cur_length += 1;
                    },
                    0x66 => {
                        prefixes.2 = Some(byte);
                        *cur_length += 1;
                    },
                    0x67 => {
                        prefixes.3 = Some(byte);
                        *cur_length += 1;
                    },
                    _ => {
                        return Ok(prefixes)
                    }
                }
            }
            else {
                return Err("Ran out of input stream!")
            }

            // Consume the processed byte if it was indeed a prefix
            self.binary.next();
        }

        Err("Maxmimum instruction length exceeded!")
    }

    /*
     *
     */
     fn decode_opcode(&mut self, cur_length: &mut usize) -> Result<Opcode, &'static str> {
        Err("Not implmented...")
     }
}