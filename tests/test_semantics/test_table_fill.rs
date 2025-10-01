#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (table (export "tab") 21 21 funcref)
      (elem declare func $f111)
      (func $f111 (result i32) i32.const 111)
      (func (export "fun")
        i32.const 1     ;; Start offset in table;              push: 1      stack: 1
        ref.func $f111  ;; Reference value to fill the table;  push: $f111  stack: $f111 1
        i32.const 20    ;; Number of elements to be filled;    push: 20     stack: 20 $f111 1
        table.fill 0    ;; Fill the table;                                  stack: (empty)
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
  assert!(tab.get(&mut store, 2).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 3).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 4).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 5).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 6).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 7).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 8).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 9).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 10).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 11).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 12).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 13).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 14).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 15).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 16).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 17).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 18).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 19).unwrap().as_func().unwrap().is_some());
  assert!(tab.get(&mut store, 20).unwrap().as_func().unwrap().is_some());
}
