use std::{cell::RefCell, fs::File, io::Read, net::SocketAddr, sync::Arc};

use anyhow::{Ok, Result};
use data::ServerData;
use mlua::Lua;
use modded::{install_modded_require, module::goldmine_module};
use parking_lot::Mutex;
use registry::Registries;
use tokio::sync::watch;

pub mod blocks;
pub mod constants;
pub mod data;
pub mod game_packets;
pub mod modded;
pub mod packets;
pub mod registry;
pub mod tasks;
pub mod u24;

#[derive(Clone)]
pub struct Server {
    data: Arc<Mutex<RefCell<ServerData>>>,
    lua: Arc<Mutex<Lua>>,
    registries: Arc<Mutex<RefCell<Registries>>>,
    addr: SocketAddr,
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
            addr: addr.parse()?,
            server_name: "MCCPP;Demo;A GoldMineMC server!".to_owned(),
            guid: rand::random(),
        };

        Ok(server)
    }

    pub async fn execute(&mut self) -> Result<()> {
        let (pl_tx, _pl_rx) = watch::channel("".to_owned());
        let pl_task =
            tokio::task::spawn(tasks::packet_listener::packet_listener(self.clone(), pl_tx));
        let other_task = tokio::task::spawn(async {});

        tokio::join!(pl_task, other_task).0??;
        Ok(())
    }
}
