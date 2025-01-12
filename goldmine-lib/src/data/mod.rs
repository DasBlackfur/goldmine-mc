use std::collections::HashMap;

use attachment::AttachmentStorage;
use chunk::Chunk;
use entity::Entity;
use inventory::Inventory;
use parking_lot::RwLock;
use state::ServerState;

pub mod attachment;
pub mod chunk;
pub mod entity;
pub mod inventory;
pub mod state;

pub type Vec3 = (f32, f32, f32);

pub struct ServerData {
    pub attachments: RwLock<AttachmentStorage>,
    pub chunk: RwLock<HashMap<(u32, u32), Chunk>>,
    pub entity: RwLock<HashMap<u32, Entity>>,
    pub inventory: RwLock<HashMap<u32, Inventory>>,
    pub state: RwLock<ServerState>,
}
