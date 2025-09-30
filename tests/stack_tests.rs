use stack_vm_jit::vm::stack::OperandStack;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_stack_creation() {
    let stack = OperandStack::new();
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_stack_push_pop() {
    let mut stack = OperandStack::new();

    stack.push(Value::Integer(42));
    assert_eq!(stack.size(), 1);
    assert!(!stack.is_empty());

    let value = stack.pop().expect("Stack should not be empty");
    assert_eq!(value, Value::Integer(42));
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_stack_pop_empty() {
    let mut stack = OperandStack::new();
    let result = stack.pop();
    assert!(result.is_err());
}

#[test]
fn test_stack_overflow_protection() {
    let mut stack = OperandStack::with_capacity(2);

    stack.push(Value::Integer(1));
    stack.push(Value::Integer(2));

    // This should trigger overflow protection
    let result = stack.try_push(Value::Integer(3));
    assert!(result.is_err());
    assert_eq!(stack.size(), 2);
}

#[test]
fn test_stack_dynamic_sizing() {
    let mut stack = OperandStack::new();

    // Push many values to test dynamic growth
    for i in 0..100 {
        stack.push(Value::Integer(i));
    }

    assert_eq!(stack.size(), 100);

    // Verify all values are correct (LIFO order)
    for i in (0..100).rev() {
        let value = stack.pop().expect("Stack should have value");
        assert_eq!(value, Value::Integer(i));
    }
}

#[test]
fn test_stack_peek() {
    let mut stack = OperandStack::new();

    stack.push(Value::Integer(42));
    stack.push(Value::Float(1.414));

    let top = stack.peek().expect("Stack should not be empty");
    assert_eq!(*top, Value::Float(1.414));
    assert_eq!(stack.size(), 2); // peek shouldn't change size
}

#[test]
fn test_stack_clear() {
    let mut stack = OperandStack::new();

    stack.push(Value::Integer(1));
    stack.push(Value::Integer(2));
    stack.push(Value::Integer(3));

    stack.clear();
    assert_eq!(stack.size(), 0);
    assert!(stack.is_empty());
}

#[test]
fn test_stack_types() {
    let mut stack = OperandStack::new();

    stack.push(Value::Integer(42));
    stack.push(Value::Float(1.414));
    stack.push(Value::Boolean(true));

    assert_eq!(stack.pop().unwrap(), Value::Boolean(true));
    assert_eq!(stack.pop().unwrap(), Value::Float(1.414));
    assert_eq!(stack.pop().unwrap(), Value::Integer(42));
}
