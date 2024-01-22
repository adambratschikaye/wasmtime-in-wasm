use wasmtime::{Config, Engine};

fn main() {
    let wat = std::fs::read("input.wat").unwrap();
    let wasm = wat::parse_bytes(&wat).unwrap();

    let mut config = Config::default();
    config.target("x86_64-unknown-linux-gnu").unwrap();
    config.static_memory_maximum_size(0x100_000_000);
    config.static_memory_guard_size(2147483648);
    config.debug_info(false);
    config.wasm_tail_call(false);
    let engine = Engine::new(&config).unwrap();
    let result = engine.precompile_module(&wasm).unwrap();

    std::fs::write("serialized_module.cwasm", &result).unwrap();
    println!("compiled module starts with {:x?}", &result[0..20]);
}
