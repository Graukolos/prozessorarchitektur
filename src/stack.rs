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
    stack: Vec<u32>,
    mem: [u32; u8::MAX as usize],
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor<Instruction> for Machine {
    fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            println!("{:?}", self.stack);

            match instruction {
                Instruction::Ld(address) => self.stack.push(self.mem[address as usize]),
                Instruction::St(address) => self.mem[address as usize] = self.stack.pop().unwrap(),
                Instruction::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b)
                }
                Instruction::Sub => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a - b)
                }
                Instruction::Mul => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a * b)
                }
                Instruction::Div => {
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
