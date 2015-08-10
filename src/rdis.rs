use super::format::Binary;
use std::error::Error;
use super::format::Mode;

use super::instruction::Instruction;
use super::operand::Operand;

use std::iter::Peekable;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Syntax {
    Intel,
    Att
}

// Prefixes of g1, g2, g3, g4, REX(x64 only) respectively
pub type Prefixes = (Option<u8>, Option<u8>, Option<u8>, Option<u8>, Option<u8>);

// Todo: add all these
#[derive(Clone, Copy)]
pub enum Mnemonic {
    Imov,
    Ixor
}

pub struct Opcode(u8, Option<u8>, Option<u8>);

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.0, self.1, self.2) {
            _ => write!(f, "Not implemented!")
        }
    }
}

pub struct Rdis {
    binary: Peekable<Box<Binary<Item=u8>>>,
    pub syntax: Syntax,
    pub mode: Mode,
}

impl Rdis {

    /*
     * Disassembles the next instruction in the input stream
     */
    pub fn disassemble(&mut self) -> Result<Instruction, &'static str> {
        let ref mut cur_length : usize = 0;
        let mode = self.mode;
        if let Ok(prefixes) = self.decode_prefixes(cur_length) {
            let opcode = self.decode_opcode(&prefixes, cur_length);
        }
        panic!()
    }

    /*
     * Decodes the prefixes from the instruction stream
     * Returns Ok(num), where num=the number of prefixes, or Err(String) with the error description
     * TODO: handle REX prefixes
     */
    fn decode_prefixes(&mut self, cur_length: &mut usize) -> Result<Prefixes, &'static str> {
        let mut prefixes = (None, None, None, None, None);

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
                    0x40...0x4F => {
                        if let Mode::B64 = self.mode {
                            prefixes.4 = Some(byte);
                            *cur_length += 1;
                        }

                        // Either we are in x64 mode and encountered a REX prefix, which must
                        // be the last prefix, or we are in a different mode and should return
                        return Ok(prefixes)
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

        Err("Maximum instruction length exceeded!")
    }

    /*
     * TODO: mandatory prefixes 66h, f2h, f3h - can we just use the precomputed prefix list?
     */
     fn decode_opcode(&mut self, prefixes: &Prefixes, cur_length: &mut usize) -> Result<Opcode, &'static str> {
        if let Some(b1) = self.binary.next() {
            *cur_length += 1;
            let opcode = match b1 {
                0x0F => {
                    if let Some(b2) = self.binary.next() {
                        match b2 {
                            0x38 | 0x3A => {
                                if let Some(b3) = self.binary.next() {
                                    Opcode(b1, Some(b2), Some(b3))
                                } else {
                                    return Err("Ran out of input stream!")
                                }
                            },
                            _ => Opcode(b1, Some(b2), None)
                        }
                    } else {
                        return Err("Ran out of input stream!")
                    }
                },
                _ => {
                    Opcode(b1, None, None)
                }
            };

            match opcode {
                Opcode(_, None, None) => *cur_length += 1,
                Opcode(_, Some(_), None) => *cur_length += 2,
                Opcode(_, Some(_), Some(_)) => *cur_length += 3,
                _ => unreachable!(),
            };

            if *cur_length > 15 {
                return Err("Maximum instruction length exceeded!")
            }

            return Ok(opcode)
        }

        Err("Ran out of input stream!")
     }
}