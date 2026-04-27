use std::error::Error;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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

    let prog_start = Instant::now();

    let lua = if config.debug {
        unsafe { Lua::unsafe_new() }
    } else {
        Lua::new()
    };
    lua.globals().set("input_driver", driver)?;

    let time_utc = lua.create_function(|_lua, ()| {
        Ok(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64)
    })?;
    let time_mono = lua.create_function(move |_lua, ()| {
        Ok(prog_start.elapsed().as_millis() as i64) // Relative
    })?;

    lua.globals().set("time_utc", time_utc)?;
    lua.globals().set("time_mono", time_mono)?;

    info!(
        "Press duration: {}ms ~ {}ms",
        config.min_hold_duration, config.max_hold_duration
    );
    let chunk = lua.load(lua_script);
    chunk.exec().inspect_err(|e| error!("{e}"))?;

    Ok(())
}
