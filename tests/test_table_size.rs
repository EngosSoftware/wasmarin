#[test]
fn table_size_should_work() {
  let wat_str = r#"
    (module
      (table 2025 funcref)
      (func (export "fun") (result i32)
        table.size 0   ;; Return the size of the first table.
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();

  // The size of the table is 2025.
  assert_eq!(2025, fun.call(&mut store, ()).unwrap());
}
