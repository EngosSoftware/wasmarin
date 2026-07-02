#[test]
fn wasmer_metering_memory_copy() {
  let wat_str = r#"
    (module
      (memory 1)
      (global (export "wasmer_metering_remaining_points") (mut i64) i64.const 0)  ;; Remaining points
      (global (export "wasmer_metering_points_exhausted") (mut i32) i32.const 0)  ;; Points exhausted: 0-not exhausted, 1-exhausted
      (global (export "wasmer_metering_length") (mut i32) i32.const 0)            ;; Length of bulk-memory operation
      (global (export "wasmer_metering_cost") (mut i64) i64.const 0)              ;; Total cost until bulk-memory operator
      (func (export "fun")
        i32.const 2          ;; Destination offset in memory.
        i32.const 0          ;; Source offset in memory.
        i32.const 14         ;; Memory length in bytes to be copied.

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
  let remaining_points = instance.get_global(&mut store, "wasmer_metering_remaining_points").unwrap();
  let points_exhausted = instance.get_global(&mut store, "wasmer_metering_points_exhausted").unwrap();
  let fun = instance.get_typed_func::<(), ()>(&mut store, "fun").unwrap();

  let text = b"Hello world!$$##__-";

  // Initialize the memory.
  memory.write(&mut store, 0, text).unwrap();

  // Set initial remaining points to 35.
  remaining_points.set(&mut store, wasmtime::Val::I64(35)).unwrap();
  // Points are not exhausted.
  assert_eq!(0, points_exhausted.get(&mut store).i32().unwrap());

  // Burn some points by copying memory.
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHello world!$$__-", &memory.data(&mut store)[0..text.len()]);
  // Burned 16 points, so 19 remain.
  assert_eq!(19, remaining_points.get(&mut store).i64().unwrap());
  // Points are not exhausted.
  assert_eq!(0, points_exhausted.get(&mut store).i32().unwrap());

  // Burn some more points by copying memory.
  fun.call(&mut store, ()).unwrap();
  assert_eq!(b"HeHeHello world!__-", &memory.data(&mut store)[0..text.len()]);
  // Burned another 16 points, so 3 points remain
  assert_eq!(3, remaining_points.get(&mut store).i64().unwrap());
  // Points are not exhausted.
  assert_eq!(0, points_exhausted.get(&mut store).i32().unwrap());

  // There are not enough points to copy memory again.
  fun.call(&mut store, ()).unwrap_err();
  // No changes in memory, because the function execution was stopped before reaching `memory.copy`.
  assert_eq!(b"HeHeHello world!__-", &memory.data(&mut store)[0..text.len()]);
  // There should be a small amount of remaining points.
  assert_eq!(3, remaining_points.get(&mut store).i64().unwrap());
  // Points are exhausted.
  assert_eq!(1, points_exhausted.get(&mut store).i32().unwrap());
}

#[test]
fn _0001() {
  let wat_str = r#"
    (module
      (global (export "wasmer_metering_remaining_points") (mut i64) i64.const 0)  ;; Remaining gas points
      (global (export "wasmer_metering_points_exhausted") (mut i32) i32.const 0)  ;; Points exhausted: 0-not exhausted, 1-exhausted
      (global (export "wasmer_metering_data_length") (mut i32) i32.const 0)       ;; Data length of bulk-memory operation
      (global (export "wasmer_metering_dynamic_cost") (mut i64) i64.const 0)      ;; Dynamic cost of bulk-memory operation
      (func (export "fun") (result i32)
        i32.const 117        ;; Data length is on top of the stack                      [117(i32)]

        ;; Begin of the injected code

        global.set 2         ;; Pop $length from stack and save in $data_length         []
        global.get 2         ;; Push $length to stack                                   [117(i32)]
        i64.extend_i32_u     ;; Convert i32 $length to i64 value                        [117]
        i64.const 31         ;; Push $decrUnitSize                                      [31, 117]
        i64.add              ;; Add $length + $decUnitSize                              [148]
        i64.const 32         ;; Push $unitSize                                          [32, 148]
        i64.div_u            ;; Div ($length + $decUnitSize) / $unitSize                [4]
        i64.const 13         ;; Push $unitCost                                          [13, 4]
        i64.mul              ;; Mul (($length + $decUnitSize) / $unitSize) * $unitCost  [52]
        i64.const 3          ;; Push $accumulatedCost                                   [3, 52]
        i64.add              ;; $dynamicCost                                            [55]
        global.set 3         ;; Pop $dynamicCost and save in $dynamic_cost              []
        global.get 0         ;; Push $remainingPoints to stack                          [100]
        global.get 3         ;; Push $dynamicCost                                       [55, 100]
        i64.lt_u             ;; $remainingPoints < $dynamicCost                         [0]
        if
          i32.const 1        ;; Prepare exhausted flag
          global.set 1       ;; Set exhausted global variable
          unreachable        ;; Break execution
        end
        global.get 0         ;; Push $remainingPoints> for calculations
        global.get 3         ;; Push $dynamicCost for calculations
        i64.sub              ;; Sub $remainingPoints - $dynamicCost
        global.set 0         ;; Save $remainingPoints in global variable
        global.get 2         ;; Push $length back to stack

        ;; End of injected code
      )
    )
  "#;

  let wasm_bytes = wat::parse_str(wat_str).unwrap();
  let engine = wasmtime::Engine::default();
  let module = wasmtime::Module::from_binary(&engine, &wasm_bytes).unwrap();
  let mut store = wasmtime::Store::new(&engine, ());
  let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
  let remaining_points = instance.get_global(&mut store, "wasmer_metering_remaining_points").unwrap();
  // let points_exhausted = instance.get_global(&mut store, "wasmer_metering_points_exhausted").unwrap();
  let fun = instance.get_typed_func::<(), i32>(&mut store, "fun").unwrap();
  remaining_points.set(&mut store, wasmtime::Val::I64(100)).unwrap();
  assert_eq!(117, fun.call(&mut store, ()).unwrap());
  assert_eq!(45, remaining_points.get(&mut store).i64().unwrap());
}
