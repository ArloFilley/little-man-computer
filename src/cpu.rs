pub struct CPU {
    program_counter: usize,
    accumulator: isize,
}

impl CPU {
    pub fn new() -> Self {
        Self { 
            program_counter: 0, 
            accumulator: 0, 
            current_instruction_register: 0, 
            memory_address_register: 0, 
            memory_data_register: 0 
        }
    }

    pub fn execute(&mut self) {

    }
}