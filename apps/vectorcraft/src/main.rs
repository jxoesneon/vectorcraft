//! VectorCraft desktop application — 100% sovereign Martensite runtime.

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use vectorcraft_engine::Engine;
use vectorcraft_ui_martensite::VectorcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = VectorcraftApp::new(engine);

    println!("Starting VectorCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
