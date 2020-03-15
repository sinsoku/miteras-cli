use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub org: String,
    pub username: String,
    pub password: String,
}

fn config_path() -> PathBuf {
    home_dir().unwrap().join(".config/miteras.toml")
}

impl Config {
    pub fn new(org: String, username: String, password: String) -> Config {
        Config {
            org: org,
            username: username,
            password: password,
        }
    }

    pub fn load() -> Option<Config> {
        let config_path = config_path();
        let content: String = fs::read_to_string(config_path).unwrap();
        let config = toml::from_str(&content).ok()?;

        Some(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = config_path();
        let toml = toml::to_string(&self).unwrap();
        let mut file = File::create(config_path)?;
        write!(file, "{}", toml)?;
        file.flush()?;

        Ok(())
    }
}
