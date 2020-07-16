/***********************************************
WASMER:
- https://github.com/wasmerio/wasmer
************************************************/

/// Fuzzing `wasmer::validate`
pub fn fuzz_wasmer_validate(data: &[u8]) -> bool {
    extern crate wasmer_runtime;
    wasmer_runtime::validate(&data)
}

/// Fuzzing wasmer::compile with Cranelift compiler backend
pub fn fuzz_wasmer_compile_clif(data: &[u8]) -> bool {
    use wasmer_runtime::compile;
    compile(&data).is_ok()
}

/// Fuzzing `wasmer::compile` with `SinglePass` compiler backend
pub fn fuzz_wasmer_compile_singlepass(data: &[u8]) -> bool {
    use wasmer_runtime::compile_with;
    use wasmer_singlepass_backend::SinglePassCompiler;
    compile_with(&data, &SinglePassCompiler::new()).is_ok()
}

/// Fuzzing `wasmer::instantiate` with empty import_object
pub fn fuzz_wasmer_instantiate(data: &[u8]) -> bool {
    use wasmer_runtime::{imports, instantiate};
    let import_object = imports! {};
    // allow_missing_functions should prevent wasmer to reject
    // modules with imported functions but generate more false positive bugs
    // import_object.allow_missing_functions = true;
    instantiate(&data, &import_object).is_ok()

    // TODO(RM3): improve or create new fuzz harness that iterate
    // over module functions and call them all
}

/*

// TODO: LLVMCompiler not available throw crates.io
pub fn TODO_fuzz_wasmer_compile_llvm(data: &[u8]) {
    extern crate wasmer_runtime;
    use std::str::FromStr;
    use wasmer_runtime::{compiler_for_backend, compile_with, Backend};
    // LLVM backend
    let backend = match Backend::from_str("llvm"){
           Ok(backend) => backend,
        _ => return,
    };
    let compiler = match compiler_for_backend(backend) {
        Some(compiler) => compiler,
        _ => return,
    };
    let _res = compile_with(&data, compiler.as_ref());
}

// TODO(RM3) - wasmer_runtime::validate_and_report_errors_with_features
*/
