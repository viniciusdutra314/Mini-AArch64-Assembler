use Mini_AArch64_Assembler::{
    assembly_code::{AsmCode, Symbol, SymbolKind::Function},
    instructions::{Encode, Instruction, ret::RetInstr},
};
use std::collections::HashMap;

#[test]
fn test_empty_function() {
    let code_str = include_str!("assets/test_empty_function.s");
    let asm_code: AsmCode = code_str.parse().unwrap();
    assert_eq!(
        asm_code,
        AsmCode {
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
fn test_instruction_dispatches_encode() {
    let instruction: Instruction = "neg x0,x1".parse().unwrap();

    assert_eq!(instruction.encode(), 0xcb01_03e0);
}
