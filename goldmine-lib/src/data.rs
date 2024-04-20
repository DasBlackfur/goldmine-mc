use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ServerData {
    entities: Vec<EntityData>,
    inventories: HashMap<u32, Inventory>,
}

#[derive(Serialize, Deserialize)]
pub struct EntityData {
    id: u32,
    pos: Vec3,
    rot: Vec3,
}

pub type Vec3 = (f64, f64, f64);

#[derive(Serialize, Deserialize, Default)]
pub struct Inventory {
    items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: u32,
    count: u32,
}
