mod commands;
mod gerber;

pub use gerber::{ObfuscateOptions, ProcessRequest, ProcessResult};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::process_gerber,
            commands::get_default_options,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
