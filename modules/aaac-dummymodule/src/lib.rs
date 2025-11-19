// File: ac-module-dummy/src/lib.rs

use aaac_interface::{ACModule, DetectionResult};

struct DummyModule {
    // module variables here (e.g., last_scan_time, file_paths)
    counter: u32,
}

impl ACModule for DummyModule {
    fn name(&self) -> &str {
        "Dummy Module"
    }

    fn load(&mut self) {
        println!("[{}] Loaded!", self.name());
    }

    fn execute(&mut self) -> DetectionResult {
        self.counter += 1;

        // detection logic goes here
        if self.counter > 5 {
            return DetectionResult::Suspicious("Counter is too high!".to_string());
        }

        DetectionResult::Clean
    }

    fn unload(&mut self) {
        println!("[{}] Unloading...", self.name());
    }
}

// --- THE BOILERPLATE EXPORT ---
// This specific block is required in EVERY module file.
// It allows the AC Core to find the entry point.

#[no_mangle]
pub extern "C" fn create() -> Box<dyn ACModule> {
    Box::new(DummyModule { counter: 0 })
}