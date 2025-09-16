use wasmtime::{Engine, Instance, Module, Store};

#[test]
fn a() {
  let wat_str = r#"
    (module
      (memory (export "memory") 1)
      (type $add_t (func (param i32) (result i32)))
      (func $add_one_f (type $add_t) (param $value i32) (result i32)
        local.get $value
        i32.const 1
        i32.add
      )
      (export "add_one" (func $add_one_f)))
  "#;
  let wasm_binary = wat::parse_str(wat_str).unwrap();
  // compile module
  let engine = Engine::default();
  let module = Module::from_binary(&engine, &wasm_binary).unwrap();
  // instantiate
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[]).unwrap();
  // get the function handle
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();
  let output = add_one.call(&mut store, 2).unwrap();
  assert_eq!(3, output);
}
