use stack_vm_jit::vm::instruction::{Instruction, InstructionDispatcher, Opcode};
use stack_vm_jit::vm::stack::OperandStack;
use stack_vm_jit::vm::call_frame::{CallStack, CallFrame};
use stack_vm_jit::vm::types::Value;

#[test]
fn test_load_local_variable() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame with some local variables
    let mut frame = CallFrame::new(0, 0, 3);
    frame.set_local(0, Value::Integer(42)).unwrap();
    frame.set_local(1, Value::String("hello".to_string())).unwrap();
    frame.set_local(2, Value::Boolean(true)).unwrap();
    call_stack.push_unchecked(frame);
    
    // Test loading local variable at index 0
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(0)));
    dispatcher.execute(&load_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Integer(42));
    
    // Test loading local variable at index 1
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(1)));
    dispatcher.execute(&load_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::String("hello".to_string()));
    
    // Test loading local variable at index 2
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(2)));
    dispatcher.execute(&load_instruction, &mut stack, &mut call_stack).unwrap();
    
    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
}

#[test]
fn test_store_local_variable() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame with local variables
    let frame = CallFrame::new(0, 0, 3);
    call_stack.push_unchecked(frame);
    
    // Push values to store
    stack.push(Value::Integer(100));
    stack.push(Value::String("world".to_string()));
    stack.push(Value::Boolean(false));
    
    // Test storing to local variable at index 2
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(2)));
    dispatcher.execute(&store_instruction, &mut stack, &mut call_stack).unwrap();
    
    // Test storing to local variable at index 1
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(1)));
    dispatcher.execute(&store_instruction, &mut stack, &mut call_stack).unwrap();
    
    // Test storing to local variable at index 0
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(0)));
    dispatcher.execute(&store_instruction, &mut stack, &mut call_stack).unwrap();
    
    // Verify the values were stored correctly
    let current_frame = call_stack.current().unwrap();
    assert_eq!(current_frame.get_local(0).unwrap(), &Value::Integer(100));
    assert_eq!(current_frame.get_local(1).unwrap(), &Value::String("world".to_string()));
    assert_eq!(current_frame.get_local(2).unwrap(), &Value::Boolean(false));
}

#[test]
fn test_load_store_roundtrip() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame
    let frame = CallFrame::new(0, 0, 5);
    call_stack.push_unchecked(frame);
    
    let test_value = Value::Float(1.414);
    let local_index = 3;
    
    // Store a value
    stack.push(test_value.clone());
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(local_index)));
    dispatcher.execute(&store_instruction, &mut stack, &mut call_stack).unwrap();
    
    // Load the same value
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(local_index)));
    dispatcher.execute(&load_instruction, &mut stack, &mut call_stack).unwrap();
    
    // Should be the same value
    assert_eq!(stack.pop().unwrap(), test_value);
}

#[test]
fn test_load_invalid_index() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame with only 2 locals
    let frame = CallFrame::new(0, 0, 2);
    call_stack.push_unchecked(frame);
    
    // Try to load from index 5 (out of bounds)
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(5)));
    let result = dispatcher.execute(&load_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
}

#[test]
fn test_store_invalid_index() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame with only 2 locals
    let frame = CallFrame::new(0, 0, 2);
    call_stack.push_unchecked(frame);
    
    // Try to store to index 10 (out of bounds)
    stack.push(Value::Integer(42));
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(10)));
    let result = dispatcher.execute(&store_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
}

#[test]
fn test_load_store_no_call_frame() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Empty call stack - should fail
    
    // Try to load
    let load_instruction = Instruction::new(Opcode::Load, Some(Value::Integer(0)));
    let result = dispatcher.execute(&load_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
    
    // Try to store
    stack.push(Value::Integer(42));
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(0)));
    let result = dispatcher.execute(&store_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
}

#[test]
fn test_store_insufficient_stack() {
    let mut dispatcher = InstructionDispatcher::new();
    let mut stack = OperandStack::new();
    let mut call_stack = CallStack::new();
    
    // Create a call frame
    let frame = CallFrame::new(0, 0, 1);
    call_stack.push_unchecked(frame);
    
    // Empty stack - store should fail
    let store_instruction = Instruction::new(Opcode::Store, Some(Value::Integer(0)));
    let result = dispatcher.execute(&store_instruction, &mut stack, &mut call_stack);
    
    assert!(result.is_err());
}