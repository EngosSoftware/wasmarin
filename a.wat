(module
  (table 2 funcref)
  (elem func $f1)
  (func $f1)
  (func (export "fun") (result i32 i32)
    ref.func $f1
    i32.const 100
    table.grow 0
    table.size 0
  )
)
