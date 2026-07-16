use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

const MEM_SIZE: [usize; 11] = [
  1,
  100,
  1_000,
  10_000,
  100_000,
  1_000_000,
  10_000_000,
  100_000_000,
  1_000_000_000,
  2_000_000_000,
  4_000_000_000,
];

const TEMPLATE: &str = r#"
(module
  (memory 65536)
  (func (export "fun")
    i32.const 294967295    ;; Destination offset in memory
    i32.const 0            ;; Source offset in memory
    i32.const <LENGTH>     ;; Length in bytes to be copied
    memory.copy            ;; Execute memory copy
  )
)
"#;

fn _0001(c: &mut Criterion) {
  let mut group = c.benchmark_group("memory-copy");
  for mem_size in MEM_SIZE {
    let wasm_bytes = wat::parse_str(wat_source(mem_size)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    group.bench_with_input(format!("size = {mem_size}"), &mem_size, |b, &_mem_size| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

fn wat_source(length: usize) -> String {
  TEMPLATE.replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(20, 0))
    .sample_size(20)
    .configure_from_args()
}

criterion_group!(name = memory_copy; config = make_config(); targets = _0001);
criterion_main!(memory_copy);
