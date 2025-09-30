use crate::vm::call_frame::{CallFrame, CallFrameError, CallStack};
use crate::vm::heap::{Heap, Object};
use crate::vm::stack::{OperandStack, StackError};
use crate::vm::types::Value;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // Arithmetic operations
    Add = 0x01,
    Sub = 0x02,
    Mul = 0x03,
    Div = 0x04,
    Mod = 0x05,

    // Stack operations
    Push = 0x10,
    Pop = 0x11,
    Dup = 0x12,
    Swap = 0x13,

    // Control flow
    Jump = 0x20,
    JumpIfTrue = 0x21,
    JumpIfFalse = 0x22,
    Call = 0x23,
    Return = 0x24,

    // Comparison operations
    Equal = 0x30,
    NotEqual = 0x31,
    LessThan = 0x32,
    LessEqual = 0x33,
    GreaterThan = 0x34,
    GreaterEqual = 0x35,

    // Logical operations
    And = 0x40,
    Or = 0x41,
    Not = 0x42,
    Xor = 0x43,

    // Memory operations
    Load = 0x50,
    Store = 0x51,
    NewObject = 0x52,
    GetField = 0x53,
    SetField = 0x54,

    // Halt/Debug
    Halt = 0xFF,
}

impl Opcode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Opcode::Add),
            0x02 => Some(Opcode::Sub),
            0x03 => Some(Opcode::Mul),
            0x04 => Some(Opcode::Div),
            0x05 => Some(Opcode::Mod),
            0x10 => Some(Opcode::Push),
            0x11 => Some(Opcode::Pop),
            0x12 => Some(Opcode::Dup),
            0x13 => Some(Opcode::Swap),
            0x20 => Some(Opcode::Jump),
            0x21 => Some(Opcode::JumpIfTrue),
            0x22 => Some(Opcode::JumpIfFalse),
            0x23 => Some(Opcode::Call),
            0x24 => Some(Opcode::Return),
            0x30 => Some(Opcode::Equal),
            0x31 => Some(Opcode::NotEqual),
            0x32 => Some(Opcode::LessThan),
            0x33 => Some(Opcode::LessEqual),
            0x34 => Some(Opcode::GreaterThan),
            0x35 => Some(Opcode::GreaterEqual),
            0x40 => Some(Opcode::And),
            0x41 => Some(Opcode::Or),
            0x42 => Some(Opcode::Not),
            0x43 => Some(Opcode::Xor),
            0x50 => Some(Opcode::Load),
            0x51 => Some(Opcode::Store),
            0x52 => Some(Opcode::NewObject),
            0x53 => Some(Opcode::GetField),
            0x54 => Some(Opcode::SetField),
            0xFF => Some(Opcode::Halt),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    opcode: Opcode,
    operand: Option<Value>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operand: Option<Value>) -> Self {
        Self { opcode, operand }
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn operand(&self) -> Option<&Value> {
        self.operand.as_ref()
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    StackError(StackError),
    CallFrameError(CallFrameError),
    TypeError(String),
    DivisionByZero,
    InvalidJumpAddress(i64),
    UnknownOpcode(u8),
    InsufficientOperands,
    InvalidOperand(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::StackError(e) => write!(f, "Stack error: {}", e),
            ExecutionError::CallFrameError(e) => write!(f, "Call frame error: {}", e),
            ExecutionError::TypeError(msg) => write!(f, "Type error: {}", msg),
            ExecutionError::DivisionByZero => write!(f, "Division by zero"),
            ExecutionError::InvalidJumpAddress(addr) => write!(f, "Invalid jump address: {}", addr),
            ExecutionError::UnknownOpcode(code) => write!(f, "Unknown opcode: 0x{:02X}", code),
            ExecutionError::InsufficientOperands => write!(f, "Insufficient operands on stack"),
            ExecutionError::InvalidOperand(msg) => write!(f, "Invalid operand: {}", msg),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl From<StackError> for ExecutionError {
    fn from(err: StackError) -> Self {
        ExecutionError::StackError(err)
    }
}

impl From<CallFrameError> for ExecutionError {
    fn from(err: CallFrameError) -> Self {
        ExecutionError::CallFrameError(err)
    }
}

pub struct InstructionDispatcher {
    program_counter: usize,
    instruction_count: u64,
    branch_predictions: std::collections::HashMap<usize, bool>,
}

impl InstructionDispatcher {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            instruction_count: 0,
            branch_predictions: std::collections::HashMap::new(),
        }
    }

    pub fn current_pc(&self) -> usize {
        self.program_counter
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.program_counter = pc;
    }

    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }

    pub fn record_branch_prediction(&mut self, pc: usize, taken: bool) {
        self.branch_predictions.insert(pc, taken);
    }

    pub fn get_branch_prediction(&self, pc: usize) -> Option<bool> {
        self.branch_predictions.get(&pc).copied()
    }

    pub fn execute_with_constants(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
        call_stack: &mut CallStack,
        constants: &[Value],
        heap: &mut Heap,
    ) -> Result<(), ExecutionError> {
        self.instruction_count += 1;

        match instruction.opcode() {
            // Arithmetic operations
            Opcode::Add => self.execute_add(stack),
            Opcode::Sub => self.execute_sub(stack),
            Opcode::Mul => self.execute_mul(stack),
            Opcode::Div => self.execute_div(stack),
            Opcode::Mod => self.execute_mod(stack),

            // Stack operations
            Opcode::Push => self.execute_push_with_constants(instruction, stack, constants),
            Opcode::Pop => self.execute_pop(stack),
            Opcode::Dup => self.execute_dup(stack),
            Opcode::Swap => self.execute_swap(stack),

            // Control flow
            Opcode::Jump => self.execute_jump(instruction),
            Opcode::JumpIfTrue => self.execute_jump_if_true(instruction, stack),
            Opcode::JumpIfFalse => self.execute_jump_if_false(instruction, stack),
            Opcode::Call => self.execute_call(instruction, call_stack),
            Opcode::Return => self.execute_return(call_stack),

            // Comparison operations
            Opcode::Equal => self.execute_equal(stack),
            Opcode::NotEqual => self.execute_not_equal(stack),
            Opcode::LessThan => self.execute_less_than(stack),
            Opcode::LessEqual => self.execute_less_equal(stack),
            Opcode::GreaterThan => self.execute_greater_than(stack),
            Opcode::GreaterEqual => self.execute_greater_equal(stack),

            // Logical operations
            Opcode::And => self.execute_and(stack),
            Opcode::Or => self.execute_or(stack),
            Opcode::Not => self.execute_not(stack),
            Opcode::Xor => self.execute_xor(stack),

            // Memory operations
            Opcode::Load => self.execute_load(instruction, stack, call_stack),
            Opcode::Store => self.execute_store(instruction, stack, call_stack),
            Opcode::NewObject => self.execute_new_object(stack, heap),
            Opcode::GetField => self.execute_get_field(instruction, stack),
            Opcode::SetField => self.execute_set_field(instruction, stack),

            Opcode::Halt => Ok(()),
        }
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
        call_stack: &mut CallStack,
    ) -> Result<(), ExecutionError> {
        self.instruction_count += 1;

        match instruction.opcode() {
            // Arithmetic operations
            Opcode::Add => self.execute_add(stack),
            Opcode::Sub => self.execute_sub(stack),
            Opcode::Mul => self.execute_mul(stack),
            Opcode::Div => self.execute_div(stack),
            Opcode::Mod => self.execute_mod(stack),

            // Stack operations
            Opcode::Push => self.execute_push(instruction, stack),
            Opcode::Pop => self.execute_pop(stack),
            Opcode::Dup => self.execute_dup(stack),
            Opcode::Swap => self.execute_swap(stack),

            // Control flow
            Opcode::Jump => self.execute_jump(instruction),
            Opcode::JumpIfTrue => self.execute_jump_if_true(instruction, stack),
            Opcode::JumpIfFalse => self.execute_jump_if_false(instruction, stack),
            Opcode::Call => self.execute_call(instruction, call_stack),
            Opcode::Return => self.execute_return(call_stack),

            // Comparison operations
            Opcode::Equal => self.execute_equal(stack),
            Opcode::NotEqual => self.execute_not_equal(stack),
            Opcode::LessThan => self.execute_less_than(stack),
            Opcode::LessEqual => self.execute_less_equal(stack),
            Opcode::GreaterThan => self.execute_greater_than(stack),
            Opcode::GreaterEqual => self.execute_greater_equal(stack),

            // Logical operations
            Opcode::And => self.execute_and(stack),
            Opcode::Or => self.execute_or(stack),
            Opcode::Not => self.execute_not(stack),
            Opcode::Xor => self.execute_xor(stack),

            // Memory operations
            Opcode::Load => self.execute_load(instruction, stack, call_stack),
            Opcode::Store => self.execute_store(instruction, stack, call_stack),
            Opcode::NewObject => Err(ExecutionError::InvalidOperand(
                "NewObject requires heap access - use execute_with_constants".to_string()
            )),
            Opcode::GetField => self.execute_get_field(instruction, stack),
            Opcode::SetField => self.execute_set_field(instruction, stack),

            Opcode::Halt => Ok(()),
        }
    }

    // Arithmetic implementations
    fn execute_add(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot add these types".to_string(),
                ));
            }
        };

        stack.push(result);
        Ok(())
    }

    fn execute_sub(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 - b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a - b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot subtract these types".to_string(),
                ));
            }
        };

        stack.push(result);
        Ok(())
    }

    fn execute_mul(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a * b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot multiply these types".to_string(),
                ));
            }
        };

        stack.push(result);
        Ok(())
    }

    fn execute_div(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b == 0 {
                    return Err(ExecutionError::DivisionByZero);
                }
                Value::Integer(a / b)
            }
            (Value::Float(a), Value::Float(b)) => {
                if b == 0.0 {
                    return Err(ExecutionError::DivisionByZero);
                }
                Value::Float(a / b)
            }
            (Value::Integer(a), Value::Float(b)) => {
                if b == 0.0 {
                    return Err(ExecutionError::DivisionByZero);
                }
                Value::Float(a as f64 / b)
            }
            (Value::Float(a), Value::Integer(b)) => {
                if b == 0 {
                    return Err(ExecutionError::DivisionByZero);
                }
                Value::Float(a / b as f64)
            }
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot divide these types".to_string(),
                ));
            }
        };

        stack.push(result);
        Ok(())
    }

    fn execute_mod(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => {
                if b == 0 {
                    return Err(ExecutionError::DivisionByZero);
                }
                Value::Integer(a % b)
            }
            _ => {
                return Err(ExecutionError::TypeError(
                    "Modulo only supported for integers".to_string(),
                ));
            }
        };

        stack.push(result);
        Ok(())
    }

    // Stack operations
    fn execute_push(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
    ) -> Result<(), ExecutionError> {
        if let Some(value) = instruction.operand() {
            stack.push(value.clone());
        } else {
            return Err(ExecutionError::InsufficientOperands);
        }
        Ok(())
    }

    fn execute_push_with_constants(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
        constants: &[Value],
    ) -> Result<(), ExecutionError> {
        match instruction.operand() {
            Some(Value::Integer(index)) => {
                // If constants pool is empty, treat as literal value for backward compatibility
                if constants.is_empty() {
                    stack.push(Value::Integer(*index));
                    return Ok(());
                }
                
                // Push from constants pool
                let const_index = *index as usize;
                if const_index >= constants.len() {
                    return Err(ExecutionError::InvalidOperand(
                        format!("Constant index {} out of bounds (pool size: {})", 
                                const_index, constants.len())
                    ));
                }
                stack.push(constants[const_index].clone());
                Ok(())
            }
            Some(value) => {
                // Push literal value
                stack.push(value.clone());
                Ok(())
            }
            None => Err(ExecutionError::InsufficientOperands),
        }
    }

    fn execute_pop(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        stack.pop()?;
        Ok(())
    }

    fn execute_dup(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let value = stack.peek()?.clone();
        stack.push(value);
        Ok(())
    }

    fn execute_swap(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let a = stack.pop()?;
        let b = stack.pop()?;
        stack.push(a);
        stack.push(b);
        Ok(())
    }

    // Control flow
    fn execute_jump(&mut self, instruction: &Instruction) -> Result<(), ExecutionError> {
        if let Some(Value::Integer(addr)) = instruction.operand() {
            if *addr < 0 {
                return Err(ExecutionError::InvalidJumpAddress(*addr));
            }
            self.program_counter = *addr as usize;
        } else {
            return Err(ExecutionError::InsufficientOperands);
        }
        Ok(())
    }

    fn execute_jump_if_true(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
    ) -> Result<(), ExecutionError> {
        let condition = stack.pop()?;
        if condition.is_truthy() {
            self.execute_jump(instruction)?;
        }
        Ok(())
    }

    fn execute_jump_if_false(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
    ) -> Result<(), ExecutionError> {
        let condition = stack.pop()?;
        if !condition.is_truthy() {
            self.execute_jump(instruction)?;
        }
        Ok(())
    }

    fn execute_call(
        &mut self,
        instruction: &Instruction,
        call_stack: &mut CallStack,
    ) -> Result<(), ExecutionError> {
        if let Some(Value::Integer(function_addr)) = instruction.operand() {
            if *function_addr < 0 {
                return Err(ExecutionError::InvalidJumpAddress(*function_addr));
            }
            let return_addr = self.program_counter + 1;
            let frame = CallFrame::new(*function_addr as usize, return_addr, 0);
            call_stack.push_unchecked(frame);
            // Jump to the function address
            self.program_counter = *function_addr as usize;
        } else {
            return Err(ExecutionError::InsufficientOperands);
        }
        Ok(())
    }

    fn execute_return(&mut self, call_stack: &mut CallStack) -> Result<(), ExecutionError> {
        let frame = call_stack.pop()?;
        self.program_counter = frame.return_address();
        Ok(())
    }

    // Comparison operations
    fn execute_equal(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        stack.push(Value::Boolean(a == b));
        Ok(())
    }

    fn execute_not_equal(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        stack.push(Value::Boolean(a != b));
        Ok(())
    }

    fn execute_less_than(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a < b,
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::Integer(a), Value::Float(b)) => (a as f64) < b,
            (Value::Float(a), Value::Integer(b)) => a < (b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compare these types".to_string(),
                ));
            }
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_less_equal(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a <= b,
            (Value::Float(a), Value::Float(b)) => a <= b,
            (Value::Integer(a), Value::Float(b)) => (a as f64) <= b,
            (Value::Float(a), Value::Integer(b)) => a <= (b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compare these types".to_string(),
                ));
            }
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_greater_than(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a > b,
            (Value::Float(a), Value::Float(b)) => a > b,
            (Value::Integer(a), Value::Float(b)) => (a as f64) > b,
            (Value::Float(a), Value::Integer(b)) => a > (b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compare these types".to_string(),
                ));
            }
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    fn execute_greater_equal(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;

        let result = match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => a >= b,
            (Value::Float(a), Value::Float(b)) => a >= b,
            (Value::Integer(a), Value::Float(b)) => (a as f64) >= b,
            (Value::Float(a), Value::Integer(b)) => a >= (b as f64),
            _ => {
                return Err(ExecutionError::TypeError(
                    "Cannot compare these types".to_string(),
                ));
            }
        };

        stack.push(Value::Boolean(result));
        Ok(())
    }

    // Logical operations
    fn execute_and(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        stack.push(Value::Boolean(a.is_truthy() && b.is_truthy()));
        Ok(())
    }

    fn execute_or(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        stack.push(Value::Boolean(a.is_truthy() || b.is_truthy()));
        Ok(())
    }

    fn execute_not(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let a = stack.pop()?;
        stack.push(Value::Boolean(!a.is_truthy()));
        Ok(())
    }

    fn execute_xor(&mut self, stack: &mut OperandStack) -> Result<(), ExecutionError> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        stack.push(Value::Boolean(a.is_truthy() != b.is_truthy()));
        Ok(())
    }

    // Memory operations
    fn execute_load(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
        call_stack: &mut CallStack,
    ) -> Result<(), ExecutionError> {
        let local_index = match instruction.operand() {
            Some(Value::Integer(index)) => *index as usize,
            Some(_) => {
                return Err(ExecutionError::InvalidOperand(
                    "Load instruction requires integer operand".to_string(),
                ))
            }
            None => {
                return Err(ExecutionError::InvalidOperand(
                    "Load instruction requires operand".to_string(),
                ))
            }
        };

        let current_frame = call_stack.current()?;

        let value = current_frame.get_local(local_index)?;
        stack.push(value.clone());
        Ok(())
    }

    fn execute_store(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
        call_stack: &mut CallStack,
    ) -> Result<(), ExecutionError> {
        let local_index = match instruction.operand() {
            Some(Value::Integer(index)) => *index as usize,
            Some(_) => {
                return Err(ExecutionError::InvalidOperand(
                    "Store instruction requires integer operand".to_string(),
                ))
            }
            None => {
                return Err(ExecutionError::InvalidOperand(
                    "Store instruction requires operand".to_string(),
                ))
            }
        };

        let value = stack.pop()?;

        let current_frame = call_stack.current_mut()?;

        current_frame.set_local(local_index, value)?;
        Ok(())
    }

    fn execute_new_object(
        &mut self,
        stack: &mut OperandStack,
        heap: &mut Heap,
    ) -> Result<(), ExecutionError> {
        // Create a new empty object and allocate it on the heap
        let object = Object::new();
        
        match heap.allocate_object(object) {
            Ok(gc_object) => {
                stack.push(Value::GcObject(gc_object));
                Ok(())
            }
            Err(heap_error) => {
                Err(ExecutionError::InvalidOperand(
                    format!("Failed to allocate object: {}", heap_error)
                ))
            }
        }
    }

    fn execute_get_field(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
    ) -> Result<(), ExecutionError> {
        // Get field name from instruction operand
        let field_name = match instruction.operand() {
            Some(Value::String(name)) => name.clone(),
            Some(Value::Integer(index)) => format!("field_{}", index), // Support numeric field names
            Some(_) => {
                return Err(ExecutionError::InvalidOperand(
                    "GetField instruction requires string or integer operand".to_string(),
                ))
            }
            None => {
                return Err(ExecutionError::InvalidOperand(
                    "GetField instruction requires operand".to_string(),
                ))
            }
        };

        // Pop object from stack
        let object = stack.pop()?;
        
        match object {
            Value::GcObject(gc_obj) => {
                // Get field value from object
                if let Some(field_value) = gc_obj.get_field(&field_name) {
                    stack.push(field_value.clone());
                } else {
                    // Field doesn't exist, push null
                    stack.push(Value::Null);
                }
                Ok(())
            }
            _ => Err(ExecutionError::TypeError(
                "GetField can only be used on objects".to_string()
            ))
        }
    }

    fn execute_set_field(
        &mut self,
        instruction: &Instruction,
        stack: &mut OperandStack,
    ) -> Result<(), ExecutionError> {
        // Get field name from instruction operand
        let _field_name = match instruction.operand() {
            Some(Value::String(name)) => name.clone(),
            Some(Value::Integer(index)) => format!("field_{}", index), // Support numeric field names
            Some(_) => {
                return Err(ExecutionError::InvalidOperand(
                    "SetField instruction requires string or integer operand".to_string(),
                ))
            }
            None => {
                return Err(ExecutionError::InvalidOperand(
                    "SetField instruction requires operand".to_string(),
                ))
            }
        };

        // Pop value and object from stack
        let value = stack.pop()?;
        let object = stack.pop()?;

        match object {
            Value::GcObject(gc_obj) => {
                // Unfortunately, we can't mutate through GcPtr directly due to shared ownership
                // In a real implementation, this would require interior mutability (RefCell/Mutex)
                // For now, we'll push the object back and return an error explaining this limitation
                stack.push(Value::GcObject(gc_obj));
                stack.push(value);
                
                Err(ExecutionError::InvalidOperand(
                    "SetField not yet implemented - requires interior mutability in GcPtr".to_string()
                ))
            }
            _ => {
                // Push values back in reverse order
                stack.push(object);
                stack.push(value);
                Err(ExecutionError::TypeError(
                    "SetField can only be used on objects".to_string()
                ))
            }
        }
    }
}

impl Default for InstructionDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_roundtrip() {
        let opcodes = [
            Opcode::Add,
            Opcode::Sub,
            Opcode::Push,
            Opcode::Jump,
            Opcode::Equal,
        ];

        for opcode in opcodes {
            let byte = opcode as u8;
            let restored = Opcode::from_u8(byte).unwrap();
            assert_eq!(opcode, restored);
        }
    }
}
