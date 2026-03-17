use std::collections::HashMap;

pub struct RAM([i64; 256], Identifiers);

pub type Identifiers = HashMap<String, u8>;

impl RAM {
    pub fn new() -> Self {
        Self([0; 256], HashMap::new())
    }

    pub fn clear(&mut self) {
        self.0.iter_mut().for_each(|i| *i = 0)
    }

    pub fn get(&self, address: AddressingMode) -> i64 {
        match address {
            AddressingMode::Direct(i) => {
                return self.0[i as usize];
            }
            AddressingMode::Immediate(i) => {
                return i;
            }
            AddressingMode::Indirect(i) => {
                return self.0[self.0[i as usize] as usize]
            }
        }
    }

    pub fn set(&mut self, index: u8, value: i64) {
        self.0[index as usize] = value
    }
}

#[derive(Clone, Debug)]
pub enum AddressingMode {
    Direct(u8),
    Immediate(i64),
    Indirect(u8),
}