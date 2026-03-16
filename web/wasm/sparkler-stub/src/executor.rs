//! Executor types

use serde::{Deserialize, Serialize};
use crate::vm::{Class, Function};

/// Bytecode output from compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bytecode {
    pub data: Vec<u8>,
    pub strings: Vec<String>,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
}
