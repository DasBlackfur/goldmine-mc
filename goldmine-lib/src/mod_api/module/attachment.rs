use crate::error::{LuaInitModuleSnafu, Result};
use mlua::{Lua, Table};
use snafu::ResultExt;

pub fn attachment_module(lua: &Lua) -> Result<Table> {
    lua.create_table().context(LuaInitModuleSnafu)
}
