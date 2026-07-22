use Mini_AArch64_Assembler::{
    assembly_code::{Code, Symbol, SymbolKind::Function},
    instructions::{
        Encode, Instruction,
        ret::RetInstr,
        sub::{SubInstr, SubOperands},
    },
    registers::{Shift64, ShiftKind, XRegister},
};
use arbitrary_int::u5;
use std::collections::HashMap;

#[test]
fn test_empty_function() {
    let code_str = include_str!("assets/test_empty_function.s");
    let asm_code: Code = code_str.parse().unwrap();
    assert_eq!(
        asm_code,
        Code {
            instructions: vec![Instruction::Ret(RetInstr::default())],
            labels: HashMap::from([(
                "noop".to_owned(),
                Symbol {
                    is_global: true,
                    kind: Some(Function),
                    instruction_index: Some(0),
                },
            )]),
        }
    );
}

#[test]
fn test_negate_value_and_shift() {
    let code_str = include_str!("assets/test_neg_function.s");
    let asm_code: Code = code_str.parse().unwrap();
    assert_eq!(
        asm_code,
        Code {
            instructions: vec![
                Instruction::Sub(SubInstr::X(SubOperands {
                    d: XRegister(u5::new(0)),
                    n: XRegister::ZERO,
                    m: XRegister(u5::new(0)),
                    shift: Some(Shift64::new(ShiftKind::Lsl, 1).unwrap()),
                })),
                Instruction::Ret(RetInstr::default())
            ],
            labels: HashMap::from([(
                "negate_val".to_owned(),
                Symbol {
                    is_global: true,
                    kind: Some(Function),
                    instruction_index: Some(0),
                },
            )]),
        }
    );
}

#[test]
fn test_neg_alias_lowers_to_sub() {
    let alias: Instruction = "neg x0,x1".parse().unwrap();
    let canonical: Instruction = "sub x0,xzr,x1".parse().unwrap();
    assert_eq!(alias, canonical);
}

#[test]
fn test_shifted_neg_alias_lowers_to_sub() {
    let alias: Instruction = "neg w2, w3, asr #7".parse().unwrap();
    let canonical: Instruction = "sub w2,wzr,w3,asr #7".parse().unwrap();
    assert_eq!(alias, canonical);
}

#[test]
fn test_rejects_shift_amount_outside_typed_limit() {
    assert!("sub w0,w1,w2,lsl #32".parse::<Instruction>().is_err());
    assert!("sub x0,x1,x2,lsl #32".parse::<Instruction>().is_ok());
    assert!("sub x0,x1,x2,lsl #64".parse::<Instruction>().is_err());
}

#[test]
fn test_instruction_dispatches_encode() {
    let instruction: Instruction = "neg x0,x1".parse().unwrap();

    assert_eq!(instruction.encode(), 0xcb01_03e0);
}
