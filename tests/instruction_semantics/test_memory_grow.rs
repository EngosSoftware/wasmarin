///
/// Demonstrates the use of the `memory.grow` instruction in WebAssembly.
///
/// This example defines a module with memory with initial size of 1 page (64kB).
///
/// The exported function `fun` grows the memory with 2 additional pages (2 x 64kB)
/// using `memory.grow` instruction.
///
/// # NOTES:
///
/// - Before executing the `memory.grow` instruction, the number of pages
///   to be added to memory is placed on the top of the stack.
///
/// - The `memory.grow` instruction should be benchmarked based on the number of pages.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (func (export "fun") (result i32)
        i32.const 2   ;; Number of pages to grow the memory;     push: 2  stack: 2
        memory.grow   ;; Grow the memory, return previous size;  push: 1  stack: 1
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  // The size before growing is 1 page, the new size is 3 pages.
  assert_eq!(1, fun.call(&mut store, ()).unwrap());
  assert_eq!(3, memory.size(&mut store));
  assert_eq!(196608, memory.data_size(&mut store));
  // The size before growing is 3 pages, the new size is 5 pages.
  assert_eq!(3, fun.call(&mut store, ()).unwrap());
  assert_eq!(5, memory.size(&mut store));
  assert_eq!(327680, memory.data_size(&mut store));
}
