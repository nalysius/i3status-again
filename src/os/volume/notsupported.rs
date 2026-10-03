//! The os::volume::notsupported module is the default empty implementation
//! that is used on unsupported operating systems.

use crate::os::volume::VolumeError;

pub type VolumeCtx = ();

pub fn volume_ctx_new() -> VolumeCtx {
    ()
}

pub fn is_volume_ctx_valid(ctx_opt: &VolumeCtx) -> bool {
    true
}

pub fn get_volume(ctx: &mut VolumeCtx) -> Result<(u8, bool), VolumeError> {
    Err(VolumeError::OsNotSupported)
}
