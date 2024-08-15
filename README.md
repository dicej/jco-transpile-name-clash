## Test case for Jco resource type name clash

### Prerequisites

- Rust, with `wasm32-wasip1` target installed
- NodeJS
- Curl
- `wasm-tools`

### Instructions

```
cargo build --target wasm32-wasip1
curl -OL https://github.com/bytecodealliance/wasmtime/releases/download/v23.0.2/wasi_snapshot_preview1.reactor.wasm
wasm-tools component new --adapt wasi_snapshot_preview1.reactor.wasm target/wasm32-wasip1/debug/foo.wasm -o foo.wasm
npm install @bytecodealliance/jco
node host.mjs
```

### Expected result

Script completes successfully with no output

### Actual result

```
file:///home/dicej/Downloads/foo/foo.mjs:114
  if (!(ret instanceof Pollable)) {
                       ^

ReferenceError: Pollable is not defined
    at trampoline2 (file:///home/dicej/Downloads/foo/foo.mjs:114:24)
    at foo.wasm._ZN3foo4wasi11clocks0_2_015monotonic_clock18subscribe_duration17hf156969e87e5c5c2E (wasm://wasm/foo.wasm-006a0b22:wasm-function[56]:0x2483)
    at foo.wasm._ZN42_$LT$foo..MyTest$u20$as$u20$foo..Guest$GT$4test17hf70b3a2356b0c5e9E (wasm://wasm/foo.wasm-006a0b22:wasm-function[11]:0x3ae)
    at foo.wasm._ZN3foo17_export_test_cabi17h40936b815bea4d23E (wasm://wasm/foo.wasm-006a0b22:wasm-function[29]:0xc50)
    at foo.wasm.test (wasm://wasm/foo.wasm-006a0b22:wasm-function[12]:0x4d6)
    at Module.test (file:///home/dicej/Downloads/foo/foo.mjs:1797:12)
    at file:///home/dicej/Downloads/foo/host.mjs:14:10

Node.js v18.19.1
```
