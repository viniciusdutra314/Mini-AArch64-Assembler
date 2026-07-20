use crate::errors::ParseError;
use crate::{instructions::Instruction, registers::XRegister};
use std::str::SplitAsciiWhitespace;

pub struct Ret {
    register: XRegister,
}

impl Instruction for Ret {
    fn parse(text: &mut SplitAsciiWhitespace) -> Result<Self, ParseError> {
        Ok(Self {
            register: text.next().unwrap_or("x30").parse::<XRegister>()?,
        })
    }
    fn encode(&self) -> u32 {
        0u32 | 0b1101011001011111 << 15 | ((self.register.0 as u32) << 5)
    }
}
