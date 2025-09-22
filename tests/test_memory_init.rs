#[test]
fn memory_init_should_work() {
  let wat_str = r#"
    (module
      (memory 1)
      (data "Hello WebAssembly!")
      (func (export "fun")
        i32.const 2    ;; Destination offset in memory.
        i32.const 6    ;; Source offset in passive data segment.
        i32.const 12   ;; Number of bytes to be copied
        memory.init 0  ;; Use the first data segment.
      )
      (export "mem" (memory 0))
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"\0\0WebAssembly!\0\0", &memory.data(&mut store)[0..16]);
}
