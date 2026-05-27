// AdRename Library - Main entry point for Tauri application
// Implements the IPC bridge between Svelte frontend and Rust backend

pub mod file_manager;
pub mod method_engine;
pub mod methods;
pub mod metadata_reader;
pub mod models;
pub mod tag_system;
pub mod undo_manager;

// Tauri Commands - IPC interface layer
pub mod commands;

use commands::file_commands::AppState;
use undo_manager::UndoManager;

/// Get the app data directory for storing persistent data
/// On Windows, this typically resolves to: C:\Users\<user>\AppData\Roaming\com.adrename.app
fn get_app_data_dir() -> std::path::PathBuf {
    // Use a standard app data directory
    let base = dirs::data_dir().unwrap_or_else(|| {
        std::path::PathBuf::from(".")
    });
    base.join("AdRename")
}

/// Initialize and run the Tauri application
/// 
/// This function sets up:
/// - Application state (shared across all commands)
/// - Tauri plugins (shell, filesystem access)
/// - All IPC command handlers
/// - Window configuration
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // For now, use a simplified initialization to avoid icon/dependency issues
    // Full Tauri app will be enabled when UI development starts (Task 11)
    
    #[cfg(not(test))]
    {
        // Initialize undo manager and load persisted history
        let data_dir = get_app_data_dir();
        let undo_mgr = UndoManager::new(&data_dir);
        let loaded_history = undo_mgr.load();
        log::info!("Loaded {} undo entries from disk", loaded_history.len());

        tauri::Builder::default()
            .plugin(tauri_plugin_dialog::init())
            .manage(AppState {
                files: std::sync::Mutex::new(vec![]),
                current_dir: std::sync::Mutex::new(None),
                undo_history: std::sync::Mutex::new(loaded_history),
                undo_manager: undo_mgr,
            })
            .invoke_handler(tauri::generate_handler![
                commands::file_commands::scan_directory,
                commands::file_commands::scan_files,
                commands::file_commands::clear_files,
                commands::file_commands::sort_files,
                commands::preview_commands::preview_rename,
                commands::rename_commands::execute_rename,
                commands::rename_commands::undo_last_rename,
                commands::rename_commands::get_undo_status,
            ])
            .setup(|app| {
                log::info!("AdRename v1.0 initializing...");
                
                #[cfg(debug_assertions)]
                {
                    use tauri::Manager;
                    if let Some(window) = app.get_webview_window("main") {
                        window.open_devtools();
                        log::info!("DevTools opened (debug mode)");
                    }
                }
                
                let _ = app;
                
                log::info!("AdRename ready");
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    
    #[cfg(test)]
    {
        println!("AdRename v1.0 - Test mode (Tauri runtime disabled)");
    }
}
