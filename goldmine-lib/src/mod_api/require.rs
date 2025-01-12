use mlua::{Function, Lua, Table};
use snafu::ResultExt;

use crate::error::{LuaInitRequireSnafu, Result};

pub fn install_modded_require(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let lua_require: Function = globals.get("require").context(LuaInitRequireSnafu)?;

    let require = lua
        .create_function(move |lua, name: String| {
            if name == "@goldmine" {
                Ok(lua.globals().get::<Table>("goldmine"))
            } else if name.starts_with("@goldmine") {
                Ok(lua
                    .globals()
                    .get::<Table>("goldmine")?
                    .get(name.split("/").last()))
            } else {
                Ok(lua_require.call::<Table>(name))
            }
        })
        .context(LuaInitRequireSnafu)?;

    globals.set("require", require).context(LuaInitRequireSnafu)
}
