# Metering for WebAssembly bulk-memory proposal

## Instructions introduced in bulk-memory proposal

```text
MemoryInit { data_index: u32, mem: u32 }
MemoryFill { mem: u32 }
MemoryCopy { dst_mem: u32, src_mem: u32 }
DataDrop { data_index: u32 }
TableInit { elem_index: u32, table: u32 }
TableCopy { dst_table: u32, src_table: u32 }
ElemDrop { elem_index: u32 }
```

### MemoryInit

```webassembly
(module
  (memory 1)
  (data "Hello WebAssembly!")
  (func (export "fun")
    i32.const 2    ;; Destination offset in memory.
    i32.const 6    ;; Source offset in passive data segment.
    i32.const 12   ;; Number of bytes to be copied
    memory.init 0  ;; Use the first data segment.
  )
  (export "mem" (memory 0))
)
```

### MemoryFill

```webassembly
(module
  (memory 1)
  (func (export "fun")
    i32.const 22  ;; Start offset in memory.
    i32.const 64  ;; Fill with letter '@'.
    i32.const 11  ;; Number of bytes to be filled.
    memory.fill
  )
  (export "mem" (memory 0))
)
```

### MemoryCopy

```webassembly
(module
  (memory 1)
  (func (export "fun")
    i32.const 2   ;; Destination offset in memory.
    i32.const 0   ;; Source offset in memory.
    i32.const 12  ;; Number of bytes to be copied.
    memory.copy
  )
  (export "mem" (memory 0))
)
```

> [!NOTE]  
> During execution, when the `memory.copy` instruction is encountered,
> the number of bytes to be copied is placed at the top of the stack. 

> [!WARNING]  
> The check must be performed before executing `memory.copy` instruction!
> Otherwise, someone could copy memory not having enough oil. 

### TableInit

```webassembly
(module)
```

### TableCopy

```webassembly
(module)
```

### DataDrop

```webassembly
(module)
```

### ElemDrop

```webassembly
(module)
```

## Total cost calculation for memory operations

Inputs:

- `memory_unit_size` - the number of bytes in one memory unit,
- `memory_unit_cost` - cost of the operation per one memory unit,
- `memory_length` - the number of bytes of memory in operation,
- `accumulated_cost` - accumulated cost of operations until `memory.copy` instruction (including).

Calculation:

```math
total\_cost = (\frac{memory\_length + memory\_unit\_size - 1}{memory\_unit\_size}) \times memory\_unit\_cost + accumulated\_cost
```
