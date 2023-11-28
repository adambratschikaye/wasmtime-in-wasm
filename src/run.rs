#[cfg(target_arch = "x86_64")]
fn main() {
    use wasmtime::*;

    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let module = unsafe { Module::deserialize_file(&engine, "serialized_module.cwasm").unwrap() };

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let host_func = Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
        println!("Got {} from WebAssembly", param);
        println!("my host state is: {}", caller.data());
    });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = Instance::new(&mut store, &module, &[host_func.into()]).unwrap();
    let hello = instance
        .get_typed_func::<(), ()>(&mut store, "hello")
        .unwrap();

    // And finally we can call the wasm!
    hello.call(&mut store, ()).unwrap();
}

#[cfg(not(target_arch = "x86_64"))]
fn main() {}
