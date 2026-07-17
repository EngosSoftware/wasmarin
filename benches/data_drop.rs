use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Sizes of data segments used for benchmarking.
const LENGTHS: [usize; 8] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

const TEMPLATE: &str = r#"
(module
  (data "<DATA>")
  (func (export "fun")
    data.drop 0    ;; Drop passive data segment 0
  )
)
"#;

fn wat_source(length: usize) -> String {
  let data = "A".repeat(length);
  TEMPLATE.replace("<DATA>", &data)
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(1, 0))
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
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    fun.call(&mut store).unwrap();
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("data-drop");
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

criterion_group!(name = data_drop; config = make_config(); targets = _0001);
criterion_main!(data_drop);
