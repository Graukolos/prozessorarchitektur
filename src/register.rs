use crate::Processor;

pub enum Address {
    Mem(u8),
    Reg(u8),
}

pub enum Instruction {
    St(Address, Address),
    Add(Address, Address, Address),
    Sub(Address, Address, Address),
    Mul(Address, Address, Address),
    Div(Address, Address, Address),
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
                Instruction::St(from, to) => {
                    self.set_addr(to, self.get_addr(from));
                }
                Instruction::Add(to, addr_a, addr_b) => {
                    let value = self.get_addr(addr_a) + self.get_addr(addr_b);
                    self.set_addr(to, value);
                }
                Instruction::Sub(to, addr_a, addr_b) => {
                    let value = self.get_addr(addr_a) - self.get_addr(addr_b);
                    self.set_addr(to, value);
                }
                Instruction::Mul(to, addr_a, addr_b) => {
                    let value = self.get_addr(addr_a) * self.get_addr(addr_b);
                    self.set_addr(to, value);
                }
                Instruction::Div(to, addr_a, addr_b) => {
                    let value = self.get_addr(addr_a) / self.get_addr(addr_b);
                    self.set_addr(to, value);
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

impl Machine {
    fn set_addr(&mut self, address: Address, value: u32) {
        match address {
            Address::Mem(addr) => self.mem[addr as usize] = value,
            Address::Reg(addr) => self.registers[addr as usize] = value,
        }
    }

    fn get_addr(&self, address: Address) -> u32 {
        match address {
            Address::Mem(addr) => self.mem[addr as usize],
            Address::Reg(addr) => self.registers[addr as usize],
        }
    }
}
