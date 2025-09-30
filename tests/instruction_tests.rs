use stack_vm_jit::vm::call_frame::CallStack;
use stack_vm_jit::vm::instruction::{Instruction, InstructionDispatcher, Opcode};
use stack_vm_jit::vm::stack::OperandStack;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_opcode_variants() {
    // Arithmetic operations
    assert_eq!(Opcode::Add as u8, 0x01);
    assert_eq!(Opcode::Sub as u8, 0x02);
    assert_eq!(Opcode::Mul as u8, 0x03);
    assert_eq!(Opcode::Div as u8, 0x04);

    // Stack operations
    assert_eq!(Opcode::Push as u8, 0x10);
    assert_eq!(Opcode::Pop as u8, 0x11);
    assert_eq!(Opcode::Dup as u8, 0x12);
    assert_eq!(Opcode::Swap as u8, 0x13);

    // Control flow
    assert_eq!(Opcode::Jump as u8, 0x20);
    assert_eq!(Opcode::JumpIfTrue as u8, 0x21);
    assert_eq!(Opcode::JumpIfFalse as u8, 0x22);
    assert_eq!(Opcode::Call as u8, 0x23);
    assert_eq!(Opcode::Return as u8, 0x24);

    // Comparison
    assert_eq!(Opcode::Equal as u8, 0x30);
    assert_eq!(Opcode::NotEqual as u8, 0x31);
    assert_eq!(Opcode::LessThan as u8, 0x32);
    assert_eq!(Opcode::LessEqual as u8, 0x33);
    assert_eq!(Opcode::GreaterThan as u8, 0x34);
    assert_eq!(Opcode::GreaterEqual as u8, 0x35);
}

#[test]
fn test_instruction_creation() {
    let instr = Instruction::new(Opcode::Push, Some(Value::Integer(42)));
    assert_eq!(instr.opcode(), Opcode::Push);
    assert_eq!(instr.operand(), Some(&Value::Integer(42)));

    let instr_no_operand = Instruction::new(Opcode::Add, None);
    assert_eq!(instr_no_operand.opcode(), Opcode::Add);
    assert_eq!(instr_no_operand.operand(), None);
}

#[test]
fn test_instruction_dispatcher_creation() {
    let dispatcher = InstructionDispatcher::new();
    assert_eq!(dispatcher.instruction_count(), 0);
    assert_eq!(dispatcher.current_pc(), 0);
}

#[test]
fn test_arithmetic_instructions() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Test addition: 5 + 3 = 8
    stack.push(Value::Integer(5));
    stack.push(Value::Integer(3));

    let add_instr = Instruction::new(Opcode::Add, None);
    dispatcher
        .execute(&add_instr, &mut stack, &mut call_stack)
        .unwrap();

    let result = stack.pop().unwrap();
    assert_eq!(result, Value::Integer(8));
}

#[test]
fn test_comparison_instructions() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Test less than: 3 < 5 = true
    stack.push(Value::Integer(3));
    stack.push(Value::Integer(5));

    let lt_instr = Instruction::new(Opcode::LessThan, None);
    dispatcher
        .execute(&lt_instr, &mut stack, &mut call_stack)
        .unwrap();

    let result = stack.pop().unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_stack_manipulation() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Test push
    let push_instr = Instruction::new(Opcode::Push, Some(Value::Integer(42)));
    dispatcher
        .execute(&push_instr, &mut stack, &mut call_stack)
        .unwrap();
    assert_eq!(stack.size(), 1);

    // Test duplicate
    let dup_instr = Instruction::new(Opcode::Dup, None);
    dispatcher
        .execute(&dup_instr, &mut stack, &mut call_stack)
        .unwrap();
    assert_eq!(stack.size(), 2);

    let val1 = stack.pop().unwrap();
    let val2 = stack.pop().unwrap();
    assert_eq!(val1, Value::Integer(42));
    assert_eq!(val2, Value::Integer(42));
}

#[test]
fn test_control_flow_jump() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Test unconditional jump
    let jump_instr = Instruction::new(Opcode::Jump, Some(Value::Integer(100)));
    dispatcher
        .execute(&jump_instr, &mut stack, &mut call_stack)
        .unwrap();

    // PC should be updated for jump
    assert_eq!(dispatcher.current_pc(), 100);
}

#[test]
fn test_conditional_jump() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Test jump if true with true condition
    stack.push(Value::Boolean(true));
    let jump_true_instr = Instruction::new(Opcode::JumpIfTrue, Some(Value::Integer(50)));
    dispatcher
        .execute(&jump_true_instr, &mut stack, &mut call_stack)
        .unwrap();
    assert_eq!(dispatcher.current_pc(), 50);

    // Reset and test jump if false with false condition
    dispatcher.set_pc(0);
    stack.push(Value::Boolean(false));
    let jump_false_instr = Instruction::new(Opcode::JumpIfFalse, Some(Value::Integer(75)));
    dispatcher
        .execute(&jump_false_instr, &mut stack, &mut call_stack)
        .unwrap();
    assert_eq!(dispatcher.current_pc(), 75);
}

#[test]
fn test_instruction_execution_count() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    let push_instr = Instruction::new(Opcode::Push, Some(Value::Integer(1)));

    dispatcher
        .execute(&push_instr, &mut stack, &mut call_stack)
        .unwrap();
    dispatcher
        .execute(&push_instr, &mut stack, &mut call_stack)
        .unwrap();

    assert_eq!(dispatcher.instruction_count(), 2);
}

#[test]
fn test_error_handling() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();

    // Try to add with insufficient operands
    let add_instr = Instruction::new(Opcode::Add, None);
    let result = dispatcher.execute(&add_instr, &mut stack, &mut call_stack);
    assert!(result.is_err());
}
