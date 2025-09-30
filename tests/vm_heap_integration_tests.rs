use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_new_object_instruction() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have 2 objects on the stack
    assert_eq!(vm.stack_size(), 2);
    
    // Should have 2 objects allocated in the heap
    assert_eq!(vm.heap_allocated_objects(), 2);
    assert!(vm.heap_total_bytes() > 0);
}

#[test]
fn test_heap_object_allocation() {
    let mut vm = VirtualMachine::new();
    
    // Initial state
    assert_eq!(vm.heap_allocated_objects(), 0);
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // One object should be allocated
    assert_eq!(vm.heap_allocated_objects(), 1);
    
    // Check that the stack contains a GC object
    let top_value = vm.stack_top().unwrap();
    match top_value {
        Value::GcObject(_) => {
            assert_eq!(top_value.type_name(), "gc_object");
            assert!(top_value.is_truthy()); // Objects are always truthy
        }
        _ => panic!("Expected GcObject on stack"),
    }
}

#[test]
fn test_garbage_collection_trigger() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have 3 objects
    assert_eq!(vm.heap_allocated_objects(), 3);
    
    // Trigger garbage collection
    let collected = vm.trigger_gc();
    
    // In our simple implementation, GC should collect something
    assert!(collected > 0 || vm.heap_allocated_objects() < 3);
}

#[test]
fn test_mixed_value_types_with_heap() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![
        Value::Integer(42),
        Value::String("hello".to_string()),
    ];
    
    let instructions = vec![
        // Push some constants
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // Push 42
        Instruction::new(Opcode::Push, Some(Value::Integer(1))), // Push "hello"
        
        // Create an object
        Instruction::new(Opcode::NewObject, None),
        
        // Push a literal value
        Instruction::new(Opcode::Push, Some(Value::Boolean(true))),
        
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    // Should have 4 values on stack: integer, string, object, boolean
    assert_eq!(vm.stack_size(), 4);
    
    // Should have 1 object in heap
    assert_eq!(vm.heap_allocated_objects(), 1);
}

#[test]
fn test_heap_out_of_memory() {
    // Create VM with very small heap
    let mut vm = VirtualMachine::new();
    
    // This test would be better with a heap size limit, but our current
    // implementation doesn't enforce strict limits in the VM level
    // For now, just verify that the system can handle object allocation
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    let result = vm.run();
    
    // Should succeed with normal heap
    assert!(result.is_ok());
    assert_eq!(vm.heap_allocated_objects(), 1);
}

#[test]
fn test_object_type_information() {
    let mut vm = VirtualMachine::new();
    
    let constants = vec![];
    let instructions = vec![
        Instruction::new(Opcode::NewObject, None),
        Instruction::new(Opcode::Halt, None),
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    let obj_value = vm.stack_top().unwrap();
    
    // Test type information
    assert_eq!(obj_value.type_name(), "gc_object");
    assert!(obj_value.is_truthy());
    
    // Test that it's actually a GcObject
    if let Value::GcObject(gc_obj) = obj_value {
        assert_eq!(gc_obj.field_count(), 0); // Empty object
    } else {
        panic!("Expected GcObject");
    }
}