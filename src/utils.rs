use std::error::Error;
use std::ffi::OsString;
use std::num::NonZero;
use std::thread;
use std::time::Duration;

use interception::{Device, Interception, KeyState, ScanCode, Stroke};
use spdlog::info;

use crate::config::Config;
use crate::rng::NormalInRange as _;

pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms as _));
}

pub fn find_keyboard(interception: &Interception) -> Vec<i32> {
    (1..=10)
        .filter(|&dev| interception::is_keyboard(dev))
        .filter(|&dev| get_device_hwid(interception, dev).is_some())
        .collect()
}

pub fn find_mouse(interception: &Interception) -> Vec<i32> {
    (11..=20)
        .filter(|&dev| interception::is_mouse(dev))
        .filter(|&dev| get_device_hwid(interception, dev).is_some())
        .collect()
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

/// - prefix: `VEN_`, `VID_`, `DEV_`, e.g.
fn extract_id<'a>(hardware_id: &'a str, prefix: &str) -> Option<&'a str> {
    hardware_id
        .find(prefix)
        .map(|start| start + prefix.len())
        .map(|start| &hardware_id[start..start + 4])
}

/// parses hex string without 0x prefix, but sometimes ends with &
pub fn parse_hex_id(hex: &str) -> Option<NonZero<u16>> {
    let hex = hex.split_once('&').map(|(left, _)| left).unwrap_or(hex);
    u16::from_str_radix(hex, 16)
        .ok()
        .and_then(|num| NonZero::new(num))
}

#[allow(unused)]
pub fn guess_vendor(hardware_id: &str) -> (Option<&str>, Option<&str>) {
    let ven = extract_id(hardware_id, "VEN_");
    let dev = extract_id(hardware_id, "DEV_");
    let vid = extract_id(hardware_id, "VID_");
    let pid = extract_id(hardware_id, "PID_");

    if let Some(ven) = ven {
        match ven {
            "DLLK" | "DELL" => return (Some("Dell"), None),
            _ => {
                #[cfg(feature = "pci-ids")]
                {
                    use pci_ids::FromId as _;

                    if let Some(ven_id) = parse_hex_id(ven)
                        && let Some(vendor) = pci_ids::Vendor::from_id(ven_id.into())
                    {
                        if let Some(dev) = dev
                            && let Some(dev_id) = parse_hex_id(dev)
                            && let Some(device) =
                                pci_ids::Device::from_vid_pid(ven_id.into(), dev_id.into())
                        {
                            return (Some(vendor.name()), Some(device.name()));
                        } else {
                            return (Some(vendor.name()), None);
                        }
                    }
                }
            }
        }
    }

    #[cfg(feature = "usb-ids")]
    {
        use usb_ids::FromId as _;

        if let Some(vid) = vid
            && let Some(vid) = parse_hex_id(vid)
            && let Some(vendor) = usb_ids::Vendor::from_id(vid.into())
        {
            if let Some(pid) = pid
                && let Some(pid) = parse_hex_id(pid)
                && let Some(device) = usb_ids::Device::from_vid_pid(vid.into(), pid.into())
            {
                return (Some(vendor.name()), Some(device.name()));
            } else {
                return (Some(vendor.name()), None);
            }
        }
    }

    (None, None)
}
