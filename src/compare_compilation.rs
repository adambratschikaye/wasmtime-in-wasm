#[cfg(not(target_arch = "wasm32"))]
mod go {
    use std::time::Instant;
    use wasmtime::{Config, Engine, Linker, Module, Store};
    use wasmtime_wasi::{Dir, WasiCtxBuilder};

    pub(super) fn go() {
        let args: Vec<_> = std::env::args().collect();
        let compilation_wasm_path = &args[1];
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

        let wasi = WasiCtxBuilder::new()
            // .inherit_stdio()
            // .inherit_stderr()
            .preopened_dir(Dir::from_std_file(std::fs::File::open(".").unwrap()), ".")
            .unwrap()
            .build();
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_file(&engine, compilation_wasm_path).unwrap();
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let main = instance
            .get_typed_func::<(), ()>(&mut store, "_start")
            .unwrap();

        let start = Instant::now();
        main.call(&mut store, ()).unwrap();
        println!("Compilation within wasmtime took {:?}", start.elapsed());

        let start = Instant::now();
        compile_native();
        println!("Compilation natively took {:?}", start.elapsed());
    }

    fn compile_native() {
        let wat = std::fs::read("input.wat").unwrap();
        // let engine = Engine::default();
        let engine = Engine::new(
            Config::default()
                // .parallel_compilation(false)
                .cranelift_opt_level(wasmtime::OptLevel::None),
        )
        .unwrap();
        let result = engine.precompile_module(&wat).unwrap();
        std::fs::write("serialized_module_native.cwasm", &result).unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    go::go()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
