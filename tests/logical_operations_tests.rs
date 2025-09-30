use stack_vm_jit::vm::instruction::{Instruction, InstructionDispatcher, Opcode};
use stack_vm_jit::vm::stack::OperandStack;
use stack_vm_jit::vm::call_frame::CallStack;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_logical_and_operation() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test true AND true = true
    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(true));
    
    let and_instruction = Instruction::new(Opcode::And, None);
    dispatcher.execute(&and_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
    
    // Test true AND false = false
    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(false));
    
    dispatcher.execute(&and_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
    
    // Test false AND false = false
    stack.push(Value::Boolean(false));
    stack.push(Value::Boolean(false));
    
    dispatcher.execute(&and_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_or_operation() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test true OR false = true
    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(false));
    
    let or_instruction = Instruction::new(Opcode::Or, None);
    dispatcher.execute(&or_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
    
    // Test false OR false = false
    stack.push(Value::Boolean(false));
    stack.push(Value::Boolean(false));
    
    dispatcher.execute(&or_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_not_operation() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test NOT true = false
    stack.push(Value::Boolean(true));
    
    let not_instruction = Instruction::new(Opcode::Not, None);
    dispatcher.execute(&not_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
    
    // Test NOT false = true
    stack.push(Value::Boolean(false));
    
    dispatcher.execute(&not_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
}

#[test]
fn test_logical_xor_operation() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test true XOR false = true
    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(false));
    
    let xor_instruction = Instruction::new(Opcode::Xor, None);
    dispatcher.execute(&xor_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
    
    // Test true XOR true = false
    stack.push(Value::Boolean(true));
    stack.push(Value::Boolean(true));
    
    dispatcher.execute(&xor_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
    
    // Test false XOR false = false
    stack.push(Value::Boolean(false));
    stack.push(Value::Boolean(false));
    
    dispatcher.execute(&xor_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false));
}

#[test]
fn test_logical_operations_with_truthy_values() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test integer values with logical operations
    // Non-zero integers should be truthy
    stack.push(Value::Integer(42));
    stack.push(Value::Integer(0));
    
    let and_instruction = Instruction::new(Opcode::And, None);
    dispatcher.execute(&and_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false)); // 42 AND 0 = false
    
    // Test string values
    stack.push(Value::String("hello".to_string()));
    stack.push(Value::String("".to_string()));
    
    dispatcher.execute(&and_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(false)); // "hello" AND "" = false
}

#[test]
fn test_logical_operations_error_handling() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Test binary logical operation with insufficient operands
    stack.push(Value::Boolean(true));
    // Missing second operand
    
    let and_instruction = Instruction::new(Opcode::And, None);
    let result = dispatcher.execute(&and_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
    
    // Test unary logical operation with no operands
    stack.clear();
    
    let not_instruction = Instruction::new(Opcode::Not, None);
    let result = dispatcher.execute(&not_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
}