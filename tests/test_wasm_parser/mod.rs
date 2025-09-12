use std::fs;

#[test]
fn _0001() {
  let data = fs::read("./tests/test_wasm_parser/burner.wasm").unwrap();
  wasmarin::parse_wasm(&data).unwrap();
}
