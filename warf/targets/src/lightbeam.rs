/***********************************************
LIGHTBEAM:
- https://github.com/bytecodealliance/wasmtime/tree/master/crates/lightbeam
***********************************************/

/// Fuzzing `lightbeam::translate` using translate methods.
///
/// NOTE: lightbeam not called the same way here than in wasmtime.
/// NOTE: I'm not sure this method validate the module first.
pub fn fuzz_lightbeam_translate(data: &[u8]) -> bool {
    use lightbeam::translate;

    translate(&data).is_ok()
}
