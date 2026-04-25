use std::error::Error;
use std::thread;
use std::time::Duration;

use interception::{Device, Interception, KeyState, ScanCode, Stroke};
use spdlog::info;

use crate::config::Config;
use crate::rng::NormalInRange as _;

pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms as _));
}

pub fn find_keyboard() -> Option<i32> {
    (0..20).find(|&dev| interception::is_keyboard(dev))
}

pub fn is_extended(scan_code: u16) -> bool {
    scan_code >> 8 == 0xE0
}

pub fn press_key(
    config: &Config,
    rng: &mut fastrand::Rng,
    interception: &Interception,
    keyboard: Device,
    scan_code: u16,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // FIXME: Pause, Break are unsupported
    let extended_flag = if is_extended(scan_code) {
        KeyState::E0
    } else {
        KeyState::empty()
    };

    let code = ScanCode::try_from(scan_code)?;

    let stroke_down = Stroke::Keyboard {
        code,
        state: KeyState::DOWN | extended_flag,
        information: 0,
    };
    let stroke_up = Stroke::Keyboard {
        code,
        state: KeyState::UP | extended_flag,
        information: 0,
    };

    let press = rng.norm_rand(config.hold_duration());
    info!("Pressing for {press}ms...");

    interception.send(keyboard, &[stroke_down]);
    sleep(press);
    interception.send(keyboard, &[stroke_up]);

    Ok(())
}
