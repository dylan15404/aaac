// File: ac-interface/src/lib.rs

#[derive(Debug)]
pub enum DetectionResult {
    Clean,
    Suspicious(String),
    ConfirmedCheat(String),
}

// The trait that all modules must implement
pub trait ACModule {
    // Returns the name of the module (e.g., "Memory Scanner")
    fn name(&self) -> &str;

    // Setup logic (runs once when loaded)
    fn load(&mut self);

    // The main logic (runs every loop)
    fn execute(&mut self) -> DetectionResult;

    // Cleanup logic (runs when unloading)
    fn unload(&mut self);
}

// Defines the function signature for creating a module.
// This helps ensures type safety when loading the symbol from the .so file.
#[allow(improper_ctypes_definitions)]
pub type ModuleCreator = unsafe extern "C" fn() -> Box<dyn ACModule>;