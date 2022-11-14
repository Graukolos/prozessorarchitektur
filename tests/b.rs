use prozessorarchitektur::{Instruction, Processor, Program, QueueMachine};

#[test]
fn b1() {
    let (mut queue_machine, program) = setup();

    let value = 1;
    queue_machine.set(1, value);
    queue_machine.set(2, value);
    queue_machine.set(3, value);
    queue_machine.set(4, value);
    queue_machine.set(5, value);

    queue_machine.execute(program);

    assert_eq!(0, queue_machine.get(0))
}

#[test]
fn b2() {
    let (mut queue_machine, program) = setup();

    let value = 2;
    queue_machine.set(1, value);
    queue_machine.set(2, value);
    queue_machine.set(3, value);
    queue_machine.set(4, value);
    queue_machine.set(5, 1);

    queue_machine.execute(program);

    assert_eq!(2, queue_machine.get(0))
}

fn setup() -> (QueueMachine, Program) {
    let queue_machine = QueueMachine::new();

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

    (queue_machine, program)
}
