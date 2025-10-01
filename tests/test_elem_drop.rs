///
/// Demonstrates the use of the `elem.drop` instruction in WebAssembly.
///
/// This example defines two passive element segments.
///
/// The exported function `fun` drops both passive element segments using `elem.drop` instruction.
///
/// # NOTES:
///
/// - The `elem.drop` instruction does not depend on element segment size in runtime,
///   but dropping element segment itself may cause freeing the memory used by this
///   element segment which may (or not) consume more resources.
///
/// - The `elem.drop` instruction should be benchmarked based on the size of passive element segment.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (elem func $f1 $f2 $f3 $f4 $f5 $f6)
      (elem func $f2 $f3 $f4)
      (func $f1)
      (func $f2)
      (func $f3)
      (func $f4)
      (func $f5)
      (func $f6)
      (func (export "fun")
        elem.drop 0  ;; Drop passive element segment 0;  stack: (empty)
        elem.drop 1  ;; Drop passive element segment 1;  stack: (empty)
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
