#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
pub enum Instruction {
    HLT,            // 000 Halt
    ADDIMM(i16),    // 0xx Add Immediate
    ADD(i16),       // 1xx Add
    SUB(i16),       // 2xx Subtract
    STA(i16),       // 3xx Store
                    // 400 No-op
    SUBIMM(i16),    // 4xx Subtract Immediate
    LDA(i16),       // 5xx Load
    BRZ(i16),       // 6xx Branch Negative
    BRA(i16),       // 7xx Branch Always
    BRP(i16),       // 8xx Branch Positive
    INP,            // 901 Input
    OUT,            // 902 Output
}

#[derive(Debug)]
pub enum InstructionError {
    NoOp,
    InvalidOpcode,
}

impl TryFrom<i16> for Instruction {
    type Error = InstructionError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0         => Ok(Instruction::HLT),
            001..=099 => Ok(Instruction::ADDIMM(value)),
            100..=199 => Ok(Instruction::ADD(value-100)),
            200..=299 => Ok(Instruction::SUB(value-200)),
            300..=399 => Ok(Instruction::STA(value-300)),
            400       => Err(InstructionError::NoOp),
            401..=499 => Ok(Instruction::SUBIMM(value-400)),
            500..=599 => Ok(Instruction::LDA(value-500)),
            600..=699 => Ok(Instruction::BRZ(value-600)),
            700..=799 => Ok(Instruction::BRA(value-700)),
            800..=899 => Ok(Instruction::BRP(value-800)),
            901       => Ok(Instruction::INP),
            902       => Ok(Instruction::OUT),
            903..=999 => Err(InstructionError::NoOp),
            _         => Err(InstructionError::InvalidOpcode),
        }
    }
}