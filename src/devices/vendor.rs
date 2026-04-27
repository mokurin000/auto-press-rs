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

    #[cfg(feature = "device-info")]
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
