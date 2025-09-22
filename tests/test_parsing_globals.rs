use wasmarin::{Encoder, Parser};

#[test]
fn parsing_globals_should_work() {
  let wat_str = r#"
    (module
      (global (mut i32) i32.const 2)
      (type (func (param i32) (result i32)))
      (func (type 0) (param i32) (result i32)
        local.get 0
        global.get 0
        i32.add
      )
      (export "add_one" (func 0))
      (export "offset" (global 0))
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let mut parser = Parser::new();
  let model = parser.parse_wasm_bytes(&wasm_bytes).unwrap();
  let mut encoder = Encoder::new_with_metering();
  let wasm_bytes = encoder.encode(model).unwrap();
  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);
}

#[test]
fn a() {
  let word = 32;
  for i in 0..=100 {
    print!("{:3} ", i);
  }
  println!();
  for i in 0..=100 {
    let coefficient = (i + word - 1) / word;
    print!("{:3} ", coefficient);
  }
}
