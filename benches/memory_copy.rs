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
    let wasm_bytes = wat::parse_str(create_wat(mem_size)).unwrap();
    let compiler = wasmer::sys::Singlepass::default();
    let mut store = wasmer::Store::new(compiler);
    let module = wasmer::Module::from_binary(&store, &wasm_bytes).unwrap();
    let instance = wasmer::Instance::new(&mut store, &module, &wasmer::imports! {}).unwrap();
    let fun = instance.exports.get_typed_function::<(), ()>(&store, "fun").unwrap();
    group.bench_with_input(format!("size = {mem_size}"), &mem_size, |b, &_mem_size| b.iter(|| fun.call(&mut store).unwrap()));
  }
}

fn create_wat(length: usize) -> String {
  TEMPLATE.replace("<LENGTH>", &length.to_string())
}

fn make_config() -> Criterion {
  Criterion::default()
    .without_plots()
    .measurement_time(Duration::new(10, 0))
    .sample_size(100)
    .configure_from_args()
}

criterion_group!(name = memory_copy; config = make_config(); targets = _0001);
criterion_main!(memory_copy);

/*

|    Offset    |      1    | 200_000_000    |
| 1            | 18.628 ns |      18.433 ns |
| 100          | 20.741 ns |      20.653 ns |
| 1000         | 28.245 ns |      34.086 ns |
| 10000        | 108.00 ns |      197.97 ns |
| 100000       | 978.22 ns |      1.0393 µs |
| 1000000      | 15.474 µs |      11.930 µs |
| 10000000     | 155.88 µs |      120.25 µs |
| 100000000    | 1.6329 ms |      1.5797 ms |
| 1000000000   | 16.761 ms |      17.683 ms |
| 2000000000   | 33.573 ms |      35.146 ms |
| 4000000000   | 67.054 ms |      69.970 ms |

*/
