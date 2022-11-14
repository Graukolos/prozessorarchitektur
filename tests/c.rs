use prozessorarchitektur::accumulator::{Instruction, Machine};
use prozessorarchitektur::Processor;

#[test]
fn c1() {
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
fn c2() {
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
        Instruction::Ld(4),
        Instruction::Sub(5),
        Instruction::St(0),
        Instruction::Ld(1),
        Instruction::Add(2),
        Instruction::Div(3),
        Instruction::Mul(0),
        Instruction::St(0),
    ];

    (machine, program)
}
