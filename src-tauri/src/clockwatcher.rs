use chrono::{DateTime, Duration, Local};
use std::fs;
use tauri::Emitter;

const TIME_FILE_PATH: &str = "./time.clockwatcher";
const SETTINGS_FILE_PATH: &str = "./settings.clockwatcher";

// struct Settings {
//     timer_length: DateTime<Local>,
// }

pub fn should_write_time() -> bool {
    if !fs::metadata(TIME_FILE_PATH).is_ok() {
        // We have no file at all, of course we can write
        return true;
    }

    // We have a file
    let time = read_time_from_file();
    if time == None {
        // No time - reset
        return true;
    }
    // else if let Some(file_time) = time {
    //     // We don't really want to reset instantly
    //     // Work out if we've gone past the original time by a certain margin
    //     // return (Local::now() - file_time) > Duration::hours(1);
    // }

    return false;
}

pub fn read_time_from_file() -> Option<chrono::DateTime<chrono::Local>> {
    let contents = fs::read_to_string(TIME_FILE_PATH)
        .expect(format!("Unable to read file {0}", TIME_FILE_PATH).as_str());

    let time = chrono::NaiveDateTime::parse_from_str(contents.as_str(), "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse DateTime.")
        .and_local_timezone(chrono::Local)
        .earliest()
        .unwrap();

    if contents.len() > 0 {
        return Some(time);
    }

    return None;
}

pub fn write_current_time() {
    use chrono::Local;

    let time = Local::now();
    let formatted_time = time.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("Done");
    fs::write(TIME_FILE_PATH, formatted_time)
        .expect(format!("Unable to write file: {0}", TIME_FILE_PATH).as_str());
}

pub fn settings_file_exists() -> bool {
    return fs::metadata(SETTINGS_FILE_PATH).is_ok();
}

pub fn write_settings_file() {
    fs::write(SETTINGS_FILE_PATH, "Milliseconds: ")
        .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());
}

pub fn change_page<R: tauri::Runtime>(app: &tauri::AppHandle<R>, page: &str) {
    let _ = app.emit("change-page", page.to_owned() + ".html");
}
