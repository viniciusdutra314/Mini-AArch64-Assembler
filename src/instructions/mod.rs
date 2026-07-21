pub mod ret;
pub mod sub;

use self::ret::RetInstr;
use self::sub::SubInstr;
use crate::errors::ParseError;
use crate::lexer::{Mnemonic, Token, tokenize};
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[enum_dispatch]
pub trait Encode {
    fn encode(&self) -> u32;
}

pub trait ParseTokens: Sized {
    fn parse(tokens: &[Token]) -> Result<Self, ParseError>;
}

#[enum_dispatch(Encode)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Sub(SubInstr),
    Ret(RetInstr),
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(source)?;

        match tokens.as_slice() {
            [Token::Mnemonic(Mnemonic::Sub | Mnemonic::Neg), ..] => {
                SubInstr::parse(&tokens).map(Into::into)
            }
            [Token::Mnemonic(Mnemonic::Ret), ..] => RetInstr::parse(&tokens).map(Into::into),
            [Token::Mnemonic(mnemonic), ..] => {
                Err(ParseError::UnknownInstruction(mnemonic.to_string()))
            }
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}
