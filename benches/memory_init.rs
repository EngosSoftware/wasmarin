use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Lengths used for benchmarking.
const LENGTHS: [usize; 21] = [
  0,
  1,
  10,
  100,
  200,
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
];

const TEMPLATE: &str = r#"
(module
  (memory (export "mem") 65536)
  (data "<DATA>")
  (func (export "fun")
    i32.const 0         ;; Destination offset in memory
    i32.const 0         ;; Source offset in passive data segment
    i32.const <LENGTH>  ;; Number of bytes to be copied
    memory.init 0       ;; Use the first data segment to initialize the memory
  )
)
"#;

fn wat_source(length: usize) -> String {
  let data = "A".repeat(length + 1);
  TEMPLATE.replace("<DATA>", &data).replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(10, 0))
    .sample_size(20)
    .configure_from_args()
}

/// Checks if the benchmarked Wasm code works properly.
fn precheck() {
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let memory = instance.exports.get_memory("mem").unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    fun.call(&mut store).unwrap();
    let start_index = length.saturating_sub(1) as u64;
    let data = memory.view(&store).copy_range_to_vec(start_index..(start_index + 2)).unwrap();
    if length == 0 {
      assert_eq!(vec![0, 0], data);
    } else {
      assert_eq!(vec![65, 0], data);
    }
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("memory-init");
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

criterion_group!(name = memory_init; config = make_config(); targets = _0001);
criterion_main!(memory_init);
