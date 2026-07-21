use std::str::FromStr;

use crate::errors::ParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterKind {
    W(WRegister),
    X(XRegister),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WRegister(pub u8);

impl WRegister {
    pub const ZERO: Self = Self(31);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XRegister(pub u8);

impl XRegister {
    pub const ZERO: Self = Self(31);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftKind {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl ShiftKind {
    pub const fn bits(self) -> u32 {
        match self {
            Self::Lsl => 0b00,
            Self::Lsr => 0b01,
            Self::Asr => 0b10,
            Self::Ror => 0b11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shift {
    pub kind: ShiftKind,
    pub amount: u8,
}

impl Shift {
    pub fn from_immediate(kind: ShiftKind, immediate: &str) -> Result<Self, ParseError> {
        let amount = immediate
            .strip_prefix('#')
            .ok_or(ParseError::InvalidSyntax)?
            .parse()?;

        Ok(Self { kind, amount })
    }

    pub const fn encoded_bits(self) -> u32 {
        (self.kind.bits() << 22) | ((self.amount as u32) << 10)
    }
}

pub enum Width {
    W32,
    X64,
}

impl FromStr for WRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "wzr" {
            return Ok(Self::ZERO);
        }
        let number = s
            .strip_prefix('w')
            .ok_or_else(|| ParseError::InvalidRegister(s.to_owned()))?;
        let number = number.parse::<u8>()?;
        if number > 30 {
            return Err(ParseError::InvalidRegisterNumber(number));
        }
        Ok(Self(number))
    }
}

impl FromStr for XRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "xzr" {
            return Ok(Self::ZERO);
        }
        let number = s
            .strip_prefix('x')
            .ok_or_else(|| ParseError::InvalidRegister(s.to_owned()))?;
        let number = number.parse::<u8>()?;
        if number > 30 {
            return Err(ParseError::InvalidRegisterNumber(number));
        }
        Ok(Self(number))
    }
}
