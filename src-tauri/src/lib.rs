mod clockwatcher;
#[cfg(desktop)]
mod tray;

#[tauri::command]
fn run_startup() -> String {
    use chrono;

    if !clockwatcher::settings_file_exists() {
        clockwatcher::write_settings_file();
    }

    let time = chrono::Local::now().timestamp_millis() + (30600000);

    return time.to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![run_startup])
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
