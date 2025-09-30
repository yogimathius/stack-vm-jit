use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test] 
fn test_comprehensive_vm_functionality() {
    // Test arithmetic operations with constants pool
    let constants = vec![
        Value::Integer(15),     // constants[0]
        Value::Integer(25),     // constants[1] 
        Value::Integer(2),      // constants[2]
    ];
    
    let instructions = vec![
        // Calculate: (15 + 25) / 2 = 20
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),    // Push constants[0] (15)
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),    // Push constants[1] (25)
        Instruction::new(Opcode::Add, None),                         // 15 + 25 = 40
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),    // Push constants[2] (2)  
        Instruction::new(Opcode::Div, None),                         // 40 / 2 = 20
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    assert_eq!(vm.stack_size(), 1);
    assert_eq!(vm.stack_top().unwrap(), &Value::Integer(20));
}

#[test]
fn test_vm_error_handling() {
    // Test various error conditions
    let mut vm = VirtualMachine::new();
    
    // Test running without program
    let result = vm.run();
    assert!(result.is_err());
    
    // Test invalid constant index
    let constants = vec![Value::Integer(1)];
    let instructions = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(5))), // Index 5 out of bounds
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    assert!(result.is_err());
}

#[test] 
fn test_vm_performance_limits() {
    // Test that VM respects instruction count limits
    let mut vm = VirtualMachine::with_max_instructions(5);
    
    let constants = vec![Value::Integer(0)];
    let instructions = vec![
        Instruction::new(Opcode::Jump, Some(Value::Integer(0))), // Infinite loop
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should fail due to instruction limit
    assert!(result.is_err());
    assert!(vm.instruction_count() >= 5);
}

#[test]
fn test_mixed_push_modes() {
    // Test that Push works both with constants pool and literal values
    let constants = vec![Value::String("from_pool".to_string())];
    
    let instructions = vec![
        // Push from constants pool (integer operand when pool is not empty)
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),
        
        // Push literal string value 
        Instruction::new(Opcode::Push, Some(Value::String("literal".to_string()))),
        
        // Push literal boolean (non-integer, so definitely literal)
        Instruction::new(Opcode::Push, Some(Value::Boolean(true))),
        
        Instruction::new(Opcode::Halt, None),
    ];
    
    let mut vm = VirtualMachine::new();
    vm.load_bytecode_module(instructions, constants).unwrap();
    
    vm.run().unwrap();
    
    assert_eq!(vm.stack_size(), 3);
    
    // Note: For this test to work exactly as expected, we'd need stack inspection methods
    // For now, we're just testing that it runs without error
}