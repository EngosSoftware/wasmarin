#[test]
fn memory_fill_should_work() {
  let wat_str = r#"
    (module
      (memory 1)
      (func (export "fun_memory_fill")
        i32.const 22  ;; Start offset in memory.
        i32.const 64  ;; Fill with letter '@'.
        i32.const 11  ;; Length in bytes to be filled.
        memory.fill
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
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun_memory_fill").unwrap();
  fun.call(&mut store, ()).unwrap();
  let data = &memory.data(&mut store)[20..40];
  assert_eq!(b"\0\0@@@@@@@@@@@\0\0\0\0\0\0\0", data);
}
