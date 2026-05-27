///
/// Demonstrates the use of the `table.init` instruction in WebAssembly.
///
/// This example defines a table with space for 10 function references.
///
/// The exported function `fun` initializes this table with 3 function references
/// starting from destination offset 5 using `table.init` instruction.
///
/// After calling `fun`, the table should contain the following sequence of elements:
///
/// ```text
/// 0 0 0 0 0 Ref(f1) Ref(f2) Ref(f3) 0 0
/// ```
///
/// # NOTES:
///
/// - Before executing the `table.init` instruction, the number of elements to be initialized
///   is placed on the top of the stack.
///
/// - The `table.init` instruction should be benchmarked based on the number
///   of elements to be initialized.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (table (export "tab") 10 funcref)
      (elem func $f1 $f2 $f3 $f4 $f5)
      (func $f1)
      (func $f2)
      (func $f3)
      (func $f4)
      (func $f5)
      (func (export "fun")
        i32.const 5     ;; Destination offset in the table;                  push: 5  stack: 5
        i32.const 0     ;; Source offset in the elements;                    push: 0  stack: 0 5
        i32.const 3     ;; Number of elements to be used as initialization;  push: 3  stack: 3 0 5
        table.init 0 0  ;; Initialize table;                                          stack: (empty)
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(10, tab.size(&mut store));
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();
  assert!(tab.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 1).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 2).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 3).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 4).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 5).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 6).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 7).unwrap().as_func().unwrap().is_some()); // filled
  assert!(tab.get(&mut store, 8).unwrap().as_func().unwrap().is_none());
  assert!(tab.get(&mut store, 9).unwrap().as_func().unwrap().is_none());
}
