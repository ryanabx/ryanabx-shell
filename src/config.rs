use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO: {0}")]
    IO(#[from] io::Error),
    #[error("Serializing: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub app_tray: AppTrayConfig,
}

impl<'a> Default for PanelConfig {
    fn default() -> Self {
        Self {
            app_tray: AppTrayConfig::default(),
        }
    }
}

impl PanelConfig {
    pub fn from_file_or_default(path: &Path) -> Result<Self, ConfigError> {
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let config: PanelConfig = serde_json::from_str(&data).unwrap_or_default();
        Ok(config)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), ConfigError> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppTrayConfig {
    pub favorites: Vec<String>,
}

impl<'a> Default for AppTrayConfig {
    fn default() -> Self {
        Self {
            favorites: vec!["org.mozilla.firefox".to_string()],
        }
    }
}
