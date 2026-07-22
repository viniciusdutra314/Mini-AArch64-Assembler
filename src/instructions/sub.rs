use crate::errors::ParseError;
use crate::instructions::{Encode, InstructionStatement, Operand, ShiftOperand};
use crate::lexer::Mnemonic;
use crate::registers::{RegisterKind, Shift, Shift32, Shift64, ShiftKind, WRegister, XRegister};

#[derive(Debug, PartialEq)]
pub struct SubOperands<R, const MAX_SHIFT: u8> {
    pub d: R,
    pub n: R,
    pub m: R,
    pub shift: Option<Shift<MAX_SHIFT>>,
}

#[derive(Debug, PartialEq)]
pub enum SubInstr {
    W(SubOperands<WRegister, 31>),
    X(SubOperands<XRegister, 63>),
}

impl TryFrom<&InstructionStatement> for SubInstr {
    type Error = ParseError;
    fn try_from(statement: &InstructionStatement) -> Result<Self, Self::Error> {
        let (d, n, m, shift) = match (statement.mnemonic, statement.operands.as_slice()) {
            (
                Mnemonic::Sub,
                [
                    Operand::Register(d),
                    Operand::Register(n),
                    Operand::Register(m),
                ],
            ) => (*d, *n, *m, None),
            (
                Mnemonic::Sub,
                [
                    Operand::Register(d),
                    Operand::Register(n),
                    Operand::Register(m),
                    Operand::Shift(shift),
                ],
            ) => (*d, *n, *m, Some(*shift)),
            (Mnemonic::Neg, [Operand::Register(d), Operand::Register(m)]) => {
                (*d, d.zero(), *m, None)
            }
            (
                Mnemonic::Neg,
                [
                    Operand::Register(d),
                    Operand::Register(m),
                    Operand::Shift(shift),
                ],
            ) => (*d, d.zero(), *m, Some(*shift)),
            _ => return Err(ParseError::InvalidSyntax),
        };

        if shift.is_some_and(|shift| shift.kind == ShiftKind::Ror) {
            return Err(ParseError::InvalidSyntax);
        }

        match (d, n, m) {
            (RegisterKind::W(d), RegisterKind::W(n), RegisterKind::W(m)) => {
                let shift = shift
                    .map(|ShiftOperand { kind, amount }| Shift32::new(kind, amount))
                    .transpose()?;
                Ok(Self::W(SubOperands { d, n, m, shift }))
            }
            (RegisterKind::X(d), RegisterKind::X(n), RegisterKind::X(m)) => {
                let shift = shift
                    .map(|ShiftOperand { kind, amount }| Shift64::new(kind, amount))
                    .transpose()?;
                Ok(Self::X(SubOperands { d, n, m, shift }))
            }
            _ => Err(ParseError::InvalidSyntax),
        }
    }
}

impl Encode for SubInstr {
    fn encode(&self) -> u32 {
        let (base, d, n, m, shift_bits) = match self {
            Self::W(instruction) => (
                0x4b00_0000,
                instruction.d.0,
                instruction.n.0,
                instruction.m.0,
                instruction.shift.map_or(0, Shift32::encoded_bits),
            ),
            Self::X(instruction) => (
                0xcb00_0000,
                instruction.d.0,
                instruction.n.0,
                instruction.m.0,
                instruction.shift.map_or(0, Shift64::encoded_bits),
            ),
        };

        base | shift_bits | (u32::from(m) << 16) | (u32::from(n) << 5) | u32::from(d)
    }
}
