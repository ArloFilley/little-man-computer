use std::collections::HashMap;
use crate::instruction::{Instruction, InstructionError};

#[derive(Debug)]
pub enum ParseError {
    InvalidString,
    TooManyInstructions(usize),
    InvalidInstruction(String, InstructionError),
    UnresolvedLabel(String),
}

pub fn parse(input: &str) -> Result<[i16; 100], ParseError> {
    // Normalise lines: strip comments, trim, strip # and @ prefixes from operands
    let lines: Vec<String> = input
        .lines()
        .filter_map(|l| {
            let l = l.split("//").next().unwrap().trim();
            if l.is_empty() { None } else { Some(l.to_string()) }
        })
        .collect();

    // --- Pass 1: assign an address to every non-empty line and collect labels ---
    let mut labels: HashMap<String, i16> = HashMap::new();
    let mut address_lines: Vec<String> = Vec::new(); // one entry per RAM slot

    for line in &lines {
        let mut tokens = line.split_whitespace().peekable();
        let first = tokens.peek().unwrap();

        // A label is any token that isn't a known mnemonic
        let is_mnemonic = matches!(
            *first,
            "HLT" | "ADD" | "ADDIMM" | "SUB" | "SUBIMM" | "STA" | "LDA" | "BRZ" | "BRA" | "BRP" | "INP" | "OUT" | "DAT"
        );

        let label = if !is_mnemonic {
            Some(tokens.next().unwrap().to_string())
        } else {
            None
        };

        let addr = address_lines.len();

        if addr >= 100 {
            return Err(ParseError::TooManyInstructions(addr));
        }

        if let Some(lbl) = label {
            labels.insert(lbl, addr as i16);
        }

        // Reconstruct the rest of the line (mnemonic + operand) for pass 2
        let rest: String = tokens.collect::<Vec<_>>().join(" ");
        address_lines.push(rest);
    }

    // --- Pass 2: encode each line, resolving labels in operands ---
    let mut ram = [0i16; 100];

    for (idx, line) in address_lines.iter().enumerate() {
        let mut tokens = line.split_whitespace();
        let mnemonic = match tokens.next() {
            Some(m) => m,
            None => continue,
        };

        if mnemonic == "DAT" {
            ram[idx] = tokens
                .next()
                .map(|s| s.trim_start_matches(['#', '@']))
                .map(|s| s.parse::<i16>().unwrap_or(0))
                .unwrap_or(0);
            continue;
        }

        // Resolve operand: either a label, or a bare/prefixed number
        let operand = tokens.next().map(|s| s.trim_start_matches(['#', '@']));
        let resolved = operand
            .map(|s| {
                if let Some(&addr) = labels.get(s) {
                    Ok(addr)
                } else {
                    s.parse::<i16>()
                        .map_err(|_| ParseError::UnresolvedLabel(s.to_string()))
                }
            })
            .transpose()?;

        let operand = resolved.unwrap_or(0);

        let encoded: i16 = match mnemonic {
            "HLT"    => 0,
            "ADDIMM" => operand.clamp(1, 99),
            "ADD"    => 100 + operand,
            "SUB"    => 200 + operand,
            "STA"    => 300 + operand,
            "SUBIMM" => 400 + operand.clamp(1, 99),
            "LDA"    => 500 + operand,
            "BRZ"    => 600 + operand,
            "BRA"    => 700 + operand,
            "BRP"    => 800 + operand,
            "INP"    => 901,
            "OUT"    => 902,
            other => return Err(ParseError::InvalidInstruction(
                other.to_string(),
                InstructionError::NoOp,
            )),
        };

        // Validate via TryFrom before storing
        Instruction::try_from(encoded)
            .map_err(|e| ParseError::InvalidInstruction(mnemonic.to_string(), e))?;

        ram[idx] = encoded;
    }

    Ok(ram)
}