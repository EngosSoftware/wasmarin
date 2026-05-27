# Metering for WebAssembly bulk-memory proposal

## Instruction summary

- `memory.init`
- `memory.grow`
- `memory.fill`
- `memory.copy`
- `table.init`
- `table.grow`
- `table.fill`
- `table.copy`
- `data.drop`
- `elem.drop`
- `memory.size`
- `table.size`
- `table.get`
- `table.set`

## Instruction semantics

### memory.init

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

> [!NOTE]  
> Can be handled with the same algorithm as [memory.copy](#memorycopy).

### memory.grow

```webassembly
(module
  (memory 1)
  (func (export "fun") (result i32)
    i32.const 2   ;; Number of pages to grow the memory.
    memory.grow
  )
  (export "mem" (memory 0))
)
```

> [!NOTE]  
> On the stack is the number of pages, not bytes, so the `memory.copy` algorithm has to be adjusted.

### memory.fill

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

> [!NOTE]  
> Can be handled with the same algorithm as `memory.copy`.

### memory.copy

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
> Otherwise, someone could copy memory not having enough gas. 

### table.init

```webassembly
(module
  (table 10 funcref)
  (elem func $f1 $f2 $f3 $f4 $f5)
  (func $f1)
  (func $f2)
  (func $f3)
  (func $f4)
  (func $f5)
  (func (export "fun")
    i32.const 5     ;; Destination offset in the table.
    i32.const 0     ;; Source offset in the elements.
    i32.const 3     ;; Number of elements to be used as initialization.
    table.init 0 0  ;; Initialize table.
  )
)
```

### table.grow

```webassembly
(module
  (table 2 funcref)
  (func (export "fun") (result i32)
    ref.null func  ;; New table elements with be null function references.
    i32.const 100  ;; Number of new elements in the table.
    table.grow 0   ;; Grow the table.
    drop           ;; Drop the old table size.
    table.size 0   ;; Return the new table size.
  )
)
```

### table.fill

```webassembly
(module
  (table 21 funcref)
  (elem declare func $f111)
  (func $f111 (result i32) i32.const 111)
  (func (export "fun")
    i32.const 1      ;; Start offset in table.
    ref.func $f111   ;; Reference value to fill the table.
    i32.const 20     ;; Number of elements to be filled.
    table.fill 0     ;; Fill the table.
  )
)
```

### table.copy

```webassembly
(module
  (table $dst 7 funcref)
  (table $src 5 funcref)
  (elem (table $src) (i32.const 1) func $f1 $f2 $f3)
  (func $f1)
  (func $f2)
  (func $f3)
  (func (export "fun")
    i32.const 2           ;; Destination ofset in table $dst.
    i32.const 1           ;; Source ofset in table $src.
    i32.const 3           ;; Number of elements to be copied.
    table.copy $dst $src  ;; Copy elements from table $src to table $dst.
  )
)
```

### data.drop

```webassembly
(module
  (data "Hello WebAssembly!")
  (data "Hello world!")
  (func (export "fun")
    data.drop 0    ;; Drop passive data segment 0.
    data.drop 1    ;; Drop passive data segment 1.
  )
)
```

### elem.drop

```webassembly
(module
  (elem func $f1 $f2 $f3 $f4 $f5 $f6)
  (elem func $f2 $f3 $f4)
  (func $f1)
  (func $f2)
  (func $f3)
  (func $f4)
  (func $f5)
  (func $f6)
  (func (export "fun")
    elem.drop 0  ;; Drop passive element segment 0
    elem.drop 1  ;; Drop passive element segment 1
  )
)
```

## Cost calculation

Inputs:

- `length` - number of bytes used in block operation,
- `unitSize` - arbitrary number of bytes in one unit,
- `unitCost` - arbitrary cost of the operation per one unit,
- `accumulated` - accumulated cost of operations until block instruction was encountered (including the block operation).

Calculation:

\\[ total = (\frac{length + unitSize - 1}{unitSize}) \times unitCost + accumulated \\]

## Implementation

| Operation     | Where is the <br/> `length` | Remarks              |
|---------------|-----------------------------|----------------------|
| `memory.init` | Top of the stack.           |                      |
| `memory.grow` | Top of the stack.           |                      |
| `memory.fill` | Top of the stack.           |                      |
| `memory.copy` | Top of the stack.           |                      |
| `table.init`  | Top of the stack.           |                      |
| `table.grow`  | Top of the stack.           |                      |
| `table.fill`  | Top of the stack.           |                      |
| `table.copy`  | Top of the stack.           |                      |
| `data.drop`   |                             | Benchmark to decide. |
| `elem.drop`   |                             | Benchmark to decide. |
| `memory.size` |                             | Benchmark to decide. |
| `table.size`  |                             | Benchmark to decide. |
| `table.get`   |                             | Benchmark to decide. |
| `table.set`   |                             | Benchmark to decide. |
