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
    let base_path = if cfg!(test) {
        std::env::current_dir().unwrap().join("tmp")
    } else {
        dirs::home_dir().unwrap().join(".config")
    };
    base_path.join("miteras.toml")
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

#[cfg(test)]
mod tests {
    use super::{config_path, Config};
    use std::fs;

    #[test]
    fn test_config_path() {
        let path = std::env::current_dir().unwrap().join("tmp/miteras.toml");
        assert_eq!(path, config_path());
    }

    #[test]
    fn test_save_and_load() {
        let path = config_path();
        if path.exists() {
            fs::remove_file(path).ok();
        }

        let config = Config::new(
            "A123456".to_string(),
            "sinsoku".to_string(),
            "pass1234".to_string(),
        );

        config.save().ok();
        assert_eq!(true, config_path().exists());

        let loaded = Config::load().unwrap();
        assert_eq!("A123456", loaded.org);
        assert_eq!("sinsoku", loaded.username);
        assert_eq!("pass1234", loaded.password);
    }
}
