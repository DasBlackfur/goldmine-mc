use std::{cell::RefCell, sync::Arc};

use anyhow::Result;
use mlua::{Function, Lua, RegistryKey, Table};
use parking_lot::Mutex;

use crate::registry::Registries;

pub type LuaModValue = RegistryKey;

pub mod module;

pub struct Mod {}

pub fn install_modded_require(lua: &Lua, registries: Arc<Mutex<RefCell<Registries>>>) -> Result<()> {
    let globals = lua.globals();
    let require_key = lua.create_registry_value(globals.get::<_, Function>("require")?)?;
    let gm_module_key = lua.registry_value(registries.lock().borrow().api_registry.get("goldmine")?)?;

    let rust_require = lua.create_function(move |lua, name: String| {
        if name == "goldmine" {
            Ok(lua.registry_value::<Table>(&gm_module_key)?)
        } else {
            let lua_require: Function = lua.registry_value(&require_key)?;
            Ok(lua_require.call::<_, Table>(name)?)
        }
    })?;

    globals.set("require", rust_require)?;

    Ok(())
}
