#[test]
fn memory_size_should_work() {
  let wat_str = r#"
    (module
      (memory (export "mem") 7)
      (func (export "fun") (result i32)
        memory.size  ;; Return the size of the memory.
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

  // The size of the memory is 7 pages.
  assert_eq!(7, fun.call(&mut store, ()).unwrap());
  assert_eq!(7, memory.size(&mut store));
}
