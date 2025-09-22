#[test]
fn memory_grow_should_work() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (func (export "fun") (result i32)
        i32.const 2   ;; Number of pages to grow the memory.
        memory.grow   ;; Grow the memory.
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();

  // The size before growing is 1 page, the new size is 3 pages.
  assert_eq!(1, fun.call(&mut store, ()).unwrap());
  assert_eq!(3, memory.size(&mut store));

  // The size before growing is 3 pages, the new size is 5 pages.
  assert_eq!(3, fun.call(&mut store, ()).unwrap());
  assert_eq!(5, memory.size(&mut store));
}
