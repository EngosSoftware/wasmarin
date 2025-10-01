///
/// Demonstrates the use of the `memory.fill` instruction in WebAssembly.
///
/// This example defines a module with one memory of size 1 page (64kB).
///
/// The exported function `fun` fills 11 bytes of memory starting from offset 22
/// with byte value 64 ('@' character) using `memory.fill` instruction.
///
/// After calling `fun`, the memory should contain the sequence:
///
/// ```text
/// [...20 zeros] 00 00 64 64 64 64 64 64 64 64 64 64 64 00 00 [more zeros...]
///                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ filled bytes ("@@@@@@@@@@@")
/// ```
///
/// This test asserts this by comparing the memory slice against `\0\0@@@@@@@@@@@\0\0`.
///
/// # NOTES:
///
/// - Before executing the `memory.fill` instruction, the number of bytes to be filled
///   is placed on the top of the stack.
///
/// - The `memory.fill` instruction should be benchmarked based on the number of bytes filled.
///
#[test]
fn memory_fill_should_work() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (func (export "fun")
        i32.const 22  ;; Start offset in memory;        push: 22;  stack: 22
        i32.const 64  ;; Fill with letter '@';          push: 64;  stack: 64 22
        i32.const 11  ;; Length in bytes to be filled;  push: 11;  stack: 11 64 22
        memory.fill   ;; Fill the memory;                          stack: (empty}
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
  fun.call(&mut store, ()).unwrap();
  let data = &memory.data(&mut store)[20..35];
  assert_eq!(b"\0\0@@@@@@@@@@@\0\0", data);
}
