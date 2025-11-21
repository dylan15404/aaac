use aaac_interface::{DetectionResult, ModuleCreator}; // ACModule removed if unused
use libloading::{Library, Symbol};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::{thread, time};

// Structs to parse the config file
#[derive(Deserialize)]
struct Config {
    system: SystemConfig,
    modules: Vec<ModuleConfig>,
}

#[derive(Deserialize)]
struct SystemConfig {
    update_ms: u64,
}

#[derive(Deserialize)]
struct ModuleConfig {
    name: String,
    filename: String,
    enabled: bool,
}

// Wrapper to keep the Library loaded alongside the Module
struct ActiveModule {
    instance: Box<dyn aaac_interface::ACModule>, // We use the full path here if import is removed
    _lib: Library,
}

fn main() {
    // 1. Load Configuration
    // use "./config.toml" assuming you run from the workspace root
    let config_str = fs::read_to_string("./config.toml")
        .expect("Could not read config.toml. Are you running from the root folder?");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config.toml");

    let mut loaded_modules = Vec::new();
    let lib_base_path = Path::new("target/debug/"); // Location of compiled modules

    println!("--- AAAC Core Starting ---");

    // 2. Dynamic Loading Loop
    for mod_conf in config.modules {
        if !mod_conf.enabled {
            println!("Skipping disabled module: {}", mod_conf.name);
            continue;
        }

        let lib_path = lib_base_path.join(&mod_conf.filename);
        println!("Loading [{}] from {:?}", mod_conf.name, lib_path);

        unsafe {
            let lib = Library::new(&lib_path).unwrap_or_else(|e| {
                panic!("Failed to load {}: {}", mod_conf.name, e);
            });

            let func: Symbol<ModuleCreator> = lib.get(b"create").expect("No 'create' function found!");

            let mut instance = func();
            instance.load();

            loaded_modules.push(ActiveModule {
                instance,
                _lib: lib,
            });
        }
    }

    // 3. Main Protection Loop
    println!("\n--- System Active (Press Ctrl+C to stop) ---");
    for _ in 0..10 {
        for module in &mut loaded_modules {
            match module.instance.execute() {
                DetectionResult::Clean => { /* Do nothing */ },
                DetectionResult::Suspicious(msg) => println!("[WARN] {}: {}", module.instance.name(), msg),
                DetectionResult::ConfirmedCheat(msg) => println!("[BAN] {}: {}", module.instance.name(), msg),
            }
        }
        thread::sleep(time::Duration::from_millis(config.system.update_ms));
    }

    // 4. Shutdown
    println!("--- Shutting Down ---");
    for mut module in loaded_modules {
        module.instance.unload();
    }
}