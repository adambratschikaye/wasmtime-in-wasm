use wasmtime::*;

fn main() {
    let mut config = Config::new();
    config
        .target("x86_64-unknown-linux-gnu")
        .unwrap()
        .static_memory_maximum_size(0x1_0000 * 64 * 1024)
        .static_memory_guard_size(2147483648);
    // Modules can be compiled through either the text or binary format
    let engine = Engine::new(&config).unwrap();
    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let result = engine.precompile_module(wat.as_bytes()).unwrap();

    std::fs::write("serialized_module", &result).unwrap();
    println!("compiled module starts with {:x?}", &result[0..20]);
}
