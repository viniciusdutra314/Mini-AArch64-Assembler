use std::str::FromStr;

use crate::errors::ParseError;

pub enum Width {
    W32,
    X64,
}

#[derive(Debug, PartialEq)]
pub struct WRegister(pub u8);

impl FromStr for WRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (letter, number) = s.split_at(1);
        if letter != "w" {
            return Err(ParseError::InvalidRegister(s.to_owned()));
        };
        let number = number.parse::<u8>()?;
        if number > 30 {
            return Err(ParseError::InvalidRegisterNumber(number));
        }
        Ok(Self(number))
    }
}

#[derive(Debug, PartialEq)]
pub struct XRegister(pub u8);

impl FromStr for XRegister {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (letter, number) = s.split_at(1);
        if letter != "x" {
            return Err(ParseError::InvalidRegister(s.to_owned()));
        };
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
