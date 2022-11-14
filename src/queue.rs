use std::collections::VecDeque;

use crate::Processor;

pub struct QueueMachine {
    queue: VecDeque<u32>,
    mem: [u32; u8::MAX as usize],
}

impl QueueMachine {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor for QueueMachine {
    fn execute(&mut self, program: crate::Program) {
        for instruction in program {
            println!("{:?}", self.queue);

            match instruction {
                crate::Instruction::Ld(address) => self.queue.push_back(self.mem[address as usize]),
                crate::Instruction::St(address) => {
                    self.mem[address as usize] = self.queue.pop_front().unwrap()
                }
                crate::Instruction::Add => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a + b)
                }
                crate::Instruction::Sub => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a - b)
                }
                crate::Instruction::Mul => {
                    let a = self.queue.pop_front().unwrap();
                    let b = self.queue.pop_front().unwrap();
                    self.queue.push_back(a * b)
                }
                crate::Instruction::Div => {
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
