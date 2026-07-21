use crate::errors::ParseError;
use crate::instructions::{Encode, ParseTokens};
use crate::lexer::{Mnemonic, RegisterKind, Shift, ShiftKind, Token};
use crate::registers::{WRegister, XRegister};

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
        match tokens {
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(n)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
                rest @ ..,
            ] => Ok(Self::WVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: parse_optional_shift(rest, 31)?,
            }),
            [
                Token::Mnemonic(Mnemonic::Sub),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(n)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
                rest @ ..,
            ] => Ok(Self::XVariant {
                d: *d,
                n: *n,
                m: *m,
                shift: parse_optional_shift(rest, 63)?,
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::W(d)),
                Token::Comma,
                Token::Register(RegisterKind::W(m)),
                rest @ ..,
            ] => Ok(Self::WVariant {
                d: *d,
                n: WRegister::ZERO,
                m: *m,
                shift: parse_optional_shift(rest, 31)?,
            }),
            [
                Token::Mnemonic(Mnemonic::Neg),
                Token::Register(RegisterKind::X(d)),
                Token::Comma,
                Token::Register(RegisterKind::X(m)),
                rest @ ..,
            ] => Ok(Self::XVariant {
                d: *d,
                n: XRegister::ZERO,
                m: *m,
                shift: parse_optional_shift(rest, 63)?,
            }),
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for SubInstr {
    fn encode(&self) -> u32 {
        let (base, d, n, m, shift) = match self {
            Self::WVariant { d, n, m, shift } => (0x4b00_0000, d.0, n.0, m.0, shift),
            Self::XVariant { d, n, m, shift } => (0xcb00_0000, d.0, n.0, m.0, shift),
        };
        let (shift_kind, shift_amount) = shift
            .map(|shift| (encode_shift(shift.kind), shift.amount))
            .unwrap_or((0, 0));

        base | (shift_kind << 22)
            | (u32::from(m) << 16)
            | (u32::from(shift_amount) << 10)
            | (u32::from(n) << 5)
            | u32::from(d)
    }
}

fn parse_optional_shift(tokens: &[Token], maximum: u8) -> Result<Option<Shift>, ParseError> {
    match tokens {
        [] => Ok(None),
        [Token::Comma, Token::Shift(kind), Token::Immediate(amount)] => {
            if *kind == ShiftKind::Ror {
                return Err(ParseError::InvalidSyntax);
            }
            let amount = amount
                .strip_prefix('#')
                .ok_or(ParseError::InvalidSyntax)?
                .parse::<u8>()?;
            if amount > maximum {
                return Err(ParseError::InvalidSyntax);
            }
            Ok(Some(Shift {
                kind: *kind,
                amount,
            }))
        }
        _ => Err(ParseError::InvalidSyntax),
    }
}

fn encode_shift(kind: ShiftKind) -> u32 {
    match kind {
        ShiftKind::Lsl => 0b00,
        ShiftKind::Lsr => 0b01,
        ShiftKind::Asr => 0b10,
        ShiftKind::Ror => unreachable!("ROR is not valid for SUB (shifted register)"),
    }
}
