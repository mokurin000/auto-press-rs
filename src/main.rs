use std::error::Error;
use std::time::Instant;

use auto_press_rs::controller::Controller;

use auto_press_rs::config::Config;
use spdlog::{error, info};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config: Config = argh::from_env();

    let mut driver = Controller::new(&config).inspect_err(|e| error!("{e}"))?;
    driver.list_devices()?;

    let start_time = Instant::now();

    loop {
        if let Some(run_duration) = config.run_duration
            && start_time.elapsed().as_secs() >= run_duration
        {
            info!("Quitting gracefully...");
            break Ok(());
        }

        driver.normal_delay(config.min_interval, config.max_interval);
        driver.press_key(config.scan_code)?;
    }
}
