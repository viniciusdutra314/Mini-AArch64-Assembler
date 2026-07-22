use std::str::FromStr;

use arbitrary_int::{u2, u5, u6};

use crate::errors::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterKind {
    W(WRegister),
    X(XRegister),
}

impl RegisterKind {
    pub const fn number(&self) -> u5 {
        match self {
            Self::W(register) => register.0,
            Self::X(register) => register.0,
        }
    }

    pub const fn width(&self) -> Width {
        match self {
            Self::W(_) => Width::W32,
            Self::X(_) => Width::X64,
        }
    }

    pub const fn zero(&self) -> Self {
        match self {
            Self::W(_) => Self::W(WRegister::ZERO),
            Self::X(_) => Self::X(XRegister::ZERO),
        }
    }

    pub fn common_width(registers: &[Self]) -> Option<Width> {
        let (first, remaining) = registers.split_first()?;
        let width = first.width();

        remaining
            .iter()
            .all(|register| register.width() == width)
            .then_some(width)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WRegister(pub u5);

impl WRegister {
    pub const ZERO: Self = Self(u5::new(31));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XRegister(pub u5);

impl XRegister {
    pub const ZERO: Self = Self(u5::new(31));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftKind {
    Lsl,
    Lsr,
    Asr,
    Ror,
}

impl ShiftKind {
    pub const fn bits(self) -> u2 {
        u2::new(match self {
            Self::Lsl => 0b00,
            Self::Lsr => 0b01,
            Self::Asr => 0b10,
            Self::Ror => 0b11,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shift<const MAX_AMOUNT: u8> {
    kind: ShiftKind,
    amount: u6,
}

impl<const MAX_AMOUNT: u8> Shift<MAX_AMOUNT> {
    pub fn new(kind: ShiftKind, amount: u8) -> Result<Self, ParseError> {
        if amount > MAX_AMOUNT {
            return Err(ParseError::InvalidSyntax);
        }

        let amount = u6::try_new(amount).map_err(|_| ParseError::InvalidSyntax)?;
        Ok(Self { kind, amount })
    }

    pub fn from_immediate(kind: ShiftKind, immediate: &str) -> Result<Self, ParseError> {
        let amount = immediate
            .strip_prefix('#')
            .ok_or(ParseError::InvalidSyntax)?
            .parse()?;

        Self::new(kind, amount)
    }

    pub const fn kind(self) -> ShiftKind {
        self.kind
    }

    pub const fn amount(self) -> u6 {
        self.amount
    }

    pub const fn encoded_bits(self) -> u32 {
        ((self.kind.bits().value() as u32) << 22) | ((self.amount.value() as u32) << 10)
    }
}

pub type Shift32 = Shift<31>;
pub type Shift64 = Shift<63>;
pub type ExtendedRegisterShift = Shift<4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Width {
    W32,
    X64,
}

impl Width {
    pub const fn bits(self) -> u8 {
        match self {
            Self::W32 => 32,
            Self::X64 => 64,
        }
    }
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
        Ok(Self(u5::new(number)))
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
        Ok(Self(u5::new(number)))
    }
}
