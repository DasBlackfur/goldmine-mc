use std::{cell::RefCell, sync::Arc};

use anyhow::{Ok, Result};
use mlua::{Lua, RegistryKey};
use parking_lot::Mutex;

use crate::registry::Registries;

pub fn goldmine_module(lua: &Lua, registries: Arc<Mutex<RefCell<Registries>>>) -> Result<RegistryKey> {
    let gm_module = lua.create_table()?;

    Ok(lua.create_registry_value(gm_module)?)
}
