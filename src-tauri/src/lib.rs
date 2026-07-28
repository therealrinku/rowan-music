mod db;
use crate::db::Music;

#[tauri::command]
fn get_songs() -> Result<Vec<Music>, String> {
    db::get_songs().map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_folder(folder: String) -> Result<Vec<Music>, String> {
    db::scan(&folder).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![scan_folder, get_songs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
