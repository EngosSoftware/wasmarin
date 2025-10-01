///
/// Demonstrates the use of the `data.drop` instruction in WebAssembly.
///
/// This example defines two passive data segments.
///
/// The exported function `fun` drops both passive data segments using `data.drop` instruction.
///
/// # NOTES:
///
/// - The `data.drop` instruction does not depend on data segment size in runtime,
///   but dropping data segment itself may cause freeing the memory used by this
///   data segment which may (or not) consume more resources.
///
/// - The `data.drop` instruction should be benchmarked based on the size of passive data segment.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (data "Hello WebAssembly!")
      (data "Hello world!")
      (func (export "fun")
        data.drop 0    ;; Drop passive data segment 0;  stack: (empty)
        data.drop 1    ;; Drop passive data segment 1;  stack: (empty)
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();
}
