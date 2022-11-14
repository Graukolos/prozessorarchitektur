use crate::Processor;

pub struct StackMachine {
    stack: Vec<u32>,
    mem: [u32; u8::MAX as usize],
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor for StackMachine {
    fn execute(&mut self, program: crate::Program) {
        for instruction in program {
            println!("{:?}", self.stack);

            match instruction {
                crate::Instruction::Ld(address) => self.stack.push(self.mem[address as usize]),
                crate::Instruction::St(address) => {
                    self.mem[address as usize] = self.stack.pop().unwrap()
                }
                crate::Instruction::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b)
                }
                crate::Instruction::Sub => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a - b)
                }
                crate::Instruction::Mul => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a * b)
                }
                crate::Instruction::Div => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a / b)
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
