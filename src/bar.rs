//! The bar module handles the i3bar protocol.

use serde::Serialize;

/// Represents the output of a block.
/// It is returned by backends and used to generate the
/// JSON for i3bar.
#[derive(Debug, Serialize)]
pub struct BlockOutput {
	full_text: String,
}

impl BlockOutput {
	pub fn new(full_text: &str) -> Self {
		Self {
			full_text: full_text.to_string(),
		}
	}
}
