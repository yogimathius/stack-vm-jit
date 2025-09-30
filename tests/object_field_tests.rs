use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_get_field_nonexistent() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),                        // Create object
        Instruction::new(Opcode::GetField, Some(Value::String("name".to_string()))), // Get field "name"
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have the field value (null for nonexistent field) on stack
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.stack_top().unwrap(), &Value::Null);
}

#[test]
fn test_get_field_with_numeric_index() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),                        // Create object
        Instruction::new(Opcode::GetField, Some(Value::Integer(0))),     // Get field_0
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have null since field doesn't exist
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.stack_top().unwrap(), &Value::Null);
}

#[test]
fn test_get_field_invalid_operand() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),                       // Create object
        Instruction::new(Opcode::GetField, Some(Value::Boolean(true))), // Invalid operand type
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail due to invalid operand
    assert!(result.is_err());
}

#[test]
fn test_get_field_no_operand() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),  // Create object
        Instruction::new(Opcode::GetField, None),   // No operand
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail due to missing operand
    assert!(result.is_err());
}

#[test]
fn test_get_field_non_object() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![Value::Integer(42)];
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),         // Push integer
        Instruction::new(Opcode::GetField, Some(Value::String("field".to_string()))), // Try to get field
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail because integer is not an object
    assert!(result.is_err());
}

#[test]
fn test_set_field_current_limitation() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![Value::Integer(123)];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),                       // Create object
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),        // Push value 123
        Instruction::new(Opcode::SetField, Some(Value::String("value".to_string()))), // Set field
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail due to current limitation (no interior mutability)
    assert!(result.is_err());
    
    // But the values should be pushed back onto the stack
    assert_eq!(vm.stack_size(), 2); // object and value should be back on stack
}

#[test]
fn test_set_field_non_object() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![Value::Integer(42), Value::Integer(123)];
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),         // Push integer 42
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),         // Push value 123
        Instruction::new(Opcode::SetField, Some(Value::String("field".to_string()))), // Try to set field
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail because integer is not an object
    assert!(result.is_err());
    
    // Values should be pushed back onto stack
    assert_eq!(vm.stack_size(), 2);
}

#[test]
fn test_object_creation_and_field_access_workflow() {
    // Test the complete workflow even though SetField has limitations
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        // Create two objects
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::NewObject, None),
        
        // Try to get fields from both (should be null)
        Instruction::new(Opcode::Dup, None), // Duplicate second object
        Instruction::new(Opcode::GetField, Some(Value::String("name".to_string()))),
        
        Instruction::new(Opcode::Swap, None), // Swap to get first object on top
        Instruction::new(Opcode::GetField, Some(Value::String("id".to_string()))),
        
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have: [object, null, null] on stack
    assert_eq!(vm.stack_size(), 3);
    
    // Top should be null from the last GetField
    assert_eq!(vm.stack_top().unwrap(), &Value::Null);
    
    // Should have 2 objects allocated in heap  
    assert_eq!(vm.heap_allocated_objects(), 2);
}

#[test]
fn test_field_access_opcodes_exist() {
    // Test that the opcodes are properly defined
    use stack_vm_jit::vm::instruction::Opcode;
    
    // This test ensures the opcodes can be created and compared
    assert_eq!(Opcode::GetField as u8, 0x53);
    assert_eq!(Opcode::SetField as u8, 0x54);
    
    // Test opcode parsing
    assert_eq!(Opcode::from_u8(0x53), Some(Opcode::GetField));
    assert_eq!(Opcode::from_u8(0x54), Some(Opcode::SetField));
}