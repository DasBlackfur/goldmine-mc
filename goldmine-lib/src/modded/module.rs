use std::{cell::RefCell, sync::Arc};

use anyhow::Result;
use mlua::{
    Lua, RegistryKey, Table,
    Value::{self, Nil},
};
use parking_lot::Mutex;

use crate::registry::Registries;

pub fn goldmine_module(
    lua: &Lua,
    registries: Arc<Mutex<RefCell<Registries>>>,
) -> Result<RegistryKey> {
    let gm_module = lua.create_table()?;

    let registry_handle = registries.clone();
    let register_mod = lua.create_function(move |lua, lua_mod: Table| {
        let mod_name: String = lua_mod.get("name")?;
        let new_mod = lua.create_registry_value(lua_mod)?;
        registry_handle
            .lock()
            .borrow_mut()
            .lm_registry
            .register(&mod_name, new_mod);

        std::result::Result::Ok(())
    })?;
    gm_module.set("register_mod", register_mod)?;

    gm_module.set("registry", registry_module(lua, registries.clone())?)?;

    Ok(lua.create_registry_value(gm_module)?)
}

fn registry_module(lua: &Lua, registries: Arc<Mutex<RefCell<Registries>>>) -> Result<Table> {
    let registry_module = lua.create_table()?;

    registry_module.set(
        "pl_registry",
        registry_functions(lua, registries.clone(), "pl_registry".to_owned())?,
    )?;
    registry_module.set(
        "lm_registry",
        registry_functions(lua, registries.clone(), "lm_registry".to_owned())?,
    )?;
    registry_module.set(
        "api_registry",
        registry_functions(lua, registries.clone(), "api_registry".to_owned())?,
    )?;

    Ok(registry_module)
}

fn registry_functions(
    lua: &Lua,
    registries: Arc<Mutex<RefCell<Registries>>>,
    registry_name: String,
) -> Result<Table<'_>> {
    let registry_table = lua.create_table()?;

    let registry_name_internal = registry_name.clone();
    let registry_handle = registries.clone();
    let register_func = lua.create_function(move |lua, args: (String, Value)| {
        let registries_lock = registry_handle.lock();
        let mut registries_handle = registries_lock.borrow_mut();
        let registry = match registry_name_internal.as_str() {
            "pl_registry" => Some(&mut registries_handle.pl_registry),
            "lm_registry" => Some(&mut registries_handle.lm_registry),
            "api_registry" => Some(&mut registries_handle.api_registry),
            _ => None,
        }
        .unwrap();
        let lua_key = lua.create_registry_value(args.1)?;
        registry.register(&args.0, lua_key);
        Ok(())
    })?;

    let registry_name_internal = registry_name.clone();
    let registry_handle = registries.clone();
    let get_func = lua.create_function(move |lua, arg: String| {
        let registries_lock = registry_handle.lock();
        let registries_handle = registries_lock.borrow();
        let registry = match registry_name_internal.as_str() {
            "pl_registry" => Some(&registries_handle.pl_registry),
            "lm_registry" => Some(&registries_handle.lm_registry),
            "api_registry" => Some(&registries_handle.api_registry),
            _ => None,
        }
        .unwrap();
        match registry.get(&arg) {
            Ok(value) => Ok(lua.registry_value(value)?),
            Err(_) => Ok(Nil),
        }
    })?;

    let registry_handle = registries.clone();
    let values_func = lua.create_function(move |lua, ()| {
        let registries_lock = registry_handle.lock();
        let registries_handle = registries_lock.borrow_mut();
        let registry = match registry_name.as_str() {
            "pl_registry" => Some(&registries_handle.pl_registry),
            "lm_registry" => Some(&registries_handle.lm_registry),
            "api_registry" => Some(&registries_handle.api_registry),
            _ => None,
        }
        .unwrap();
        Ok(registry
            .values()
            .map(|key| lua.registry_value(key).unwrap())
            .collect::<Vec<Value>>())
    })?;

    registry_table.set("register", register_func)?;
    registry_table.set("get", get_func)?;
    registry_table.set("values", values_func)?;

    Ok(registry_table)
}
