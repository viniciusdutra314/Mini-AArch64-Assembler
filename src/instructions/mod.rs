pub mod ret;
pub mod sub;

use self::ret::RetInstr;
use self::sub::SubInstr;
use crate::errors::ParseError;
use crate::lexer::{Mnemonic, Token, tokenize};
use crate::registers::{RegisterKind, ShiftKind};
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[enum_dispatch]
pub trait Encode {
    fn encode(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftOperand {
    pub kind: ShiftKind,
    pub amount: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(RegisterKind),
    Immediate(String),
    Shift(ShiftOperand),
}

impl TryFrom<&[Token]> for Operand {
    type Error = ParseError;

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        match tokens {
            [Token::Register(register)] => Ok(Self::Register(*register)),
            [Token::Immediate(immediate)] => Ok(Self::Immediate(immediate.clone())),
            [Token::Shift(kind), Token::Immediate(amount)] => {
                let amount = amount
                    .strip_prefix('#')
                    .ok_or(ParseError::InvalidSyntax)?
                    .parse()?;
                Ok(Self::Shift(ShiftOperand {
                    kind: *kind,
                    amount,
                }))
            }
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionStatement {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

impl TryFrom<&[Token]> for InstructionStatement {
    type Error = ParseError;

    fn try_from(tokens: &[Token]) -> Result<Self, Self::Error> {
        let Some((Token::Mnemonic(mnemonic), remaining)) = tokens.split_first() else {
            return match tokens.first() {
                None => Err(ParseError::MissingInstruction),
                Some(_) => Err(ParseError::InvalidSyntax),
            };
        };

        let operands = match remaining {
            [] => Vec::new(),
            tokens => tokens
                .split(|token| matches!(token, Token::Comma))
                .map(Operand::try_from)
                .collect::<Result<_, _>>()?,
        };

        Ok(Self {
            mnemonic: *mnemonic,
            operands,
        })
    }
}

#[enum_dispatch(Encode)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    Sub(SubInstr),
    Ret(RetInstr),
}

impl TryFrom<&InstructionStatement> for Instruction {
    type Error = ParseError;
    fn try_from(statement: &InstructionStatement) -> Result<Self, Self::Error> {
        match statement.mnemonic {
            Mnemonic::Sub | Mnemonic::Neg => SubInstr::try_from(statement).map(Into::into),
            Mnemonic::Ret => RetInstr::try_from(statement).map(Into::into),
            mnemonic => Err(ParseError::UnknownInstruction(mnemonic.to_string())),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(source)?;
        let statement = InstructionStatement::try_from(tokens.as_slice())?;
        Self::try_from(&statement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::XRegister;

    #[test]
    fn parses_tokens_into_an_instruction_statement() {
        let tokens = tokenize("sub x0,x1,x2,lsl #3").unwrap();

        assert_eq!(
            InstructionStatement::try_from(tokens.as_slice()).unwrap(),
            InstructionStatement {
                mnemonic: Mnemonic::Sub,
                operands: vec![
                    Operand::Register(RegisterKind::X(XRegister(0))),
                    Operand::Register(RegisterKind::X(XRegister(1))),
                    Operand::Register(RegisterKind::X(XRegister(2))),
                    Operand::Shift(ShiftOperand {
                        kind: ShiftKind::Lsl,
                        amount: 3,
                    }),
                ],
            }
        );
    }

    #[test]
    fn instruction_statement_requires_operand_separators() {
        for source in [
            "sub x0 x1,x2",
            "sub ,x0,x1,x2",
            "sub x0,,x1,x2",
            "sub x0,x1,x2,",
        ] {
            let tokens = tokenize(source).unwrap();

            assert!(InstructionStatement::try_from(tokens.as_slice()).is_err());
        }
    }

    #[test]
    fn parses_instruction_without_explicit_operands() {
        let tokens = tokenize("ret").unwrap();

        assert_eq!(
            InstructionStatement::try_from(tokens.as_slice()).unwrap(),
            InstructionStatement {
                mnemonic: Mnemonic::Ret,
                operands: Vec::new(),
            }
        );
    }
}
