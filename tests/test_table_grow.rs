#[test]
fn table_grow_should_work() {
  let wat_str = r#"
    (module
      (table 2 funcref)
      (func (export "fun") (result i32)
        ref.null func  ;; New table elements with be null function references.
        i32.const 100  ;; Number of new elements in the table.
        table.grow 0   ;; Grow the table.
        drop           ;; Drop the old table size.
        table.size 0   ;; Return the new table size.
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  assert_eq!(102, fun.call(&mut store, ()).unwrap());
}
