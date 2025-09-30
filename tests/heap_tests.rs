use stack_vm_jit::vm::heap::{Heap, Object};
use stack_vm_jit::vm::types::Value;

#[test]
fn test_heap_creation() {
    let heap = Heap::new();
    assert_eq!(heap.allocated_objects(), 0);
    assert_eq!(heap.total_allocated_bytes(), 0);
}

#[test]
fn test_string_allocation() {
    let mut heap = Heap::new();
    
    let string_value = "Hello, GC World!";
    let gc_string = heap.allocate_string(string_value.to_string()).unwrap();
    
    assert_eq!(heap.allocated_objects(), 1);
    assert!(heap.total_allocated_bytes() > 0);
    assert_eq!(gc_string.as_str(), string_value);
}

#[test]
fn test_object_allocation() {
    let mut heap = Heap::new();
    
    // Create an object with some fields
    let mut obj = Object::new();
    obj.set_field("name".to_string(), Value::String("test".to_string()));
    obj.set_field("value".to_string(), Value::Integer(42));
    
    let gc_object = heap.allocate_object(obj).unwrap();
    
    assert_eq!(heap.allocated_objects(), 1);
    assert_eq!(gc_object.get_field("name").unwrap(), &Value::String("test".to_string()));
    assert_eq!(gc_object.get_field("value").unwrap(), &Value::Integer(42));
}

#[test]
fn test_gc_ptr_deref() {
    let mut heap = Heap::new();
    
    let test_string = "Test string for GC pointer";
    let gc_string = heap.allocate_string(test_string.to_string()).unwrap();
    
    // Test dereferencing GcPtr
    assert_eq!(&*gc_string, test_string);
    assert_eq!(gc_string.len(), test_string.len());
}

#[test]
fn test_basic_mark_and_sweep() {
    let mut heap = Heap::new();
    
    // Allocate some objects
    let _string1 = heap.allocate_string("kept".to_string()).unwrap();
    let _string2 = heap.allocate_string("also kept".to_string()).unwrap();
    let _unreferenced = heap.allocate_string("should be collected".to_string()).unwrap();
    
    assert_eq!(heap.allocated_objects(), 3);
    
    // Create a root set (objects that should be kept)
    let mut roots = Vec::new();
    roots.push(&_string1);
    roots.push(&_string2);
    // Note: unreferenced is not in roots, so should be collected
    
    // Run garbage collection
    let collected = heap.collect_garbage(&roots);
    
    assert!(collected > 0); // Should have collected the unreferenced string
    assert_eq!(heap.allocated_objects(), 2); // Only the rooted objects remain
}

#[test]
fn test_heap_size_limits() {
    let heap = Heap::with_initial_size(1024);
    
    assert_eq!(heap.max_heap_size(), Some(1024));
    assert_eq!(heap.current_heap_size(), 0);
}

#[test]
fn test_allocation_failure_on_oom() {
    let mut heap = Heap::with_initial_size(100); // Very small heap
    
    // Try to allocate a large string that won't fit
    let large_string = "x".repeat(200);
    let result = heap.allocate_string(large_string);
    
    assert!(result.is_err());
}

#[test]
fn test_generational_collection() {
    let mut heap = Heap::new();
    
    // Allocate objects in young generation
    let young_obj1 = heap.allocate_string("young1".to_string()).unwrap();
    let young_obj2 = heap.allocate_string("young2".to_string()).unwrap();
    
    // Trigger minor collection (young generation only)
    let mut roots = vec![&young_obj1, &young_obj2];
    let _collected_minor = heap.collect_young_generation(&roots);
    
    // Objects should survive and be promoted to old generation
    assert_eq!(heap.young_generation_objects(), 0);
    assert_eq!(heap.old_generation_objects(), 2);
    
    // Allocate more objects
    let new_young = heap.allocate_string("new_young".to_string()).unwrap();
    roots.push(&new_young);
    
    // Trigger full collection
    let _collected_full = heap.collect_full(&roots);
    
    assert_eq!(heap.allocated_objects(), 3);
}

#[test]
fn test_weak_references() {
    let mut heap = Heap::new();
    
    let strong_ref = heap.allocate_string("strong".to_string()).unwrap();
    let weak_ref = heap.create_weak_reference(&strong_ref);
    
    assert!(weak_ref.is_alive());
    assert_eq!(weak_ref.upgrade().unwrap().as_str(), "strong");
    
    // After dropping strong reference and collecting
    drop(strong_ref);
    heap.collect_garbage::<String>(&[]);
    
    assert!(!weak_ref.is_alive());
    assert!(weak_ref.upgrade().is_none());
}

#[test]
fn test_allocation_tracking() {
    let mut heap = Heap::new();
    
    // Enable allocation tracking
    heap.enable_allocation_tracking();
    
    let _obj1 = heap.allocate_string("test1".to_string()).unwrap();
    let _obj2 = heap.allocate_string("test2".to_string()).unwrap();
    
    let stats = heap.allocation_stats();
    assert_eq!(stats.total_allocations, 2);
    assert!(stats.bytes_allocated > 0);
    assert_eq!(stats.string_allocations, 2);
    assert_eq!(stats.object_allocations, 0);
}

#[test]
fn test_heap_compaction() {
    let mut heap = Heap::new();
    
    // Allocate and deallocate objects to create fragmentation
    let obj1 = heap.allocate_string("keep1".to_string()).unwrap();
    let _obj2 = heap.allocate_string("discard".to_string()).unwrap();
    let obj3 = heap.allocate_string("keep2".to_string()).unwrap();
    
    // Collect garbage (obj2 should be collected)
    heap.collect_garbage(&[&obj1, &obj3]);
    
    // Check fragmentation
    let fragmentation_before = heap.fragmentation_ratio();
    
    // Perform compaction
    heap.compact(&[&obj1, &obj3]);
    
    // Fragmentation should be reduced
    let fragmentation_after = heap.fragmentation_ratio();
    assert!(fragmentation_after <= fragmentation_before);
}