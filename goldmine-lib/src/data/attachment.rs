use std::collections::HashMap;

use bimap::BiMap;
use many_to_many::ManyToMany;
use serde_json::Value;

pub struct AttachmentStorage {
    attachment_type_id_to_name: BiMap<u32, String>,

    entity_to_type: ManyToMany<u32, u32>,
    entity_type_to_type: ManyToMany<u32, u32>,
    block_to_type: ManyToMany<(u32, u8, u32), u32>,
    block_type_to_type: ManyToMany<(u8, u8), u32>,

    entity_to_id: BiMap<(u32, u32), u32>,
    entity_type_to_id: BiMap<(u32, u32), u32>,
    block_to_id: BiMap<((u32, u8, u32), u32), u32>,
    block_type_to_id: BiMap<((u8, u8), u32), u32>,

    attachments: HashMap<u32, Value>,
}

impl AttachmentStorage {
    pub fn get_attachment_name_by_id(&self, attachment_type: u32) -> Option<&str> {
        self.attachment_type_id_to_name
            .get_by_left(&attachment_type)
            .map(|x| x.as_str())
    }

    pub fn get_attachment_id_by_name(&self, attachment_type: &str) -> Option<u32> {
        self.attachment_type_id_to_name
            .get_by_right(attachment_type)
            .copied()
    }

    pub fn get_attachment_types_by_block(&self, block_pos: (u32, u8, u32)) -> Vec<u32> {
        self.block_to_type.get_left(&block_pos).unwrap_or_default()
    }

    pub fn get_attachment_types_by_block_type(&self, block_id: u8, block_aux: u8) -> Vec<u32> {
        self.block_type_to_type
            .get_left(&(block_id, block_aux))
            .unwrap_or_default()
    }

    pub fn get_attachment_types_by_entity(&self, entity_id: u32) -> Vec<u32> {
        self.entity_to_type.get_left(&entity_id).unwrap_or_default()
    }

    pub fn get_attachment_types_by_entity_type(&self, entity_type: u32) -> Vec<u32> {
        self.entity_type_to_type
            .get_left(&entity_type)
            .unwrap_or_default()
    }

    pub fn get_attachment_by_block(
        &self,
        block_pos: (u32, u8, u32),
        attachment_type: u32,
    ) -> Option<u32> {
        self.block_to_id
            .get_by_left(&(block_pos, attachment_type))
            .copied()
    }

    pub fn get_attachment_by_block_type(
        &self,
        block_id: u8,
        block_aux: u8,
        attachment_type: u32,
    ) -> Option<u32> {
        self.block_type_to_id
            .get_by_left(&((block_id, block_aux), attachment_type))
            .copied()
    }

    pub fn get_attachment_by_entity(&self, entity_id: u32, attachment_type: u32) -> Option<u32> {
        self.entity_to_id
            .get_by_left(&(entity_id, attachment_type))
            .copied()
    }

    pub fn get_attachment_by_entity_type(
        &self,
        entity_type: u32,
        attachment_type: u32,
    ) -> Option<u32> {
        self.entity_type_to_id
            .get_by_left(&(entity_type, attachment_type))
            .copied()
    }

    pub fn get_attachment_data(&self, attachment_id: u32) -> &Value {
        self.attachments.get(&attachment_id).unwrap_or(&Value::Null)
    }

    pub fn set_attachment_data(&mut self, attachment_id: u32, value: Value) {
        self.attachments.insert(attachment_id, value);
    }

    pub fn get_blocks(&self, attachment_type: u32) -> Vec<(u32, u8, u32)> {
        self.block_to_type
            .get_right(&attachment_type)
            .unwrap_or_default()
    }

    pub fn get_blocks_in_chunk(
        &self,
        attachment_type: u32,
        chunk_x: u32,
        chunk_z: u32,
    ) -> Vec<(u32, u8, u32)> {
        let mut pos = self
            .block_to_type
            .get_right(&attachment_type)
            .unwrap_or_default();
        pos.retain(|pos| pos.0 / 16 == chunk_x && pos.2 / 16 == chunk_z);
        pos
    }

    pub fn get_entities(&self, attachment_type: u32) -> Vec<u32> {
        self.entity_to_type
            .get_right(&attachment_type)
            .unwrap_or_default()
    }
}
