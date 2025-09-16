use wasmarin::Features;

#[test]
fn new_should_enable_all_features() {
  let features = Features::new();
  let wasm_features: wasmparser::WasmFeatures = features.into();
  assert!(wasm_features.threads());
  assert!(wasm_features.reference_types());
  assert!(wasm_features.simd());
  assert!(wasm_features.bulk_memory());
  assert!(wasm_features.multi_value());
  assert!(wasm_features.tail_call());
  assert!(wasm_features.multi_memory());
  assert!(wasm_features.memory64());
  assert!(wasm_features.exceptions());
  assert!(wasm_features.extended_const());
  assert!(wasm_features.relaxed_simd());
  assert!(wasm_features.mutable_global());
  assert!(wasm_features.saturating_float_to_int());
  assert!(wasm_features.floats());
  assert!(wasm_features.sign_extension());
  assert!(wasm_features.gc_types());
  assert!(!wasm_features.component_model());
  assert!(!wasm_features.function_references());
  assert!(!wasm_features.memory_control());
  assert!(!wasm_features.gc());
}

#[test]
fn default_should_disable_all_features() {
  let features = Features::default();
  let wasm_features: wasmparser::WasmFeatures = features.into();
  assert!(!wasm_features.threads());
  assert!(!wasm_features.reference_types());
  assert!(!wasm_features.simd());
  assert!(!wasm_features.bulk_memory());
  assert!(!wasm_features.multi_value());
  assert!(!wasm_features.tail_call());
  assert!(!wasm_features.multi_memory());
  assert!(!wasm_features.memory64());
  assert!(!wasm_features.exceptions());
  assert!(!wasm_features.extended_const());
  assert!(!wasm_features.relaxed_simd());
  assert!(wasm_features.mutable_global());
  assert!(wasm_features.saturating_float_to_int());
  assert!(wasm_features.floats());
  assert!(wasm_features.sign_extension());
  assert!(wasm_features.gc_types());
  assert!(!wasm_features.component_model());
  assert!(!wasm_features.function_references());
  assert!(!wasm_features.memory_control());
  assert!(!wasm_features.gc());
}
