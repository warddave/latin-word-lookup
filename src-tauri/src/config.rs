use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { api_key: None }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let config_dir = app
            .path()
            .app_config_dir()
            .map_err(|e| format!("Failed to get config directory: {}", e))?;

        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;

        let config_path = config_dir.join("config.json");

        Ok(Self { config_path })
    }

    pub fn load(&self) -> Config {
        match fs::read_to_string(&self.config_path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self, config: &Config) -> Result<(), String> {
        let json = serde_json::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&self.config_path, json).map_err(|e| format!("Failed to write config: {}", e))?;

        Ok(())
    }

    pub fn get_api_key(&self) -> Option<String> {
        self.load().api_key
    }

    pub fn set_api_key(&self, api_key: String) -> Result<(), String> {
        let mut config = self.load();
        config.api_key = if api_key.is_empty() {
            None
        } else {
            Some(api_key)
        };
        self.save(&config)
    }

    pub fn has_api_key(&self) -> bool {
        self.load().api_key.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config_manager() -> (ConfigManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let manager = ConfigManager {
            config_path: config_path.clone(),
        };
        (manager, temp_dir)
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_save_and_load_config() {
        let (manager, _temp_dir) = create_test_config_manager();

        let config = Config {
            api_key: Some("test-key".to_string()),
        };

        manager.save(&config).unwrap();
        let loaded = manager.load();

        assert_eq!(loaded.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_set_and_get_api_key() {
        let (manager, _temp_dir) = create_test_config_manager();

        assert!(!manager.has_api_key());
        assert!(manager.get_api_key().is_none());

        manager.set_api_key("test-api-key".to_string()).unwrap();

        assert!(manager.has_api_key());
        assert_eq!(manager.get_api_key(), Some("test-api-key".to_string()));
    }

    #[test]
    fn test_empty_api_key_removes_it() {
        let (manager, _temp_dir) = create_test_config_manager();

        manager.set_api_key("test-key".to_string()).unwrap();
        assert!(manager.has_api_key());

        manager.set_api_key("".to_string()).unwrap();
        assert!(!manager.has_api_key());
    }
}
