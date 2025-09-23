#[test]
fn table_init_should_work() {
  let wat_str = r#"
    (module
      (table (export "tab") 10 funcref)
      (elem func $f1 $f2 $f3 $f4 $f5)
      (func $f1)
      (func $f2)
      (func $f3)
      (func $f4)
      (func $f5)
      (func (export "fun")
        i32.const 5     ;; Destination offset in the table.
        i32.const 0     ;; Source offset in the elements.
        i32.const 3     ;; Number of elements to be used as initialization.
        table.init 0 0  ;; Initialize table.
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(10, tab.size(&mut store));
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert!(tab.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 1).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 2).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 3).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 4).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 5).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 6).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 7).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 8).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 9).unwrap().as_func().unwrap().is_none());
}
