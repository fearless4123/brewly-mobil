use std::time::{SystemTime, UNIX_EPOCH};

#[tauri::command]
fn siparis_olustur() -> String {
    let zaman = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("BREW-{:06X}", zaman % 0xFF_FFFF)
}

mod backend;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| backend::commands::setup(app))
        .invoke_handler(tauri::generate_handler![
            siparis_olustur,
            backend::commands::auth_register,
            backend::commands::auth_login,
            backend::commands::auth_logout,
            backend::commands::auth_me,
            backend::commands::user_index,
            backend::commands::user_store,
            backend::commands::user_sync_roles,
            backend::commands::user_sync_permissions,
            backend::commands::user_set_status,
            backend::commands::role_index,
            backend::commands::role_store,
            backend::commands::role_update,
            backend::commands::role_destroy,
            backend::commands::role_sync_permissions,
            backend::commands::permission_index,
            backend::commands::permission_generate,
            backend::commands::crud_resources,
            backend::commands::crud_index,
            backend::commands::crud_show,
            backend::commands::crud_store,
            backend::commands::crud_update,
            backend::commands::crud_destroy,
            backend::commands::crud_destroy_many,
            backend::commands::crud_aggregate,
            backend::commands::db_migrate_status,
            backend::commands::contact_submit,
            backend::domain::urun_ara,
            backend::domain::sepet_toplami,
            backend::domain::siparis_ver,
            backend::domain::siparis_ilerlet,
            backend::domain::siparis_iptal,
            backend::domain::urun_fiyat_ayarla,
            backend::domain::brewly_dashboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running Brewly");
}