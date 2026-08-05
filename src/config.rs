//! The config module handles the configuration.

use crate::backends::*;
use serde::Deserialize;
use std::error;
use std::fs;
use toml;

/// Lists the different types of blocks that can be found in the configuration.
#[derive(Debug, Deserialize)]
#[serde(tag = "block", rename_all = "lowercase")]
pub enum BlockConfig {
	DateTime(DateTimeConfig),
}

/// The configuration for the "datetime" block.
#[derive(Debug, Deserialize)]
pub struct DateTimeConfig {
	pub format: String,
}

/// The global configuration structure
#[derive(Debug, Deserialize)]
pub struct Config {
    pub blocks: Vec<BlockConfig>,
}

impl Config {
	pub fn to_backends(&self) -> Vec<BackendType> {
		let mut backends = Vec::new();
		for block in &self.blocks {
			match block {
				BlockConfig::DateTime(d) => {
					let dt_backend = DateTimeBackend::from_config(&d);
					backends.push(BackendType::DateTime(dt_backend));
				},
			}
		}
		backends
	}
}

/// Read a TOML configuration file.
///
/// # Errors
///
/// This function can return errors in the following situations:
/// 1. path cannot be read. The file doesn't exist or isn't readable.
/// 2. the TOML is invalid or doesn't match the Config struct.
pub fn load_config(path: &str) -> Result<Config, Box<dyn error::Error>> {
	let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
