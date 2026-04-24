use std::{error::Error, time::SystemTime};

use fastrand::Rng;
use interception::Interception;
use spdlog::{error, info};

use auto_press_rs::utils::{find_keyboard, press_key, sleep};
use auto_press_rs::{config::Config, rng::NormalInRange};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config @ Config { scan_code, .. } = argh::from_env();
    let start = SystemTime::now();

    let Some(interception) = Interception::new() else {
        error!("Driver initialization failed!");
        return Ok(());
    };

    let mut rng = fastrand::Rng::new();

    let Some(keyboard) = find_keyboard() else {
        error!("Keyboard device not found!");
        return Ok(());
    };

    info!("Keyboard device: {keyboard}");
    let wait_delay = |rng: &mut Rng| {
        let wait = rng.norm_rand(config.press_delay());
        info!("Waiting for {wait}ms...");
        sleep(wait);
    };

    loop {
        let Ok(elapsed) = start.elapsed() else {
            break Ok(());
        };

        if config.run_duration != 0 && elapsed.as_secs() / 60 >= config.run_duration {
            info!("Quitting...");
            break Ok(());
        }

        wait_delay(&mut rng);
        press_key(&config, &mut rng, &interception, keyboard, scan_code)?;
    }
}
