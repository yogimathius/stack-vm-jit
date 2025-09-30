use stack_vm_jit::vm::instruction::{Instruction, Opcode};
use stack_vm_jit::vm::jit::{HotSpotProfiler, OptimizationLevel};
use stack_vm_jit::vm::runtime::VirtualMachine;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_hotspot_profiler_creation() {
    let profiler = HotSpotProfiler::new();
    
    assert_eq!(profiler.total_executions(), 0);
    assert_eq!(profiler.hot_functions().len(), 0);
    assert_eq!(profiler.hot_loops().len(), 0);
}

#[test]
fn test_function_execution_counting() {
    let mut profiler = HotSpotProfiler::new();
    
    // Simulate function executions
    profiler.record_function_entry(1); // function id 1
    profiler.record_function_entry(1);
    profiler.record_function_entry(2); // function id 2
    profiler.record_function_entry(1);
    
    assert_eq!(profiler.get_function_count(1), 3);
    assert_eq!(profiler.get_function_count(2), 1);
    assert_eq!(profiler.get_function_count(99), 0); // non-existent function
}

#[test]
fn test_loop_execution_counting() {
    let mut profiler = HotSpotProfiler::new();
    
    // Simulate loop iterations
    for _ in 0..1000 {
        profiler.record_loop_iteration(5); // loop at PC 5
    }
    
    for _ in 0..50 {
        profiler.record_loop_iteration(10); // loop at PC 10
    }
    
    assert_eq!(profiler.get_loop_count(5), 1000);
    assert_eq!(profiler.get_loop_count(10), 50);
}

#[test]
fn test_hot_spot_detection() {
    let mut profiler = HotSpotProfiler::with_thresholds(100, 500); // function_threshold, loop_threshold
    
    // Cold function (below threshold)
    for _ in 0..50 {
        profiler.record_function_entry(1);
    }
    
    // Hot function (above threshold)
    for _ in 0..150 {
        profiler.record_function_entry(2);
    }
    
    // Cold loop
    for _ in 0..200 {
        profiler.record_loop_iteration(10);
    }
    
    // Hot loop
    for _ in 0..600 {
        profiler.record_loop_iteration(20);
    }
    
    let hot_functions = profiler.hot_functions();
    let hot_loops = profiler.hot_loops();
    
    assert_eq!(hot_functions.len(), 1);
    assert!(hot_functions.contains(&2));
    assert!(!hot_functions.contains(&1));
    
    assert_eq!(hot_loops.len(), 1);
    assert!(hot_loops.contains(&20));
    assert!(!hot_loops.contains(&10));
}

#[test]
fn test_type_profiling() {
    let mut profiler = HotSpotProfiler::new();
    
    // Record type observations for instruction at PC 5
    profiler.record_type_observation(5, &Value::Integer(42));
    profiler.record_type_observation(5, &Value::Integer(100));
    profiler.record_type_observation(5, &Value::Float(1.414));
    profiler.record_type_observation(5, &Value::Integer(7));
    
    let type_profile = profiler.get_type_profile(5).unwrap();
    
    assert_eq!(type_profile.total_observations(), 4);
    assert_eq!(type_profile.get_type_frequency("integer"), 3);
    assert_eq!(type_profile.get_type_frequency("float"), 1);
    assert_eq!(type_profile.get_type_frequency("string"), 0);
    
    // Check if this is monomorphic (mostly one type)
    assert!(type_profile.is_monomorphic(0.7)); // 75% integers, should be monomorphic at 70% threshold
}

#[test]
fn test_branch_profiling() {
    let mut profiler = HotSpotProfiler::new();
    
    // Record branch predictions for conditional jump at PC 15
    profiler.record_branch_taken(15, true);
    profiler.record_branch_taken(15, true);
    profiler.record_branch_taken(15, false);
    profiler.record_branch_taken(15, true);
    profiler.record_branch_taken(15, true);
    
    let branch_profile = profiler.get_branch_profile(15).unwrap();
    
    assert_eq!(branch_profile.total_branches(), 5);
    assert_eq!(branch_profile.taken_count(), 4);
    assert_eq!(branch_profile.not_taken_count(), 1);
    assert_eq!(branch_profile.taken_percentage(), 0.8);
    
    // Should predict taken since it's 80% taken
    assert!(branch_profile.predict_taken());
}

#[test]
fn test_profiled_instruction_tracking() {
    let mut profiler = HotSpotProfiler::new();
    
    // Record instruction executions
    profiler.record_instruction_execution(10, Opcode::Add);
    profiler.record_instruction_execution(10, Opcode::Add);
    profiler.record_instruction_execution(11, Opcode::Mul);
    profiler.record_instruction_execution(10, Opcode::Add);
    
    let hot_instructions = profiler.get_hot_instructions(2); // threshold of 2
    
    assert_eq!(hot_instructions.len(), 1);
    assert!(hot_instructions.iter().any(|inst| inst.pc == 10 && inst.opcode == Opcode::Add));
    
    let instruction_profile = profiler.get_instruction_profile(10).unwrap();
    assert_eq!(instruction_profile.execution_count, 3);
    assert_eq!(instruction_profile.opcode, Opcode::Add);
}

#[test]
fn test_optimization_level_selection() {
    let mut profiler = HotSpotProfiler::new();
    
    // Low execution count
    for _ in 0..10 {
        profiler.record_function_entry(1);
    }
    
    // Medium execution count
    for _ in 0..100 {
        profiler.record_function_entry(2);
    }
    
    // High execution count
    for _ in 0..1000 {
        profiler.record_function_entry(3);
    }
    
    // Very high execution count
    for _ in 0..10000 {
        profiler.record_function_entry(4);
    }
    
    assert_eq!(profiler.suggested_optimization_level(1), OptimizationLevel::None);
    assert_eq!(profiler.suggested_optimization_level(2), OptimizationLevel::O1);
    assert_eq!(profiler.suggested_optimization_level(3), OptimizationLevel::O2);
    assert_eq!(profiler.suggested_optimization_level(4), OptimizationLevel::O3);
}

#[test]
fn test_deoptimization_tracking() {
    let mut profiler = HotSpotProfiler::new();
    
    // Record some optimistic compilations that failed
    profiler.record_deoptimization(5, "Type assumption violated");
    profiler.record_deoptimization(5, "Overflow check failed");
    profiler.record_deoptimization(10, "Null check failed");
    
    assert_eq!(profiler.get_deoptimization_count(5), 2);
    assert_eq!(profiler.get_deoptimization_count(10), 1);
    assert_eq!(profiler.get_deoptimization_count(99), 0);
    
    // Function with too many deoptimizations should not be optimized aggressively
    assert!(profiler.should_avoid_optimization(5, 2)); // threshold of 2
    assert!(!profiler.should_avoid_optimization(10, 2));
}

#[test]
fn test_vm_profiling_integration() {
    let mut vm = VirtualMachine::new();
    vm.enable_profiling();
    
    let constants = vec![
        Value::Integer(10),
        Value::Integer(20),
    ];
    
    let instructions = vec![
        // Simple program that executes several instructions 
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // 0: push 10
        Instruction::new(Opcode::Push, Some(Value::Integer(1))), // 1: push 20
        Instruction::new(Opcode::Add, None),                      // 2: add (hot instruction)
        Instruction::new(Opcode::Push, Some(Value::Integer(0))), // 3: push 10 again
        Instruction::new(Opcode::Add, None),                      // 4: add again (another hot instruction)
        Instruction::new(Opcode::Pop, None),                     // 5: discard result
        Instruction::new(Opcode::Halt, None),                    // 6: halt
    ];
    
    vm.load_bytecode_module(instructions, constants).unwrap();
    vm.run().unwrap();
    
    let profiler = vm.get_profiler().unwrap();
    
    // Should have recorded execution of instructions
    assert!(profiler.get_instruction_profile(2).is_some()); // First Add instruction
    assert!(profiler.get_instruction_profile(4).is_some()); // Second Add instruction
    
    // Should detect some instruction execution (low threshold since we only run once)
    let hot_instructions = profiler.get_hot_instructions(1);
    assert!(!hot_instructions.is_empty());
}

#[test]
fn test_profiling_data_export() {
    let mut profiler = HotSpotProfiler::new();
    
    // Generate some profiling data
    for _ in 0..100 {
        profiler.record_function_entry(1);
        profiler.record_loop_iteration(5);
        profiler.record_type_observation(10, &Value::Integer(42));
        profiler.record_branch_taken(15, true);
    }
    
    let profile_data = profiler.export_profile_data();
    
    assert!(profile_data.contains("function_counts"));
    assert!(profile_data.contains("loop_counts"));
    assert!(profile_data.contains("type_profiles"));
    assert!(profile_data.contains("branch_profiles"));
    
    // Should be able to import the data back
    let mut new_profiler = HotSpotProfiler::new();
    new_profiler.import_profile_data(&profile_data).unwrap();
    
    assert_eq!(new_profiler.get_function_count(1), 100);
    assert_eq!(new_profiler.get_loop_count(5), 100);
}

#[test]
fn test_profiling_reset() {
    let mut profiler = HotSpotProfiler::new();
    
    // Generate some data
    profiler.record_function_entry(1);
    profiler.record_loop_iteration(5);
    profiler.record_type_observation(10, &Value::Integer(42));
    
    assert!(profiler.total_executions() > 0);
    
    // Reset and verify clean state
    profiler.reset();
    
    assert_eq!(profiler.total_executions(), 0);
    assert_eq!(profiler.get_function_count(1), 0);
    assert_eq!(profiler.get_loop_count(5), 0);
    assert!(profiler.get_type_profile(10).is_none());
}