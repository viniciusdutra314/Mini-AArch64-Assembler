use std::str::FromStr;

use crate::errors::ParseError;

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

pub struct GeneralRegister {
    number: u8,
    width: Width,
}

impl GeneralRegister {
    pub fn new(number: u8, width: Width) -> Result<Self, ParseError> {
        if number >= 31 {
            return Err(ParseError::InvalidRegisterNumber(number));
        }
        Ok(Self { number, width })
    }
}

pub enum Register {
    General(GeneralRegister),
    Special,
}

impl std::str::FromStr for Register {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (width, number) = if let Some(number) = s.strip_prefix('x') {
            (Width::X64, number)
        } else if let Some(number) = s.strip_prefix('w') {
            (Width::W32, number)
        } else {
            return Err(ParseError::InvalidRegister(s.to_owned()));
        };

        GeneralRegister::new(number.parse()?, width).map(Register::General)
    }
}
