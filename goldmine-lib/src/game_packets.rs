use declio::{ctx, util, Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        HANDSHAKE_COOKIE, HANDSHAKE_DATA, HANDSHAKE_DOUBLE_NULL, HANDSHAKE_FLAGS, HANDSHAKE_UNKNOWN,
    },
    u24::u24,
};

#[derive(Serialize, Deserialize, Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum Encapsulation {
    #[declio(id = "0x00")]
    Simple {
        #[declio(ctx = "ctx::Endian::Big")]
        length: u16,
        #[declio(ctx = "ctx::Len((length/8).into())")]
        game_packet: Vec<u8>,
    },
    #[declio(id = "0x40")]
    ExtendedCount {
        #[declio(ctx = "ctx::Endian::Big")]
        length: u16,
        #[declio(ctx = "ctx::Endian::Little")]
        count: u24,
        #[declio(ctx = "ctx::Len((length/8).into())")]
        game_packet: Vec<u8>,
    },
    #[declio(id = "0x60")]
    ExtendedFull {
        #[declio(ctx = "ctx::Endian::Big")]
        length: u16,
        #[declio(ctx = "ctx::Endian::Little")]
        count: u24,
        unknown: [u8; 4],
        #[declio(ctx = "ctx::Len((length/8).into())")]
        game_packet: Vec<u8>,
    },
}

impl Encapsulation {
    pub fn to_game_packet(self) -> Vec<u8> {
        match self {
            Encapsulation::Simple {
                length: _,
                game_packet,
            } => game_packet,
            Encapsulation::ExtendedCount {
                length: _,
                count: _,
                game_packet,
            } => game_packet,
            Encapsulation::ExtendedFull {
                length: _,
                count: _,
                unknown: _,
                game_packet,
            } => game_packet,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Encode, Decode)]
#[declio(id_type = "u8")]
pub enum GamePacket {
    #[declio(id = "0x00")]
    CSPing {
        #[declio(ctx = "ctx::Endian::Big")]
        ping_id: u64,
    },
    #[declio(id = "0x03")]
    SCPong {
        #[declio(ctx = "ctx::Endian::Big")]
        ping_id: u64,
        #[declio(ctx = "ctx::Endian::Big")]
        pong_id: u64,
    },
    #[declio(id = "0x09")]
    CSClientConnect {
        #[declio(ctx = "ctx::Endian::Big")]
        client_id: u64,
        #[declio(ctx = "ctx::Endian::Big")]
        session: u64,
        unknown: u8,
    },
    #[declio(id = "0x10")]
    SCServerHandshake {
        cookie: HANDSHAKE_COOKIE,
        flags: HANDSHAKE_FLAGS,
        #[declio(ctx = "ctx::Endian::Big")]
        server_port: u16,
        data: HANDSHAKE_DATA,
        unknown1: HANDSHAKE_DOUBLE_NULL,
        #[declio(ctx = "ctx::Endian::Big")]
        session: u64,
        unknown2: HANDSHAKE_UNKNOWN,
    },
    #[declio(id = "0x13")]
    CSClientHandshake {
        // We just skip all of this useless data here since its a pure pain to implement
        dummy: u8,
    },
    #[declio(id = "0x15")]
    CSClientCancelConnect {},
    #[declio(id = "0x82")]
    CSLogin {
        #[declio(ctx = "ctx::Endian::Big")]
        username_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*username_len).into())")]
        username: String,
        #[declio(ctx = "ctx::Endian::Big")]
        proto1: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        proto2: u32,
    },
    #[declio(id = "0x83")]
    SCLoginStatus {
        #[declio(ctx = "ctx::Endian::Big")]
        status: u32,
    },
    #[declio(id = "0x84")]
    CSReady {
        #[declio(ctx = "ctx::Endian::Big")]
        status: u8,
    },
    #[declio(id = "0x85")]
    SCMessage { // TODO: test if MessagePacket works both ways
        #[declio(ctx = "ctx::Endian::Big")]
        message_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*message_len).into())")]
        message: String,
    },
    #[declio(id = "0x86")]
    SCSetTime {
        #[declio(ctx = "ctx::Endian::Big")]
        time: u32,
    },
    #[declio(id = "0x87")]
    SCStartGame {
        #[declio(ctx = "ctx::Endian::Big")]
        seed: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        worldgen_version: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        gamemode: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
    },
    #[declio(id = "0x88")]
    SCAddMob {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_type: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_x: f32,
        metadata: u8, // This is wrong, but there is no metadata implementation yet
    },
    #[declio(id = "0x89")]
    SCAddPlayer {
        #[declio(ctx = "ctx::Endian::Big")]
        client_id: i32,
        #[declio(ctx = "ctx::Endian::Big")]
        username_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*username_len).into())")]
        username: String,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_y: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_x: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        held_item_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        held_item_aux: u32,
        metadata: u8
    },
    #[declio(id = "0x8a")]
    SCRemovePlayer {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        client_id: i32,
    },
    #[declio(id = "0x8c")]
    SCAddEntity {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_type: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        has_motion: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_x: u32, // TODO: Mark as optional
        #[declio(ctx = "ctx::Endian::Big")]
        speed_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_z: u32,
    },
    #[declio(id = "0x8d")]
    SCRemoveEntity {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
    },
    #[declio(id = "0x8e")]
    SCAddItemEntity {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        item_id: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        item_amount: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        item_data: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_x: i8, // Every other implementation/documentation has pitch yaw roll here, but I'm pretty sure all of them are wrong
        #[declio(ctx = "ctx::Endian::Big")]
        speed_y: i8,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_z: i8,
    },
    #[declio(id = "0x8f")]
    SCTakeItemEntity {
        #[declio(ctx = "ctx::Endian::Big")]
        target: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
    },
    #[declio(id = "0x90")]
    SCMoveEntity {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
    },
    #[declio(id = "0x93")]
    SCMoveEntityWithRotation {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_x: f32,
    },
    #[declio(id = "0x94")]
    MovePlayer {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        rot_x: f32,
    },
    #[declio(id = "0x95")]
    PlaceBlock { // TODO: Figure out the direction. The MCPI way of changing blocks is weird
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        block_id: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        block_aux: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        face: u8,
    },
    #[declio(id = "0x96")]
    RemoveBlock {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u8,
    },
    #[declio(id = "0x97")]
    SCUpdateBlock {
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        block_id: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        block_aux: u8,
    },
    #[declio(id = "0x98")]
    SCAddPainting {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        direction: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        title_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*title_len).into())")]
        title: String,
    },
    // ExplodePacket
    #[declio(id = "0x9a")]
    SCLevelEvent {
        #[declio(ctx = "ctx::Endian::Big")]
        event_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        data: u32,
    },
    #[declio(id = "0x9b")]
    SCTileEvent {
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        case1: u32, // TODO: Reverse engineer what this is
        #[declio(ctx = "ctx::Endian::Big")]
        case2: u32,
    },
    #[declio(id = "0x9c")]
    EntityEvent {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        event: u8, // TODO: Reverse engineer what this is
    },
    #[declio(id = "0x9d")]
    CSRequestChunk {
        #[declio(ctx = "ctx::Endian::Big")]
        index_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        index_z: u32,
    },
    #[declio(id = "0x9e")]
    SCChunkDataPacket {
        #[declio(ctx = "ctx::Endian::Big")]
        index_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        index_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        chunk_data: u8, // The format is a bit confusing, probably needs a custom datatype to en/decode
        // Rust doesnt let me compile [u8; 49408], so this is u8 for now
    },
    // PlayerEquipmentPacket
    #[declio(id = "0xa0")]
    PlayerArmorEquipment {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        slot0: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        slot1: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        slot2: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        slot3: u8,
    },
    #[declio(id = "0xa1")]
    Interact {
        #[declio(ctx = "ctx::Endian::Big")]
        action: u8, // TODO: Reverse engineer what this is
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        target: u32,
    },
    // UseItemPacket
    #[declio(id = "0xa3")]
    CSPlayerAction {
        #[declio(ctx = "ctx::Endian::Big")]
        action: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        face: u32, // A byte could have been enough here since there are only 6 faces, damn you mojank!
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
    },
    #[declio(id = "0xa5")]
    SCHurtArmor {
        #[declio(ctx = "ctx::Endian::Big")]
        health: u8,
    },
    #[declio(id = "0xa6")]
    SCSetEntityData {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        metadata: u8
    },
    #[declio(id = "0xa7")]
    SCSetEntityMotion {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_y: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        speed_z: u32,
    },
    #[declio(id = "0xa8")]
    SCSetHealth {
        #[declio(ctx = "ctx::Endian::Big")]
        health: u8,
    },
    #[declio(id = "0xa9")]
    SCSetSpawnPosition {
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u8,
    },
    #[declio(id = "0xaa")]
    Animate {
        #[declio(ctx = "ctx::Endian::Big")]
        action: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
    },
    #[declio(id = "0xab")]
    Respawn {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: f32,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: f32,
    },
    // SendInventoryPacket
    #[declio(id = "0xad")]
    CSDropItem {
        #[declio(ctx = "ctx::Endian::Big")]
        entity_id: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        is_death: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        item_id: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        item_amount: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        item_data: u16,
    },
    #[declio(id = "0xae")]
    SCContainerOpen {
        #[declio(ctx = "ctx::Endian::Big")]
        window_id: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        window_type: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        slot: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        title_len: u16, // TODO: Test if the title actually controls anything
        #[declio(with = "util::utf8", ctx = "ctx::Len((*title_len).into())")]
        title: String,
    },
    #[declio(id = "0xaf")]
    ContainerClose {
        #[declio(ctx = "ctx::Endian::Big")]
        window_id: u8,
    },
    // ContainerSetSlotPacket
    #[declio(id = "0xb1")]
    SCContainerSetData {
        #[declio(ctx = "ctx::Endian::Big")]
        window_id: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        property: u32,
        #[declio(ctx = "ctx::Endian::Big")]
        value: u32,
    },
    // ContainerSetContentPacket
    // ContainerAckPacket
    #[declio(id = "0xb4")]
    CSChat {
        #[declio(ctx = "ctx::Endian::Big")]
        message_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*message_len).into())")]
        message: String,
    },
    #[declio(id = "0xb5")]
    SignUpdate {
        #[declio(ctx = "ctx::Endian::Big")]
        pos_x: u16,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_y: u8,
        #[declio(ctx = "ctx::Endian::Big")]
        pos_z: u16,
        #[declio(ctx = "ctx::Endian::Little")]
        line_1_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*line_1_len).into())")]
        line_1: String,
        #[declio(ctx = "ctx::Endian::Little")]
        line_2_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*line_2_len).into())")]
        line_2: String,
        #[declio(ctx = "ctx::Endian::Little")]
        line_3_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*line_3_len).into())")]
        line_3: String,
        #[declio(ctx = "ctx::Endian::Little")]
        line_4_len: u16,
        #[declio(with = "util::utf8", ctx = "ctx::Len((*line_4_len).into())")]
        line_4: String,
    },
    // AdventureSettingsPacket
}
