//! This example shows how to read output sound level using sndio.

use i3status_again::sensors::sndio::*;
use std::{thread, time};

fn main() {
    let mut ctx = SndioCtx::new().unwrap();

    loop {
        if ctx.is_dead() {
            ctx = SndioCtx::new().unwrap();
        }
        ctx.poll();
        if let Some((vol_cur, vol_max, is_muted)) = ctx.volume() {
            println!("Volume: {} / {} (muted: {})", vol_cur, vol_max, is_muted);
        } else {
            println!("Couldn't read volume");
        }
        thread::sleep(time::Duration::from_millis(1000))
    }
}
