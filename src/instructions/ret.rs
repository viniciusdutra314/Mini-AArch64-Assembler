use crate::errors::ParseError;
use crate::{
    instructions::{Encode, InstructionStatement, Operand},
    lexer::Mnemonic,
    registers::{RegisterKind, XRegister},
};

#[derive(Debug, PartialEq)]
pub struct RetInstr {
    register: XRegister,
}

impl Default for RetInstr {
    fn default() -> Self {
        Self {
            register: XRegister(30),
        }
    }
}

impl TryFrom<&InstructionStatement> for RetInstr {
    type Error = ParseError;

    fn try_from(statement: &InstructionStatement) -> Result<Self, Self::Error> {
        match (statement.mnemonic, statement.operands.as_slice()) {
            (Mnemonic::Ret, []) => Ok(Self::default()),
            (Mnemonic::Ret, [Operand::Register(RegisterKind::X(register))]) => Ok(Self {
                register: *register,
            }),
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for RetInstr {
    fn encode(&self) -> u32 {
        0u32 | 0b1101011001011111 << 15 | ((self.register.0 as u32) << 5)
    }
}
