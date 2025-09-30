use crate::vm::types::Value;
use std::fmt;

#[derive(Debug)]
pub enum StackError {
    Underflow,
    Overflow,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::Underflow => {
                write!(f, "Stack underflow: attempted to pop from empty stack")
            }
            StackError::Overflow => write!(f, "Stack overflow: maximum stack size exceeded"),
        }
    }
}

impl std::error::Error for StackError {}

pub struct OperandStack {
    values: Vec<Value>,
    max_size: Option<usize>,
}

impl OperandStack {
    const DEFAULT_CAPACITY: usize = 1024;
    const MAX_STACK_SIZE: usize = 1_000_000; // 1M elements max for safety

    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(Self::DEFAULT_CAPACITY),
            max_size: None, // Unlimited growth up to MAX_STACK_SIZE
        }
    }

    pub fn with_capacity(max_size: usize) -> Self {
        let actual_max = max_size.min(Self::MAX_STACK_SIZE);
        Self {
            values: Vec::with_capacity(actual_max),
            max_size: Some(actual_max),
        }
    }

    pub fn push(&mut self, value: Value) {
        // For unlimited stacks, check against absolute maximum
        if self.max_size.is_none() && self.values.len() >= Self::MAX_STACK_SIZE {
            panic!("Stack overflow: exceeded absolute maximum size");
        }

        // For limited stacks, panic on overflow (as per test expectations)
        if let Some(max) = self.max_size {
            if self.values.len() >= max {
                panic!("Stack overflow: exceeded capacity");
            }
        }

        self.values.push(value);
    }

    pub fn try_push(&mut self, value: Value) -> Result<(), StackError> {
        // Check overflow conditions
        if self.max_size.is_none() && self.values.len() >= Self::MAX_STACK_SIZE {
            return Err(StackError::Overflow);
        }

        if let Some(max) = self.max_size {
            if self.values.len() >= max {
                return Err(StackError::Overflow);
            }
        }

        self.values.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, StackError> {
        self.values.pop().ok_or(StackError::Underflow)
    }

    pub fn peek(&self) -> Result<&Value, StackError> {
        self.values.last().ok_or(StackError::Underflow)
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn max_size(&self) -> Option<usize> {
        self.max_size
    }
}

impl Default for OperandStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_growth() {
        let mut stack = OperandStack::new();
        let initial_capacity = stack.capacity();

        // Push more than initial capacity to trigger growth
        for i in 0..(initial_capacity + 100) {
            stack.push(Value::Integer(i as i64));
        }

        assert!(stack.capacity() > initial_capacity);
        assert_eq!(stack.size(), initial_capacity + 100);
    }
}
