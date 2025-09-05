use wasmtime::*;
use wasmtime::component::{Component, Linker};
use std::fs;

// Include the generated bindings
wasmtime::component::bindgen!({
    world: "foo-world",
    path: "wit"
});

fn main() -> anyhow::Result<()> {
    // Enable comprehensive wasmtime debug logging
    std::env::set_var("WASMTIME_BACKTRACE_DETAILS", "1");
    std::env::set_var("WASMTIME_LOG", "debug");
    std::env::set_var("RUST_LOG", "debug");
    
    // Initialize logging
    env_logger::init();
    
    println!("🔧 Starting Wasmtime host app with debug enabled...");

    // Load the component
    let component_bytes = fs::read("../wasm-component/target/wasm32-unknown-unknown/debug/wasm_component.wasm")?;
    println!("📦 Loaded component: {} bytes", component_bytes.len());
    
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.debug_info(true);           // Enable debug info
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.cranelift_debug_verifier(true);  // Enable Cranelift verification
    
    let engine = Engine::new(&config)?;
    println!("🚀 Engine created with debug configuration");
    
    let component = Component::new(&engine, &component_bytes)?;
    println!("🧩 Component instantiated successfully");
    
    let mut store = Store::new(&engine, ());
    
    let linker = Linker::new(&engine);
    println!("🔗 Linker created");
    
    let bindings = FooWorld::instantiate(&mut store, &component, &linker)?;
    println!("⚡ Component bindings established");
    
    let input = 41;
    println!("📞 Calling foo({})...", input);
    
    let result = bindings.component_foo_math().call_foo(&mut store, input)?;
    
    println!("✅ foo({}) = {}", input, result);
    
    Ok(())
}