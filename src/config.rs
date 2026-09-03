//! The config module handles the configuration.

use crate::blocks::*;
use crate::common::TempUnit;
use serde::Deserialize;
use std::error;
use std::fs;
use toml;

/// Lists the different types of blocks that can be found in the configuration.
/// In the configuration, the field "block" identifies the type of block
/// (e.g.: "datetime") and the fields of the corresponding configuration fields
/// are there, no need of a [[blocks.datetime]] section or anything. Example:
///
/// [[blocks]]
/// block = "datetime"
/// format = "%Y-%m-%d %H:%M"
///
/// The fields format comes directly from the DateTimeConfig.
#[derive(Debug, Deserialize)]
#[serde(tag = "block", rename_all = "lowercase")]
pub enum BlockConfig {
    DateTime(DateTimeConfig),
    Battery(BatteryConfig),
    #[serde(rename = "cpu_temp")]
    CpuTemp(CpuTempConfig),
}

/// The configuration for the "datetime" block.
#[derive(Debug, Deserialize)]
pub struct DateTimeConfig {
    /// The string used to format the date time.
    /// See the crate chrono for the placeholders. Example: %Y-%m-%d %H:%M.
    pub format: String,
}

/// The configuration for the "battery" block.
#[derive(Debug, Deserialize)]
pub struct BatteryConfig {
    /// The string used to format the battery display.
    /// Supports placeholders:
    /// - {rem_percent} the percentage of remaining energy, like 42. Doesn't
    ///   contain the percent character.
    /// - {rem_time} the estimated remaining time, like 02:42.
    /// - {chr_state} the charging state of the battery, either CHR if charging
    ///   or BAT otherwise.
    pub format: String,
    /// The identifier of the battery to monitor, starting from 0.
    /// If None, all the batteries are monitored and displayed as one.
    pub index: Option<u8>,
}

/// The configuration for the "cpu_temp" block.
#[derive(Debug, Deserialize)]
pub struct CpuTempConfig {
    /// The string used to format the CPU temperature display.
    /// Supports placeholders:
    /// - {temp} the temperature without the unit.
    /// - {unit} the unit of the temperature, like °C or °F.
    pub format: String,
    /// The identifier of the CPU to monitor, starting from 0.
    /// If None, all the CPUs are monitored and displayed as one.
    pub index: Option<u8>,
    /// The unit of the temperature.
    /// Default to Celsius.
    #[serde(default)]
    pub unit: TempUnit,
}

/// The global configuration structure.
///
/// It's a list of [[blocks]] section with BlockConfig in each. See BlockConfig
/// and docs/config.toml for an example of configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub blocks: Vec<BlockConfig>,
}

impl Config {
    /// Convert the configuration to a list of BlockType.
    pub fn to_blocks(&self) -> Vec<BlockType> {
        let mut blocks = Vec::new();
        for block in &self.blocks {
            match block {
                BlockConfig::Battery(b) => {
                    let bt_block = BatteryBlock::from_config(&b);
                    blocks.push(BlockType::Battery(bt_block));
                }
                BlockConfig::CpuTemp(c) => {
                    let ct_block = CpuTempBlock::from_config(&c);
                    blocks.push(BlockType::CpuTemp(ct_block));
                }
                BlockConfig::DateTime(d) => {
                    let dt_block = DateTimeBlock::from_config(&d);
                    blocks.push(BlockType::DateTime(dt_block));
                }
            }
        }
        blocks
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
