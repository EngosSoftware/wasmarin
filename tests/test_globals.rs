#![cfg(feature = "wasmtime")]

use wasmtime::{Engine, Instance, Module, Store, Val};

#[test]
fn accessing_global_variables_should_work() {
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
  let wasm_bytes = wat::parse_str(wat_str).unwrap();

  // Compile the WASM module.
  let engine = Engine::default();
  let module = Module::from_binary(&engine, &wasm_bytes).unwrap();

  // Instantiate the module.
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  // Get the 'add_one' function.
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();

  // Execute 'add_one' function.
  let output = add_one.call(&mut store, 2).unwrap();

  // The global 'offset' variable is initially set to 2, so the result should be 2 + 2 = 4.
  assert_eq!(4, output);

  // Get the 'offset' global variable.
  let offset = instance.get_global(&mut store, "offset").unwrap();

  // Change the offset value.
  offset.set(&mut store, Val::I32(4)).unwrap();

  // Execute 'add_one' function.
  let output = add_one.call(&mut store, 1).unwrap();

  // Now the global 'offset' variable is set to 4, so the result should be 1 + 4 = 5.
  assert_eq!(5, output);
}
