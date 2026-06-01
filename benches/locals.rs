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
