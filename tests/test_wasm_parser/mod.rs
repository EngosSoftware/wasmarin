use std::fs;
use wasmarin::Parser;

#[test]
fn _0001() {
  let data = fs::read("./tests/test_wasm_parser/burner.wasm").unwrap();
  let mut parser = Parser::new();
  parser.parse_wasm(&data).unwrap();
}
