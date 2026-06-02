/// Round-trip without metering.
#[test]
fn _0001() {
  let data = std::fs::read("tests/contracts/burner.wasm").unwrap();
  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&data).unwrap();
  let mut encoder = wasmarin::Encoder::default();
  let wasm_bytes = encoder.encode(model).unwrap();
  assert!(wasmparser::validate(&wasm_bytes).is_ok());
}

/// Round-trip with metering.
#[test]
fn _0002() {
  let bytes = std::fs::read("tests/contracts/burner.wasm").unwrap();
  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&bytes).unwrap();
  let mut encoder = wasmarin::Encoder::new_with_metering();
  let wasm_bytes = encoder.encode(model).unwrap();
  assert!(wasmparser::validate(&wasm_bytes).is_ok());
}
