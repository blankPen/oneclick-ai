mod tool;
mod runtime;
mod commands;

pub use tool::*;
pub use commands::*;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_tools,
            check_claude_code_cmd,
            install_claude_code_cmd,
            uninstall_claude_code_cmd,
            update_claude_code_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
