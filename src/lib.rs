pub mod accumulator;
pub mod queue;
pub mod stack;
pub mod register;

pub trait Processor<ISA> {
    fn execute(&mut self, program: Vec<ISA>);
    fn set(&mut self, address: u8, value: u32);
    fn get(&self, address: u8) -> u32;
}
