/// WebAssembly features.
///
/// Usually, each feature has it's corresponding [proposal].
/// [proposal]: https://github.com/WebAssembly/proposals
///
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Features {
  /// Enables `Threads` proposal.
  pub threads: bool,
  /// Enables `Reference Types` proposal.
  pub reference_types: bool,
  /// Enables `SIMD` proposal.
  pub simd: bool,
  /// Enables the WebAssembly bulk memory operations proposal.
  pub bulk_memory: bool,
  /// Multi Value proposal.
  pub multi_value: bool,
  /// Tail call proposal.
  pub tail_call: bool,
  /// Multi Memory proposal.
  pub multi_memory: bool,
  /// 64-bit Memory proposal.
  pub memory64: bool,
  /// Wasm exceptions proposal.
  pub exceptions: bool,
  /// Extended constant expressions proposal should be enabled
  pub extended_const: bool,
  /// Relaxed SIMD proposal.
  pub relaxed_simd: bool,
}

impl Features {
  /// Creates a new feature set with all features enabled.
  pub fn new() -> Self {
    Self {
      threads: true,
      reference_types: true,
      simd: true,
      bulk_memory: true,
      multi_value: true,
      tail_call: true,
      multi_memory: true,
      memory64: true,
      exceptions: true,
      extended_const: true,
      relaxed_simd: true,
    }
  }
}

impl From<Features> for wasmparser::WasmFeatures {
  fn from(value: Features) -> Self {
    let mut wasm_features = wasmparser::WasmFeatures::default();
    wasm_features.set(wasmparser::WasmFeatures::THREADS, value.threads);
    wasm_features.set(wasmparser::WasmFeatures::REFERENCE_TYPES, value.reference_types);
    wasm_features.set(wasmparser::WasmFeatures::SIMD, value.simd);
    wasm_features.set(wasmparser::WasmFeatures::BULK_MEMORY, value.bulk_memory);
    wasm_features.set(wasmparser::WasmFeatures::MULTI_VALUE, value.multi_value);
    wasm_features.set(wasmparser::WasmFeatures::TAIL_CALL, value.tail_call);
    wasm_features.set(wasmparser::WasmFeatures::MULTI_MEMORY, value.multi_memory);
    wasm_features.set(wasmparser::WasmFeatures::MEMORY64, value.memory64);
    wasm_features.set(wasmparser::WasmFeatures::EXCEPTIONS, value.exceptions);
    wasm_features.set(wasmparser::WasmFeatures::EXTENDED_CONST, value.extended_const);
    wasm_features.set(wasmparser::WasmFeatures::RELAXED_SIMD, value.relaxed_simd);
    wasm_features.set(wasmparser::WasmFeatures::MUTABLE_GLOBAL, true);
    wasm_features.set(wasmparser::WasmFeatures::SATURATING_FLOAT_TO_INT, true);
    wasm_features.set(wasmparser::WasmFeatures::FLOATS, true);
    wasm_features.set(wasmparser::WasmFeatures::SIGN_EXTENSION, true);
    wasm_features.set(wasmparser::WasmFeatures::GC_TYPES, true);
    wasm_features.set(wasmparser::WasmFeatures::COMPONENT_MODEL, false);
    wasm_features.set(wasmparser::WasmFeatures::FUNCTION_REFERENCES, false);
    wasm_features.set(wasmparser::WasmFeatures::MEMORY_CONTROL, false);
    wasm_features.set(wasmparser::WasmFeatures::GC, false);
    wasm_features
  }
}
