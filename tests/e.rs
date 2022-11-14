use prozessorarchitektur::load_store::{Instruction, Machine};
use prozessorarchitektur::Processor;

#[test]
fn e1() {
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
fn e2() {
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
        Instruction::Ld(0, 1),
        Instruction::Ld(1, 2),
        Instruction::Add(0, 0, 1),
        Instruction::Ld(1, 3),
        Instruction::Div(0, 0, 1),
        Instruction::Ld(1, 4),
        Instruction::Ld(2, 5),
        Instruction::Sub(1, 1, 2),
        Instruction::Mul(0, 0, 1),
        Instruction::St(0, 0),
    ];

    (machine, program)
}
