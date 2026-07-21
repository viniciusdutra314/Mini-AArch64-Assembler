use crate::{
    errors::ParseError::{self, InvalidSyntax},
    instructions::Instruction,
};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum SymbolKind {
    Function,
}

#[derive(Debug, Default, PartialEq)]
pub struct Symbol {
    pub is_global: bool,
    pub kind: Option<SymbolKind>,
    pub instruction_index: Option<usize>,
}

#[derive(Debug, PartialEq)]
pub struct AsmCode {
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, Symbol>,
}

impl FromStr for AsmCode {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = HashMap::<String, Symbol>::new();
        let mut instructions = Vec::<Instruction>::new();
        for line in s.lines().map(str::trim).filter(|l| !l.is_empty()) {
            if line.starts_with('.') {
                let directive_fields: Vec<_> = line
                    .split(|c: char| c.is_ascii_whitespace() || c == ',')
                    .filter(|field| !field.is_empty())
                    .collect();
                match directive_fields.as_slice() {
                    [".global", name] => {
                        symbols.entry((*name).to_owned()).or_default().is_global = true;
                    }
                    [".type", name, "%function"] => {
                        symbols.entry((*name).to_owned()).or_default().kind =
                            Some(SymbolKind::Function);
                    }
                    _ => return Err(InvalidSyntax),
                }
            } else if let Some(label) = line.strip_suffix(":") {
                symbols
                    .entry(label.to_owned())
                    .or_default()
                    .instruction_index = Some(instructions.len());
            } else {
                instructions.push(line.parse::<Instruction>()?);
            }
        }
        Ok(Self {
            instructions,
            labels: symbols,
        })
    }
}
