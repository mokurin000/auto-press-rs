use std::borrow::Cow;
use std::num::NonZero;

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
    u16::from_str_radix(hex, 16).ok().and_then(NonZero::new)
}

#[allow(unused)]
pub fn guess_vendor<'a, 'b: 'a>(
    hardware_id: &'a str,
    api: &'b hidapi::HidApi,
) -> (Option<Cow<'a, str>>, Option<Cow<'a, str>>) {
    let ven = extract_id(hardware_id, "VEN_");
    let dev = extract_id(hardware_id, "DEV_");
    let vid = extract_id(hardware_id, "VID_");
    let pid = extract_id(hardware_id, "PID_");

    if ven.is_some() {
        return (ven.map(Cow::Borrowed), dev.map(Cow::Borrowed));
    }

    if let Some(vid_s) = vid
        && let Some(vid_num) = parse_hex_id(vid_s)
        && let Some(pid_s) = pid
        && let Some(pid_num) = parse_hex_id(pid_s)
        && let Some(dev) = api
            .device_list()
            .find(|d| d.product_id() == pid_num.into() && d.vendor_id() == vid_num.into())
    {
        let oem = dev.manufacturer_string();
        let dev = dev.product_string();

        return (
            oem.or(vid).map(Cow::Borrowed),
            dev.or(pid).map(Cow::Borrowed),
        );
    }

    (None, None)
}
