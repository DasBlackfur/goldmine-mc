use crate::error::{LuaInitModuleSnafu, Result};
use mlua::{Lua, Table};
use snafu::ResultExt;

mod attachment;
mod entity;
mod packet;
mod world;

pub fn goldmine_module(lua: &Lua) -> Result<Table> {
    let goldmine_module = lua.create_table().context(LuaInitModuleSnafu)?;

    goldmine_module
        .set("attachment", attachment::attachment_module(lua)?)
        .context(LuaInitModuleSnafu)?;
    goldmine_module
        .set("entity", entity::entity_module(lua)?)
        .context(LuaInitModuleSnafu)?;
    goldmine_module
        .set("packet", packet::packet_module(lua)?)
        .context(LuaInitModuleSnafu)?;
    goldmine_module
        .set("world", world::world_module(lua)?)
        .context(LuaInitModuleSnafu)?;

    Ok(goldmine_module)
}
