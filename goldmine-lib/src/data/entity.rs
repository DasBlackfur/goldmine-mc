use std::collections::HashMap;

use bimap::BiMap;

use super::Vec3;

pub type Entities = HashMap<u32, Entity>;

pub struct Entity {
    pub id: u32,
    pub entity_type: u32,
    pub pos: Vec3,
    pub rot: Vec3,
}

pub type EntityAttachment = BiMap<u32, u32>;