use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
fn siparis_olustur() -> String {
    let zaman = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("BREW-{:06X}", zaman % 0xFF_FFFF)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![siparis_olustur])
        .run(tauri::generate_context!())
        .expect("error while running Brewly");
}