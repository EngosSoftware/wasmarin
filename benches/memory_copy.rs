use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

#[cfg(target_os = "macos")]
const MEASUREMENT_TIME: u64 = 35;
#[cfg(target_os = "linux")]
const MEASUREMENT_TIME: u64 = 20;

const SAMPLE_SIZE: usize = 20;

/// Lengths used for benchmarking.
const LENGTHS: [usize; 30] = [
  1,
  2,
  5,
  10,
  20,
  50,
  100,
  200,
  500,
  1_000,
  2_000,
  5_000,
  10_000,
  20_000,
  50_000,
  100_000,
  200_000,
  500_000,
  1_000_000,
  2_000_000,
  5_000_000,
  10_000_000,
  20_000_000,
  50_000_000,
  100_000_000,
  200_000_000,
  500_000_000,
  1_000_000_000,
  2_000_000_000,
  4_000_000_000,
];

const TEMPLATE: &str = r#"
(module
  (memory 65536)
  (func (export "fun")
    i32.const 10           ;; Destination offset in memory
    i32.const 0            ;; Source offset in memory
    i32.const <LENGTH>     ;; Length in bytes to be copied
    memory.copy            ;; Execute memory copy
  )
)
"#;

fn wat_source(length: usize) -> String {
  TEMPLATE.replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(MEASUREMENT_TIME, 0))
    .sample_size(SAMPLE_SIZE)
    .configure_from_args()
}

fn _0001(c: &mut Criterion) {
  let mut group = c.benchmark_group("m.copy");
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    group.bench_with_input(format!("{length}"), &length, |b, _| {
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

criterion_group!(name = memory_copy; config = make_config(); targets = _0001);
criterion_main!(memory_copy);
