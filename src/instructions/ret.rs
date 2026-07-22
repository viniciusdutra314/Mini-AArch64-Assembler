use crate::errors::ParseError;
use crate::{
    instructions::{Encode, InstructionStatement, Operand},
    lexer::Mnemonic,
    registers::{RegisterKind, XRegister},
};
use arbitrary_int::u5;

#[derive(Debug, PartialEq)]
pub struct RetInstr {
    register: XRegister,
}

impl Default for RetInstr {
    fn default() -> Self {
        Self {
            register: XRegister(u5::new(30)),
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
        0u32 | 0b1101011001011111 << 15 | (u32::from(self.register.0.value()) << 5)
    }
}
