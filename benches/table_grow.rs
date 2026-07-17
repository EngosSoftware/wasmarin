use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Number of elements the table will be extended in benchmarks.
const GROWTHS: [usize; 9] = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

/// Initial size of the benchmarked table.
const INITIAL: usize = 0;

const TEMPLATE: &str = r#"
(module
  (table (export "tab") <INITIAL> funcref)
  (elem func $f1)
  (func $f1)
  (func (export "fun") (result i32)
    ref.func $f1
    i32.const <GROW>
    table.grow 0
  )
)
"#;

fn wat_source(grow: usize) -> String {
  TEMPLATE.replace("<INITIAL>", &INITIAL.to_string()).replace("<GROW>", &grow.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(5, 0))
    .sample_size(20)
    .configure_from_args()
}

/// Checks if the benchmarked Wasm code works as expected.
fn precheck() {
  for grow in GROWTHS {
    let wasm_bytes = wat::parse_str(wat_source(grow)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let tab = instance.exports.get_table("tab").unwrap();
    assert_eq!(INITIAL, tab.size(&store) as usize);
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    assert_eq!(INITIAL as i32, fun.call(&mut store).unwrap());
    assert_eq!(INITIAL + grow, tab.size(&store) as usize);
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  // Execute benchmarks.
  let mut group = c.benchmark_group("table-grow");
  for grow in GROWTHS {
    let wasm_bytes = wat::parse_str(wat_source(grow)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    group.bench_with_input(format!("grow = {grow}"), &grow, |b, &_| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

criterion_group!(name = table_grow; config = make_config(); targets = _0001);
criterion_main!(table_grow);
