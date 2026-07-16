use criterion::{criterion_group, criterion_main, Criterion};

const LOCALS: [usize; 36] = [
  1, 2, 5, 10, 15, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000,
  50000,
];

fn _0001(c: &mut Criterion) {
  let mut group = c.benchmark_group("g_wasmtime_cranelift");
  for locals in LOCALS {
    let wasm_bytes = wat::parse_str(create_wat(locals)).unwrap();
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
    let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
    group.bench_with_input(format!("locals = {locals}"), &locals, |b, &_locals| b.iter(|| fun.call(&mut store, ()).unwrap()));
  }
}

fn _0002(c: &mut Criterion) {
  let mut group = c.benchmark_group("g_wasmtime_winch");
  for locals in LOCALS {
    let wasm_bytes = wat::parse_str(create_wat(locals)).unwrap();
    let mut config = wasmtime::Config::new();
    config.strategy(wasmtime::Strategy::Winch);
    let engine = wasmtime::Engine::new(&config).unwrap();
    let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
    let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
    group.bench_with_input(format!("locals = {locals}"), &locals, |b, &_locals| b.iter(|| fun.call(&mut store, ()).unwrap()));
  }
}

fn _0003(c: &mut Criterion) {
  let mut group = c.benchmark_group("g_wasmer_singlepass");
  for locals in LOCALS {
    let wasm_bytes = wat::parse_str(create_wat(locals)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    group.bench_with_input(format!("locals = {locals}"), &locals, |b, &_locals| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

const TEMPLATE: &str = r#"
(module
  (memory 0 1)
  (func (export "fun") (result i32)
    (local<LOCALS>)
    i32.const 10
  )
  (export "mem" (memory 0))
)
"#;

fn create_wat(mut n: usize) -> String {
  n = n.max(1);
  let locals = " i32".repeat(n);
  TEMPLATE.replace("<LOCALS>", &locals)
}

criterion_group!(benches, _0001, _0002, _0003);
criterion_main!(benches);
