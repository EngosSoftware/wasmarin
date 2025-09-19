# Metering for WebAssembly bulk-memory proposal

## Instructions introduced in bulk-memory proposal

```text
MemoryInit { data_index: u32, mem: u32 }
MemoryFill { mem: u32 }
MemoryCopy { dst_mem: u32, src_mem: u32 }
TableInit { elem_index: u32, table: u32 }
TableCopy { dst_table: u32, src_table: u32 }
DataDrop { data_index: u32 }
ElemDrop { elem_index: u32 }
```

### MemoryInit

### MemoryFill

```webassembly
(module
  (memory 1)
  (func (export "fun_memory_fill")
    i32.const 22  ;; Start offset in memory.
    i32.const 64  ;; Fill with letter '@'.
    i32.const 11  ;; Length in bytes to be filled.
    memory.fill
  )
  (export "mem" (memory 0))
)
```

### MemoryCopy

```webassembly
(module
  (memory 1)
  (func (export "fun_memory_copy")
    i32.const 2   ;; Destination offset in memory.
    i32.const 0   ;; Source offset in memory.
    i32.const 12  ;; Length in bytes to be copied.
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

#### Metering algorithm for `memory.copy`

Inputs:

- `word` - the size of memory block that is a metering unit,
- `unit_cost` - cost of the one unit (`word`),
- `length` - the number of memory bytes to be copied,
- `accumulated_cost` - statically analyzed (calculated) cost of operations before `memory.copy` instruction.

```math
\lceil \frac{a}{b} \rceil = \frac{a + b - 1}{b} 
```

```math
total\_cost = (\frac{length + word - 1}{word}) \times unit\_cost + accumulated\_cost
```

### TableInit

### TableCopy

### DataDrop

### ElemDrop

## WebAssembly operators semantics used in metering

### i64.sub

```webassembly
(module
  (func
    i64.const 10   ;; push 10,        stack: 10
    i64.const 3    ;; push 3,         stack: 3, 10 
    i64.sub        ;; compute 10-3,   stack: 7 
  )
)
```
