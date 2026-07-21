use crate::errors::ParseError;
use crate::lexer::{Mnemonic, Token};
use crate::{
    instructions::{Encode, ParseTokens},
    registers::{RegisterKind, XRegister},
};

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

impl ParseTokens for RetInstr {
    fn parse(tokens: &[Token]) -> Result<Self, ParseError> {
        match tokens {
            [Token::Mnemonic(Mnemonic::Ret)] => Ok(Self::default()),
            [
                Token::Mnemonic(Mnemonic::Ret),
                Token::Register(RegisterKind::X(register)),
            ] => Ok(Self {
                register: *register,
            }),
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for RetInstr {
    fn encode(&self) -> u32 {
        0u32 | 0b1101011001011111 << 15 | ((self.register.0 as u32) << 5)
    }
}
