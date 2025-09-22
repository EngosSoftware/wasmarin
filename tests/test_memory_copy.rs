#[test]
fn memory_copy_should_work() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (func (export "fun")
        i32.const 2   ;; Destination offset in memory.
        i32.const 0   ;; Source offset in memory.
        i32.const 12  ;; Length in bytes to be copied.
        memory.copy
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();

  memory.write(&mut store, 0, b"Hello world!_______-").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHello world!_____-", &memory.data(&mut store)[0..20]);
}
