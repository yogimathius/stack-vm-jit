use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_vm_creation() {
    let vm = VirtualMachine::new();
    assert_eq!(vm.stack_size(), 0);
    assert_eq!(vm.call_depth(), 0);
    assert_eq!(vm.program_counter(), 0);
}

#[test]
fn test_simple_arithmetic_program() {
    let mut vm = VirtualMachine::new();

    // Program: push 5, push 3, add
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(5))),
        Instruction::new(Opcode::Push, Some(Value::Integer(3))),
        Instruction::new(Opcode::Add, None),
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);
    vm.run().unwrap();

    // Result should be 8 on top of stack
    let result = vm.stack_top().unwrap();
    assert_eq!(*result, Value::Integer(8));
}

#[test]
fn test_comparison_and_branching() {
    let mut vm = VirtualMachine::new();

    // Simpler test: push 3, push 5, less_than, jump_if_true to address 6
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(3))), // 0
        Instruction::new(Opcode::Push, Some(Value::Integer(5))), // 1
        Instruction::new(Opcode::LessThan, None),                // 2 - 3 < 5 = true
        Instruction::new(Opcode::JumpIfTrue, Some(Value::Integer(6))), // 3 - should jump
        Instruction::new(Opcode::Push, Some(Value::Integer(100))), // 4 - should be skipped
        Instruction::new(Opcode::Halt, None),                    // 5
        Instruction::new(Opcode::Push, Some(Value::Integer(200))), // 6 - jump target
        Instruction::new(Opcode::Halt, None),                    // 7
    ];

    vm.load_program(program);
    vm.run().unwrap();

    // Should have 200 on stack (3 < 5 is true, so we jump to address 6)
    let result = vm.stack_top().unwrap();
    assert_eq!(*result, Value::Integer(200));
}

#[test]
fn test_function_call_and_return() {
    let mut vm = VirtualMachine::new();

    // Simple call/return test: call to address 2, which pushes 42 and returns
    let program = vec![
        Instruction::new(Opcode::Call, Some(Value::Integer(2))), // 0 - call address 2
        Instruction::new(Opcode::Halt, None),                    // 1 - end of main
        Instruction::new(Opcode::Push, Some(Value::Integer(42))), // 2 - function body
        Instruction::new(Opcode::Return, None),                  // 3 - return
    ];

    vm.load_program(program);
    vm.run().unwrap();

    // Should have 42 on stack from function call
    let result = vm.stack_top().unwrap();
    assert_eq!(*result, Value::Integer(42));
}

#[test]
fn test_complex_arithmetic_expression() {
    let mut vm = VirtualMachine::new();

    // Compute (10 + 5) * 2 - 3 = 27
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(10))),
        Instruction::new(Opcode::Push, Some(Value::Integer(5))),
        Instruction::new(Opcode::Add, None), // Stack: 15
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),
        Instruction::new(Opcode::Mul, None), // Stack: 30
        Instruction::new(Opcode::Push, Some(Value::Integer(3))),
        Instruction::new(Opcode::Sub, None), // Stack: 27
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);
    vm.run().unwrap();

    let result = vm.stack_top().unwrap();
    assert_eq!(*result, Value::Integer(27));
}

#[test]
fn test_stack_manipulation() {
    let mut vm = VirtualMachine::new();

    // Test dup and swap operations
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(10))),
        Instruction::new(Opcode::Push, Some(Value::Integer(20))),
        Instruction::new(Opcode::Dup, None),  // Stack: 10, 20, 20
        Instruction::new(Opcode::Swap, None), // Stack: 10, 20, 20 -> 20, 10, 20
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);
    vm.run().unwrap();

    assert_eq!(vm.stack_size(), 3);
    let top = vm.stack_top().unwrap();
    assert_eq!(*top, Value::Integer(20));
}

#[test]
fn test_vm_step_execution() {
    let mut vm = VirtualMachine::new();

    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),
        Instruction::new(Opcode::Add, None),
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);

    // Step through execution
    assert!(!vm.is_halted());
    vm.step().unwrap();
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.program_counter(), 1);

    vm.step().unwrap();
    assert_eq!(vm.stack_size(), 2);
    assert_eq!(vm.program_counter(), 2);

    vm.step().unwrap();
    assert_eq!(vm.stack_size(), 1);
    let result = vm.stack_top().unwrap();
    assert_eq!(*result, Value::Integer(3));

    vm.step().unwrap();
    assert!(vm.is_halted());
}

#[test]
fn test_error_handling() {
    let mut vm = VirtualMachine::new();

    // Try to add with insufficient operands
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(5))),
        Instruction::new(Opcode::Add, None), // Should fail - needs 2 operands
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);
    let result = vm.run();
    assert!(result.is_err());
}

#[test]
fn test_vm_reset() {
    let mut vm = VirtualMachine::new();

    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(42))),
        Instruction::new(Opcode::Halt, None),
    ];

    vm.load_program(program);
    vm.run().unwrap();

    assert_eq!(vm.stack_size(), 1);
    assert!(vm.is_halted());

    vm.reset();
    assert_eq!(vm.stack_size(), 0);
    assert_eq!(vm.program_counter(), 0);
    assert!(!vm.is_halted());
}
