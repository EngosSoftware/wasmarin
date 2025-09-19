use wasmarin::{Encoder, Parser};
use wasmtime::{Engine, Instance, Module, Store, Val};

#[test]
fn metering_should_work() {
  let wat_str = r#"
    (module
      (global (mut i32) i32.const 2)
      (type (func (param i32) (result i32)))
      (func (type 0) (param i32) (result i32)
        local.get 0
        global.get 0
        i32.add
      )
      (export "add_one" (func 0))
      (export "offset" (global 0))
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let mut parser = Parser::new();
  let model = parser.parse_wasm_bytes(&wasm_bytes).unwrap();
  let mut encoder = Encoder::new_with_metering();
  let wasm_bytes = encoder.encode(model);
  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);

  // compile module
  let engine = Engine::default();
  let module = Module::from_binary(&engine, &wasm_bytes).unwrap();

  // instantiate
  let mut store = Store::new(&engine, ());
  let instance = Instance::new(&mut store, &module, &[]).unwrap();

  let remaining_points = instance.get_global(&mut store, wasmarin::REMAINING_POINTS_EXPORT_NAME).unwrap();
  remaining_points.set(&mut store, Val::I64(4)).unwrap();

  // get the function handle
  let add_one = instance.get_typed_func::<i32, i32>(&mut store, "add_one").unwrap();
  match add_one.call(&mut store, 2) {
    Ok(_) => {
      println!("ok");
    }
    Err(_) => {
      println!("error");
    }
  }

  println!("remaining points: {}", remaining_points.get(&mut store).i64().unwrap());
}

/*

(module
  (type (;0;) (func))
  (global (;0;) (mut i64) i64.const 0)
  (func (;0;) (type 0)
    global.get 0
    i64.const 4
    i64.sub
    global.set 0
    global.get 0
    i64.const 0
    i64.lt_s
    if ;; label = @1
      unreachable
    end
  )
)

wasm[0]::function[0]:
  push rbp
  mov  rbp, rsp
  mov  rax, qword ptr [rdi + 0x60]
  sub  rax, 0x4
  mov  qword ptr [rdi + 0x60], rax
  test rax, rax
  jl   0x1e <wasm[0]::function[0]+0x1e>
  mov  rsp, rbp
  pop  rbp
  ret
  ud2

*/

/*

(module
  (type (;0;) (func))
  (global (;0;) (mut i64) i64.const 0)
  (global (;1;) (mut i32) i32.const 0)
  (func (;0;) (type 0)
    global.get 0
    i64.const 4
    i64.lt_u
    if ;; label = @1
      i32.const 1
      global.set 1
      unreachable
    end
    global.get 0
    i64.const 4
    i64.sub
    global.set 0
  )
)


wasm[0]::function[0]:
 push	rbp
 mov	rbp, rsp
 mov	rax, qword ptr [rdi + 0x60]
 cmp	rax, 0x4
 jb	0x1f <wasm[0]::function[0]+0x1f>
 sub	rax, 0x4
 mov	qword ptr [rdi + 0x60], rax
 mov	rsp, rbp
 pop	rbp
 ret
 mov	dword ptr [rdi + 0x70], 0x1
 ud2

*/

/// This is an example of metering code used to calculate
/// the cost of copying memory of specified size.
#[test]
fn memory_copy_metering_code() {
  let wat_str = r#"
    (module
      (func (export "fun") (param $length i32) (result i64)
        local.get $length   ;; This simulates the memory length on the top of the stack.
        i64.extend_i32_u    ;; Convert i32 value to i64 value.
        i64.const 31        ;; Push (bulk_memory_operation_unit - 1)
        i64.add             ;; Add (length + (bulk_memory_operation_unit - 1))
        i64.const 32        ;; Push bulk_memory_operation_unit
        i64.div_u           ;; Divide ((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit)
        i64.const 13        ;; Push unit_cost
        i64.mul             ;; Multiply (((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit) * unit_cost)
        i64.const 3         ;; Push accumulated_cost
        i64.add             ;; Add ((((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit) * unit_cost) + accumulated_cost)
      )
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let fun = instance.get_typed_func::<i32, i64>(&mut store, "fun").unwrap();
  assert_eq!(68, fun.call(&mut store, 158).unwrap());
}

/*

wasm[0]::function[0]:
 push	rbp
 mov	rbp, rsp
 mov	esi, edx
 lea	rax, [rsi + 0x1f]
 mov	esi, 0x20
 xor	rdx, rdx
 div	rsi
 imul	rsi, rax, 0xd
 lea	rax, [rsi + 0x3]
 mov	rsp, rbp
 pop	rbp
 ret

*/
