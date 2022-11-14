use crate::{Instruction, Processor};

pub struct AccumulatorMachine {
    accumulator: Option<u32>,
    mem: [u32; u8::MAX as usize],
}

impl AccumulatorMachine {
    pub fn new() -> Self {
        Self {
            accumulator: None,
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor for AccumulatorMachine {
    fn execute(&mut self, program: crate::Program) {
        for instruction in program {
            println!("{:?}", self.accumulator);

            match instruction {
                Instruction::Ld(address) => self.accumulator = Some(self.mem[address as usize]),
                Instruction::St(address) => {
                    self.mem[address as usize] = self.accumulator.take().unwrap()
                }
                Instruction::AddAcc(address) => {
                    let a = self.mem[address as usize];
                    self.accumulator = Some(self.accumulator.unwrap() + a);
                }
                Instruction::SubAcc(address) => {
                    let a = self.mem[address as usize];
                    self.accumulator = Some(self.accumulator.unwrap() - a);
                }
                Instruction::MulAcc(address) => {
                    let a = self.mem[address as usize];
                    self.accumulator = Some(self.accumulator.unwrap() * a);
                }
                Instruction::DivAcc(address) => {
                    let a = self.mem[address as usize];
                    self.accumulator = Some(self.accumulator.unwrap() / a);
                }
                _ => {}
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
