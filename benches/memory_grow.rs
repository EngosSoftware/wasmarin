use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

/// Number of pages to grow memory during benchmarking.
const PAGES: [usize; 10] = [1, 10, 100, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 65_535];

/// Page size of the Wasm memory.
const WASM_PAGE_SIZE: usize = 65536;

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
    .measurement_time(Duration::new(5, 0))
    .sample_size(20)
    .configure_from_args()
}

fn _0001(c: &mut Criterion) {
  // Check if the Wasm code works properly.
  for pages in PAGES {
    let wasm_bytes = wat::parse_str(wat_source(pages)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let memory = instance.exports.get_memory("mem").unwrap();
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    assert_eq!(0, fun.call(&mut store).unwrap());
    assert_eq!(pages, memory.view(&store).size().0 as usize);
    assert_eq!(pages * WASM_PAGE_SIZE, memory.view(&store).data_size() as usize);
  }
  // Execute benchmarks.
  let mut group = c.benchmark_group("memory-grow");
  for pages in PAGES {
    let wasm_bytes = wat::parse_str(wat_source(pages)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), i32>(&store, "fun").unwrap();
    group.bench_with_input(format!("pages = {pages}"), &pages, |b, &_| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

criterion_group!(name = memory_grow; config = make_config(); targets = _0001);
criterion_main!(memory_grow);
