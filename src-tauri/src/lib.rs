mod clockwatcher;
#[cfg(desktop)]
mod tray;

#[tauri::command]
fn run_startup() -> String {
    if !clockwatcher::settings_file_exists() {
        clockwatcher::write_settings_file();
    }

    if clockwatcher::should_write_time() {
        clockwatcher::write_current_time();
    }

    let mut time : i64 = 0;
    if let Some(file_time) = clockwatcher::read_time_from_file() {
        time = file_time.timestamp_millis() + (30600000);
    }

    if time == 0 {
        return "Invalid".to_string();
    }

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
