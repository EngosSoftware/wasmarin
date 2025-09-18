use wasmarin::{Encoder, Parser};

#[test]
fn _0001() {
  let data = std::fs::read("./tests/test_round_trip/burner.wasm").unwrap();
  let mut parser = Parser::new();
  let model = parser.parse_wasm_bytes(&data).unwrap();
  let mut encoder = Encoder::default();
  let wasm_bytes = encoder.encode(model);

  println!("{:?}", wasmparser::validate(&wasm_bytes).err());

  //assert!(wasmparser::validate(&wasm_bytes).is_ok());

  // let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  // println!("{}", wat);
}
