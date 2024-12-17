
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
fn greet(app: AppHandle, name: &str) -> String {
    let sidecar_command = app.shell().sidecar("test_binary").unwrap();
    let (mut _rx, mut _child) = sidecar_command.spawn().expect("Failed to spawn sidecar");
    println!("Spawned {}", _child.pid());
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
