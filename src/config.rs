#![allow(unused)]

use std::{
    fs,
    path::PathBuf,
};

use anyhow::{
    Context,
    Result,
    bail,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
struct Config {}

impl Config {
    fn path() -> Result<PathBuf> {
        match dirs::config_local_dir() {
            Some(directory) => Ok(directory.join("smallen").join("config.json")),
            None => bail!("Failed to determine config directory"),
        }
    }

    fn load() -> Result<Option<Self>> {
        let config_path = Self::path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;

        let config: Self = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse config file at {:?}", config_path))?;

        Ok(Some(config))
    }

    fn save(&self) -> Result<()> {
        let config_path = Self::path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory at {:?}", parent))?;
        }

        let temp_path = config_path.with_extension("json.tmp");
        let contents = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&temp_path, &contents)
            .with_context(|| format!("Failed to write temp config file at {:?}", temp_path))?;

        fs::rename(&temp_path, &config_path)
            .with_context(|| format!("Failed to move config into place at {:?}", config_path))?;

        Ok(())
    }
}
