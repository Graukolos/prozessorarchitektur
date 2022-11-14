use std::collections::VecDeque;

use crate::Processor;

pub enum Instruction {
    Ld(u8),
    St(u8),
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Machine {
    queue: VecDeque<u32>,
    mem: [u32; u8::MAX as usize],
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor<Instruction> for Machine {
    fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            println!("{:?}", self.queue);

            match instruction {
                Instruction::Ld(address) => self.queue.push_back(self.mem[address as usize]),
                Instruction::St(address) => {
                    self.mem[address as usize] = self.queue.pop_front().unwrap()
                }
                Instruction::Add => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a + b)
                }
                Instruction::Sub => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a - b)
                }
                Instruction::Mul => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a * b)
                }
                Instruction::Div => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a / b)
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
