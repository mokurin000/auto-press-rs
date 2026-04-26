use std::error::Error;
use std::time::Instant;

use auto_press_rs::devices::enum_devices;
use fastrand::Rng;
use interception::Interception;
use spdlog::{error, info};

use auto_press_rs::config::Config;
use auto_press_rs::rng::NormalInRange;
use auto_press_rs::utils::{find_keyboard, find_mouse, get_device_hwid, press_key, sleep};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config @ Config { scan_code, .. } = argh::from_env();
    let start = Instant::now();

    let Some(interception) = Interception::new() else {
        error!("Driver initialization failed!");
        return Ok(());
    };

    let mut rng = fastrand::Rng::new();

    info!("Scanning devices...");
    let win_devices = enum_devices()?;

    info!("Listing interception devices...");
    let keyboards = find_keyboard(&interception);
    let mouses = find_mouse(&interception);

    for &keyboard in &keyboards {
        let hwids = get_device_hwid(&interception, keyboard).unwrap();
        let Some(devinfo) = win_devices.get(&hwids) else {
            info!("Keyboard \\\\.\\interception{keyboard:02}: Unknown",);
            continue;
        };
        info!(
            "Keyboard \\\\.\\interception{keyboard:02}: {} - {}",
            devinfo.manufacturer, devinfo.name
        );
    }
    for &mouse in &mouses {
        let hwids = get_device_hwid(&interception, mouse).unwrap();
        let Some(devinfo) = win_devices.get(&hwids) else {
            info!("Mouse \\\\.\\interception{mouse:02}: Unknown",);
            continue;
        };
        info!(
            "Mouse \\\\.\\interception{mouse:02}: {} - {}",
            devinfo.manufacturer, devinfo.name
        );
    }

    let keyboard = keyboards[0];

    let wait_delay = |rng: &mut Rng| {
        let wait = rng.norm_rand(config.press_delay());
        info!("Waiting for {wait}ms...");
        sleep(wait);
    };

    loop {
        let elapsed = start.elapsed();

        if config.run_duration != 0 && elapsed.as_secs() / 60 >= config.run_duration {
            info!("Quitting...");
            break Ok(());
        }

        wait_delay(&mut rng);
        press_key(&config, &mut rng, &interception, keyboard, scan_code)?;
    }
}
