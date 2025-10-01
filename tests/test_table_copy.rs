///
/// Demonstrates the use of the `table.copy` instruction in WebAssembly.
///
/// This example defines two tables. The source table is of size 5 and has initially
/// 3 non-zero elements filled in. The destination table is of size 7 and is initially empty.
///
/// The exported function `fun` copies 3 elements from source table starting from offset 1
/// to destination table to offset 2 using `table.copy` instruction.
///
/// # NOTES:
///
/// - Before executing the `table.copy` instruction, the number of elements to be copied
///   is placed on the top of the stack.
///
/// - The `table.copy` instruction should be benchmarked based on the number of elements copied.
///
#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (table $dst (export "tab_dst") 7 funcref)
      (table $src (export "tab_src") 5 funcref)
      (elem (table $src) (i32.const 1) func $f1 $f2 $f3)
      (func $f1)
      (func $f2)
      (func $f3)
      (func (export "fun")
        i32.const 2           ;; Destination ofset in table $dst;              push: 2  stack: 2
        i32.const 1           ;; Source ofset in table $src;                   push: 1  stack: 1 2
        i32.const 3           ;; Number of elements to be copied;              push: 3  stack: 3 1 2
        table.copy $dst $src  ;; Copy elements from table $src to table $dst;           stack: (empty)
      )
    )
    "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

  let tab_src = instance.get_table(&mut store, "tab_src").unwrap();
  assert_eq!(5, tab_src.size(&mut store));
  assert!(tab_src.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab_src.get(&mut store, 1).unwrap().as_func().unwrap().is_some());
  assert!(tab_src.get(&mut store, 2).unwrap().as_func().unwrap().is_some());
  assert!(tab_src.get(&mut store, 3).unwrap().as_func().unwrap().is_some());
  assert!(tab_src.get(&mut store, 4).unwrap().as_func().unwrap().is_none());

  let tab_dst = instance.get_table(&mut store, "tab_dst").unwrap();
  assert_eq!(7, tab_dst.size(&mut store));
  assert!(tab_dst.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 1).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 2).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 3).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 4).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 5).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 6).unwrap().as_func().unwrap().is_none());

  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();
  fun.call(&mut store, ()).unwrap();

  assert!(tab_dst.get(&mut store, 0).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 1).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 2).unwrap().as_func().unwrap().is_some());
  assert!(tab_dst.get(&mut store, 3).unwrap().as_func().unwrap().is_some());
  assert!(tab_dst.get(&mut store, 4).unwrap().as_func().unwrap().is_some());
  assert!(tab_dst.get(&mut store, 5).unwrap().as_func().unwrap().is_none());
  assert!(tab_dst.get(&mut store, 6).unwrap().as_func().unwrap().is_none());
}
