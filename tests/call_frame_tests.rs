use stack_vm_jit::vm::call_frame::CallFrame;
use stack_vm_jit::vm::types::Value;

#[test]
fn test_call_frame_creation() {
    let frame = CallFrame::new(100, 0x1000, 5);

    assert_eq!(frame.function_index(), 100);
    assert_eq!(frame.return_address(), 0x1000);
    assert_eq!(frame.local_count(), 5);
    assert_eq!(frame.stack_base(), 0);
}

#[test]
fn test_call_frame_with_stack_base() {
    let frame = CallFrame::new_with_stack_base(200, 0x2000, 3, 10);

    assert_eq!(frame.function_index(), 200);
    assert_eq!(frame.return_address(), 0x2000);
    assert_eq!(frame.local_count(), 3);
    assert_eq!(frame.stack_base(), 10);
}

#[test]
fn test_local_variable_access() {
    let mut frame = CallFrame::new(1, 0x1000, 3);

    // Set local variables
    frame.set_local(0, Value::Integer(42)).unwrap();
    frame.set_local(1, Value::Float(1.414)).unwrap();
    frame.set_local(2, Value::Boolean(true)).unwrap();

    // Get local variables
    assert_eq!(frame.get_local(0).unwrap(), &Value::Integer(42));
    assert_eq!(frame.get_local(1).unwrap(), &Value::Float(1.414));
    assert_eq!(frame.get_local(2).unwrap(), &Value::Boolean(true));
}

#[test]
fn test_local_variable_out_of_bounds() {
    let mut frame = CallFrame::new(1, 0x1000, 2);

    // Accessing beyond local count should fail
    let result = frame.get_local(2);
    assert!(result.is_err());

    let result = frame.set_local(3, Value::Integer(123));
    assert!(result.is_err());
}

#[test]
fn test_local_variable_uninitialized() {
    let frame = CallFrame::new(1, 0x1000, 3);

    // Uninitialized locals should return Null
    assert_eq!(frame.get_local(0).unwrap(), &Value::Null);
    assert_eq!(frame.get_local(1).unwrap(), &Value::Null);
    assert_eq!(frame.get_local(2).unwrap(), &Value::Null);
}

#[test]
fn test_call_frame_program_counter() {
    let mut frame = CallFrame::new(1, 0x1000, 0);

    assert_eq!(frame.program_counter(), 0);

    frame.set_program_counter(42);
    assert_eq!(frame.program_counter(), 42);

    frame.advance_program_counter();
    assert_eq!(frame.program_counter(), 43);
}

#[test]
fn test_call_frame_metadata() {
    let mut frame = CallFrame::new(5, 0xABCD, 10);

    // Test that we can update return address
    frame.set_return_address(0x5678);
    assert_eq!(frame.return_address(), 0x5678);

    // Test function name (optional metadata)
    frame.set_function_name("test_function".to_string());
    assert_eq!(frame.function_name(), Some("test_function"));
}

#[test]
fn test_call_stack_operations() {
    use stack_vm_jit::vm::call_frame::CallStack;

    let mut call_stack = CallStack::new();

    assert!(call_stack.is_empty());
    assert_eq!(call_stack.depth(), 0);

    // Push frames
    let frame1 = CallFrame::new(1, 0x1000, 2);
    let frame2 = CallFrame::new(2, 0x2000, 3);

    call_stack.push_unchecked(frame1);
    call_stack.push_unchecked(frame2);

    assert!(!call_stack.is_empty());
    assert_eq!(call_stack.depth(), 2);

    // Access current frame
    let current = call_stack.current_mut().unwrap();
    assert_eq!(current.function_index(), 2);

    // Pop frame
    let popped = call_stack.pop().unwrap();
    assert_eq!(popped.function_index(), 2);
    assert_eq!(call_stack.depth(), 1);
}
