//! The bar module handles the i3bar protocol.

use crate::common::Color;
use serde::Serialize;

/// Represents the output of a block.
/// It is returned by backends and used to generate the
/// JSON for i3bar.
#[derive(Debug, Serialize)]
pub struct BlockOutput {
    /// The text to display in the block. Example: "BAT 42% 02:12".
    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Color>,
}

impl BlockOutput {
    pub fn new(full_text: &str) -> Self {
        Self {
            full_text: full_text.to_string(),
            background: None,
        }
    }
}
