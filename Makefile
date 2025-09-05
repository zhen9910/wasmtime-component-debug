.PHONY: all build-component build-host clean run debug

all: build-component build-host

build-component:
	rustup target add wasm32-wasip2
	cd wasm-component && cargo build --target wasm32-wasip2

build-host:
	cd host-app && cargo build

run: build-component build-host
	cd host-app && ./target/debug/host-app

clean:
	cd wasm-component && cargo clean
	cd host-app && cargo clean
	cd wasm-module && make clean

debug: build-component build-host
	@echo "Open VS Code and use the 'Debug Host App' launch configuration"
	@echo "Or run manually with debug logging:"
	@echo "cd host-app && WASMTIME_BACKTRACE_DETAILS=1 WASMTIME_LOG=debug ./target/debug/host-app"

# Legacy core module
build-legacy:
	cd wasm-module && make

test: run
	@echo "WebAssembly component test completed successfully!"
