///
/// Demonstrates the use of the `table.grow` instruction in WebAssembly.
///
/// This example defines a table with initial size of 2 elements.
///
/// The exported function `fun` grows the table with 100 additional elements
/// being null references using `table.grow` instruction.
///
/// # NOTES:
///
/// - New element is also placed on the stack.
///
/// - Before executing the `table.grow` instruction, the number of elements
///   to be added to the table is placed on the top of the stack.
///
/// - The `table.grow` instruction should be benchmarked based on the number of elements to grow.
///
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (table 2 funcref)
      (func (export "fun") (result i32)
        ref.null func  ;; New elements with be null function references;  push: null  stack: null
        i32.const 100  ;; Number of new elements in the table;            push: 100   stack: 100 null
        table.grow 0   ;; Grow the table;                                             stack: 2 (old size)
        drop           ;; Drop the old table size.                                    stack: (empty)
        table.size 0   ;; Return the new table size.                                  stack: 102
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  assert_eq!(102, fun.call(&mut store, ()).unwrap());
}
