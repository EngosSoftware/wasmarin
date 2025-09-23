#[test]
fn table_fill_should_work() {
  let wat_str = r#"
    (module
      (table (export "tab") 21 21 funcref)
      (elem declare func $f111)
      (func $f111 (result i32) i32.const 111)
      (func (export "fun")
        i32.const 1      ;; Start offset in table.
        ref.func $f111   ;; Reference value to fill the table.
        i32.const 20     ;; Number of elements to be filled.
        table.fill 0     ;; Fill the table.
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(21, tab.size(&mut store));
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert!(tab.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 1).unwrap().as_func().unwrap().is_some());
}
