use crate::vm::heap::{GcPtr, Object};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    GcString(GcPtr<String>),
    GcObject(GcPtr<Object>),
    Null,
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Boolean(_) => "boolean",
            Value::String(_) => "string",
            Value::GcString(_) => "gc_string",
            Value::GcObject(_) => "gc_object",
            Value::Null => "null",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::GcString(s) => !s.is_empty(),
            Value::GcObject(_) => true, // Objects are always truthy
            Value::Null => false,
        }
    }
}
