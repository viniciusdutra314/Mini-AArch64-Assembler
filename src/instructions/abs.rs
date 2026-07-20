use crate::errors::ParseError;
use crate::instructions::Instruction;
use crate::registers::{WRegister, XRegister};
use std::str::SplitAsciiWhitespace;

pub enum Abs {
    Wvariant { d: WRegister, n: WRegister },
    Xvariant { d: XRegister, n: XRegister },
}

impl Instruction for Abs {
    fn parse(text: &mut SplitAsciiWhitespace) -> Result<Self, ParseError> {
        let d = text
            .next()
            .ok_or(ParseError::MissingOperand("destination"))?;
        let n = text.next().ok_or(ParseError::MissingOperand("source"))?;
        if text.next().is_some() {
            return Err(ParseError::TooManyOperands);
        }

        match d.as_bytes().first() {
            Some(b'w') => Ok(Self::Wvariant {
                d: d.parse()?,
                n: n.parse()?,
            }),
            Some(b'x') => Ok(Self::Xvariant {
                d: d.parse()?,
                n: n.parse()?,
            }),
            _ => Err(ParseError::InvalidRegister(d.to_owned())),
        }
    }

    fn encode(&self) -> u32 {
        let (d, n, sf) = match self {
            Self::Wvariant { d, n } => (d.0, n.0, 0),
            Self::Xvariant { d, n } => (d.0, n.0, 1),
        };
        (sf << 31) | (0b101101011000000001000 << 10) | (u32::from(n) << 5) | u32::from(d)
    }
}
