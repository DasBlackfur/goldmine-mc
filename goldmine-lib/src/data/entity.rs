use super::Vec3;

pub struct Entity {
    pub id: u32,
    pub entity_type: u32,
    pub pos: Vec3,
    pub rot: Vec3,
}
