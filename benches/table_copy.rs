use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Number of elements to copy in benchmarks.
const LENGTHS: [usize; 9] = [0, 1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 9_999_999];

const TEMPLATE: &str = r#"
(module
  (table $src (export "src") <INITIAL> funcref)
  (table $dst (export "dst") <INITIAL> funcref)
  (elem (table $src) (i32.const 0) func <ELEMENTS>)
  (func $f1)
  (func (export "fun")
    i32.const 0          ;; Destination offset in table $dst
    i32.const 0          ;; Source offset in table $src
    i32.const <LENGTH>   ;; Number of elements to be copied
    table.copy $dst $src ;; Copy elements from table $src to table $dst
  )
)
"#;

fn wat_source(length: usize) -> String {
  let initial = length + 1;
  let elements = " $f1".repeat(length + 1);
  TEMPLATE
    .replace("<INITIAL>", &initial.to_string())
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
    let tab_src = instance.exports.get_table("src").unwrap();
    assert_eq!(length + 1, tab_src.size(&store) as usize);
    assert!(tab_src.get(&mut store, 0).unwrap().funcref().unwrap().is_some());
    assert!(tab_src.get(&mut store, length as u32).unwrap().funcref().unwrap().is_some());
    let tab_dst = instance.exports.get_table("dst").unwrap();
    assert_eq!(length + 1, tab_dst.size(&store) as usize);
    assert!(tab_dst.get(&mut store, 0).unwrap().funcref().unwrap().is_none());
    assert!(tab_dst.get(&mut store, length as u32).unwrap().funcref().unwrap().is_none());
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    fun.call(&mut store).unwrap();
    if length == 0 {
      assert!(tab_dst.get(&mut store, 0).unwrap().funcref().unwrap().is_none());
    } else {
      assert!(tab_dst.get(&mut store, 0).unwrap().funcref().unwrap().is_some());
      assert!(tab_dst.get(&mut store, (length - 1) as u32).unwrap().funcref().unwrap().is_some());
      assert!(tab_dst.get(&mut store, length as u32).unwrap().funcref().unwrap().is_none());
    }
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("table-copy");
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

criterion_group!(name = table_copy; config = make_config(); targets = _0001);
criterion_main!(table_copy);
