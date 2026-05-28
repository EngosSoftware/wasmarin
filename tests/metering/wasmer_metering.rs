///
#[test]
fn wasmer_metering_memory_copy() {
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
  // Burned 16 barrels.
  assert_eq!(19, oil.get(&mut store).i64().unwrap());

  // Burn some more oil by copying memory.
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHeHello worl_____-", &memory.data(&mut store)[0..20]);
  // Burned 16 barrels again.
  assert_eq!(3, oil.get(&mut store).i64().unwrap());

  // There is not enough oil to copy memory again.
  fun.call(&mut store, ()).unwrap_err();
  // No changes in memory, because the function was stopped before calling `memory.copy`.
  assert_eq!(b"HeHeHello worl_____-", &memory.data(&mut store)[0..20]);
  // Now much additional oil do we need next time? 13 barrels!
  assert_eq!(-13, oil.get(&mut store).i64().unwrap());
}
