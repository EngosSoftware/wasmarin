use std::fs;
use wasmarin::{Encoder, Parser};

#[test]
fn _0001() {
  let data = fs::read("./tests/test_round_trip/burner.wasm").unwrap();
  let mut parser = Parser::new();
  let model = parser.parse_wasm(&data).unwrap();
  let encoder = Encoder::new();
  let wasm_bytes = encoder.encode(model);

  assert!(wasmparser::validate(&wasm_bytes).is_ok());

  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);
}
