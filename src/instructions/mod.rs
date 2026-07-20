pub mod abs;
pub mod ret;

use super::instructions::abs::*;
use super::instructions::ret::*;

use crate::errors::ParseError;
use std::str::SplitAsciiWhitespace;

pub trait Instruction: Sized {
    fn parse(text: &mut SplitAsciiWhitespace) -> Result<Self, ParseError>;
    fn encode(&self) -> u32;
}

pub enum Instructions {
    Abs(Abs),
    Ret(Ret),
}
