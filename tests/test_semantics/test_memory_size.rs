///
/// Demonstrates the use of the `memory.size` instruction in WebAssembly.
///
/// This example defines a module with one memory of size 7 pages (7 x 64kB).
///
/// # NOTES:
///
/// The `memory.size` instruction execution time does not depend on memory size.
///
#[test]
fn memory_size_should_work() {
  let wat_str = r#"
    (module
      (memory (export "mem") 7)
      (func (export "fun") (result i32)
        memory.size  ;; Return the size of the memory;  push: 7  stack: 7
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
  assert_eq!(7, fun.call(&mut store, ()).unwrap());
  assert_eq!(7, memory.size(&mut store));
  assert_eq!(458752, memory.data_size(&mut store));
}
