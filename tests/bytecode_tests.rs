use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_bytecode_module_creation() {
    // Create a simple bytecode module with constants pool
    let constants = vec![
        Value::Integer(42),
        Value::String("hello world".to_string()),
        Value::Float(1.414),
        Value::Boolean(true),
    ];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // Push constant 0 (42)
        Instruction::new(Opcode::Push, Some(Value::Integer(1))), // Push constant 1 ("hello world")
        Instruction::new(Opcode::Push, Some(Value::Integer(2))), // Push constant 2 (1.414)
        Instruction::new(Opcode::Push, Some(Value::Integer(3))), // Push constant 3 (true)
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    assert_eq!(vm.constants_pool_size(), 4);
    assert_eq!(vm.program_length(), 5);
}

#[test]
fn test_push_from_constants_pool() {
    let constants = vec![
        Value::Integer(100),
        Value::String("test".to_string()),
    ];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // Push constants[0]
        Instruction::new(Opcode::Push, Some(Value::Integer(1))), // Push constants[1]  
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    // Stack should have both values
    assert_eq!(vm.stack_size(), 2);
    
    // Top should be the string "test"
    let top = vm.stack_top().unwrap();
    assert_eq!(top, &Value::String("test".to_string()));
}

#[test]
fn test_constants_pool_bounds_checking() {
    let constants = vec![Value::Integer(1), Value::Integer(2)];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(5))), // Index 5 out of bounds
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    // Should fail when trying to push from invalid constant index
    let result = vm.run();
    assert!(result.is_err());
}

#[test]
fn test_constants_pool_type_safety() {
    let constants = vec![
        Value::Integer(42),
        Value::String("hello".to_string()),
        Value::Boolean(false),
    ];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    // Verify each type was pushed correctly
    assert_eq!(vm.stack_size(), 3);
}

#[test]
fn test_program_execution_with_constants() {
    // Test a simple arithmetic program using constants
    let constants = vec![
        Value::Integer(10),
        Value::Integer(5),
        Value::Integer(2),
    ];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // Push 10
        Instruction::new(Opcode::Push, Some(Value::Integer(1))), // Push 5
        Instruction::new(Opcode::Add, None),                      // 10 + 5 = 15
        Instruction::new(Opcode::Push, Some(Value::Integer(2))), // Push 2
        Instruction::new(Opcode::Mul, None),                      // 15 * 2 = 30
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.stack_top().unwrap(), &Value::Integer(30));
}

#[test]
fn test_bytecode_module_validation() {
    let constants = vec![Value::Integer(1)];
    
    // Empty instruction list should be invalid
    let instructions = vec![];
    
    let mut vm = VirtualMachine::new();
    let result = vm.load_bytecode_module(instructions, constants);
    
    assert!(result.is_err());
}

#[test]
fn test_constants_pool_string_interning() {
    // Test that string constants are properly stored and retrieved
    let string_value = "This is a test string with unicode: 🦀";
    let constants = vec![
        Value::String(string_value.to_string()),
        Value::String(string_value.to_string()), // Duplicate for testing
    ];
    
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),
        Instruction::new(Opcode::Equal, None), // Should be equal
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    // Result should be true (strings are equal)
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.stack_top().unwrap(), &Value::Boolean(true));
}