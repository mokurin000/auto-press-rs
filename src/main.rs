use std::error::Error;

use auto_press_rs::controller::Controller;

use auto_press_rs::config::Config;
use mlua::Lua;
use spdlog::error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config: Config = argh::from_env();

    let driver = Controller::new(&config).inspect_err(|e| error!("{e}"))?;
    driver.log_devices()?;

    let lua = Lua::new();
    lua.globals().set("input_driver", driver)?;

    Ok(())
}
