use std::collections::HashMap;
use std::fs;

const SETTINGS_FILE_PATH: &str = "./settings.clockwatcher";

pub fn settings_file_exists() -> bool {
    return fs::metadata(SETTINGS_FILE_PATH).is_ok();
}

pub fn write_setting(property: &str, value: &str) {
    // Create the file if it doesn't exist
    if !settings_file_exists() {
        fs::write(SETTINGS_FILE_PATH, "")
            .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());
    }

    // Read the file so we can filter it
    let contents = fs::read_to_string(SETTINGS_FILE_PATH)
        .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());

    // Split the file into a dictionary of properties and values
    let mut properties = contents
        .lines()
        .map(|line| {
            (
                String::from(line.split(": ").collect::<Vec<&str>>()[0]),
                String::from(line.split(": ").collect::<Vec<&str>>()[1]),
            )
        })
        .collect::<HashMap<_, _>>();

    // Upsert our new value
    properties.insert(String::from(property), String::from(value));

    // Cobble our properties into a string we can write to the file
    let mut write_values = "".to_string();
    for property in properties {
        write_values.push_str(format!("{0}: {1}", property.0, property.1).as_str());
    }

    // Overwrite the file with our new data
    fs::write(SETTINGS_FILE_PATH, write_values)
        .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());
}

pub fn read_setting(property: &str) -> Option<String> {
    if !settings_file_exists() {
        return None;
    }

    // Read the file so we can filter it
    let contents = fs::read_to_string(SETTINGS_FILE_PATH)
        .expect(format!("Unable to write file: {0}", SETTINGS_FILE_PATH).as_str());

    // Split the file into a dictionary of properties and values
    let properties = contents
        .lines()
        .map(|line| {
            (
                String::from(line.split(": ").collect::<Vec<&str>>()[0]),
                String::from(line.split(": ").collect::<Vec<&str>>()[1]),
            )
        })
        .collect::<HashMap<_, _>>();

    // If we have the property, return a copy of that
    if properties.contains_key(property) {
        return properties.get(property).cloned();
    }

    // We haven't found anything
    return None;
}
