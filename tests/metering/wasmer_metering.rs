///
#[test]
fn wasmer_metering_memory_copy() {
  let wat_str = r#"
    (module
      (memory 1)
      (global (export "wasmer_metering_remaining_points") (mut i64) i64.const 0)  ;; Remaining points
      (global (export "wasmer_metering_points_exhausted") (mut i32) i32.const 0)  ;; Points exhausted flag: 0-not exhausted, 1-exhausted
      (global (export "wasmer_metering_bytes_length") (mut i32) i32.const 0)      ;; Cache for length of bulk-memory operation
      (global (export "wasmer_metering_total_cost") (mut i64) i64.const 0)        ;; Cache for total cost of bulk-memory operation
      (func (export "fun")
        i32.const 2          ;; Destination offset in memory.
        i32.const 0          ;; Source offset in memory.
        i32.const 12         ;; Memory length in bytes to be copied.

        ;; Begin of the injected code

        global.set 2         ;; Pop length and save in global variable
        global.get 2         ;; Push length back for calculation
        i64.extend_i32_u     ;; Convert i32 value to i64 value
        i64.const 31         ;; Push <unitSize - 1>
        i64.add              ;; Add (<length> + <unitSize - 1>)
        i64.const 32         ;; Push <unitSize>
        i64.div_u            ;; Divide ((<length> + <unitSize - 1>) / <unitSize>)
        i64.const 13         ;; Push <unitCost>
        i64.mul              ;; Multiply (((<length> + <unitSize - 1>) / <unitSize>) * <unitCost>)
        i64.const 3          ;; Push <accumulatedCost>
        i64.add              ;; Add ((((<length> + <unitSize - 1>) / <unitSize>) * <unitCost>) + <accumulatedCost>) = totalCost
        global.set 3         ;; Pop <totalCost> and save in global variable
        global.get 0         ;; Push <remainingPoints> for calculations
        global.get 3         ;; Push <totalCost> for calculations
        i64.lt_u             ;; <remainingPoints> < <totalCost>
        if
          i32.const 1        ;; Prepare exhausted flag
          global.set 1       ;; Set exhausted global variable
          unreachable        ;; Break execution
        end
        global.get 0         ;; Push <remainingPoints> for calculations
        global.get 3         ;; Push <totalCost> for calculations
        i64.sub              ;; Subtract (<remainingPoints> - <totalCost>)
        global.set 0         ;; Save <remainingPoints> in global variable
        global.get 2         ;; Push <length> back for bulk-memory calculations

        ;; End of injected code

        memory.copy          ;; Perform the copy memory if there are enough points
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
