use crate::errors::ParseError;
use crate::{instructions::Instruction, registers::XRegister};
use std::str::SplitAsciiWhitespace;

#[derive(Debug, PartialEq)]
pub struct RetInstr {
    register: XRegister,
}

impl Default for RetInstr {
    fn default() -> Self {
        Self {
            register: XRegister(30),
        }
    }
}

impl Instruction for RetInstr {
    fn parse(text: &mut SplitAsciiWhitespace) -> Result<Self, ParseError> {
        Ok(Self {
            register: text.next().unwrap_or("x30").parse::<XRegister>()?,
        })
    }
    fn encode(&self) -> u32 {
        0u32 | 0b1101011001011111 << 15 | ((self.register.0 as u32) << 5)
    }
}
