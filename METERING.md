# Metering for WebAssembly smart contracts

## Motivation

- **Prevent denial of service (DoS) attacks**
  
  Without metering, a malicious contract could loop infinitely or consume excessive computation,
  locking up the blockchain’s execution environment. Metering ensures each instruction has a cost,
  so contracts cannot run indefinitely for free.

- **Deterministic and fair execution**

  Blockchains need all nodes to deterministically reproduce the same state transitions.
  Metering ensures each operation has a well-defined `price` (gas cost), making resource consumption
  predictable and uniform across all nodes.

- **Economic incentives & resource accounting**

  Just like Ethereum’s gas model, metering ensures that users pay for the computation/storage they consume.
  This prevents `free riders` and encourages efficient smart contract design.

- **Security and isolation**

  Wasm itself provides memory safety and sandboxing, but not resource limits.
  Metering provides a second layer of defense, making sure contracts don’t hog resources,
  even if they’re memory-safe.

- **Predictable block production times**

  Validators and miners need to know that block execution finishes within a bounded time.
  With metering, block producers can reject contracts that exceed the resource limits.

## Observables

From the user’s perspective, execution of a smart contract consumes resources such as CPU time and memory.
When a contract entry point is invoked, it is supplied with a fixed amount of gas, representing the maximum
resources that may be consumed during execution. As execution proceeds, each instruction consumes gas according
to its cost model (e.g., CPU cycles, memory (de)allocations). If sufficient gas remains, execution completes
successfully, and the total gas consumed is deducted from the initial value. If execution attempts to consume
more gas than was provided, it is aborted immediately, and any state changes are rolled back,
except for the gas that has already been consumed.

## Metering approaches for WebAssembly code

### Bytecode instrumentation (static metering)

Before deploying a contract, the blockchain rewrites the Wasm bytecode.
For each Wasm instruction (or basic block), it injects extra instructions that decrement a gas counter.
If the counter hits zero, execution traps (aborts). This approach:

- is **deterministic**, as all nodes see the same modified Wasm 👍,
- is **flexible**, as different operations can have different costs 👍,
- increases code size and runtime overhead 👎.

### Host function wrapping (dynamic metering)

Instead of rewriting the Wasm code, the blockchain runtime wraps host functions (syscalls, storage ops, etc.)
with gas charging. Only heavyweight operations (storage, cryptography, I/O) are metered. This approach:

- is **efficient**, less instrumentation overhead 👍,
- but doesn’t cover _pure computation_ (like infinite loops with just Wasm instructions) 👎.

### Interpreter or VM-level metering

If the blockchain uses its own Wasm interpreter (instead of a JIT or AoT), it can directly account
for gas at runtime per instruction. In this approach:

- no code rewriting is needed 👍,
- but it's harder to optimize (slower compared to native JIT or AoT) 👎.

### Hybrid approaches

Many systems combine strategies:

- static instrumentation for Wasm arithmetic/logic,
- dynamic host-function charging for storage and external calls,
- VM-level safety as a fallback.
