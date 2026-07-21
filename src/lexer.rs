use crate::errors::ParseError;
use crate::registers::{RegisterKind, ShiftKind};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mnemonic {
    Abs,
    Add,
    Neg,
    Ret,
    Sub,
}

impl Mnemonic {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Abs => "abs",
            Self::Add => "add",
            Self::Neg => "neg",
            Self::Ret => "ret",
            Self::Sub => "sub",
        }
    }
}

impl TryFrom<&str> for Mnemonic {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("abs") {
            Ok(Self::Abs)
        } else if value.eq_ignore_ascii_case("add") {
            Ok(Self::Add)
        } else if value.eq_ignore_ascii_case("neg") {
            Ok(Self::Neg)
        } else if value.eq_ignore_ascii_case("ret") {
            Ok(Self::Ret)
        } else if value.eq_ignore_ascii_case("sub") {
            Ok(Self::Sub)
        } else {
            Err(ParseError::UnknownInstruction(value.to_owned()))
        }
    }
}

impl FromStr for Mnemonic {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl AsRef<str> for Mnemonic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Mnemonic(Mnemonic),
    Comma,
    Register(RegisterKind),
    Shift(ShiftKind),
    Immediate(String),
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, ParseError> {
    let source = source
        .split_once("//")
        .map_or(source, |(code, _comment)| code)
        .replace(',', " , ");

    let mut words = source
        .split_ascii_whitespace()
        .map(|w| w.to_ascii_lowercase());
    let Some(mnemonic) = words.next() else {
        return Ok(Vec::new());
    };
    let mut tokens = vec![Token::Mnemonic(mnemonic.parse()?)];

    for word in words {
        let token = match word.as_str() {
            "," => Token::Comma,
            "lsl" => Token::Shift(ShiftKind::Lsl),
            "lsr" => Token::Shift(ShiftKind::Lsr),
            "asr" => Token::Shift(ShiftKind::Asr),
            "ror" => Token::Shift(ShiftKind::Ror),
            _ if word.starts_with('#') => Token::Immediate(word),
            _ if word.starts_with('w') => Token::Register(RegisterKind::W(word.parse()?)),
            _ if word.starts_with('x') => Token::Register(RegisterKind::X(word.parse()?)),
            _ => return Err(ParseError::InvalidSyntax),
        };
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::XRegister;

    #[test]
    fn tokenizes_common_comma_spacing() {
        let expected = vec![
            Token::Mnemonic(Mnemonic::Neg),
            Token::Register(RegisterKind::X(XRegister(0))),
            Token::Comma,
            Token::Register(RegisterKind::X(XRegister(1))),
        ];

        for source in ["neg x0,x1", "neg x0, x1", "neg x0 , x1"] {
            assert_eq!(tokenize(source).unwrap(), expected);
        }
    }

    #[test]
    fn tokenize_ignore_comments() {
        let expected = vec![
            Token::Mnemonic(Mnemonic::Abs),
            Token::Register(RegisterKind::X(XRegister(0))),
            Token::Comma,
            Token::Register(RegisterKind::X(XRegister(1))),
        ];

        assert_eq!(tokenize("abs x0,x1 //comment").unwrap(), expected);
        assert_eq!(tokenize("abs x0,x1").unwrap(), expected);
    }

    #[test]
    fn converts_mnemonics_to_and_from_strings() {
        assert_eq!("ADD".parse::<Mnemonic>().unwrap(), Mnemonic::Add);
        assert_eq!(Mnemonic::try_from("neg").unwrap(), Mnemonic::Neg);
        assert_eq!(Mnemonic::Sub.as_ref(), "sub");
        assert_eq!(Mnemonic::Ret.to_string(), "ret");
    }
}
