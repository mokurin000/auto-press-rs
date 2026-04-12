use std::error::Error;

use interception::{Interception, KeyState, ScanCode, Stroke};
use spdlog::{error, info};

use auto_press_rs::config::Config;
use auto_press_rs::utils::{find_keyboard, is_extended, sleep};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config @ Config { scan_code, .. } = argh::from_env();

    let interception = Interception::new().expect("Initialization failed");
    let mut rng = fastrand::Rng::new();

    let Some(keyboard) = find_keyboard() else {
        error!("Keyboard device not found!");
        return Ok(());
    };

    info!("Keyboard device: {keyboard}");

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

    loop {
        let wait = rng.u32(config.press_delay());
        info!("Waiting for {wait}ms...");
        sleep(wait);

        let press = rng.u32(config.hold_delay());
        info!("Pressing for {press}ms...");

        interception.send(keyboard, &[stroke_down]);
        sleep(press);
        interception.send(keyboard, &[stroke_up]);
    }
}
