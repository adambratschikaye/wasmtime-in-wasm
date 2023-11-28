use std::str::FromStr;

use target_lexicon::Triple;
use wasmparser::WasmFeatures;
use wasmtime_compile::{build_artifacts, build_compiler, CompilerConfig, ModuleVersionStrategy};
use wasmtime_environ::Tunables;

fn main() {
    let wat = r#"
        (module
            (import "host" "host_func" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let wasm = wat::parse_str(wat).unwrap();

    let mut initial_compiler_config = CompilerConfig::default();
    initial_compiler_config.target = Some(Triple::from_str("x86_64-unknown-linux-gnu").unwrap());
    let mut tunables = Tunables::default();
    tunables.static_memory_bound = 0x1_0000;
    tunables.static_memory_offset_guard_size = 2147483648;
    let features = WasmFeatures::default();
    let module_version = ModuleVersionStrategy::WasmtimeVersion;
    let (_, compiler) = build_compiler(initial_compiler_config, &tunables, features, None).unwrap();

    let (result, _): (Vec<u8>, _) = build_artifacts(
        compiler.as_ref(),
        &tunables,
        features,
        &module_version,
        None,
        &wasm,
    )
    .unwrap();

    std::fs::write("serialized_module", &result).unwrap();
    println!("compiled module starts with {:x?}", &result[0..20]);
}
