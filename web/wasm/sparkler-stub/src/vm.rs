//! VM types

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Class definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Class {
    pub name: String,
    pub fields: HashMap<String, Value>,
    pub private_fields: HashSet<String>,
    pub methods: HashMap<String, Method>,
    pub native_methods: HashMap<String, String>,
    pub native_create: Option<String>,
    pub native_destroy: Option<String>,
    pub is_native: bool,
    pub parent_interfaces: Vec<String>,
    pub vtable: Vec<String>,
    pub is_interface: bool,
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Function {
    pub name: String,
    pub bytecode: Vec<u8>,
    pub param_count: u8,
    pub register_count: u8,
    pub source_file: Option<String>,
}

/// Method definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Method {
    pub name: String,
    pub bytecode: Vec<u8>,
    pub register_count: u8,
}

/// Runtime value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Bool(bool),
    Null,
}

impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}
