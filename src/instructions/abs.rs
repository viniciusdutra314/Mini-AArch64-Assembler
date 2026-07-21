use crate::errors::ParseError;
use crate::instructions::{Encode, ParseTokens};
use crate::lexer::{Mnemonic, RegisterKind, Token};
use crate::registers::{WRegister, XRegister};

#[derive(Debug, PartialEq)]
pub enum AbsInstr {
    Wvariant { d: WRegister, n: WRegister },
    Xvariant { d: XRegister, n: XRegister },
}

impl ParseTokens for AbsInstr {
    fn parse(tokens: &[Token]) -> Result<Self, ParseError> {
        match tokens {
            [
                Token::Mnemonic(Mnemonic::Abs),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(n)),
            ] => Ok(Self::Wvariant { d: *d, n: *n }),
            [
                Token::Mnemonic(Mnemonic::Abs),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(n)),
            ] => Ok(Self::Xvariant { d: *d, n: *n }),
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for AbsInstr {
    fn encode(&self) -> u32 {
        let (d, n, sf) = match self {
            Self::Wvariant { d, n } => (d.0, n.0, 0),
            Self::Xvariant { d, n } => (d.0, n.0, 1),
        };
        (sf << 31) | (0b101101011000000001000 << 10) | (u32::from(n) << 5) | u32::from(d)
    }
}
