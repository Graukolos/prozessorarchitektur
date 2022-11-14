use prozessorarchitektur::{Instruction, Processor, Program, StackMachine};

#[test]
fn a1() {
    let (mut stack_machine, program) = setup();

    let value = 1;
    stack_machine.set(1, value);
    stack_machine.set(2, value);
    stack_machine.set(3, value);
    stack_machine.set(4, value);
    stack_machine.set(5, value);

    stack_machine.execute(program);

    assert_eq!(0, stack_machine.get(0))
}

#[test]
fn a2() {
    let (mut stack_machine, program) = setup();

    let value = 2;
    stack_machine.set(1, value);
    stack_machine.set(2, value);
    stack_machine.set(3, value);
    stack_machine.set(4, value);
    stack_machine.set(5, 1);

    stack_machine.execute(program);

    assert_eq!(2, stack_machine.get(0))
}

fn setup() -> (StackMachine, Program) {
    let stack_machine = StackMachine::new();

    let program = vec![
        Instruction::Ld(3),
        Instruction::Ld(1),
        Instruction::Ld(2),
        Instruction::Add,
        Instruction::Div,
        Instruction::Ld(5),
        Instruction::Ld(4),
        Instruction::Sub,
        Instruction::Mul,
        Instruction::St(0),
    ];

    (stack_machine, program)
}