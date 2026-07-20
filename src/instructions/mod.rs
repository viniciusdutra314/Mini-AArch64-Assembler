pub mod abs;
pub mod ret;

use super::instructions::abs::*;
use super::instructions::ret::*;

use crate::errors::ParseError;
use std::str::FromStr;
use std::str::SplitAsciiWhitespace;

pub trait Instruction: Sized {
    fn parse(text: &mut SplitAsciiWhitespace) -> Result<Self, ParseError>;
    fn encode(&self) -> u32;
}

#[derive(Debug, PartialEq)]
pub enum Instructions {
    Abs(AbsInstr),
    Ret(RetInstr),
}

impl FromStr for Instructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        let first = words.next().ok_or(ParseError::MissingInstruction)?;
        match first {
            "abs" => AbsInstr::parse(&mut words).map(|inst| Self::Abs(inst)),
            "ret" => RetInstr::parse(&mut words).map(|inst| Self::Ret(inst)),
            _ => Err(ParseError::UnknownInstruction(s.to_owned())),
        }
    }
}
