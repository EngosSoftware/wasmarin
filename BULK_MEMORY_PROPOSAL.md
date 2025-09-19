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

### TableInit

### TableCopy

### DataDrop

### ElemDrop
