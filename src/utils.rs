use std::error::Error;
use std::ffi::OsString;
use std::ops::RangeBounds;
use std::thread;
use std::time::Duration;

use interception::{Device, Interception, KeyState, ScanCode, Stroke};
use spdlog::info;

use crate::rng::NormalInRange as _;

pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms as _));
}

pub fn find_keyboard(interception: &Interception) -> Vec<Device> {
    (1..=10)
        .filter(|&dev| interception::is_keyboard(dev))
        .filter(|&dev| get_device_hwid(interception, dev).is_some())
        .collect()
}

pub fn find_mouse(interception: &Interception) -> Vec<Device> {
    (11..=20)
        .filter(|&dev| interception::is_mouse(dev))
        .filter(|&dev| get_device_hwid(interception, dev).is_some())
        .collect()
}

/// Press key by scan 1 make code
///
/// the normal distributed random hold duration respects config
pub fn keyboard_send(
    rng: &mut fastrand::Rng,
    interception: &Interception,
    keyboard: Device,
    scan_code: u16,
    hold_duration_range: impl RangeBounds<u32>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let extended_flag = if scan_code >> 8 == 0xE0 {
        KeyState::E0
    } else if scan_code >> 8 == 0xE1 {
        KeyState::E1
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

    let press = rng.norm_rand(hold_duration_range);
    info!("Pressing for {press}ms...");

    interception.send(keyboard, &[stroke_down]);
    sleep(press);
    interception.send(keyboard, &[stroke_up]);

    Ok(())
}

pub fn get_device_hwid(interception: &Interception, device: Device) -> Option<String> {
    use std::mem::transmute;
    use std::os::windows::ffi::OsStringExt as _;

    const MAX_HARDWARE_WIDE_LEN: usize = 201;

    let mut buffer = [0_u8; MAX_HARDWARE_WIDE_LEN * size_of::<u16>()];
    let length = interception.get_hardware_id(device, &mut buffer);
    if length == 0 {
        return None;
    }
    let buffer: [u16; MAX_HARDWARE_WIDE_LEN] = unsafe { transmute(buffer) };
    Some(
        OsString::from_wide(&buffer[0..(length as usize / size_of::<u16>()) - 1])
            .to_string_lossy()
            .split('\0')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
    )
}
