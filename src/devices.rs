use ahash::AHashMap;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    DIGCF_ALLCLASSES, DIGCF_PRESENT, HDEVINFO, SETUP_DI_REGISTRY_PROPERTY, SP_DEVINFO_DATA,
    SPDRP_DEVICEDESC, SPDRP_FRIENDLYNAME, SPDRP_HARDWAREID, SPDRP_MFG,
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW,
    SetupDiGetDeviceInstanceIdW, SetupDiGetDeviceRegistryPropertyW,
};
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows::Win32::System::Registry::{REG_MULTI_SZ, REG_SZ, REG_VALUE_TYPE};
use windows::core::Error;

use crate::devices::vendor::guess_vendor;

pub mod vendor;

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub instance_id: String,

    pub manufacturer: String,
    pub friendly_name: String,
    pub device_desc: String,

    pub vendor_name: Option<String>,
    pub device_name: Option<String>,
}

impl DeviceInfo {
    pub fn vendor_name(&self) -> &str {
        [
            self.vendor_name.as_deref().unwrap_or(""),
            &self.manufacturer,
        ]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("Unknown")
    }

    pub fn device_name(&self) -> &str {
        [
            self.device_name.as_deref().unwrap_or(""),
            &self.friendly_name,
            &self.device_desc,
        ]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("Unknown")
    }
}

/// Enumerates all currently present devices in the system.
pub fn enum_devices() -> Result<AHashMap<String, DeviceInfo>, Error> {
    unsafe {
        let h_dev_info = SetupDiGetClassDevsW(
            None, // NULL = all device classes
            None,
            None,
            DIGCF_PRESENT | DIGCF_ALLCLASSES,
        )?;

        if h_dev_info == HDEVINFO(INVALID_HANDLE_VALUE.0 as _) {
            return Err(Error::from_thread());
        }

        let mut devices = AHashMap::new();
        let mut dev_info_data = SP_DEVINFO_DATA {
            cbSize: std::mem::size_of::<SP_DEVINFO_DATA>() as u32,
            ..Default::default()
        };

        let mut index = 0u32;

        while SetupDiEnumDeviceInfo(h_dev_info, index, &mut dev_info_data).is_ok() {
            let instance_id = get_device_instance_id(h_dev_info, &dev_info_data)?;

            let friendly_name =
                get_device_property(h_dev_info, &dev_info_data, SPDRP_FRIENDLYNAME)?;
            let device_desc = get_device_property(h_dev_info, &dev_info_data, SPDRP_DEVICEDESC)?;

            let manufacturer = get_device_property(h_dev_info, &dev_info_data, SPDRP_MFG)?;

            let hardware_ids = get_device_property(h_dev_info, &dev_info_data, SPDRP_HARDWAREID)
                .unwrap_or_default();

            let (vendor_name, dev_name) = guess_vendor(&hardware_ids);
            let vendor_name = vendor_name.map(str::to_string);
            let device_name = dev_name.map(str::to_string);

            devices.insert(
                hardware_ids,
                DeviceInfo {
                    instance_id,
                    manufacturer,
                    friendly_name,
                    device_desc,
                    vendor_name,
                    device_name,
                },
            );

            index += 1;
        }

        let _ = SetupDiDestroyDeviceInfoList(h_dev_info);
        Ok(devices)
    }
}

/// Retrieves the unique device instance ID (e.g. "PCI\VEN_8086&DEV_...").
unsafe fn get_device_instance_id(
    h_dev_info: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Result<String, Error> {
    unsafe {
        let mut buffer = vec![0u16; 512];
        let mut required_size = 0u32;

        SetupDiGetDeviceInstanceIdW(
            h_dev_info,
            dev_info_data,
            Some(&mut buffer),
            Some(&mut required_size),
        )?;

        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
        Ok(String::from_utf16_lossy(&buffer[..len]))
    }
}

/// Retrieves a device registry property and converts it to a human-readable string.
///
/// Handles both REG_SZ (single string) and REG_MULTI_SZ (multiple null-terminated strings).
unsafe fn get_device_property(
    h_dev_info: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
    property: SETUP_DI_REGISTRY_PROPERTY,
) -> Result<String, Error> {
    unsafe {
        let mut data_type = 0u32;
        let mut required_size = 0u32;

        // First call: get required buffer size
        let _ = SetupDiGetDeviceRegistryPropertyW(
            h_dev_info,
            dev_info_data,
            property,
            Some(&mut data_type),
            None,
            Some(&mut required_size),
        );

        if required_size == 0 {
            return Ok(String::new());
        }

        let mut buffer = vec![0u8; required_size as usize];

        SetupDiGetDeviceRegistryPropertyW(
            h_dev_info,
            dev_info_data,
            property,
            Some(&mut data_type),
            Some(&mut buffer),
            Some(&mut required_size),
        )?;

        match REG_VALUE_TYPE(data_type) {
            REG_MULTI_SZ => {
                let wide = std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u16,
                    required_size as usize / 2,
                );
                let text = String::from_utf16_lossy(wide);
                // Join multiple strings with newlines, skipping empty ones
                let parts: Vec<&str> = text.split('\0').filter(|s| !s.is_empty()).collect();
                Ok(parts.join("\n"))
            }
            REG_SZ => {
                let wide = std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u16,
                    required_size as usize / 2,
                );
                let text = String::from_utf16_lossy(wide);
                Ok(text.trim_end_matches('\0').to_string())
            }
            _ => Ok(String::new()), // Unknown type
        }
    }
}
