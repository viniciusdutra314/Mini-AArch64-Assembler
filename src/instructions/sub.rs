use crate::errors::ParseError;
use crate::instructions::{Encode, ParseTokens};
use crate::lexer::{Mnemonic, Token};
use crate::registers::{RegisterKind, Shift, Shift32, Shift64, ShiftKind, WRegister, XRegister};

#[derive(Debug, PartialEq)]
pub struct SubOperands<R, const MAX_SHIFT: u8> {
    pub d: R,
    pub n: R,
    pub m: R,
    pub shift: Option<Shift<MAX_SHIFT>>,
}

#[derive(Debug, PartialEq)]
pub enum SubInstr {
    W(SubOperands<WRegister, 31>),
    X(SubOperands<XRegister, 63>),
}

impl ParseTokens for SubInstr {
    fn parse(tokens: &[Token]) -> Result<Self, ParseError> {
        let (d, n, m, shift) = match tokens {
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(d),
                Token::Comma,
                Token::Register(n),
                Token::Comma,
                Token::Register(m),
            ] => (*d, *n, *m, None),
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(d),
                Token::Comma,
                Token::Register(n),
                Token::Comma,
                Token::Register(m),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => (*d, *n, *m, Some((*kind, amount.as_str()))),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(d),
                Token::Comma,
                Token::Register(m),
            ] => (*d, d.zero(), *m, None),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(d),
                Token::Comma,
                Token::Register(m),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => (*d, d.zero(), *m, Some((*kind, amount.as_str()))),
            _ => return Err(ParseError::InvalidSyntax),
        };

        if shift.is_some_and(|(kind, _)| kind == ShiftKind::Ror) {
            return Err(ParseError::InvalidSyntax);
        }

        match (d, n, m) {
            (RegisterKind::W(d), RegisterKind::W(n), RegisterKind::W(m)) => {
                let shift = shift
                    .map(|(kind, amount)| Shift32::from_immediate(kind, amount))
                    .transpose()?;
                Ok(Self::W(SubOperands { d, n, m, shift }))
            }
            (RegisterKind::X(d), RegisterKind::X(n), RegisterKind::X(m)) => {
                let shift = shift
                    .map(|(kind, amount)| Shift64::from_immediate(kind, amount))
                    .transpose()?;
                Ok(Self::X(SubOperands { d, n, m, shift }))
            }
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for SubInstr {
    fn encode(&self) -> u32 {
        let (base, d, n, m, shift_bits) = match self {
            Self::W(instruction) => (
                0x4b00_0000,
                instruction.d.0,
                instruction.n.0,
                instruction.m.0,
                instruction.shift.map_or(0, Shift32::encoded_bits),
            ),
            Self::X(instruction) => (
                0xcb00_0000,
                instruction.d.0,
                instruction.n.0,
                instruction.m.0,
                instruction.shift.map_or(0, Shift64::encoded_bits),
            ),
        };

        base | shift_bits | (u32::from(m) << 16) | (u32::from(n) << 5) | u32::from(d)
    }
}
