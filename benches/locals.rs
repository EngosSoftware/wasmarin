#![feature(test)]

extern crate test;
use test::Bencher;

#[bench]
fn _0001(b: &mut Bencher) {
  const WAT: &str = include_str!("../tests/contracts/big_locals.wat");
  let wasm_bytes = wat::parse_str(WAT).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  b.iter(|| _ = fun.call(&mut store, ()).unwrap());
}

#[bench]
fn _0002(b: &mut Bencher) {
  const WAT: &str = include_str!("../tests/contracts/big_locals.wat");
  let wasm_bytes = wat::parse_str(WAT).unwrap();
  let mut config = wasmtime::Config::new();
  config.strategy(wasmtime::Strategy::Winch);
  let engine = wasmtime::Engine::new(&config).unwrap();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  b.iter(|| _ = fun.call(&mut store, ()).unwrap());
}

#[bench]
fn _0003(b: &mut Bencher) {
  const WAT: &str = include_str!("../tests/contracts/big_locals.wat");
  let wasm_bytes = wat::parse_str(WAT).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "wrapper").unwrap();
  assert_eq!(10, fun.call(&mut store, ()).unwrap());
  b.iter(|| _ = fun.call(&mut store, ()).unwrap());
}

#[bench]
fn _0004(b: &mut Bencher) {
  const WAT: &str = include_str!("../tests/contracts/big_locals.wat");
  let wasm_bytes = wat::parse_str(WAT).unwrap();
  let mut config = wasmtime::Config::new();
  config.strategy(wasmtime::Strategy::Winch);
  let engine = wasmtime::Engine::new(&config).unwrap();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "wrapper").unwrap();
  assert_eq!(10, fun.call(&mut store, ()).unwrap());
  b.iter(|| _ = fun.call(&mut store, ()).unwrap());
}

#[bench]
fn _0005(b: &mut Bencher) {
  let wasm_bytes = wat::parse_str(create_wat(500)).unwrap();
  let compiler = wasmer::sys::Singlepass::default();
  let mut store = wasmer::Store::new(compiler);
  let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
  let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
  let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
  assert_eq!(10, fun.call(&mut store).unwrap());
  b.iter(|| _ = fun.call(&mut store).unwrap());
}

#[bench]
fn _0006(b: &mut Bencher) {
  let wasm_bytes = wat::parse_str(create_wat(50000)).unwrap();
  let compiler = wasmer::sys::Singlepass::default();
  let mut store = wasmer::Store::new(compiler);
  let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
  let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
  let fun = instance.exports.get_typed_function::<(), i32>(&store, "wrapper").unwrap();
  assert_eq!(10, fun.call(&mut store).unwrap());
  b.iter(|| _ = fun.call(&mut store).unwrap());
}

const TEMPLATE: &str = r#"(module
  (memory 1)
  (func $fun (export "fun") (result i32)
    (local;;LOCAL;;)
    i32.const 10
  )
  (func (export "wrapper") (result i32)
    call $fun
  )
  (export "mem" (memory 0))
)"#;

fn create_wat(mut n: usize) -> String {
  n = n.max(1);
  let locals = " i32".repeat(n);
  TEMPLATE.replace(";;LOCAL;;", &locals)
}

#[test]
fn creating_wat_should_work() {
  assert_eq!("(module\n  (memory 1)\n  (func $fun (export \"fun\") (result i32);\n    (local i32)\n    i32.const 10\n  )\n  (func (export \"wrapper\") (result i32)\n    call $fun\n  )\n  (export \"mem\" (memory 0))\n)", create_wat(1));
}
