mod bytecode_viewer;

use bengal_compiler::compiler::{Compiler, CompilerOptions};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileResult {
    pub success: bool,
    pub error: Option<String>,
    pub assembly: Option<String>,
}

/// Compile Bengal source code to bytecode
///
/// # Arguments
/// * `source` - The Bengal source code to compile
/// * `unsafe_fast` - Enable unsafe optimizations (optional, default false)
#[wasm_bindgen]
pub fn compile(source: &str, unsafe_fast: Option<bool>) -> JsValue {
    console_error_panic_hook::set_once();

    let unsafe_flag = unsafe_fast.unwrap_or(false);

    // Create compiler with source
    let mut compiler = Compiler::with_path_and_options(source, "<input>", unsafe_flag);
    compiler.enable_type_checking = false;

    let options = CompilerOptions {
        enable_type_checking: false,
        search_paths: vec![],
        unsafe_fast: unsafe_flag,
    };

    match compiler.compile_with_options(&options) {
        Ok(bytecode) => {
            let assembly = bytecode_viewer::display_bytecode(&bytecode);
            let result = CompileResult {
                success: true,
                error: None,
                assembly: Some(assembly),
            };
            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
        }
        Err(e) => {
            let result = CompileResult {
                success: false,
                error: Some(e.to_string()),
                assembly: None,
            };
            serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
        }
    }
}

/// Disassemble code (alias for compile, kept for API compatibility)
#[wasm_bindgen]
pub fn disassemble(source: &str, unsafe_fast: Option<bool>) -> JsValue {
    compile(source, unsafe_fast)
}

/// Get version information
#[wasm_bindgen]
pub fn get_version() -> String {
    "Bengal WASM Compiler v0.1.0".to_string()
}
