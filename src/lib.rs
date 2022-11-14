pub use queue::QueueMachine;
pub use stack::StackMachine;

mod stack;
mod queue;

pub enum Instruction {
    Ld(u8),
    St(u8),
    Add,
    Sub,
    Mul,
    Div,
}

pub type Program = Vec<Instruction>;

pub trait Processor {
    fn execute(&mut self, program: Program);
    fn set(&mut self, address: u8, value: u32);
    fn get(&self, address: u8) -> u32;
}
