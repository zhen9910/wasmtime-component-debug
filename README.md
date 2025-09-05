# WebAssembly Component Debug Project

This project demonstrates a WebAssembly component implementation using wit-bindgen and Rust, with a Wasmtime-based host application that supports debugging.

## Project Structure

```
debug-wasm/
├── wasm-component/          # WebAssembly component (Rust)
│   ├── src/lib.rs          # Component implementation
│   ├── wit/world.wit       # WIT interface definition
│   └── Cargo.toml          # Component dependencies
├── host-app/               # Host application (Rust)
│   ├── src/main.rs         # Host implementation
│   ├── wit/world.wit       # WIT interface definition (copy)
│   ├── build.rs            # Build script for bindings
│   └── Cargo.toml          # Host dependencies
├── wasm-module/            # Legacy core wasm module (C)
│   ├── foo.c               # Simple C implementation
│   └── Makefile            # Build script
└── .vscode/
    └── launch.json         # VS Code debug configuration
```

## Component Interface

The WebAssembly component exports a simple `foo` function:

```wit
package component:foo;

interface math {
  foo: func(x: s32) -> s32;
}

world foo-world {
  export math;
}
```

The `foo` function takes an `s32` parameter and returns `x + 1`.

## Building

### WebAssembly Component

```bash
rustup target add wasm32-wasip2
cd wasm-component
cargo build --target wasm32-wasip2
```

### Host Application

```bash
cd host-app
cargo build
```

## Running

```bash
cd host-app
./target/debug/host-app
```

Expected output:
```
foo(41) = 42
```

## Debugging

### Wasmtime Debug Features

The host application enables Wasmtime debug features through environment variables:
- `WASMTIME_BACKTRACE_DETAILS=1`: Detailed backtraces
- `WASMTIME_LOG=debug`: Debug logging

### VS Code Debugging

A launch configuration is provided in `.vscode/launch.json` for debugging the host application:

1. Open the project in VS Code
2. Set breakpoints in `host-app/src/main.rs`
3. Press F5 or use "Debug Host App" configuration
4. The debugger will stop at the first line (`stopOnEntry: true`)

### Debug Configuration

```json
{
    "name": "Debug Host App",
    "type": "lldb",
    "request": "launch",
    "program": "${workspaceFolder}/host-app/target/debug/host-app",
    "cwd": "${workspaceFolder}/host-app",
    "env": {
        "WASMTIME_BACKTRACE_DETAILS": "1",
        "WASMTIME_LOG": "debug"
    },
    "stopOnEntry": true
}
```

## Prerequisites

- Rust toolchain with wasm32-wasip2 target: `rustup target add wasm32-wasip2`
- LLDB debugger for VS Code debugging
- VS Code with the C/C++ or CodeLLDB extension

## Legacy Core Module

The `wasm-module/` directory contains the original C-based core WebAssembly module for comparison. It can be built with:

```bash
cd wasm-module
make
```

This creates a traditional WebAssembly module that exports the same `foo` function but uses the core WebAssembly interface instead of the component model.
