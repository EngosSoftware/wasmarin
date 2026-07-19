use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Lengths used for benchmarking.
/// In wasmparser, in the limits.rs file, the maximum number of table entries is set to 10_000_000.
const LENGTHS: [usize; 22] = [
  1, 2, 5, 10, 20, 50, 100, 200, 500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 100_000, 200_000, 500_000, 1_000_000, 2_000_000, 5_000_000, 9_999_999,
];

const TEMPLATE: &str = r#"
(module
  (table (export "tab") <TABLE-LENGTH> funcref)
  (elem func <ELEMENTS>)
  (func $f1)
  (func (export "fun")
    i32.const 0         ;; Destination offset in the table
    i32.const 0         ;; Source offset in the elements
    i32.const <LENGTH>  ;; Number of elements to be used for initialization
    table.init 0 0      ;; Execute table initialization
  )
)
"#;

fn wat_source(length: usize) -> String {
  let table_length = length + 1;
  let elements = " $f1".repeat(length);
  TEMPLATE
    .replace("<TABLE-LENGTH>", &table_length.to_string())
    .replace("<ELEMENTS>", &elements)
    .replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(5, 0))
    .sample_size(20)
    .configure_from_args()
}

/// Checks if the benchmarked Wasm code works.
fn precheck() {
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let tab = instance.exports.get_table("tab").unwrap();
    assert_eq!(length + 1, tab.size(&store) as usize);
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    fun.call(&mut store).unwrap();
    for index in 0..length {
      assert!(tab.get(&mut store, index as u32).unwrap().funcref().unwrap().is_some());
    }
    assert!(tab.get(&mut store, length as u32).unwrap().funcref().unwrap().is_none());
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("table-init");
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    group.bench_with_input(format!("length = {length}"), &length, |b, _| {
      b.iter_batched(
        || {
          let mut store = wasmer::Store::new(wasmer::sys::Singlepass::default());
          let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
          let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
          (store, fun)
        },
        |(mut store, fun)| {
          fun.call(&mut store).unwrap();
        },
        criterion::BatchSize::LargeInput,
      );
    });
  }
}

criterion_group!(name = table_init; config = make_config(); targets = _0001);
criterion_main!(table_init);
