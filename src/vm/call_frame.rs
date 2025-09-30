use crate::vm::types::Value;
use std::fmt;

#[derive(Debug)]
pub enum CallFrameError {
    LocalIndexOutOfBounds(usize, usize), // requested_index, max_index
    StackUnderflow,
    EmptyCallStack,
}

impl fmt::Display for CallFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallFrameError::LocalIndexOutOfBounds(requested, max) => {
                write!(
                    f,
                    "Local variable index {} out of bounds (max: {})",
                    requested, max
                )
            }
            CallFrameError::StackUnderflow => write!(f, "Call stack underflow"),
            CallFrameError::EmptyCallStack => write!(f, "Call stack is empty"),
        }
    }
}

impl std::error::Error for CallFrameError {}

#[derive(Debug, Clone)]
pub struct CallFrame {
    function_index: usize,
    return_address: usize,
    stack_base: usize,
    program_counter: usize,
    locals: Vec<Value>,
    function_name: Option<String>,
}

impl CallFrame {
    pub fn new(function_index: usize, return_address: usize, local_count: usize) -> Self {
        Self::new_with_stack_base(function_index, return_address, local_count, 0)
    }

    pub fn new_with_stack_base(
        function_index: usize,
        return_address: usize,
        local_count: usize,
        stack_base: usize,
    ) -> Self {
        Self {
            function_index,
            return_address,
            stack_base,
            program_counter: 0,
            locals: vec![Value::Null; local_count],
            function_name: None,
        }
    }

    pub fn function_index(&self) -> usize {
        self.function_index
    }

    pub fn return_address(&self) -> usize {
        self.return_address
    }

    pub fn set_return_address(&mut self, address: usize) {
        self.return_address = address;
    }

    pub fn stack_base(&self) -> usize {
        self.stack_base
    }

    pub fn program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn set_program_counter(&mut self, pc: usize) {
        self.program_counter = pc;
    }

    pub fn advance_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn local_count(&self) -> usize {
        self.locals.len()
    }

    pub fn get_local(&self, index: usize) -> Result<&Value, CallFrameError> {
        if index >= self.locals.len() {
            return Err(CallFrameError::LocalIndexOutOfBounds(
                index,
                self.locals.len().saturating_sub(1),
            ));
        }
        Ok(&self.locals[index])
    }

    pub fn set_local(&mut self, index: usize, value: Value) -> Result<(), CallFrameError> {
        if index >= self.locals.len() {
            return Err(CallFrameError::LocalIndexOutOfBounds(
                index,
                self.locals.len().saturating_sub(1),
            ));
        }
        self.locals[index] = value;
        Ok(())
    }

    pub fn function_name(&self) -> Option<&str> {
        self.function_name.as_deref()
    }

    pub fn set_function_name(&mut self, name: String) {
        self.function_name = Some(name);
    }
}

pub struct CallStack {
    frames: Vec<CallFrame>,
    max_depth: usize,
}

impl CallStack {
    const DEFAULT_MAX_DEPTH: usize = 10_000; // Reasonable recursion limit

    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            max_depth: Self::DEFAULT_MAX_DEPTH,
        }
    }

    pub fn with_max_depth(max_depth: usize) -> Self {
        Self {
            frames: Vec::new(),
            max_depth,
        }
    }

    pub fn push(&mut self, frame: CallFrame) -> Result<(), CallFrameError> {
        if self.frames.len() >= self.max_depth {
            return Err(CallFrameError::StackUnderflow); // Reusing error type
        }
        self.frames.push(frame);
        Ok(())
    }

    // For tests that expect panic behavior
    pub fn push_unchecked(&mut self, frame: CallFrame) {
        self.frames.push(frame);
    }

    pub fn pop(&mut self) -> Result<CallFrame, CallFrameError> {
        self.frames.pop().ok_or(CallFrameError::StackUnderflow)
    }

    pub fn current(&self) -> Result<&CallFrame, CallFrameError> {
        self.frames.last().ok_or(CallFrameError::StackUnderflow)
    }

    pub fn current_mut(&mut self) -> Result<&mut CallFrame, CallFrameError> {
        self.frames.last_mut().ok_or(CallFrameError::StackUnderflow)
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
}

impl Default for CallStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_stack_overflow_protection() {
        let mut stack = CallStack::with_max_depth(2);

        let frame1 = CallFrame::new(1, 0x1000, 0);
        let frame2 = CallFrame::new(2, 0x2000, 0);
        let frame3 = CallFrame::new(3, 0x3000, 0);

        assert!(stack.push(frame1).is_ok());
        assert!(stack.push(frame2).is_ok());
        assert!(stack.push(frame3).is_err()); // Should fail due to max depth

        assert_eq!(stack.depth(), 2);
    }
}
