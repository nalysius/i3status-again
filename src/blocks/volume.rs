//! The blocks::volume module implements the volume block.

use crate::bar::BlockOutput;
use crate::blocks::Block;
use crate::config::VolumeConfig;
use crate::os::volume::*;

pub struct VolumeBlock {
    /// The format string to use.
    format: String,
    /// The format string to use when sound is muted.
    format_muted: String,
    /// The volume context. See the OpenBSD implementation for a useful
    /// example.
    ctx: VolumeCtx,
}

impl VolumeBlock {
    pub fn from_config(config: &VolumeConfig) -> Self {
        VolumeBlock {
            format: config.format.to_string(),
            format_muted: config.format_muted.to_string(),
            ctx: volume_ctx_new(),
        }
    }
}

impl Block for VolumeBlock {
    fn get_output(&mut self) -> BlockOutput {
        // If the context became invalid, recreate it
        if !is_volume_ctx_valid(&self.ctx) {
            self.ctx = volume_ctx_new();
        }
        let (vol_percentage_s, is_muted): (String, bool) = match get_volume(&mut self.ctx) {
            Ok((v_p, i_m)) => (format!("{}", v_p), i_m),
            Err(e) => (e.to_string(), false),
        };

        let format: &str = if is_muted {
            &self.format_muted
        } else {
            &self.format
        };

        let out = format.replace("{vol_percent}", &vol_percentage_s);

        let block = BlockOutput::new(&format!("{}", out));
        block
    }
}
