use wasmarin::{Encoder, Parser};

#[test]
fn simple_round_trip_should_work() {
  let _expected = r#"(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.add)
  (memory (;0;) 1)
  (export "add_one" (func 0))
)"#;

  let wat_str = r#"
    (module
      (memory 1)
      (type $add_t (func (param i32) (result i32)))
      (func $add_one_f (type $add_t) (param $value i32) (result i32)
        local.get $value
        i32.const 1
        i32.add
      )
      (export "add_one" (func $add_one_f)))
  "#;
  let wasm_bytes_before = wat::parse_str(wat_str).unwrap();
  let wat_before = wasmprinter::print_bytes(&wasm_bytes_before).unwrap();
  println!("{}", wat_before);
  let mut parser = Parser::new();
  let model = parser.parse_wasm(&wasm_bytes_before).unwrap();
  let encoder = Encoder::new();
  let wasm_bytes_after = encoder.encode(model);
  let wat_after = wasmprinter::print_bytes(&wasm_bytes_after).unwrap();
  println!("{}", wat_after);
}
