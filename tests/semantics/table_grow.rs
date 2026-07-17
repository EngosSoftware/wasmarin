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
      (table (export "tab") 2 funcref)
      (func (export "fun") (result i32 i32)
        ref.null func  ;; New elements with be null function references;  push: null  stack: null
        i32.const 100  ;; Number of new elements in the table;            push: 100   stack: 100 null
        table.grow 0   ;; Grow the table;                                             stack: 2 (old size)
        table.size 0   ;; Push the new table size;                                    stack: 102 2 (new size, old size)
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(2, tab.size(&store));
  let fun = instance.get_typed_func::<(), (i32, i32)>(&mut store, "fun").unwrap();
  assert_eq!((2, 102), fun.call(&mut store, ()).unwrap());
  assert_eq!(102, tab.size(&store));
  for index in 0..102 {
    assert!(tab.get(&mut store, index).unwrap().as_func().unwrap().is_none());
  }
}

#[test]
fn _0002() {
  let wat_str = r#"
    (module
      (table (export "tab") 2 funcref)
      (elem func $f1)
      (func $f1)
      (func (export "fun") (result i32 i32)
        ref.func $f1
        i32.const 5
        table.grow 0
        table.size 0
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(2, tab.size(&store));
  let fun = instance.get_typed_func::<(), (i32, i32)>(&mut store, "fun").unwrap();
  assert_eq!((2, 7), fun.call(&mut store, ()).unwrap());
  assert_eq!(7, tab.size(&store));
  assert!(tab.get(&mut store, 0).unwrap().as_func().unwrap().is_none()); // NULL
  assert!(tab.get(&mut store, 1).unwrap().as_func().unwrap().is_none()); // NULL
  assert!(tab.get(&mut store, 2).unwrap().as_func().unwrap().is_some()); // FUNC
  assert!(tab.get(&mut store, 3).unwrap().as_func().unwrap().is_some()); // FUNC
  assert!(tab.get(&mut store, 4).unwrap().as_func().unwrap().is_some()); // FUNC
  assert!(tab.get(&mut store, 5).unwrap().as_func().unwrap().is_some()); // FUNC
  assert!(tab.get(&mut store, 6).unwrap().as_func().unwrap().is_some()); // FUNC
}

#[test]
fn _0003() {
  let wat_str = r#"
    (module
      (table (export "tab") 0 funcref)
      (func (export "fun") (result i32 i32)
        ref.null func
        i32.const <GROW>
        table.grow 0
        table.size 0
      )
    )
    "#;
  let grow = 10_000_000;
  let wasm_bytes = wat::parse_str(wat_str.replace("<GROW>", &grow.to_string())).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(0, tab.size(&store));
  let fun = instance.get_typed_func::<(), (i32, i32)>(&mut store, "fun").unwrap();
  assert_eq!((0, grow), fun.call(&mut store, ()).unwrap());
  assert_eq!(grow as u64, tab.size(&store));
}

#[test]
fn _0004() {
  let wat_str = r#"
    (module
      (table (export "tab") <INITIAL> funcref)
      (elem func $f1)
      (func $f1)
      (func (export "fun") (result i32 i32)
        ref.func $f1
        i32.const <GROW>
        table.grow 0
        table.size 0
      )
    )
    "#;
  let initial = 10;
  let grow = 10_000_000;
  let wasm_bytes = wat::parse_str(wat_str.replace("<INITIAL>", &initial.to_string()).replace("<GROW>", &grow.to_string())).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let tab = instance.get_table(&mut store, "tab").unwrap();
  assert_eq!(10, tab.size(&store));
  let fun = instance.get_typed_func::<(), (i32, i32)>(&mut store, "fun").unwrap();
  assert_eq!((10, initial + grow), fun.call(&mut store, ()).unwrap());
  assert_eq!((initial + grow) as u64, tab.size(&store));
}
