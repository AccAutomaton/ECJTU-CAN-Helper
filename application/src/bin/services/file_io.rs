use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub username: String,
    pub password: String,
    pub operator_mode: i32,
}

const SETTINGS_PATH: &str = ".\\settings.json";

fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

pub fn read_or_new() -> Result<Settings, Box<dyn std::error::Error>> {
    if !file_exists(SETTINGS_PATH) {
        let default_settings = Settings {
            username: String::from(""),
            password: String::from(""),
            operator_mode: 0,
        };
        let default_json = serde_json::to_string(&default_settings)?;
        fs::write(SETTINGS_PATH, default_json)?;
    }

    let file_content: String = fs::read_to_string(SETTINGS_PATH)?;
    let settings: Settings = serde_json::from_str(&file_content)?;

    Ok(settings)
}

pub fn write_or_new(settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    let modified_json = serde_json::to_string(&settings)?;
    fs::write(SETTINGS_PATH, modified_json)?;
    Ok(())
}
