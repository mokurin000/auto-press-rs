use std::time::Duration;

use fastrand::Rng;
use interception::{Device, Interception};
use spdlog::info;

use crate::config::Config;
use crate::devices::enum_devices;
use crate::rng::NormalInRange;
use crate::utils::{find_keyboard, find_mouse, get_device_hwid, keyboard_send};

pub struct Controller {
    driver: Interception,
    rng: Rng,
    press_min_ms: u32,
    press_max_ms: u32,

    keyboards: Vec<Device>,
    mouses: Vec<Device>,
    selected_keyboard: Device,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interception initialization failed")]
    InterceptionInitFailed,
    #[error("Invalid scan 1 make code found")]
    InvalidScanCode,
    #[error("Keyboard device not found")]
    KeyboardNotFound,
    #[error("Mouse device not found")]
    MouseNotFound,
    #[error("Win32 Error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

impl Controller {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let rng = fastrand::Rng::new();
        let driver = Interception::new().ok_or(Error::InterceptionInitFailed)?;

        let keyboards = find_keyboard(&driver);
        let mouses = find_mouse(&driver);

        Ok(Self {
            press_min_ms: config.min_hold_duration,
            press_max_ms: config.max_hold_duration,
            selected_keyboard: keyboards[0],
            driver,
            rng,
            keyboards,
            mouses,
        })
    }

    /// send a Scan 1 Make code
    pub fn press_key(&mut self, scan_code: u16) -> Result<(), Error> {
        keyboard_send(
            &mut self.rng,
            &self.driver,
            self.selected_keyboard,
            scan_code,
            self.press_min_ms..=self.press_max_ms,
        )
        .map_err(|_| Error::InvalidScanCode)
    }

    pub fn normal_dist_delay(&mut self, min_ms: u32, max_ms: u32) {
        let ms = self.rng.norm_rand(min_ms..=max_ms);

        info!("Waiting for {:.2}s", ms as f64 / 1000.);
        std::thread::sleep(Duration::from_millis(ms as _));
    }

    pub fn log_devices(&self) -> Result<(), Error> {
        let win_devices = enum_devices()?;

        let keyboards = &self.keyboards;
        let mouses = &self.mouses;

        for (group, group_name) in [(keyboards, "Keyboard"), (mouses, "Mouse")] {
            info!("Listing {group_name}...");
            for &device in group {
                let hwids = get_device_hwid(&self.driver, device).unwrap();
                let Some(devinfo) = win_devices.get(&hwids) else {
                    continue;
                };

                info!(
                    "\\\\.\\interception{device:02}: {} - {}",
                    devinfo.vendor_name(),
                    devinfo.device_name()
                );
            }
        }

        Ok(())
    }
}
