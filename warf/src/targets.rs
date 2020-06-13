use failure::Error;
use strum::IntoEnumIterator;

use crate::env::{targets_dir, workspace_dir};
use crate::utils::copy_dir;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Targets {
    // wasmi
    WasmiValidate,
    WasmiInstantiate,
    // parity_wasm
    ParityWasmDeserialize,
    // wasmer
    WasmerValidate,
    WasmerCompileClif,
    WasmerCompileSinglepass,
    WasmerInstantiate,
    // wasmtime
    WasmtimeValidate,
    WasmtimeValidateAllFeat,
    WasmtimeCompile,
    WasmtimeCompileAllCranelift,
    WasmtimeInstantiateAllCranelift,
    // wasmparser
    WasmparserParser,
    WasmparserValidate,
    WasmparserValidateAllFeat,
    // binaryen_ffi
    BinaryenFfi,
    BinaryenOptimizeFfi,
    // wabt_ffi
    WabtWasm2watAllFeatFfi,
    WabtValidateFfi,
    // wasmprinter
    WasmprinterParser,
    // wain
    WainParser,
    WainValidate,
    // differential fuzzing
    DiffParsing,
    DiffValidateAllFeat,
    DiffInstantiate,
}

impl Targets {
    pub fn name(&self) -> String {
        match &self {
            // wasmi
            Targets::WasmiValidate => "wasmi_validate",
            Targets::WasmiInstantiate => "wasmi_instantiate",
            // parity_wasm
            Targets::ParityWasmDeserialize => "parity_wasm_deserialize",
            // wasmer
            Targets::WasmerValidate => "wasmer_validate",
            Targets::WasmerCompileClif => "wasmer_compile_clif",
            Targets::WasmerCompileSinglepass => "wasmer_compile_singlepass",
            Targets::WasmerInstantiate => "wasmer_instantiate",
            // wasmtime
            Targets::WasmtimeValidate => "wasmtime_validate",
            Targets::WasmtimeValidateAllFeat => "wasmtime_validate_all_feat",
            Targets::WasmtimeCompile => "wasmtime_compile",
            Targets::WasmtimeCompileAllCranelift => "wasmtime_compile_all_cranelift",
            Targets::WasmtimeInstantiateAllCranelift => "wasmtime_instantiate_all_cranelift",
            // wasmparser
            Targets::WasmparserParser => "wasmparser_parser",
            Targets::WasmparserValidate => "wasmparser_validate",
            Targets::WasmparserValidateAllFeat => "wasmparser_validate_all_feat",
            // binaryen_ffi
            Targets::BinaryenFfi => "binaryen_ffi",
            Targets::BinaryenOptimizeFfi => "binaryen_optimize_ffi",
            // wabt_ffi
            Targets::WabtWasm2watAllFeatFfi => "wabt_wasm2wat_all_feat_ffi",
            Targets::WabtValidateFfi => "wabt_validate_ffi",
            // wasmprinter
            Targets::WasmprinterParser => "wasmprinter_parser",
            // wain
            Targets::WainParser => "wain_parser",
            Targets::WainValidate => "wain_validate",
            // differential fuzzing
            Targets::DiffParsing => "diff_parsing",
            Targets::DiffValidateAllFeat => "diff_all_validate",
            Targets::DiffInstantiate => "diff_instantiate",
        }
        .to_string()
    }

    pub fn corpora(&self) -> String {
        match &self {
            // wasmi
            Targets::WasmiValidate
            | Targets::WasmiInstantiate
            // parity_wasm
            | Targets::ParityWasmDeserialize
            // wasmer
            | Targets::WasmerValidate
            | Targets::WasmerCompileClif
            | Targets::WasmerCompileSinglepass
            | Targets::WasmerInstantiate
            // wasmtime
            | Targets::WasmtimeValidate
            | Targets::WasmtimeValidateAllFeat
            | Targets::WasmtimeCompile
            | Targets::WasmtimeCompileAllCranelift
            | Targets::WasmtimeInstantiateAllCranelift
            // wasmparser
            | Targets::WasmparserParser
            | Targets::WasmparserValidate
            | Targets::WasmparserValidateAllFeat
            // binaryen_ffi
            | Targets::BinaryenFfi
            | Targets::BinaryenOptimizeFfi
            // wabt_ffi
            | Targets::WabtWasm2watAllFeatFfi
            | Targets::WabtValidateFfi
            // wasmprinter
            | Targets::WasmprinterParser
            // wain
            | Targets::WainParser
            | Targets::WainValidate
            // differential fuzzing
            | Targets::DiffParsing
            | Targets::DiffValidateAllFeat
            | Targets::DiffInstantiate => "wasm",
        }
        .to_string()
    }

    pub fn template(&self) -> String {
        match &self {
            // wasmi
            Targets::WasmiValidate
            | Targets::WasmiInstantiate
            // parity_wasm
            | Targets::ParityWasmDeserialize
            // wasmer
            | Targets::WasmerValidate
            | Targets::WasmerCompileClif
            | Targets::WasmerCompileSinglepass
            | Targets::WasmerInstantiate
            // wasmtime
            | Targets::WasmtimeValidate
            | Targets::WasmtimeValidateAllFeat
            | Targets::WasmtimeCompile
            | Targets::WasmtimeCompileAllCranelift
            | Targets::WasmtimeInstantiateAllCranelift
            // wasmparser
            | Targets::WasmparserParser
            | Targets::WasmparserValidate
            | Targets::WasmparserValidateAllFeat
            // binaryen_ffi
            | Targets::BinaryenFfi
            | Targets::BinaryenOptimizeFfi
            // wabt_ffi
            | Targets::WabtWasm2watAllFeatFfi
            | Targets::WabtValidateFfi
            // wasmprinter
            | Targets::WasmprinterParser
            // wain
            | Targets::WainParser
            | Targets::WainValidate
            // differential fuzzing
            | Targets::DiffParsing
            | Targets::DiffValidateAllFeat
            | Targets::DiffInstantiate => "template.rs",
        }
        .to_string()
    }

    pub fn language(&self) -> String {
        match &self {
            // wasmi
            Targets::WasmiValidate
            | Targets::WasmiInstantiate
            // parity_wasm
            | Targets::ParityWasmDeserialize
            // wasmer
            | Targets::WasmerValidate
            | Targets::WasmerCompileClif
            | Targets::WasmerCompileSinglepass
            | Targets::WasmerInstantiate
            // wasmtime
            | Targets::WasmtimeValidate
            | Targets::WasmtimeValidateAllFeat
            | Targets::WasmtimeCompile
            | Targets::WasmtimeCompileAllCranelift
            | Targets::WasmtimeInstantiateAllCranelift
            // wasmparser
            | Targets::WasmparserParser
            | Targets::WasmparserValidate
            | Targets::WasmparserValidateAllFeat
            // binaryen_ffi
            | Targets::BinaryenFfi
            | Targets::BinaryenOptimizeFfi
            // wabt_ffi
            | Targets::WabtWasm2watAllFeatFfi
            | Targets::WabtValidateFfi
            // wasmprinter
            | Targets::WasmprinterParser
            // wain
            | Targets::WainParser
            | Targets::WainValidate
            // differential fuzzing
            | Targets::DiffParsing
            | Targets::DiffValidateAllFeat
            | Targets::DiffInstantiate => "rust",
        }
        .to_string()
    }
}

pub fn get_targets() -> Vec<String> {
    Targets::iter().map(|x| x.name()).collect()
}

pub fn prepare_targets_workspace() -> Result<(), Error> {
    let from = targets_dir()?;
    let workspace = workspace_dir()?;
    copy_dir(from, workspace)?;
    Ok(())
}
