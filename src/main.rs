use std::env;
use std::time::Instant;

mod vm;

use vm::{
    runtime::VirtualMachine,
    instruction::{Instruction, Opcode},
    types::Value,
};

fn main() {
    println!("🚀 Stack-Based VM with JIT Compilation System");
    println!("============================================");

    let args: Vec<String> = env::args().collect();
    
    match args.get(1).map(|s| s.as_str()) {
        Some("demo") => run_demo(),
        Some("benchmark") => run_benchmark(),
        Some("fibonacci") => run_fibonacci_program(),
        Some("calculator") => run_calculator_program(),
        Some("profiling") => run_profiling_demo(),
        Some("gc") => run_gc_demo(),
        Some("help") | Some("-h") | Some("--help") => show_help(),
        _ => run_interactive_demo(),
    }
}

fn show_help() {
    println!("Usage: cargo run [COMMAND]");
    println!();
    println!("Commands:");
    println!("  demo         Run interactive demonstration");
    println!("  benchmark    Performance benchmarking");
    println!("  fibonacci    Fibonacci calculation example");
    println!("  calculator   Simple calculator demo");
    println!("  profiling    JIT profiling demonstration");
    println!("  gc           Garbage collection demo");
    println!("  help         Show this help message");
    println!();
    println!("Examples:");
    println!("  cargo run demo");
    println!("  cargo run benchmark");
    println!("  cargo run fibonacci");
}

fn run_interactive_demo() {
    println!("\n🎯 Interactive VM Demonstration");
    println!("-------------------------------");
    
    let mut vm = VirtualMachine::new();
    
    // Simple arithmetic program: (5 + 3) * 2
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(5))),    // Push 5
        Instruction::new(Opcode::Push, Some(Value::Integer(3))),    // Push 3  
        Instruction::new(Opcode::Add, None),                        // Add: 5 + 3 = 8
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),    // Push 2
        Instruction::new(Opcode::Multiply, None),                   // Multiply: 8 * 2 = 16
        Instruction::new(Opcode::Halt, None),                       // Halt
    ];
    
    println!("Program: (5 + 3) * 2");
    println!("Bytecode Instructions:");
    for (i, instr) in program.iter().enumerate() {
        println!("  {}: {:?}", i, instr);
    }
    
    vm.load_program(program);
    
    println!("\n🔄 Execution Trace:");
    println!("Step | PC | Instruction    | Stack State");
    println!("-----|----|--------------  |------------");
    
    let mut step = 0;
    while !vm.is_halted() {
        let pc = vm.program_counter();
        let instruction = vm.current_instruction().unwrap();
        
        print!("{:4} | {:2} | {:14} |", step, pc, format!("{:?}", instruction.opcode()));
        
        match vm.step() {
            Ok(_) => {
                if let Ok(top) = vm.stack_top() {
                    println!(" {:?}", top);
                } else {
                    println!(" [empty]");
                }
            }
            Err(e) => {
                println!(" ERROR: {}", e);
                break;
            }
        }
        
        step += 1;
        if step > 20 { // Safety limit
            println!("Maximum steps reached");
            break;
        }
    }
    
    if let Ok(result) = vm.stack_top() {
        println!("\n✅ Result: {:?}", result);
    }
    
    println!("\n📊 VM Statistics:");
    println!("  Instructions executed: {}", vm.instruction_count());
    println!("  Final stack size: {}", vm.stack_size());
    println!("  Call depth: {}", vm.call_depth());
    println!("  Heap objects: {}", vm.heap_allocated_objects());
}

fn run_fibonacci_program() {
    println!("\n🔢 Fibonacci Calculation Demo");
    println!("-----------------------------");
    
    let mut vm = VirtualMachine::new();
    
    // Calculate fibonacci(10) iteratively
    // Variables: n=10, a=0, b=1, i=0
    let program = vec![
        // Initialize: n=10, a=0, b=1, i=2
        Instruction::new(Opcode::Push, Some(Value::Integer(10))),   // n
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),    // a  
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),    // b
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),    // i
        
        // Loop start (PC=4)
        // Check if i <= n
        Instruction::new(Opcode::Duplicate, None),                  // Dup i
        Instruction::new(Opcode::Push, Some(Value::Integer(4))),    // Push n index
        Instruction::new(Opcode::LoadLocal, None),                  // Load n from stack position
        Instruction::new(Opcode::LessOrEqual, None),                // i <= n
        Instruction::new(Opcode::JumpIfFalse, Some(Value::Integer(16))), // Jump to end if false
        
        // Fibonacci step: temp = a + b, a = b, b = temp
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),    // Index for a
        Instruction::new(Opcode::LoadLocal, None),                  // Load a
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),    // Index for b  
        Instruction::new(Opcode::LoadLocal, None),                  // Load b
        Instruction::new(Opcode::Add, None),                        // temp = a + b
        
        // i = i + 1 and loop
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),    // Increment
        Instruction::new(Opcode::Add, None),                        // i++
        Instruction::new(Opcode::Jump, Some(Value::Integer(4))),    // Jump to loop start
        
        // End: result is in b
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),    // Index for b
        Instruction::new(Opcode::LoadLocal, None),                  // Load result
        Instruction::new(Opcode::Halt, None),
    ];
    
    println!("Calculating Fibonacci(10) using iterative approach");
    println!("Program loaded with {} instructions", program.len());
    
    vm.load_program(program);
    
    let start_time = Instant::now();
    
    match vm.run() {
        Ok(_) => {
            let duration = start_time.elapsed();
            
            if let Ok(result) = vm.stack_top() {
                println!("✅ Fibonacci(10) = {:?}", result);
                println!("   Expected: 55");
            }
            
            println!("\n📊 Performance Metrics:");
            println!("  Execution time: {:?}", duration);
            println!("  Instructions executed: {}", vm.instruction_count());
            println!("  Final stack size: {}", vm.stack_size());
            println!("  Memory usage: {} objects, {} bytes", 
                     vm.heap_allocated_objects(), vm.heap_total_bytes());
        }
        Err(e) => {
            println!("❌ Execution failed: {}", e);
        }
    }
}

fn run_calculator_program() {
    println!("\n🧮 Calculator Demo");
    println!("------------------");
    
    let mut vm = VirtualMachine::new();
    
    // Calculate: ((10 + 5) * 3) - (8 / 2) = 45 - 4 = 41
    let program = vec![
        // Left side: (10 + 5) * 3
        Instruction::new(Opcode::Push, Some(Value::Integer(10))),
        Instruction::new(Opcode::Push, Some(Value::Integer(5))),
        Instruction::new(Opcode::Add, None),                        // 15
        Instruction::new(Opcode::Push, Some(Value::Integer(3))),
        Instruction::new(Opcode::Multiply, None),                   // 45
        
        // Right side: 8 / 2
        Instruction::new(Opcode::Push, Some(Value::Integer(8))),
        Instruction::new(Opcode::Push, Some(Value::Integer(2))),
        Instruction::new(Opcode::Divide, None),                     // 4
        
        // Final calculation: 45 - 4
        Instruction::new(Opcode::Subtract, None),                   // 41
        Instruction::new(Opcode::Halt, None),
    ];
    
    println!("Expression: ((10 + 5) * 3) - (8 / 2)");
    println!("Expected result: 41");
    
    vm.load_program(program);
    
    match vm.run() {
        Ok(_) => {
            if let Ok(result) = vm.stack_top() {
                println!("✅ Calculated result: {:?}", result);
            }
            println!("📊 Instructions executed: {}", vm.instruction_count());
        }
        Err(e) => {
            println!("❌ Calculation failed: {}", e);
        }
    }
}

fn run_benchmark() {
    println!("\n⚡ Performance Benchmark");
    println!("------------------------");
    
    let iterations = vec![1_000, 10_000, 100_000];
    
    for &iter_count in &iterations {
        println!("\n🔄 Testing with {} iterations", iter_count);
        
        let mut vm = VirtualMachine::new();
        
        // Simple loop that decrements a counter
        let mut program = vec![
            Instruction::new(Opcode::Push, Some(Value::Integer(iter_count))), // Counter
        ];
        
        // Loop: decrement counter, check if > 0, continue or exit
        for _ in 0..5 { // Unroll loop slightly for more instructions
            program.extend(vec![
                Instruction::new(Opcode::Push, Some(Value::Integer(1))),
                Instruction::new(Opcode::Subtract, None),                   // counter--
                Instruction::new(Opcode::Duplicate, None),                  // Dup counter
                Instruction::new(Opcode::Push, Some(Value::Integer(0))),
                Instruction::new(Opcode::Greater, None),                    // counter > 0
                Instruction::new(Opcode::JumpIfTrue, Some(Value::Integer(1))), // Loop if true
            ]);
        }
        
        program.push(Instruction::new(Opcode::Halt, None));
        
        vm.load_program(program);
        
        let start_time = Instant::now();
        
        match vm.run() {
            Ok(_) => {
                let duration = start_time.elapsed();
                let instructions_per_second = vm.instruction_count() as f64 / duration.as_secs_f64();
                
                println!("  ✅ Completed in {:?}", duration);
                println!("  📊 Instructions executed: {}", vm.instruction_count());
                println!("  🚀 Performance: {:.0} instructions/second", instructions_per_second);
            }
            Err(e) => {
                println!("  ❌ Benchmark failed: {}", e);
            }
        }
    }
}

fn run_profiling_demo() {
    println!("\n📈 JIT Profiling Demonstration");
    println!("-------------------------------");
    
    let mut vm = VirtualMachine::new();
    vm.enable_profiling();
    
    // Program with a loop that will be detected as a hot spot
    let program = vec![
        Instruction::new(Opcode::Push, Some(Value::Integer(100))),  // Counter
        // Hot loop starts here (PC=1)
        Instruction::new(Opcode::Duplicate, None),                  // Dup counter
        Instruction::new(Opcode::Push, Some(Value::Integer(0))),
        Instruction::new(Opcode::Greater, None),                    // counter > 0
        Instruction::new(Opcode::JumpIfFalse, Some(Value::Integer(8))), // Exit if false
        Instruction::new(Opcode::Push, Some(Value::Integer(1))),
        Instruction::new(Opcode::Subtract, None),                   // counter--
        Instruction::new(Opcode::Jump, Some(Value::Integer(1))),    // Jump back to loop
        // Loop ends here
        Instruction::new(Opcode::Halt, None),
    ];
    
    println!("Running program with profiling enabled...");
    vm.load_program(program);
    
    let start_time = Instant::now();
    
    match vm.run() {
        Ok(_) => {
            let duration = start_time.elapsed();
            
            println!("✅ Execution completed in {:?}", duration);
            println!("📊 Total instructions: {}", vm.instruction_count());
            
            if let Some(profiler) = vm.get_profiler() {
                println!("\n🔥 Hot Spot Analysis:");
                let hot_spots = profiler.get_hot_spots(10); // Get top 10
                
                for (pc, count) in hot_spots {
                    let percentage = (count as f64 / vm.instruction_count() as f64) * 100.0;
                    println!("  PC {:2}: {:8} executions ({:.1}%)", pc, count, percentage);
                }
                
                println!("\n🎯 JIT Compilation Candidates:");
                let candidates = profiler.get_compilation_candidates();
                for pc in candidates {
                    println!("  PC {} is ready for JIT compilation", pc);
                }
            }
        }
        Err(e) => {
            println!("❌ Profiling demo failed: {}", e);
        }
    }
}

fn run_gc_demo() {
    println!("\n🗑️ Garbage Collection Demo");
    println!("---------------------------");
    
    let mut vm = VirtualMachine::new();
    
    // Program that allocates some objects
    let program = vec![
        // Simulate object allocation
        Instruction::new(Opcode::NewObject, Some(Value::Integer(100))), // Allocate object
        Instruction::new(Opcode::NewObject, Some(Value::Integer(200))), // Allocate another
        Instruction::new(Opcode::NewObject, Some(Value::Integer(300))), // And another
        
        // Pop one reference (making first object collectible)
        Instruction::new(Opcode::Pop, None),
        
        Instruction::new(Opcode::Halt, None),
    ];
    
    println!("Running program that allocates objects...");
    vm.load_program(program);
    
    println!("Initial heap state:");
    println!("  Objects: {}", vm.heap_allocated_objects());
    println!("  Bytes: {}", vm.heap_total_bytes());
    
    match vm.run() {
        Ok(_) => {
            println!("\nAfter execution:");
            println!("  Objects: {}", vm.heap_allocated_objects());
            println!("  Bytes: {}", vm.heap_total_bytes());
            
            println!("\n🧹 Triggering garbage collection...");
            let collected = vm.trigger_gc();
            
            println!("After GC:");
            println!("  Objects: {}", vm.heap_allocated_objects());
            println!("  Bytes: {}", vm.heap_total_bytes());
            println!("  Collected: {} objects", collected);
        }
        Err(e) => {
            println!("❌ GC demo failed: {}", e);
        }
    }
}

fn run_demo() {
    println!("\n🎪 Complete VM Feature Demonstration");
    println!("====================================");
    
    println!("\n1. Basic Arithmetic");
    run_interactive_demo();
    
    println!("\n2. Complex Algorithm");
    run_fibonacci_program();
    
    println!("\n3. Expression Evaluation");
    run_calculator_program();
    
    println!("\n4. Performance Analysis");
    run_profiling_demo();
    
    println!("\n5. Memory Management");
    run_gc_demo();
    
    println!("\n🎉 Demo completed! Try 'cargo run help' for individual examples.");
}
