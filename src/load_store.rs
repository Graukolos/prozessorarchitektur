use crate::Processor;

pub enum Instruction {
    Ld(u8, u8),
    St(u8, u8),
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
}

pub struct Machine {
    registers: [u32; u8::MAX as usize],
    mem: [u32; u8::MAX as usize],
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            registers: [0; u8::MAX as usize],
            mem: [0; u8::MAX as usize],
        }
    }
}

impl Processor<Instruction> for Machine {
    fn execute(&mut self, program: Vec<Instruction>) {
        for instruction in program {
            match instruction {
                Instruction::Ld(reg_addr, mem_addr) => {
                    self.registers[reg_addr as usize] = self.mem[mem_addr as usize];
                }
                Instruction::St(reg_addr, mem_addr) => {
                    self.mem[mem_addr as usize] = self.registers[reg_addr as usize];
                }
                Instruction::Add(to, addr_a, addr_b) => {
                    self.registers[to as usize] =
                        self.registers[addr_a as usize] + self.registers[addr_b as usize];
                }
                Instruction::Sub(to, addr_a, addr_b) => {
                    self.registers[to as usize] =
                        self.registers[addr_a as usize] - self.registers[addr_b as usize];
                }
                Instruction::Mul(to, addr_a, addr_b) => {
                    self.registers[to as usize] =
                        self.registers[addr_a as usize] * self.registers[addr_b as usize];
                }
                Instruction::Div(to, addr_a, addr_b) => {
                    self.registers[to as usize] =
                        self.registers[addr_a as usize] / self.registers[addr_b as usize];
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
