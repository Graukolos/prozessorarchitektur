use prozessorarchitektur::queue::{Instruction, Machine};
use prozessorarchitektur::Processor;

#[test]
fn b1() {
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
fn b2() {
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
        Instruction::Ld(1),
        Instruction::Ld(2),
        Instruction::Add,
        Instruction::Ld(3),
        Instruction::Ld(4),
        Instruction::Ld(5),
        Instruction::Div,
        Instruction::Sub,
        Instruction::Mul,
        Instruction::St(0),
    ];

    (machine, program)
}
