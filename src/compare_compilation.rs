use std::time::Instant;

use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::{Dir, WasiCtxBuilder};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let compilation_wasm_path = &args[1];
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let wasi = WasiCtxBuilder::new()
        // .inherit_stdio()
        .preopened_dir(Dir::from_std_file(std::fs::File::open(".").unwrap()), ".")
        .unwrap()
        .build();
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
}
