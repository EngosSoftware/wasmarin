///
/// Demonstrates the use of the `memory.init` instruction in WebAssembly.
///
/// This example defines a module with one memory and one passive data segment
/// containing the string `Hello WebAssembly!`.
///
/// The exported function `fun` copies 12 bytes starting from offset 6 in the data segment
/// into memory at offset 2, using `memory.init` instruction.
///
/// After calling `fun`, the memory should contain the sequence:
///
/// ```text
/// 00 00 57 65 62 41 73 73 65 6d 62 6c 79 21 00 00
///       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ copied bytes ("WebAssembly!")
/// ```
///
/// This test asserts this by comparing the memory slice against `\0\0WebAssembly!\0\0`.
///
/// # NOTES:
///
/// - Before executing the `memory.init` instruction, the number of bytes to be copied
///   from passive data segment to memory offset is placed on the top of the stack.
///
/// - The `memory.init` instruction should be benchmarked based on the number of bytes copied.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (data "Hello WebAssembly!")
      (func (export "fun")
        i32.const 2    ;; Destination offset in memory;                         push: 2   stack: 2
        i32.const 6    ;; Source offset in passive data segment;                push: 6   stack: 6 2
        i32.const 12   ;; Number of bytes to be copied;                         push: 12  stack: 12 6 2
        memory.init 0  ;; Use the first data segment to initialize the memory;            stack: (empty)
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
  assert_eq!(b"\0\0WebAssembly!\0\0", &memory.data(&mut store)[0..16]);
}
