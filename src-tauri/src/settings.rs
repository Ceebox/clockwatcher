use std::fs;

const SETTINGS_FILE_PATH: &str = "./settings.clockwatcher";

pub fn settings_file_exists() -> bool {
    return fs::metadata(SETTINGS_FILE_PATH).is_ok();
}

pub fn write_settings_file(duration: i32) {
    fs::write(
        SETTINGS_FILE_PATH,
        "Milliseconds: ".to_owned() + &duration.to_string(),
    )
    .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());
}

pub fn read_settings_file() -> String {
    let mut contents = fs::read_to_string(SETTINGS_FILE_PATH)
        .expect(format!("Unable to read file {0}", SETTINGS_FILE_PATH).as_str());

    contents = contents.replace("Milliseconds: ", "");

    return contents;
}
