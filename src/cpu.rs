use std::{fmt, io::Write};

use crate::instruction::{Instruction, InstructionError};
pub type Memory = [i16; 100];

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct CPU {
    pc: usize,
    acc: i16,
    cir: i16,
    mar: usize,
    mdr: i16, 
    mem: Memory,
    out: i16,
    inp: i16
}

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {} | ", self.pc)?;
        write!(f, "acc: {} | ", self.acc)?;
        write!(f, "cir: {} | ", self.cir)?;
        write!(f, "mar: {} | ", self.mar)?;
        write!(f, "mdr: {} | ", self.mdr)?;
        write!(f, "out: {} | ", self.out)?;
        write!(f, "inp: {}", self.inp)
    }
}

impl CPU {
    pub fn new() -> Self {
        Self { 
            pc: 0, 
            acc: 0, 
            cir: 0, 
            mar: 0, 
            mdr: 0,
            mem: [0; 100],
            out: 0,
            inp: 0
        }
    }

    pub fn mem_set(&mut self, m: Memory) {
        self.mem = m;
    }

    pub fn run(&mut self) -> Result<(), InstructionError> {
        // println!("{:?}", self);
        loop {
            let r = self.fde_cycle()?;
            if r.is_none() { return Ok(()) }
            // println!("{}", self);
        }
    }

    
    pub fn fde_cycle(&mut self) -> Result<Option<()>, InstructionError> {
        self.fetch();
        self.decode()?;
        Ok(unsafe { self.execute() })
    }

    fn fetch(&mut self) {
        self.cir = self.mem[self.pc];
        self.pc += 1;
    }

    fn decode(&mut self) -> Result<(), InstructionError> {
        self.mar = match Instruction::try_from(self.cir)? {
            Instruction::ADD(addr) => addr as usize, 
            Instruction::SUB(addr) => addr as usize, 
            Instruction::STA(addr) => addr as usize, 
            Instruction::LDA(addr) => addr as usize, 
            Instruction::BRZ(addr) => addr as usize, 
            Instruction::BRA(addr) => addr as usize, 
            Instruction::BRP(addr) => addr as usize, 
            Instruction::ADDIMM(addr) => addr as usize, 
            Instruction::SUBIMM(addr) => addr as usize, 
            _ => self.pc,     
        };

        self.mdr = self.mem[self.mar];
        Ok(())
    }

    unsafe fn execute(&mut self) -> Option<()>{        
        unsafe { 
            let instruction = Instruction::try_from(self.cir);
            match instruction.unwrap_unchecked() {
                Instruction::ADD(_)    => self.acc += self.mdr, 
                Instruction::ADDIMM(_) => self.acc += self.mar as i16,
                Instruction::SUB(_)    => self.acc -= self.mdr, 
                Instruction::SUBIMM(_) => self.acc -= self.mar as i16,
                Instruction::STA(_)    => self.mem[self.mar] = self.acc, 
                Instruction::LDA(_)    => self.acc = self.mdr, 
                Instruction::BRZ(_)    => if self.acc == 0 { self.pc = self.mar }, 
                Instruction::BRA(_)    => self.pc = self.mdr as usize, 
                Instruction::BRP(_)    => if self.acc > 0 { self.pc = self.mar },
                Instruction::OUT       => { self.out = self.acc; self.output() },
                Instruction::INP       => { self.input(); self.acc = self.inp },
                Instruction::HLT       => return None,
            }

            Some(())
        }
    }

    fn output(&self) {
        println!("{}", self.out)
    }

    fn input(&mut self) {
        print!("Input Requested: ");
        let _ = std::io::stdout().flush();
        let mut input = String::new(); 
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        input = input.trim().to_string();
        self.inp = input.parse::<i16>().expect("some thing");
    }
}