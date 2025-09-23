#[test]
fn elem_drop_should_work() {
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
        elem.drop 0  ;; Drop passive element segment 0
        elem.drop 1  ;; Drop passive element segment 1
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
