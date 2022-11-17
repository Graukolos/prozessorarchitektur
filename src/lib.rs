pub mod accumulator;
pub mod load_store;
pub mod queue;
pub mod register;
pub mod stack;

pub trait Processor<ISA> {
    fn execute(&mut self, program: Vec<ISA>);
    fn set(&mut self, address: u8, value: u32);
    fn get(&self, address: u8) -> u32;
}
