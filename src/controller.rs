use std::time::Duration;

use fastrand::Rng;
use interception::{Device, Interception};
use spdlog::info;

use crate::Error;
use crate::config::Config;
use crate::devices::enum_devices;
use crate::rng::NormalInRange;
use crate::utils::{
    MouseButton, find_keyboard, find_mouse, get_device_hwid, keyboard_send, mouse_send,
};

mod lua_interop;

pub struct Controller {
    driver: Interception,
    rng: Rng,
    press_min_ms: u32,
    press_max_ms: u32,

    keyboards: Vec<Device>,
    mouses: Vec<Device>,
    selected_keyboard: Device,
    selected_mouse: Device,
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
            selected_mouse: mouses[0],
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

    /// send a Mouse Button
    pub fn press_mouse(&mut self, button: MouseButton) {
        mouse_send(
            &mut self.rng,
            &self.driver,
            self.selected_mouse,
            button,
            self.press_min_ms..=self.press_max_ms,
        )
    }

    pub fn normal_dist_delay(&mut self, min_ms: u32, max_ms: u32) {
        let ms = self.rng.norm_rand(min_ms..=max_ms);

        info!("Waiting for {:.2}s", ms as f64 / 1000.);
        std::thread::sleep(Duration::from_millis(ms as _));
    }

    pub fn log_devices(&self) -> Result<(), Error> {
        info!("Scanning devices...");

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
