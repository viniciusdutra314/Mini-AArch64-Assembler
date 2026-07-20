use crate::errors::ParseError;

pub enum Width {
    W32,
    X64,
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
