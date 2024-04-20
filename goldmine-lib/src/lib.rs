use anyhow::{Ok, Result};
use data::ServerData;
use mlua::Lua;
use modded::{install_modded_require, module::goldmine_module};
use packets::Packet;
use registry::Registries;
use tokio::{sync::watch, task::JoinSet};

pub mod blocks;
pub mod data;
pub mod modded;
pub mod packets;
pub mod registry;
pub mod tasks;

pub struct Server {
    data: ServerData,
    lua: Lua,
    registries: Registries,
}

impl Server {
    pub fn new() -> Result<Server> {
        let lua = Lua::new();
        let mut registries = Registries::default();

        registries
            .api_registry
            .register("goldmine", goldmine_module(&lua)?);

        install_modded_require(&lua, &registries)?;

        let server = Server {
            data: ServerData::default(),
            lua,
            registries,
        };

        Ok(server)
    }

    pub async fn execute(&mut self) -> Result<()> {
        let mut server_threads = JoinSet::new();
        let (pl_tx, mut pl_rx) = watch::channel(Packet::NOOP);
        server_threads.spawn(tasks::packet_listener::packet_listener(pl_tx));

        server_threads.join_next().await.map(|result| result?);
        Ok(())
    }
}
