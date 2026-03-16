//! Sparkler VM types stub for WASM compilation
//! This provides only the type definitions needed for compilation, not execution

pub mod executor;
pub mod opcodes;
pub mod vm;

pub use executor::Bytecode;
pub use opcodes::Opcode;
pub use vm::{Class, Function, Method, Value};
