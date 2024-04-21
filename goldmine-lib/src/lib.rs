use std::{cell::RefCell, fs::File, io::Read, sync::Arc};

use anyhow::{Ok, Result};
use data::ServerData;
use mlua::Lua;
use modded::{install_modded_require, module::goldmine_module};
use packets::Packet;
use parking_lot::{Mutex};
use registry::Registries;
use tokio::{sync::watch, task::JoinSet};

pub mod blocks;
pub mod data;
pub mod modded;
pub mod packets;
pub mod registry;
pub mod tasks;

#[derive(Clone)]
pub struct Server {
    data: Arc<Mutex<RefCell<ServerData>>>,
    lua: Arc<Mutex<Lua>>,
    registries: Arc<Mutex<RefCell<Registries>>>,
    addr: String,
    server_name: String,
    guid: u64,
}

impl Server {
    pub fn new(addr: &str, mod_path: &str) -> Result<Server> {
        let lua = Lua::new();
        let registries = Arc::new(Mutex::new(RefCell::new(Registries::default())));

        registries
            .lock()
            .borrow_mut()
            .api_registry
            .register("goldmine", goldmine_module(&lua, registries.clone())?);

        install_modded_require(&lua, registries.clone())?;

        let mut mod_string = String::new();
        File::open(mod_path)?.read_to_string(&mut mod_string)?;
        lua.load(mod_string).set_name(mod_path).exec()?;

        let server = Server {
            data: Arc::new(Mutex::new(RefCell::new(ServerData::default()))),
            lua: Arc::new(Mutex::new(lua)),
            registries,
            addr: addr.to_owned(),
            server_name: "A GoldMineMC server!".to_owned(),
            guid: rand::random(),
        };

        Ok(server)
    }

    pub async fn execute(&mut self) -> Result<()> {
        let mut server_threads = JoinSet::new();
        let (pl_tx, _pl_rx) = watch::channel(Packet::NoOP);
        server_threads.spawn(tasks::packet_listener::packet_listener(self.clone(), pl_tx));

        server_threads.join_next().await.map(|result| result?);
        Ok(())
    }
}
