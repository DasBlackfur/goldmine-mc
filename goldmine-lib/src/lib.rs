use std::sync::Arc;

use data::ServerData;

pub mod data;
pub mod error;
pub mod mod_api;
pub mod protocol;
pub mod task;

pub struct Server {
    pub data: Arc<ServerData>,
}
