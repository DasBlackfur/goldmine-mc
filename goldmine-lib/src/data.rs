use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ServerData {
    pub seed: u32,
    pub gamemode: u32,
    pub entities: Vec<EntityData>,
    pub inventories: HashMap<u32, Inventory>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityData {
    pub id: u32,
    pub pos: Vec3,
    pub rot: Vec3,
}

pub type Vec3 = (f32, f32, f32);

#[derive(Serialize, Deserialize, Default)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub count: u32,
}
