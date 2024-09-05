use std::fs;

const TIME_FILE_PATH : &str = "./time.clockwatcher";
const SETTINGS_FILE_PATH : &str = "./settings.clockwatcher";

pub fn shoud_write_time() -> bool {
    if fs::metadata(TIME_FILE_PATH).is_ok() {
        // We have no file at all, of course we can write
        return true;
    }

    return false;
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
