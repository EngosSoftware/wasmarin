#![cfg(feature = "wasmtime")]

use wasmtime::{Engine, Instance, Module, Store, Val};

#[test]
fn a() {
  let wat_str = r#"
    (module
      (global (mut i32) (i32.const 2))
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
  let wasm_binary = wat::parse_str(wat_str).unwrap();

  // compile module
  let engine = Engine::default();
  let module = Module::from_binary(&engine, &wasm_binary).unwrap();

  // instantiate
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  // get the function handle
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();
  let output = add_one.call(&mut store, 2).unwrap();

  // the global 'offset' variable is set to 2 so the result should be 4
  assert_eq!(4, output);

  let offset = instance.get_global(&mut store, "offset").unwrap();
  offset.set(&mut store, Val::I32(4)).unwrap();
  let output = add_one.call(&mut store, 1).unwrap();

  // the global 'offset' variable is set to 4 so the result should be 6
  assert_eq!(5, output);
}
