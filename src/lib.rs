pub use accumulator::AccumulatorMachine;
pub use queue::QueueMachine;
pub use stack::StackMachine;

mod accumulator;
mod queue;
mod stack;

pub enum Instruction {
    Ld(u8),
    St(u8),
    Add,
    Sub,
    Mul,
    Div,
    AddAcc(u8),
    SubAcc(u8),
    MulAcc(u8),
    DivAcc(u8),
}

pub type Program = Vec<Instruction>;

pub trait Processor {
    fn execute(&mut self, program: Program);
    fn set(&mut self, address: u8, value: u32);
    fn get(&self, address: u8) -> u32;
}
