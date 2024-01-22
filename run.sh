set -euxo pipefail

# Compile a wasm module which will read the file `input.wat` and produce `serialized_module.cwasm`
cargo build --release --target wasm32-wasi

# Run the wasm module in wasmtime
../../wasmtime/target/release/wasmtime run --dir . target/wasm32-wasi/release/wasmtime-in-wasm.wasm

# Test compilation worked by executing the module in wasmtime (and providing the required host function).
cargo run --bin run
