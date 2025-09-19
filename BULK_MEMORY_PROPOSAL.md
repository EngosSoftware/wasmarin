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
  (func
     memory.copy
  ) 
)
```

### TableInit

### TableCopy

### DataDrop

### ElemDrop
