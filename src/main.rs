use std::error::Error;

use auto_press_rs::controller::Controller;

use auto_press_rs::config::Config;
use mlua::Lua;
use spdlog::{error, info};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config: Config = argh::from_env();

    let driver = Controller::new(&config).inspect_err(|e| error!("{e}"))?;

    let lua_script = config.lua_script;
    if !lua_script.exists() {
        error!("lua_script not existing");
        return Ok(());
    } else if !lua_script.is_file() {
        error!("lua_script must be a file");
        return Ok(());
    }

    let lua = Lua::new();
    lua.globals().set("input_driver", driver)?;

    info!(
        "Press duration: {}ms ~ {}ms",
        config.min_hold_duration, config.max_hold_duration
    );
    let chunk = lua.load(lua_script);
    chunk.exec().inspect_err(|e| error!("{e}"))?;

    Ok(())
}
