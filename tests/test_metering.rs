#![cfg(feature = "wasmtime")]

use wasmarin::{Encoder, Parser};
use wasmtime::{Engine, Instance, Module, Store, Val};

#[test]
fn metering_should_work() {
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
  let mut encoder = Encoder::new(true);
  let wasm_bytes = encoder.encode(model);
  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);

  // compile module
  let engine = Engine::default();
  let module = Module::from_binary(&engine, &wasm_bytes).unwrap();

  // instantiate
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  let remaining_points = instance.get_global(&mut store, "wasmarin_metering_remaining_points").unwrap();
  remaining_points.set(&mut store, Val::I64(4)).unwrap();

  // get the function handle
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();
  match add_one.call(&mut store, 2) {
    Ok(_) => {
      println!("ok");
    }
    Err(_) => {
      println!("error");
    }
  }

  println!("remaining points: {}", remaining_points.get(&mut store).i64().unwrap());
}
