use std::str::FromStr;

use target_lexicon::Triple;
use wasmparser::WasmFeatures;
use wasmtime_compile::{build_artifacts, build_compiler, CompilerConfig, ModuleVersionStrategy};
use wasmtime_environ::Tunables;

fn main() {
    let wat = std::fs::read("input.wat").unwrap();
    let wasm = wat::parse_bytes(&wat).unwrap();

    let mut initial_compiler_config = CompilerConfig::default();
    initial_compiler_config.target = Some(Triple::from_str("x86_64-unknown-linux-gnu").unwrap());
    initial_compiler_config
        .settings
        .insert("opt_level".to_string(), "none".to_string());
    let mut tunables = Tunables::default();
    tunables.static_memory_bound = 0x1_0000;
    tunables.static_memory_offset_guard_size = 2147483648;
    tunables.parse_wasm_debuginfo = false;
    let mut features = WasmFeatures::default();
    features.tail_call = false;
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

    std::fs::write("serialized_module.cwasm", &result).unwrap();
    println!("compiled module starts with {:x?}", &result[0..20]);
}
