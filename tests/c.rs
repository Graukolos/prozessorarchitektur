use prozessorarchitektur::{AccumulatorMachine, Instruction, Processor, Program};

#[test]
fn c1() {
    let (mut accumulator_machine, program) = setup();

    let value = 1;
    accumulator_machine.set(1, value);
    accumulator_machine.set(2, value);
    accumulator_machine.set(3, value);
    accumulator_machine.set(4, value);
    accumulator_machine.set(5, value);

    accumulator_machine.execute(program);

    assert_eq!(0, accumulator_machine.get(0))
}

#[test]
fn c2() {
    let (mut accumulator_machine, program) = setup();

    let value = 2;
    accumulator_machine.set(1, value);
    accumulator_machine.set(2, value);
    accumulator_machine.set(3, value);
    accumulator_machine.set(4, value);
    accumulator_machine.set(5, 1);

    accumulator_machine.execute(program);

    assert_eq!(2, accumulator_machine.get(0))
}

fn setup() -> (AccumulatorMachine, Program) {
    let accumulator_machine = AccumulatorMachine::new();

    let program = vec![
        Instruction::Ld(4),
        Instruction::SubAcc(5),
        Instruction::St(0),
        Instruction::Ld(1),
        Instruction::AddAcc(2),
        Instruction::DivAcc(3),
        Instruction::MulAcc(0),
        Instruction::St(0),
    ];

    (accumulator_machine, program)
}
