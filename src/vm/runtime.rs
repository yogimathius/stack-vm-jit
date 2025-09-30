use crate::vm::call_frame::CallStack;
use crate::vm::heap::Heap;
use crate::vm::instruction::{ExecutionError, Instruction, InstructionDispatcher, Opcode};
use crate::vm::jit::HotSpotProfiler;
use crate::vm::stack::OperandStack;
use crate::vm::types::Value;
use std::fmt;

#[derive(Debug)]
pub enum VmError {
    ExecutionError(ExecutionError),
    ProgramCounterOutOfBounds(usize, usize), // pc, program_length
    InvalidProgramState(String),
    NoProgram,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::ExecutionError(e) => write!(f, "Execution error: {}", e),
            VmError::ProgramCounterOutOfBounds(pc, len) => {
                write!(
                    f,
                    "Program counter {} out of bounds (program length: {})",
                    pc, len
                )
            }
            VmError::InvalidProgramState(msg) => write!(f, "Invalid program state: {}", msg),
            VmError::NoProgram => write!(f, "No program loaded"),
        }
    }
}

impl std::error::Error for VmError {}

impl From<ExecutionError> for VmError {
    fn from(err: ExecutionError) -> Self {
        VmError::ExecutionError(err)
    }
}

pub struct VirtualMachine {
    operand_stack: OperandStack,
    call_stack: CallStack,
    dispatcher: InstructionDispatcher,
    program: Vec<Instruction>,
    constants: Vec<Value>,
    heap: Heap,
    profiler: Option<HotSpotProfiler>,
    halted: bool,
    max_instructions: u64,
}

impl VirtualMachine {
    const DEFAULT_MAX_INSTRUCTIONS: u64 = 1_000_000; // Prevent infinite loops

    pub fn new() -> Self {
        Self {
            operand_stack: OperandStack::new(),
            call_stack: CallStack::new(),
            dispatcher: InstructionDispatcher::new(),
            program: Vec::new(),
            constants: Vec::new(),
            heap: Heap::new(),
            profiler: None,
            halted: false,
            max_instructions: Self::DEFAULT_MAX_INSTRUCTIONS,
        }
    }

    pub fn with_max_instructions(max_instructions: u64) -> Self {
        Self {
            operand_stack: OperandStack::new(),
            call_stack: CallStack::new(),
            dispatcher: InstructionDispatcher::new(),
            program: Vec::new(),
            constants: Vec::new(),
            heap: Heap::new(),
            profiler: None,
            halted: false,
            max_instructions,
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.operand_stack.clear();
        self.call_stack.clear();
        self.dispatcher = InstructionDispatcher::new();
        self.halted = false;
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        if self.program.is_empty() {
            return Err(VmError::NoProgram);
        }

        while !self.halted && self.dispatcher.instruction_count() < self.max_instructions {
            self.step()?;
        }

        if self.dispatcher.instruction_count() >= self.max_instructions {
            return Err(VmError::InvalidProgramState(
                "Maximum instruction count exceeded".to_string(),
            ));
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.halted {
            return Ok(());
        }

        if self.program.is_empty() {
            return Err(VmError::NoProgram);
        }

        let pc = self.dispatcher.current_pc();

        if pc >= self.program.len() {
            return Err(VmError::ProgramCounterOutOfBounds(pc, self.program.len()));
        }

        let instruction = &self.program[pc].clone();

        // Handle halt instruction specially
        if instruction.opcode() == Opcode::Halt {
            self.halted = true;
            return Ok(());
        }

        // Profile the instruction execution if profiling is enabled
        if let Some(ref mut profiler) = self.profiler {
            profiler.record_instruction_execution(pc, instruction.opcode());
        }

        // Execute instruction
        self.dispatcher
            .execute_with_constants(instruction, &mut self.operand_stack, &mut self.call_stack, &self.constants, &mut self.heap)?;

        // For control flow instructions, PC is handled by the instruction itself
        // For all other instructions, increment PC
        match instruction.opcode() {
            Opcode::Jump
            | Opcode::JumpIfTrue
            | Opcode::JumpIfFalse
            | Opcode::Call
            | Opcode::Return => {
                // Control flow instructions manage their own PC
            }
            _ => {
                // Regular instructions: increment PC
                self.dispatcher.set_pc(pc + 1);
            }
        }

        Ok(())
    }

    // Public interface methods
    pub fn stack_size(&self) -> usize {
        self.operand_stack.size()
    }

    pub fn call_depth(&self) -> usize {
        self.call_stack.depth()
    }

    pub fn program_counter(&self) -> usize {
        self.dispatcher.current_pc()
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn stack_top(&self) -> Result<&Value, VmError> {
        self.operand_stack
            .peek()
            .map_err(|e| VmError::ExecutionError(ExecutionError::StackError(e)))
    }

    pub fn instruction_count(&self) -> u64 {
        self.dispatcher.instruction_count()
    }

    pub fn program_length(&self) -> usize {
        self.program.len()
    }

    pub fn constants_pool_size(&self) -> usize {
        self.constants.len()
    }

    pub fn load_bytecode_module(
        &mut self, 
        instructions: Vec<Instruction>, 
        constants: Vec<Value>
    ) -> Result<(), VmError> {
        if instructions.is_empty() {
            return Err(VmError::InvalidProgramState(
                "Cannot load empty instruction list".to_string()
            ));
        }
        
        self.program = instructions;
        self.constants = constants;
        self.reset();
        Ok(())
    }

    pub fn get_constant(&self, index: usize) -> Result<&Value, VmError> {
        self.constants
            .get(index)
            .ok_or_else(|| VmError::InvalidProgramState(
                format!("Constant index {} out of bounds (pool size: {})", 
                        index, self.constants.len())
            ))
    }

    pub fn heap_allocated_objects(&self) -> usize {
        self.heap.allocated_objects()
    }

    pub fn heap_total_bytes(&self) -> usize {
        self.heap.total_allocated_bytes()
    }

    pub fn trigger_gc(&mut self) -> usize {
        // Simple GC trigger - in a real implementation, this would trace all roots
        self.heap.collect_garbage::<String>(&[])
    }

    // Debug methods
    pub fn stack_contents(&self) -> Vec<Value> {
        // For debugging, we'll return an empty vec for now
        // In a real implementation, we'd add proper introspection to OperandStack
        Vec::new()
    }

    pub fn current_instruction(&self) -> Option<&Instruction> {
        let pc = self.dispatcher.current_pc();
        self.program.get(pc)
    }

    // Profiling methods
    pub fn enable_profiling(&mut self) {
        self.profiler = Some(HotSpotProfiler::new());
    }

    pub fn disable_profiling(&mut self) {
        self.profiler = None;
    }

    pub fn is_profiling_enabled(&self) -> bool {
        self.profiler.is_some()
    }

    pub fn get_profiler(&self) -> Option<&HotSpotProfiler> {
        self.profiler.as_ref()
    }

    pub fn get_profiler_mut(&mut self) -> Option<&mut HotSpotProfiler> {
        self.profiler.as_mut()
    }

    pub fn reset_profiler(&mut self) {
        if let Some(ref mut profiler) = self.profiler {
            profiler.reset();
        }
    }
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_empty_program() {
        let mut vm = VirtualMachine::new();
        let result = vm.run();
        assert!(result.is_err());

        // But step should not fail on empty program
        let result = vm.step();
        assert!(result.is_err());
    }

    #[test]
    fn test_vm_program_counter_bounds() {
        let mut vm = VirtualMachine::new();

        // Load a single instruction program
        let program = vec![Instruction::new(Opcode::Push, Some(Value::Integer(1)))];

        vm.load_program(program);
        vm.step().unwrap(); // Execute the push

        // PC should now be 1, trying to step should fail (no halt instruction)
        let result = vm.step();
        assert!(result.is_err());
    }

    #[test]
    fn test_vm_max_instructions() {
        let mut vm = VirtualMachine::with_max_instructions(3);

        // Infinite loop program
        let program = vec![
            Instruction::new(Opcode::Jump, Some(Value::Integer(0))), // Jump to self
        ];

        vm.load_program(program);
        let result = vm.run();
        assert!(result.is_err());
        assert!(vm.instruction_count() >= 3);
    }
}
