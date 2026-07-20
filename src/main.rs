use std::io::Read;

pub mod errors;
pub mod registers;
pub mod instructions;
use errors::ParseError;
use registers::Register;


impl Instruction {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Instruction, ParseError> {
        let mut line_bytes = Vec::new();
        reader.read_to_end(&mut line_bytes)?;
        if let Some(null_position) = line_bytes.iter().position(|&byte| byte == b'\0') {
            line_bytes.truncate(null_position);
        }

        let utf_string = String::from_utf8(line_bytes)?;
        let mut parts = utf_string.split_ascii_whitespace();
        let instruction_name = parts.next().ok_or(ParseError::MissingInstruction)?;

        match instruction_name {
            "abs" => Self::parse_abs(&mut parts),
            _ => Err(ParseError::UnknownInstruction(instruction_name.to_owned())),
        }
    }

    fn parse_abs<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<Self, ParseError> {
        let destination = parts
            .next()
            .ok_or(ParseError::MissingOperand("destination"))?
            .parse()?;
        let source = parts
            .next()
            .ok_or(ParseError::MissingOperand("source"))?
            .parse()?;
        if parts.next().is_some() {
            return ParseError::MissingOperand(())
        }
        Ok(Self::ABS { d: destination, n: source })
    }
}

fn main() {
    println!("Hello, world!");
}
