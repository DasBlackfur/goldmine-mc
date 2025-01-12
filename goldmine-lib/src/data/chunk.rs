use bimap::BiMap;

pub struct Chunk {
    pub x: u32,
    pub z: u32,
    pub block_data: Box<[u8; 16*16*128]>,
    pub aux_data: Box<[u8; 16*16*128/2]>
}

pub type BlockAttachment = BiMap<(u32, u32, u32), u32>;