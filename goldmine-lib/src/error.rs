use snafu::{Location, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GoldmineError {
    #[snafu(display("Could not initialize the Lua runtime."))]
    LuaInitError {
        #[snafu(source(from(GoldmineError, Box::new)))]
        source: Box<GoldmineError>,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Could not initialize the custom require function."))]
    LuaInitRequireError {
        source: mlua::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Could not initialize global values."))]
    LuaInitGlobalError {
        source: mlua::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[snafu(display("Could not initialize the Goldmine API."))]
    LuaInitModuleError {
        source: mlua::Error,
        #[snafu(implicit)]
        location: Location,
    },
}

pub type Result<T, E = GoldmineError> = std::result::Result<T, E>;
