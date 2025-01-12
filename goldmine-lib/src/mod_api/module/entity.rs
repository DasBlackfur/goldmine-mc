use mlua::{Lua, Table};
use snafu::ResultExt;

use crate::error::{LuaInitModuleSnafu, Result};

pub fn entity_module(lua: &Lua) -> Result<Table> {
    lua.create_table().context(LuaInitModuleSnafu)
}
