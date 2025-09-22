#[test]
fn data_drop_should_work() {
  let wat_str = r#"
    (module
      (data "Hello WebAssembly!")
      (func (export "fun")
        data.drop 0    ;; Drop the data.
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
