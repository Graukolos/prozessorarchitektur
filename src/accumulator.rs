use crate::Processor;

pub enum Instruction {
    Ld(u8),
    St(u8),
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

pub struct Machine {
    accumulator: u32,
    mem: [u32; u8::MAX as usize],
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            accumulator: 0,
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor<Instruction> for Machine {
    fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            println!("{:?}", self.accumulator);

            match instruction {
                Instruction::Ld(address) => self.accumulator = self.mem[address as usize],
                Instruction::St(address) => self.mem[address as usize] = self.accumulator,
                Instruction::Add(address) => {
                    self.accumulator += self.mem[address as usize];
                }
                Instruction::Sub(address) => {
                    self.accumulator -= self.mem[address as usize];
                }
                Instruction::Mul(address) => {
                    self.accumulator *= self.mem[address as usize];
                }
                Instruction::Div(address) => {
                    self.accumulator /= self.mem[address as usize];
                }
            }
        }
    }

    fn set(&mut self, address: u8, value: u32) {
        self.mem[address as usize] = value;
    }

    fn get(&self, address: u8) -> u32 {
        self.mem[address as usize]
    }
}
