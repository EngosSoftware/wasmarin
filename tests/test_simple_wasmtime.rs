/*
use wasmtime::{Config, Engine, Instance, Module, Store};

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
  let mut config = Config::new();
  config.consume_fuel(true);
  let engine = Engine::new(&config).unwrap();
  let module = Module::from_binary(&engine, &wasm_binary).unwrap();
  // instantiate
  let mut store = Store::new(&engine, ());
  store.set_fuel(10_000).unwrap();
  let instance = Instance::new(&mut store, &module, &[]).unwrap();
  // get the function handle
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();
  let output = add_one.call(&mut store, 2).unwrap();
  assert_eq!(3, output);
  println!("{}", store.get_fuel().unwrap());
}

#[test]
fn b() {
  let wat_str = r#"
    (module
      (memory (export "memory") 1)
      (data "Hello, World!")
      (func (export "init_memory")
        i32.const 10   ;; destination offset in memory
        i32.const 0    ;; source offset in data segment
        i32.const 5    ;; length
        memory.init 0
        data.drop 0
      )
    )
    "#;
  let wasm_binary = wat::parse_str(wat_str).unwrap();
  // compile module
  let mut config = Config::new();
  config.consume_fuel(true);
  let engine = Engine::new(&config).unwrap();
  let module = Module::from_binary(&engine, &wasm_binary).unwrap();
  // instantiate
  let mut store = Store::new(&engine, ());
  store.set_fuel(10_000).unwrap();
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  // get the function handle
  let init = instance.get_typed_func::<(), ()>(&mut store, "init_memory").unwrap();
  init.call(&mut store, ()).unwrap();

  println!("{}", store.get_fuel().unwrap());
}

#[test]
fn c() {
  let wat_str = r#"
    (module
      (memory 1)
      (func (export "copy_memory")
        i32.const 2     ;; destination offset in memory
        i32.const 0     ;; source offset in memory
        i32.const 40000 ;; length
        memory.copy
      )
    )
    "#;
  let wasm_binary = wat::parse_str(wat_str).unwrap();
  // compile module
  let mut config = Config::new();
  config.consume_fuel(true);
  let engine = Engine::new(&config).unwrap();
  let module = Module::from_binary(&engine, &wasm_binary).unwrap();
  // instantiate
  let mut store = Store::new(&engine, ());
  store.set_fuel(10_000).unwrap();
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  // get the function handle
  let copy_memory = instance.get_typed_func::<(), ()>(&mut store, "copy_memory").unwrap();
  copy_memory.call(&mut store, ()).unwrap();

  println!("{}", store.get_fuel().unwrap());
}
*/
