use std::collections::HashMap;
use Mini_AArch64_Assembler::{
    assembly_code::{AsmCode, Symbol, SymbolKind::Function},
    instructions::{ret::RetInstr, Instructions},
};

#[test]
fn test_empty_function() {
    let code_str = include_str!("assets/test_empty_function.s");
    let asm_code: AsmCode = code_str.parse().unwrap();
    assert_eq!(
        asm_code,
        AsmCode {
            instructions: vec![Instructions::Ret(RetInstr::default())],
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
