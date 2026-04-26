use std::str::FromStr;

use mlua::{Error, FromLua, UserData, UserDataMethods};
use spdlog::error;

use crate::controller::Controller;
use crate::utils::MouseButton;

impl FromLua for MouseButton {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Number(num) => MouseButton::try_from(num as u32)
                .map_err(|_| Error::runtime("Invalid mouse button number")),
            mlua::Value::Integer(num) => MouseButton::try_from(num as u32)
                .map_err(|_| Error::runtime("Invalid mouse button number")),
            mlua::Value::String(s) => MouseButton::from_str(&s.to_string_lossy())
                .map_err(|e| Error::runtime(e.to_string())),
            _ => Err(Error::runtime(
                "MouseButton only supports numbers and strings",
            )),
        }
    }
}

impl UserData for Controller {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("key_press", |_lua, driver, scan_code: u16| {
            if driver.press_key(scan_code).is_err() {
                return Err(Error::runtime("Bad keyboard scan code"));
            }
            Result::<i32, _>::Ok(0)
        });

        methods.add_method_mut("mouse_press", |_lua, driver, button: MouseButton| {
            driver.press_mouse(button);
            Result::<i32, _>::Ok(0)
        });

        methods.add_method_mut("delay", |_lua, driver, delay_range: (u32, u32)| {
            let (min_ms, max_ms) = delay_range;
            driver.normal_dist_delay(min_ms, max_ms);
            Result::<i32, _>::Ok(0)
        });

        // returns non-zero if scanning failed
        methods.add_method("scan_devices", |_lua, driver, _: ()| {
            if driver
                .log_devices()
                .inspect_err(|e| {
                    error!("failed to scan: {e}");
                })
                .is_err()
            {
                return Result::<i32, _>::Ok(1);
            };
            Result::<i32, _>::Ok(0)
        });
    }

    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_set("keyboard", |_lua, driver, device: i32| {
            driver.selected_keyboard = device;
            Ok(())
        });
        fields.add_field_method_set("mouse", |_lua, driver, device: i32| {
            driver.selected_mouse = device;
            Ok(())
        });
    }
}
