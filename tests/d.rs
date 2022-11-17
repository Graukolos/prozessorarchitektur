use prozessorarchitektur::register::Address::{Mem, Reg};
use prozessorarchitektur::register::{Instruction, Machine};
use prozessorarchitektur::Processor;

#[test]
fn d1() {
    let (mut machine, program) = setup();

    let value = 1;
    machine.set(1, value);
    machine.set(2, value);
    machine.set(3, value);
    machine.set(4, value);
    machine.set(5, value);

    machine.execute(program);

    assert_eq!(0, machine.get(0))
}

#[test]
fn d2() {
    let (mut machine, program) = setup();

    let value = 2;
    machine.set(1, value);
    machine.set(2, value);
    machine.set(3, value);
    machine.set(4, value);
    machine.set(5, 1);

    machine.execute(program);

    assert_eq!(2, machine.get(0))
}

fn setup() -> (Machine, Vec<Instruction>) {
    let machine = Machine::default();

    let program = vec![
        Instruction::Add(Reg(0), Mem(1), Mem(2)),
        Instruction::Div(Reg(0), Reg(0), Mem(3)),
        Instruction::Sub(Reg(1), Mem(4), Mem(5)),
        Instruction::Mul(Reg(0), Reg(0), Reg(1)),
        Instruction::St(Reg(0), Mem(0)),
    ];

    (machine, program)
}
