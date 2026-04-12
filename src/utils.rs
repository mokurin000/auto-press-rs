use std::{thread, time::Duration};

pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms as _));
}

pub fn find_keyboard() -> Option<i32> {
    (0..20).find(|&dev| interception::is_keyboard(dev))
}

pub fn is_extended(scan_code: u16) -> bool {
    scan_code >> 8 == 0xE0
}
