use crate::errors::ParseError;
use crate::instructions::{Encode, ParseTokens};
use crate::lexer::{Mnemonic, Token};
use crate::registers::{RegisterKind, Shift, ShiftKind, WRegister, XRegister};

#[derive(Debug, PartialEq)]
pub enum SubInstr {
    WVariant {
        d: WRegister,
        n: WRegister,
        m: WRegister,
        shift: Option<Shift>,
    },
    XVariant {
        d: XRegister,
        n: XRegister,
        m: XRegister,
        shift: Option<Shift>,
    },
}

impl ParseTokens for SubInstr {
    fn parse(tokens: &[Token]) -> Result<Self, ParseError> {
        let instruction = match tokens {
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(n)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
            ] => Ok(Self::WVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: None,
            }),
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(n)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => Ok(Self::WVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: Some(Shift::from_immediate(*kind, amount)?),
            }),
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(n)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
            ] => Ok(Self::XVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: None,
            }),
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(n)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => Ok(Self::XVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: Some(Shift::from_immediate(*kind, amount)?),
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
            ] => Ok(Self::WVariant {
                d: *d,
                n: WRegister::ZERO,
                m: *m,
                shift: None,
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => Ok(Self::WVariant {
                d: *d,
                n: WRegister::ZERO,
                m: *m,
                shift: Some(Shift::from_immediate(*kind, amount)?),
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
            ] => Ok(Self::XVariant {
                d: *d,
                n: XRegister::ZERO,
                m: *m,
                shift: None,
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
                Token::Comma,
                Token::Shift(kind),
                Token::Immediate(amount),
            ] => Ok(Self::XVariant {
                d: *d,
                n: XRegister::ZERO,
                m: *m,
                shift: Some(Shift::from_immediate(*kind, amount)?),
            }),
            _ => Err(ParseError::InvalidSyntax),
        }?;

        let (shift, maximum) = match &instruction {
            Self::WVariant { shift, .. } => (shift, 31),
            Self::XVariant { shift, .. } => (shift, 63),
        };

        if shift.is_some_and(|shift| shift.kind == ShiftKind::Ror || shift.amount > maximum) {
            return Err(ParseError::InvalidSyntax);
        }

        Ok(instruction)
    }
}

impl Encode for SubInstr {
    fn encode(&self) -> u32 {
        let (base, d, n, m, shift) = match self {
            Self::WVariant { d, n, m, shift } => (0x4b00_0000, d.0, n.0, m.0, shift),
            Self::XVariant { d, n, m, shift } => (0xcb00_0000, d.0, n.0, m.0, shift),
        };
        let shift_bits = shift.map_or(0, |shift| shift.encoded_bits());

        base | shift_bits | (u32::from(m) << 16) | (u32::from(n) << 5) | u32::from(d)
    }
}
