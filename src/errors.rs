use std::{error::Error, fmt, num::ParseIntError, string::FromUtf8Error};

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    InvalidUtf8(FromUtf8Error),
    InvalidNumber(ParseIntError),
    InvalidRegister(String),
    InvalidRegisterNumber(u8),
    MissingInstruction,
    MissingOperand(&'static str),
    UnknownInstruction(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidUtf8(error) => write!(f, "invalid UTF-8: {error}"),
            Self::InvalidNumber(error) => write!(f, "invalid register number: {error}"),
            Self::InvalidRegister(register) => write!(f, "invalid register: {register}"),
            Self::InvalidRegisterNumber(number) => {
                write!(f, "register number must be between 0 and 30, got {number}")
            }
            Self::MissingInstruction => write!(f, "missing instruction"),
            Self::MissingOperand(operand) => write!(f, "missing {operand} operand"),
            Self::UnknownInstruction(instruction) => {
                write!(f, "unknown instruction: {instruction}")
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::InvalidUtf8(error) => Some(error),
            Self::InvalidNumber(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(error: FromUtf8Error) -> Self {
        Self::InvalidUtf8(error)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(error: ParseIntError) -> Self {
        Self::InvalidNumber(error)
    }
}
