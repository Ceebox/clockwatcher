mod clockwatcher;
mod settings;
#[cfg(desktop)]
mod tray;

#[tauri::command]
fn run_timer_startup() -> String {
    if !settings::settings_file_exists() {
        settings::write_setting("Milliseconds", "0");
        settings::write_setting("Theme", "default");
    }

    let mut duration: i64 = 0;
    if let Some(duration_time) = clockwatcher::read_duration_from_file() {
        duration = duration_time.num_milliseconds();
    }

    if clockwatcher::should_write_time(duration) {
        clockwatcher::write_current_time();
    }

    let mut time: i64 = 0;
    if let Some(file_time) = clockwatcher::read_time_from_file() {
        time = file_time.timestamp_millis() + duration;
    }

    if time == 0 {
        return "Invalid".to_string();
    }

    return time.to_string();
}

#[tauri::command]
fn get_duration() -> String {
    return settings::read_setting("Milliseconds").unwrap();
}

#[tauri::command]
fn get_theme() -> String {
    match settings::read_setting("Theme") {
        Some(theme) => theme,
        None => "default".to_string(),
    }
}

#[tauri::command]
fn write_duration(time: i32) {
    settings::write_setting("Milliseconds", time.to_string().as_str());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            run_timer_startup,
            get_duration,
            write_duration,
            get_theme
        ])
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .on_window_event(|window: &tauri::Window, event: &tauri::WindowEvent| {
            on_window_event(window.clone(), event.clone())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_window_event(window: tauri::Window, event: tauri::WindowEvent) {
    match event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}
