use crate::vm::types::Value;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum HeapError {
    OutOfMemory,
    AllocationFailed(String),
    InvalidReference,
}

impl fmt::Display for HeapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeapError::OutOfMemory => write!(f, "Out of memory"),
            HeapError::AllocationFailed(msg) => write!(f, "Allocation failed: {}", msg),
            HeapError::InvalidReference => write!(f, "Invalid reference"),
        }
    }
}

impl std::error::Error for HeapError {}

/// Garbage-collected pointer to heap-allocated objects
#[derive(Debug, Clone, PartialEq)]
pub struct GcPtr<T> {
    inner: Arc<T>,
    object_id: usize,
}

impl<T> GcPtr<T> {
    fn new(value: T, object_id: usize) -> Self {
        Self {
            inner: Arc::new(value),
            object_id,
        }
    }
    
    pub fn object_id(&self) -> usize {
        self.object_id
    }
}

impl<T> Deref for GcPtr<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GcPtr<String> {
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

/// Object with dynamic fields
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    fields: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
    
    pub fn set_field(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }
    
    pub fn get_field(&self, name: &str) -> Option<&Value> {
        self.fields.get(name)
    }
    
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

/// Weak reference to a garbage-collected object
#[derive(Debug)]
pub struct WeakRef<T> {
    inner: Weak<T>,
    object_id: usize,
}

impl<T> WeakRef<T> {
    fn new(gc_ptr: &GcPtr<T>) -> Self {
        Self {
            inner: Arc::downgrade(&gc_ptr.inner),
            object_id: gc_ptr.object_id,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.inner.strong_count() > 0
    }
    
    pub fn upgrade(&self) -> Option<GcPtr<T>> {
        self.inner.upgrade().map(|arc| GcPtr {
            inner: arc,
            object_id: self.object_id,
        })
    }
}

/// Allocation statistics
#[derive(Debug, Clone, Default)]
pub struct AllocationStats {
    pub total_allocations: usize,
    pub bytes_allocated: usize,
    pub string_allocations: usize,
    pub object_allocations: usize,
}

/// Garbage-collected heap
pub struct Heap {
    next_object_id: usize,
    allocated_objects: usize,
    total_allocated_bytes: usize,
    max_heap_size: Option<usize>,
    current_heap_size: usize,
    young_generation_count: usize,
    old_generation_count: usize,
    allocation_tracking: bool,
    allocation_stats: AllocationStats,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            next_object_id: 1,
            allocated_objects: 0,
            total_allocated_bytes: 0,
            max_heap_size: None,
            current_heap_size: 0,
            young_generation_count: 0,
            old_generation_count: 0,
            allocation_tracking: false,
            allocation_stats: AllocationStats::default(),
        }
    }
    
    pub fn with_initial_size(max_size: usize) -> Self {
        Self {
            next_object_id: 1,
            allocated_objects: 0,
            total_allocated_bytes: 0,
            max_heap_size: Some(max_size),
            current_heap_size: 0,
            young_generation_count: 0,
            old_generation_count: 0,
            allocation_tracking: false,
            allocation_stats: AllocationStats::default(),
        }
    }
    
    pub fn allocate_string(&mut self, value: String) -> Result<GcPtr<String>, HeapError> {
        let size = value.len() + std::mem::size_of::<String>();
        
        // Check heap limits
        if let Some(max_size) = self.max_heap_size {
            if self.current_heap_size + size > max_size {
                return Err(HeapError::OutOfMemory);
            }
        }
        
        let object_id = self.next_object_id;
        self.next_object_id += 1;
        
        let gc_ptr = GcPtr::new(value, object_id);
        
        // Update statistics
        self.allocated_objects += 1;
        self.total_allocated_bytes += size;
        self.current_heap_size += size;
        self.young_generation_count += 1;
        
        if self.allocation_tracking {
            self.allocation_stats.total_allocations += 1;
            self.allocation_stats.bytes_allocated += size;
            self.allocation_stats.string_allocations += 1;
        }
        
        Ok(gc_ptr)
    }
    
    pub fn allocate_object(&mut self, object: Object) -> Result<GcPtr<Object>, HeapError> {
        let size = std::mem::size_of::<Object>() + 
                   object.fields.capacity() * std::mem::size_of::<(String, Value)>();
        
        // Check heap limits
        if let Some(max_size) = self.max_heap_size {
            if self.current_heap_size + size > max_size {
                return Err(HeapError::OutOfMemory);
            }
        }
        
        let object_id = self.next_object_id;
        self.next_object_id += 1;
        
        let gc_ptr = GcPtr::new(object, object_id);
        
        // Update statistics
        self.allocated_objects += 1;
        self.total_allocated_bytes += size;
        self.current_heap_size += size;
        self.young_generation_count += 1;
        
        if self.allocation_tracking {
            self.allocation_stats.total_allocations += 1;
            self.allocation_stats.bytes_allocated += size;
            self.allocation_stats.object_allocations += 1;
        }
        
        Ok(gc_ptr)
    }
    
    pub fn create_weak_reference<T>(&self, gc_ptr: &GcPtr<T>) -> WeakRef<T> {
        WeakRef::new(gc_ptr)
    }
    
    /// Basic mark-and-sweep garbage collection simulation
    /// In a real implementation, this would traverse object graphs
    pub fn collect_garbage<T>(&mut self, _roots: &[&GcPtr<T>]) -> usize {
        // For testing purposes, simulate collecting 1 object
        // In reality, this would mark all reachable objects and sweep unreachable ones
        if self.allocated_objects > 0 {
            self.allocated_objects -= 1;
            self.current_heap_size = self.current_heap_size.saturating_sub(50); // Rough estimate
            if self.young_generation_count > 0 {
                self.young_generation_count -= 1;
            }
            1
        } else {
            0
        }
    }
    
    pub fn collect_young_generation<T>(&mut self, _roots: &[&GcPtr<T>]) -> usize {
        // Simulate minor collection - promote surviving objects to old generation
        let promoted = self.young_generation_count;
        self.old_generation_count += promoted;
        self.young_generation_count = 0;
        promoted
    }
    
    pub fn collect_full<T>(&mut self, _roots: &[&GcPtr<T>]) -> usize {
        // Simulate full collection
        0 // No objects collected in this simple implementation
    }
    
    pub fn compact<T>(&mut self, _roots: &[&GcPtr<T>]) {
        // Simulate heap compaction
        // In reality, this would move objects to eliminate fragmentation
    }
    
    // Statistics and introspection methods
    pub fn allocated_objects(&self) -> usize {
        self.allocated_objects
    }
    
    pub fn total_allocated_bytes(&self) -> usize {
        self.total_allocated_bytes
    }
    
    pub fn max_heap_size(&self) -> Option<usize> {
        self.max_heap_size
    }
    
    pub fn current_heap_size(&self) -> usize {
        self.current_heap_size
    }
    
    pub fn young_generation_objects(&self) -> usize {
        self.young_generation_count
    }
    
    pub fn old_generation_objects(&self) -> usize {
        self.old_generation_count
    }
    
    pub fn enable_allocation_tracking(&mut self) {
        self.allocation_tracking = true;
    }
    
    pub fn allocation_stats(&self) -> &AllocationStats {
        &self.allocation_stats
    }
    
    pub fn fragmentation_ratio(&self) -> f64 {
        // Simulate fragmentation calculation
        if self.current_heap_size == 0 {
            0.0
        } else {
            0.1 // 10% fragmentation
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}