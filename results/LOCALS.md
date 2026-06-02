# Locals

## Test module
   
```wat
(module
  (memory 0 1)
  (func (export "fun") (result i32)
    (local i32)
    i32.const 10
  )
  (export "mem" (memory 0))
)
```

The number of locals is changed before each test.

## Results
          
- Wasmer 5.0.6
- Fedora Linux 43
- Intel Core i7-6700 8 core 3.40GHz
- 32GB DDR4 2133 MT/s
- Rust benchmarks using Criterion

| Locals |      Time |
|-------:|----------:|
|      1 | 68.445 ns |
|      2 | 68.003 ns |
|      5 | 72.614 ns |
|     10 | 72.058 ns |
|     15 | 73.384 ns |
|     20 | 71.936 ns |
|     30 | 77.208 ns |
|     40 | 78.275 ns |
|     50 | 79.369 ns |
|     60 | 75.077 ns |
|     70 | 76.899 ns |
|     80 | 77.756 ns |
|     90 | 78.504 ns |
|    100 | 78.904 ns |
|    200 | 90.887 ns |
|    300 | 98.764 ns |
|    400 | 109.16 ns |
|    500 | 125.12 ns |
|    600 | 135.00 ns |
|    700 | 153.44 ns |
|    800 | 153.70 ns |
|    900 | 161.81 ns |
|   1000 | 184.73 ns |
|   2000 | 288.11 ns |
|   3000 | 387.33 ns |
|   4000 | 562.77 ns |
|   5000 | 776.52 ns |
|   6000 | 930.77 ns |
|   7000 | 1.0605 µs |
|   8000 | 1.2113 µs |
|   9000 | 1.3350 µs |
|  10000 | 1.4692 µs |
|  20000 | 2.9549 µs |
|  30000 | 4.5880 µs |
|  40000 | 6.2185 µs |
|  50000 | 7.9366 µs |

## Discussion
