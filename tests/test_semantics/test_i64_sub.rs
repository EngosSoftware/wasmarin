#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (func (export "fun") (result i64)
        i64.const 10    ;; push: 10             stack: 10
        i64.const 3     ;; push: 3              stack: 3 10
        i64.sub         ;; subtract: 10-3       stack: 7
      )
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i64>(&mut store, "fun").unwrap();
  assert_eq!(7, fun.call(&mut store, ()).unwrap());
}
