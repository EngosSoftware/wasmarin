#[test]
fn i64_div_u() {
  let wat_str = r#"
    (module
      (func (export "fun") (result i64)
        i64.const 10    ;; push: 10       stack: 10
        i64.const 4     ;; push: 4        stack: 4 10
        i64.div_u       ;; divide: 10/4   stack: 2
      )
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i64>(&mut store, "fun").unwrap();
  assert_eq!(2, fun.call(&mut store, ()).unwrap());
}
