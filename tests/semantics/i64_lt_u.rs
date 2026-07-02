#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (func (export "fun") (result i32)
        i64.const 22  ;; Push 22        stack: [22]        c1 = 22
        i64.const 2   ;; Push 2         stack: [2, 22]     c2 = 2
        i64.lt_u      ;; Pop c2 = 2     stack: [22]
                      ;; Pop c1 = 22    stack: []
                      ;; Push c1 < c2   stack: [0]
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  assert_eq!(0, fun.call(&mut store, ()).unwrap());
}

#[test]
fn _0002() {
  let wat_str = r#"
    (module
      (func (export "fun") (result i32)
        i64.const 2   ;; Push 2         stack: [2]         c1 = 2
        i64.const 22  ;; Push 22        stack: [22, 2]     c2 = 22
        i64.lt_u      ;; Pop c2 = 22    stack: [2]
                      ;; Pop c1 = 2     stack: []
                      ;; Push c1 < c2   stack: [1]
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  assert_eq!(1, fun.call(&mut store, ()).unwrap());
}
