use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Sizes of data segments used for memory initialization during benchmarking.
const DATA_SIZES: [usize; 8] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

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
  let data = "A".repeat(length);
  TEMPLATE.replace("<DATA>", &data).replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(5, 0))
    .sample_size(20)
    .configure_from_args()
}

/// Checks if the benchmarked Wasm code works properly.
fn precheck() {
  for size in DATA_SIZES {
    let wasm_bytes = wat::parse_str(wat_source(size)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let memory = instance.exports.get_memory("mem").unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    fun.call(&mut store).unwrap();
    let start_index = size.saturating_sub(1) as u64;
    let data = memory.view(&store).copy_range_to_vec(start_index..(start_index + 2)).unwrap();
    assert_eq!(vec![65, 0], data);
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  // Execute benchmarks.
  let mut group = c.benchmark_group("memory-init");
  for size in DATA_SIZES {
    let wasm_bytes = wat::parse_str(wat_source(size)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    group.bench_with_input(format!("size = {size}"), &size, |b, &_| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

criterion_group!(name = memory_init; config = make_config(); targets = _0001);
criterion_main!(memory_init);
