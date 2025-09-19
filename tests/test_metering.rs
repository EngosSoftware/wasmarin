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
  let mut parser = wasmarin::Parser::new();
  let model = parser.parse_wasm_bytes(&wasm_bytes).unwrap();
  let mut encoder = wasmarin::Encoder::new_with_metering();
  let wasm_bytes = encoder.encode(model);
  let wat = wasmprinter::print_bytes(&wasm_bytes).unwrap();
  println!("{}", wat);

  // compile module
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();

  // instantiate
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();

  let remaining_points = instance.get_global(&mut store, wasmarin::REMAINING_POINTS_EXPORT_NAME).unwrap();
  remaining_points.set(&mut store, wasmtime::Val::I64(4)).unwrap();

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

#[test]
fn memory_copy_metering_code_full() {
  let wat_str = r#"
    (module
      (memory 1)
      (global (export "oil") (mut i64) i64.const 0)  ;; Single additional global variable for tracking oil usage
      (func (export "fun")
        (local i32 i64)    ;; Two additional local variables

        i32.const 2        ;; Destination offset in memory.
        i32.const 0        ;; Source offset in memory.
        i32.const 12       ;; Memory length in bytes to be copied.

        ;; Begin of the additional code.

        local.tee 0        ;; Save the length in a local variable
        local.get 0        ;; Push the length from local variable
        i64.extend_i32_u   ;; Convert i32 value to i64 value
        i64.const 31       ;; Push (bulk_memory_operation_unit - 1)
        i64.add            ;; Add (length + (bulk_memory_operation_unit - 1))
        i64.const 32       ;; Push bulk_memory_operation_unit
        i64.div_u          ;; Divide ((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit)
        i64.const 13       ;; Push unit_cost
        i64.mul            ;; Multiply (((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit) * unit_cost)
        i64.const 3        ;; Push accumulated_cost
        i64.add            ;; Add ((((length + (bulk_memory_operation_unit - 1)) / bulk_memory_operation_unit) * unit_cost) + accumulated_cost)
        local.set 1        ;; Pop the calculated total cost from stack and save in a local variable
        global.get 0       ;; Get the current oil
        local.get 1        ;; Push the total cost
        i64.sub            ;; Subtract the total cost from oil
        global.set 0       ;; Save remaining oil
        global.get 0       ;; Push the current oil
        i64.const 0        ;; Push 0
        i64.lt_s           ;; Is oil < 0
        if ;; label = @1
          unreachable      ;; Break execution
        end

        ;; End of the additional code.

        memory.copy        ;; Perform the copy memory if enough oil.
      )
      (export "mem" (memory 0))
    )
  "#;
  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let memory = instance.get_memory(&mut store, "mem").unwrap();
  let oil = instance.get_global(&mut store, "oil").unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();

  // Initialize the memory.
  memory.write(&mut store, 0, b"Hello world!_______-").unwrap();
  // Set initial oil to 35 barrels.
  oil.set(&mut store, wasmtime::Val::I64(35)).unwrap();

  // Burn some oil by copying memory.
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHello world!_____-", &memory.data(&mut store)[0..20]);
  assert_eq!(19, oil.get(&mut store).i64().unwrap());

  // Burn some more oil by copying memory.
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHeHello worl_____-", &memory.data(&mut store)[0..20]);
  assert_eq!(3, oil.get(&mut store).i64().unwrap());

  // There is not enough oil to copy memory again.
  fun.call(&mut store, ()).unwrap_err();
  // No changes in memory, because the function was stopped before calling `memory.copy`.
  assert_eq!(b"HeHeHello worl_____-", &memory.data(&mut store)[0..20]);
  // Now much additional oil do we need next time? 13 barrels!
  assert_eq!(-13, oil.get(&mut store).i64().unwrap());
}
