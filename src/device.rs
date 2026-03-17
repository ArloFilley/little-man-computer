use regex::Regex;
use std::fs;
use anyhow::{Error, Ok};

use crate::cpu::CPU;

pub struct Device {
    pub program: Program,
    pub cpu: CPU,
    pub ram: RAM,
}

impl Device {
    pub fn run(&mut self) {
        loop {
            self.cpu.execute();
            let current_instruction = self.program[self.program_counter as usize].clone();

            match current_instruction {
                Instruction::Inp => self.inp(),
                Instruction::Out => self.out(),
                Instruction::Sta(addressing_mode) => self.store(addressing_mode),
                Instruction::Lda(addressing_mode) => self.load(addressing_mode),
                Instruction::Add(addressing_mode) => self.add(addressing_mode),
                Instruction::Sub(addressing_mode) => self.subtract(addressing_mode),
                Instruction::Brp(x) => { 
                    if self.accumulator >= 1 {
                        self.program_counter = x;
                        continue;
                    }
                },
                Instruction::Bra(x) => { self.program_counter = x; continue; },
                Instruction::Brz(x) => { 
                    if self.accumulator == 0 {
                        self.program_counter = x;
                        continue;
                    }
                },
                Instruction::Hlt => { break }
            }

            self.program_counter += 1;

        }
    }
}

impl Device {
    pub fn load_program_file(&mut self, filename: &str) -> Result<(), Error>{
        let program_string = fs::read_to_string(filename)?;
        self.decode_program_file(program_string);

        Ok(())
    }

    fn decode_program_file(&mut self, program_string: String) {
        let num_re = Regex::new(r"\d+").unwrap();
        let instruction_re = Regex::new(r"(?i)\b(?:inp|out|sta|lda|add|sub|brp|bra|brz|hlt)\b").unwrap();
        let addressing_mode_re = Regex::new(r"[#@]").unwrap();

        for line in program_string.lines() {
            let instruction_option_string = match instruction_re.find(line) {
                Some(x) => Some(x.as_str().to_lowercase()),
                None => None,
            };

            let num = match num_re.find(line) {
                Some(x) => Some(x.as_str().parse::<i16>().unwrap()),
                None => None,
            };

            let addressing_mode = match addressing_mode_re.find(line) {
                Some(x) => x.as_str(),
                None => ""
            };

            if instruction_option_string.is_some() {
                self.program.push(decode_instruction(&instruction_option_string.unwrap(), num, addressing_mode))
            }   
        }
    }    
}

impl Device {
    fn inp(&mut self) {
        let mut line = String::new();
        println!("Enter a number:");
        std::io::stdin().read_line(&mut line).expect("Error Reading Input");
        self.accumulator = line.trim().parse().expect("Error Parsing Input");
    }

    fn out(&self) {
        println!("{}", self.accumulator);
    }

    fn store(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Direct(x) => self.ram[x as usize] = self.accumulator,
            AddressingMode::Indirect(x) => self.ram[self.ram[x as usize] as usize] = self.accumulator,
            AddressingMode::Immediate(x) => self.ram[x as usize] = self.accumulator,
        }
    }

    fn load(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Direct(x) => self.accumulator = self.ram[x as usize],
            AddressingMode::Indirect(x) => self.accumulator = self.ram[self.ram[x as usize] as usize],
            AddressingMode::Immediate(x) => self.accumulator = self.ram[x as usize],
        }
    }

    fn add(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Direct(x) => self.accumulator += self.ram[x as usize],
            AddressingMode::Immediate(x) => self.accumulator += x,
            AddressingMode::Indirect(x) => self.accumulator += self.ram[self.ram[x as usize] as usize],
        }
    }

    fn subtract(&mut self, addressing_mode: AddressingMode) {
        match addressing_mode {
            AddressingMode::Direct(x) => self.accumulator -= self.ram[x as usize],
            AddressingMode::Immediate(x) => self.accumulator -= x,
            AddressingMode::Indirect(x) => self.accumulator -= self.ram[self.ram[x as usize] as usize],
        }
    }
}





fn decode_instruction(instruction: &str, num: Option<i16>, addressing_mode: &str) -> Instruction {
    match instruction {
        "inp" => Instruction::Inp,
        "out" => Instruction::Out,
        a @ "sta" | a @ "lda" | a @ "add" | a @ "sub" |
        a @ "brp" | a @ "bra" | a @ "brz" 
        => {
            let num = match num {
                Some(x) => x,
                None => panic!("Error, couldn't find number for instruction")
            };

            let addressing_mode = match addressing_mode {
                "@" => AddressingMode::Indirect(num as u8),
                "#" => AddressingMode::Immediate(num),
                "" => AddressingMode::Direct(num as u8),
                _ => unreachable!(),
            };

            match a {
                "sta" => Instruction::Sta(addressing_mode),
                "lda" => Instruction::Lda(addressing_mode),
                "add" => Instruction::Add(addressing_mode),
                "sub" => Instruction::Sub(addressing_mode),

                "brp" => Instruction::Brp(num as u8),
                "bra" => Instruction::Bra(num as u8),
                "brz" => Instruction::Brz(num as u8),

                _ => unreachable!(),
            }
        },
        "hlt" => Instruction::Hlt,
        _ => unreachable!()
    }
}