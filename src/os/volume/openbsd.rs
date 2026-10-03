//! The os::volume::notsupported module is the default empty implementation
//! that is used on unsupported operating systems.

use crate::os::volume::VolumeError;
use crate::sensors::sndio::SndioCtx;

pub type VolumeCtx = Box<SndioCtx>;

/// Get a new volume context.
pub fn volume_ctx_new() -> VolumeCtx {
    SndioCtx::new().expect("Cannot initialize sndio context.")
}

/// Tells if the context is still valid or needs to be recreated.
pub fn is_volume_ctx_valid(ctx: &VolumeCtx) -> bool {
    !ctx.is_dead()
}

/// Get the current volume.
/// If Ok(), contains (volume_percentage, is_muted).
pub fn get_volume(ctx: &mut VolumeCtx) -> Result<(u8, bool), VolumeError> {
    ctx.poll();
    if let Some((vol_cur, vol_max, is_muted)) = ctx.volume()
        && vol_max > 0
    {
        return Ok((((vol_cur as f64 / vol_max as f64 * 100.0) as u8), is_muted));
    }
    return Err(VolumeError::InvalidContext);
}
