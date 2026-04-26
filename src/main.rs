use std::error::Error;
use std::time::Instant;

use auto_press_rs::devices::enum_devices;
use fastrand::Rng;
use interception::Interception;
use spdlog::{error, info};

use auto_press_rs::config::Config;
use auto_press_rs::rng::NormalInRange;
use auto_press_rs::utils::{
    find_keyboard, find_mouse, get_device_hwid, guess_vendor, press_key, sleep,
};

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

    let keyboards = find_keyboard(&interception);
    let mouses = find_mouse(&interception);

    for (group, group_name) in [(&keyboards, "Keyboard"), (&mouses, "Mouse")] {
        info!("Listing {group_name}...");
        for &device in group {
            let hwids = get_device_hwid(&interception, device).unwrap();
            let Some(devinfo) = win_devices.get(&hwids) else {
                continue;
            };

            let (vendor, name) = guess_vendor(&hwids);
            info!(
                "\\\\.\\interception{device:02}: {} - {}",
                vendor.unwrap_or(&devinfo.manufacturer),
                name.unwrap_or(&devinfo.name),
            );
        }
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
