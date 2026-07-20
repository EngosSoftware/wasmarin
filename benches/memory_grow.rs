use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

#[cfg(target_os = "macos")]
const MEASUREMENT_TIME: u64 = 1;
#[cfg(target_os = "linux")]
const MEASUREMENT_TIME: u64 = 1;

const SAMPLE_SIZE: usize = 20;

/// Lengths used for benchmarking.
const LENGTHS: [usize; 17] = [0, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 65_535];

/// Page size of the Wasm memory.
const WASM_PAGE_SIZE: usize = 65_536;

const TEMPLATE: &str = r#"
(module
  (memory (export "mem") 0)
  (func (export "fun") (result i32)
    i32.const <PAGES>  ;; Number of pages to grow the memory
    memory.grow        ;; Execute memory growing
  )
)
"#;

fn wat_source(pages: usize) -> String {
  TEMPLATE.replace("<PAGES>", &pages.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(MEASUREMENT_TIME, 0))
    .sample_size(SAMPLE_SIZE)
    .configure_from_args()
}

/// Checks if the benchmarked Wasm code works.
fn precheck() {
  for lengths in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(lengths)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let memory = instance.exports.get_memory("mem").unwrap();
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    assert_eq!(0, fun.call(&mut store).unwrap());
    assert_eq!(lengths, memory.view(&store).size().0 as usize);
    assert_eq!(lengths * WASM_PAGE_SIZE, memory.view(&store).data_size() as usize);
  }
}

fn _0001(c: &mut Criterion) {
  precheck();
  let mut group = c.benchmark_group("memory-grow");
  for length in LENGTHS {
    let wasm_bytes = wat::parse_str(wat_source(length)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    group.bench_with_input(format!("L = {length}"), &length, |b, _| {
      b.iter_batched(
        || {
          let mut store = wasmer::Store::new(wasmer::sys::Singlepass::default());
          let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
          let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
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

criterion_group!(name = memory_grow; config = make_config(); targets = _0001);
criterion_main!(memory_grow);
