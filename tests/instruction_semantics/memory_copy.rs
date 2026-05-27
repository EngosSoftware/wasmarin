///
/// Demonstrates the use of the `memory.copy` instruction in WebAssembly.
///
/// This example defines a module with one memory of size 1 page (64kB).
///
/// The exported function `fun` copies 12 bytes of memory starting from offset 0
/// to destination offset 2 using `memory.copy` instruction.
///
/// After calling `fun`, the memory should contain the sequence:
///
/// ```text
/// HeHello world!_____- 00 00[more zeros...]
///   ^^^^^^^^^^^^ copied bytes ("Hello world!")
/// ```
///
/// This test asserts this by comparing the memory slice against `HeHello world!_____-`.
///
/// # NOTES:
///
/// - Before executing the `memory.copy` instruction, the number of bytes to be copied
///   is placed on the top of the stack.
///
/// - The `memory.copy` instruction should be benchmarked based on the number of bytes copied.
///
#[test]
fn _0001() {
  let wat_str = r#"
(module
  (memory (export "mem") 1)
  (func (export "fun")
    i32.const 2   ;; Destination offset in memory;  push: 2   stack: 2
    i32.const 0   ;; Source offset in memory;       push: 0   stack: 0 2
    i32.const 12  ;; Length in bytes to be copied;  push: 12  stack: 12 0 2
    memory.copy   ;; Copy memory;                             stack: (empty)
  )
)
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  memory.write(&mut store, 0, b"Hello world!_______-").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHello world!_____-", &memory.data(&mut store)[0..20]);
}
