use crate::{errors::ParseError, instructions::Instructions};
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
    pub instructions: Vec<Instructions>,
    pub labels: HashMap<String, Symbol>,
}

impl FromStr for AsmCode {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = HashMap::<String, Symbol>::new();
        let mut instructions = Vec::<Instructions>::new();
        for line in s.lines().map(|l| l.trim()) {
            if line.is_empty() {
                continue;
            }
            if let Some(name) = line.strip_prefix(".global") {
                symbols.entry(name.trim().to_owned()).or_default().is_global = true;
                continue;
            }
            if let Some(type_def) = line.strip_prefix(".type") {
                let (name, symbol_type) = type_def
                    .trim()
                    .split_once(',')
                    .ok_or_else(|| ParseError::UnknownInstruction(line.to_owned()))?;

                let kind = match symbol_type.trim() {
                    "%function" => SymbolKind::Function,
                    _ => return Err(ParseError::UnknownInstruction(line.to_owned())),
                };

                symbols.entry(name.trim().to_owned()).or_default().kind = Some(kind);
                continue;
            }

            if let Some(name) = line.strip_suffix(":") {
                symbols
                    .entry(name.to_owned())
                    .or_default()
                    .instruction_index = Some(instructions.len());
                continue;
            }

            instructions.push(line.parse::<Instructions>()?);
        }
        Ok(Self {
            instructions,
            labels: symbols,
        })
    }
}
