#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (memory (export "mem") 1)
      (func (export "fun") (result i32)
        i32.const 2    ;; Number of pages to grow the memory;  push: 2  stack: 2
        memory.grow 1  ;; Grow non-existing memory;            validation error
      )
    )
    "#;
  assert_eq!("unknown memory 1 (at offset 0x2e)", wasmarin::Parser::new().parse_wat_str(wat_str).unwrap_err().to_string());
}
