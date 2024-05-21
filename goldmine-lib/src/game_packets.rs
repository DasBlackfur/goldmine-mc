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
    #[declio(id = "0x87")]
    SCStartGame {
        #[declio(ctx = "ctx::Endian::Big")]
        seed: u32,
        unknown: [u8; 4],
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
}
