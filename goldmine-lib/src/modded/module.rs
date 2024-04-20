use anyhow::{Ok, Result};
use mlua::{Lua, RegistryKey};

pub fn goldmine_module(lua: &Lua) -> Result<RegistryKey> {
    let gm_module = lua.create_table()?;

    Ok(lua.create_registry_value(gm_module)?)
}
