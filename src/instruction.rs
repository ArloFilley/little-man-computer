pub type Program = Vec<Instruction>;

#[derive(Clone, Debug)]
pub enum Instruction {
    Inp,
    Out,
    Sta(AddressingMode),
    Lda(AddressingMode),
    Add(AddressingMode),
    Sub(AddressingMode),
    Brp(String),
    Bra(String),
    Brz(String),
    Hlt
}