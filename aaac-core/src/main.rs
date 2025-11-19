use libloading::{Library, Symbol};
use aaac_interface::{DetectionResult, ModuleCreator};
use std::path::Path;
use std::{thread, time};

fn main() {
    // 1. Path to compiled module
    let lib_path = Path::new("../target/debug/libaaac_dummymodule.so");

    println!("Core: Looking for module at: {:?}", std::env::current_dir().unwrap().join(lib_path));

    unsafe {
        // 2. Load the dynamic library
        let lib = Library::new(lib_path).expect("Failed to load library (.so file)");

        // 3. Find the 'create' function we exposed in the module
        let func: Symbol<ModuleCreator> = lib.get(b"create").expect("Failed to find 'create' symbol");

        // 4. Call the function to get our ACModule
        let mut module = func();

        // 5. Run the Lifecycle
        module.load();

        // Simple loop to simulate the game running
        for _ in 0..10 {
            match module.execute() {
                DetectionResult::Clean => println!("Status: Clean"),
                DetectionResult::Suspicious(msg) => println!("WARNING: {}", msg),
                DetectionResult::ConfirmedCheat(msg) => println!("BAN: {}", msg),
            }
            thread::sleep(time::Duration::from_millis(500));
        }

        module.unload();
    } // 'lib' is dropped here, and the module is unloaded from memory
}