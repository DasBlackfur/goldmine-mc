use declio::{ctx, Decode, Encode};
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
}
