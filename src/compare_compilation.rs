#[cfg(not(target_arch = "wasm32"))]
mod go {
    use std::time::Instant;
    use wasmtime::{Engine, Linker, Module, Store};
    use wasmtime_wasi::{Dir, WasiCtxBuilder};

    pub(super) fn go() {
        let args: Vec<_> = std::env::args().collect();
        let compilation_wasm_path = &args[1];
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

        println!(
            "in host current dir has: {:?}",
            std::fs::read_dir(".").unwrap().collect::<Vec<_>>()
        );
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_stderr()
            .preopened_dir(Dir::from_std_file(std::fs::File::open(".").unwrap()), ".")
            .unwrap()
            .build();
        for i in 0..10 {
            println!("wasi table entry {i}: {}", wasi.table().contains_key(i))
        }
        let mut store = Store::new(&engine, wasi);
        let module = Module::from_file(&engine, compilation_wasm_path).unwrap();
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let main = instance
            .get_typed_func::<(), i32>(&mut store, "__main_void")
            .unwrap();

        let start = Instant::now();
        let result = main.call(&mut store, ()).unwrap();
        assert_eq!(result, 0);
        println!("Compilation within wasmtime took {:?}", start.elapsed());

        let start = Instant::now();
        compile_native();
        println!("Compilation natively took {:?}", start.elapsed());
    }

    fn compile_native() {
        let wat = std::fs::read("input.wat").unwrap();
        let engine = Engine::default();
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
