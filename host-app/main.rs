use wasmtime::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Enable wasmtime debug logging
    std::env::set_var("WASMTIME_BACKTRACE_DETAILS", "1");
    std::env::set_var("WASMTIME_LOG", "debug");

    // Load the wasm module
    let wasm_bytes = fs::read("../wasm-module/foo.wasm")?;
    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // Get the exported function 'foo'
    let foo = instance.get_typed_func::<i32, i32, _>(&mut store, "foo")?;
    let input = 41;
    let result = foo.call(&mut store, input)?;
    println!("foo({}) = {}", input, result);
    Ok(())
}
