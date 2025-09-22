#[test]
fn _0001() {
  let data = std::fs::read("./tests/test_round_trip/burner.wasm").unwrap();
  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&data).unwrap();
  let mut encoder = wasmarin::Encoder::default();
  let wasm_bytes = encoder.encode(model).unwrap();
  assert!(wasmparser::validate(&wasm_bytes).is_ok());
}
