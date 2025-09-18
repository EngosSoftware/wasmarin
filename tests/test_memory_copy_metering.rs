use wasmtime::Val;

#[test]
fn memory_copy_metering_should_work() {
  let wat_str = r#"
    (module
      (memory 1)
      (func (export "copy_memory")
        i32.const 2   ;; destination offset in memory
        i32.const 0   ;; source offset in memory
        i32.const 12  ;; length to be copied (in bytes)
        memory.copy
      )
      (export "mem" (memory 0))
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();

  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&wasm_bytes).unwrap();
  let mut encoder = wasmarin::Encoder::new_with_metering();
  let wasm_bytes = encoder.encode(model);
  println!("{}", wasmprinter::print_bytes(&wasm_bytes).unwrap());

  // Compile the module.
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();

  // Instantiate the module.
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

  let remaining_points = instance.get_global(&mut store, "wasmarin_metering_remaining_points").unwrap();
  remaining_points.set(&mut store, Val::I64(500)).unwrap();

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

  println!("remaining points: {}", remaining_points.get(&mut store).i64().unwrap());
}
