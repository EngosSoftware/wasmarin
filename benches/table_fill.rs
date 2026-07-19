use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Number of elements to be filled in the benchmarked table.
const LENGTHS: [usize; 9] = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 9_999_999];

const TEMPLATE: &str = r#"
(module
  (table (export "tab") <INITIAL> funcref)
  (elem func $f1)
  (func $f1 (result i32) i32.const 1)
  (func (export "fun")
    i32.const 0         ;; Start offset in table
    ref.func $f1        ;; Reference value to fill the table
    i32.const <LENGTH>  ;; Number of elements to be filled
    table.fill 0        ;; Execute filling the table
  )
)
"#;

fn wat_source(length: usize) -> String {
  let initial = length + 1;
  TEMPLATE.replace("<INITIAL>", &initial.to_string()).replace("<LENGTH>", &length.to_string())
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
    if length == 0 {
      assert!(tab.get(&mut store, 0).unwrap().funcref().unwrap().is_none());
    } else {
      assert!(tab.get(&mut store, (length - 1) as u32).unwrap().funcref().unwrap().is_some());
      assert!(tab.get(&mut store, length as u32).unwrap().funcref().unwrap().is_none());
    }
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("table-fill");
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    group.bench_with_input(format!("length = {length}"), &length, |b, &_| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

criterion_group!(name = table_fill; config = make_config(); targets = _0001);
criterion_main!(table_fill);
