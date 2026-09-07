// Copyright (c) 2026 Antonin Nivoche. All rights reserved.

use crate::domain::config::UserConfig;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct ConfigFileRepository;

impl ConfigFileRepository {
    pub fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "livediff", "livediff")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
    }

    pub fn load_config() -> UserConfig {
        let Some(path) = Self::config_path() else {
            return UserConfig::default();
        };

        if !path.exists() {
            return UserConfig::default();
        }

        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => UserConfig::default(),
        }
    }

    pub fn save_config(config: &UserConfig) -> Result<(), String> {
        let Some(path) = Self::config_path() else {
            return Err("Unable to determine user configuration directory".to_string());
        };

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let toml_str = toml::to_string_pretty(config)
            .map_err(|e| format!("Failed to serialize configuration: {}", e))?;

        fs::write(&path, toml_str).map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
