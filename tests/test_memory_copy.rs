#[test]
fn copying_memory_should_work() {
  let wat_str = r#"
    (module
      (memory 1)
      (func (export "copy_memory")
        i32.const 2     ;; destination offset in memory
        i32.const 0     ;; source offset in memory
        i32.const 12    ;; length to be copied (in bytes)
        memory.copy
      )
      (export "mem" (memory 0))
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();

  // Compile the module.
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();

  // Instantiate the module.
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

  // Get the 'mem' memory handle.
  let memory = instance.get_memory(&mut store, "mem").unwrap();

  // Write some initial data at the beginning of this memory.
  memory.write(&mut store, 0, b"Hello world!_______-").unwrap();

  // Get the 'copy_memory' function handle.
  let copy_memory = instance.get_typed_func::<(), ()>(&mut store, "copy_memory").unwrap();

  // Execute the 'copy_memory' function handle.
  copy_memory.call(&mut store, ()).unwrap();

  let data = &memory.data(&mut store)[0..20];
  assert_eq!(b"HeHello world!_____-", data);
}
